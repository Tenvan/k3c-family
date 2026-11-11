// Package local führt das native Spiel im selben Prozess über engine/room (B-793): ein Gerät mit bis zu
// view.MaxPlayers Slots, synchron getaktet über Step aus der Spielschleife (kein Manager.Run, keine Goroutine je
// Raum), Spielstände über engine/store. Je laufendem Spiel gibt es einen eigenen room.Manager mit genau einem Raum;
// stürzt der Takt ab, wird er verworfen, ohne zu speichern.
package local

import (
	"errors"
	"fmt"
	"io"
	"log/slog"
	"os"
	"path/filepath"
	"slices"
	"time"

	"k3c/engine/room"
	"k3c/engine/sim"
	"k3c/engine/store"
	"k3c/native/k3c-game/view"
)

const (
	deviceID = "local" // das eine Gerät des Prozesses im Raum
	roomCode = "LOCL"  // fester Code: ein Raum je Manager, kein Zufall
)

// Config sind die Einstellungen der Session.
type Config struct {
	Dir string           // Ordner der Spielstände; leer = DefaultSavesDir()
	Log *slog.Logger     // nil = verwerfen
	Now func() time.Time // Zeitstempel der Spielstände und Insel-ID (nicht Spiellogik); nil = time.Now
	// Recording (Aufnahme-Modus): Step wartet nach jedem Takt, bis ein in diesem Takt begonnenes Speichern
	// geschrieben ist, damit CueSaved im Frame desselben Takts liegt und das Bild nicht von der Wanduhr abhängt. Im
	// normalen Spiel schreibt der Raum im Hintergrund, ohne den Takt aufzuhalten.
	Recording bool
}

// Session erfüllt view.Session und taktet das Spiel mit Step. Alle Methoden laufen in der Goroutine der Schleife.
type Session struct {
	saves  *store.Saves
	store  *watchedStore
	log    *slog.Logger
	now    func() time.Time
	m      *room.Manager
	r      *room.Room
	p      *peer
	name   string                // Spielstand des laufenden Spiels
	devs   map[int]view.DeviceID // Slot (= lokaler Spieler) → Gerät
	last   []view.DeviceID       // Geräte des letzten Spiels, für Load und Continue
	paused bool
	steps  uint64 // Aufrufe von Step, Grundlage von Frame.Clock
	cues   []view.Cue
	ended  view.Outcome // Ausgang des laufenden bzw. zuletzt beendeten Spiels (Frame.Outcome)

	recording bool // Config.Recording
	inTick    bool // ein Takt läuft (Zählung der Speicher-Anfänge)
	encodes   int  // Speicher-Anfänge im laufenden Takt (Aufrufe von Manager.Now im Takt)

	settings     view.Settings
	settingsPath string
	cmd          map[int]sim.PlayerCommand // Puffer von Command, je Aufruf geleert
	seatBuf      []view.Seat               // Puffer von Frame.Seats
	seatOut      []view.Seat               // Puffer von Seats (getrennt von Frame.Seats, dort setzt game Lost)
}

var _ view.Session = (*Session)(nil)

// DefaultSavesDir ist der Ordner saves/ neben der EXE (Entscheidung B-793); ohne EXE-Pfad saves/ im Arbeitsordner.
func DefaultSavesDir() string {
	exe, err := os.Executable()
	if err != nil {
		return "saves"
	}
	return filepath.Join(filepath.Dir(exe), "saves")
}

// Open legt die Session an und liest die Einstellungen (SettingsFile; fehlt oder kaputt: Standardwerte); der Ordner
// entsteht erst beim ersten Speichern.
func Open(cfg Config) *Session {
	if cfg.Dir == "" {
		cfg.Dir = DefaultSavesDir()
	}
	if cfg.Log == nil {
		cfg.Log = slog.New(slog.NewTextHandler(io.Discard, nil))
	}
	if cfg.Now == nil {
		cfg.Now = time.Now
	}
	saves := &store.Saves{Dir: cfg.Dir, Now: cfg.Now}
	s := &Session{saves: saves, store: &watchedStore{saves: saves}, log: cfg.Log, now: cfg.Now, recording: cfg.Recording,
		settingsPath: filepath.Join(cfg.Dir, SettingsFile), cmd: map[int]sim.PlayerCommand{}}
	var err error
	if s.settings, err = loadSettings(s.settingsPath); err != nil {
		s.log.Warn("🩹 Einstellungen unlesbar, Standardwerte", "datei", s.settingsPath, "err", err)
	}
	return s
}

// manager legt den Manager eines Spiels an: Snapshot liefert die Welt selbst (keine Kopie, kein JSON). Now zählt die
// Aufrufe im Takt: Im Takt fragt der Raum die Uhr nur, wenn er einen Spielstand kodiert (room/saver.go › encode), also
// je begonnenem Speichern einmal (Grundlage von Config.Recording).
func (s *Session) manager() *room.Manager {
	m := room.NewManager(s.store)
	m.Log = s.log
	m.Now = func() time.Time {
		if s.inTick {
			s.encodes++
		}
		return s.now()
	}
	m.NewCode = func() string { return roomCode }
	m.Snapshot = func(w *sim.World, _ int, _ bool) any { return w }
	return m
}

// New legt ein neues Spiel an und startet es; ein laufendes wird vorher gespeichert und beendet.
func (s *Session) New(g view.NewGame) error {
	name := g.Name
	if name == "" {
		name = s.freeName()
	}
	opts := room.Options{Grade: g.Grade, Goal: g.Goal, Defeat: g.Defeat, Seed: g.Seed, World: g.World}
	return s.start(name, true, g.Depth, opts, g.Devices)
}

// Load lädt den Spielstand name mit den Geräten devices (leer: die des letzten Spiels, ohne: Tastatur links) und
// startet ihn.
func (s *Session) Load(name string, devices []view.DeviceID) error {
	devs := devices
	if len(devs) == 0 {
		devs = s.last
	}
	if len(devs) == 0 {
		devs = []view.DeviceID{{Kind: view.DeviceKeyboard}}
	}
	return s.start(name, false, 0, room.Options{}, devs)
}

// Continue lädt den zuletzt gespeicherten ladbaren Spielstand mit den Geräten devices (leer wie bei Load).
func (s *Session) Continue(devices []view.DeviceID) error {
	list, err := s.List()
	if err != nil {
		return err
	}
	for _, e := range list {
		if e.Loadable {
			return s.Load(e.Name, devices)
		}
	}
	return userErr("Kein ladbarer Spielstand vorhanden", nil)
}

// start öffnet den Raum mit den Geräten devices (Slot i = Gerät i); depth ist die Start-Tiefe eines neuen Spiels.
func (s *Session) start(name string, fresh bool, depth int, opts room.Options, devices []view.DeviceID) error {
	if len(devices) == 0 || len(devices) > view.MaxPlayers {
		return userErr(fmt.Sprintf("1 bis %d Geräte nötig", view.MaxPlayers), nil)
	}
	for i, d := range devices {
		if slices.Contains(devices[:i], d) {
			return userErr("Ein Gerät kann nur einen Spieler steuern", nil)
		}
	}
	if err := s.Close(); err != nil {
		s.log.Warn("💥 Altes Spiel nicht sauber beendet", "err", err)
	}
	m, p := s.manager(), newPeer()
	slots := make([]int, len(devices))
	devs := map[int]view.DeviceID{}
	for i, d := range devices {
		slots[i], devs[i] = i, d
	}
	r, err := m.Create(deviceID, p, name, fresh, depth, slots, opts)
	if err != nil {
		return describe(err, name)
	}
	s.m, s.r, s.p, s.name, s.devs, s.paused, s.ended = m, r, p, name, devs, false, view.OutcomeNone
	s.last = slices.Clone(devices)
	s.log.Info("🚀 Spiel gestartet", "save", name, "neu", fresh, "spieler", len(devices))
	return nil
}

// Close speichert und beendet das laufende Spiel.
func (s *Session) Close() error {
	if s.r == nil {
		return nil
	}
	name := s.name
	err := s.m.CloseRoom(name)
	s.reset()
	if err != nil {
		return describe(err, name)
	}
	s.log.Info("🛑 Spiel beendet", "save", name)
	return nil
}

// reset vergisst das laufende Spiel, ohne zu speichern.
func (s *Session) reset() {
	s.m, s.r, s.p, s.name, s.devs, s.paused = nil, nil, nil, "", nil, false
}

// Active meldet, ob ein Spiel läuft (auch pausiert).
func (s *Session) Active() bool { return s.r != nil }

// Save speichert das laufende Spiel sofort (synchron).
func (s *Session) Save() error {
	if s.r == nil {
		return errNoGame
	}
	if _, err := s.r.SaveNow(); err != nil {
		return describe(err, s.name)
	}
	return nil
}

// Pause hält die Simulation an bzw. lässt sie weiterlaufen; die gehaltenen Eingaben verfallen dabei (wie im Raum).
func (s *Session) Pause(on bool) {
	if s.r == nil || s.paused == on {
		return
	}
	if err := s.r.Pause(deviceID, s.p, on); err != nil {
		s.log.Error("💥 Pause abgelehnt", "err", err)
		return
	}
	s.paused = on
	kind := view.CueResume
	if on {
		kind = view.CuePause
	}
	s.cue(kind, -1, "")
	s.log.Info("🚦 Pause", "an", on)
}

// Join nimmt das Gerät d als weiteren Spieler auf; spielt es schon, liefert Join seinen Spieler.
func (s *Session) Join(d view.DeviceID) (int, error) {
	if s.r == nil {
		return 0, errNoGame
	}
	for slot, dv := range s.devs {
		if dv == d {
			return slot, nil
		}
	}
	slot := 0
	for ; slot < view.MaxPlayers; slot++ {
		if _, used := s.devs[slot]; !used {
			break
		}
	}
	if slot == view.MaxPlayers {
		return 0, userErr(fmt.Sprintf("Höchstens %d Spieler", view.MaxPlayers), nil)
	}
	if err := s.r.AddSlot(deviceID, s.p, slot); err != nil {
		return 0, describe(err, s.name)
	}
	s.devs[slot] = d
	s.last = s.devices()
	s.cue(view.CueJoined, slot, "")
	s.log.Info("👑 Spieler kommt", "player", slot, "device", d.String())
	return slot, nil
}

// Leave lässt den Spieler gehen; sein Monarch ist frei. Der letzte Spieler bleibt (Spiel über Close beenden).
func (s *Session) Leave(player int) error {
	if s.r == nil {
		return errNoGame
	}
	if _, ok := s.devs[player]; !ok {
		return userErr(fmt.Sprintf("Spieler %d spielt nicht", player+1), nil)
	}
	if len(s.devs) == 1 {
		return userErr("Der letzte Spieler kann nicht gehen, das Spiel über das Menü beenden", nil)
	}
	if _, err := s.r.RemoveSlot(deviceID, s.p, player); err != nil {
		return describe(err, s.name)
	}
	delete(s.devs, player)
	s.last = s.devices()
	s.cue(view.CueLeft, player, "")
	s.log.Info("👋 Spieler geht", "player", player)
	return nil
}

// devices sind die Geräte der Spieler nach Slot.
func (s *Session) devices() []view.DeviceID {
	out := []view.DeviceID{}
	for slot := range view.MaxPlayers {
		if d, ok := s.devs[slot]; ok {
			out = append(out, d)
		}
	}
	return out
}

func (s *Session) cue(kind view.CueKind, player int, text string) {
	s.cues = append(s.cues, view.Cue{Kind: kind, Player: player, Text: text})
}

// errNoGame: Aufruf ohne laufendes Spiel.
var errNoGame = userErr("Es läuft kein Spiel", nil)

// userError ist ein anzeigbarer Fehler (deutsch); Unwrap liefert den Fehler des Raums oder Stores.
type userError struct {
	msg string
	err error
}

func (e *userError) Error() string { return e.msg }
func (e *userError) Unwrap() error { return e.err }

func userErr(msg string, err error) error { return &userError{msg, err} }

// describe übersetzt Fehler von room und store in eine Meldung für das Menü.
func describe(err error, name string) error {
	var msg string
	switch {
	case errors.Is(err, room.ErrSaveExists):
		msg = fmt.Sprintf("Spielstand „%s“ gibt es schon", name)
	case errors.Is(err, room.ErrSaveNotFound), errors.Is(err, store.ErrNotFound):
		msg = fmt.Sprintf("Spielstand „%s“ gibt es nicht", name)
	case errors.Is(err, room.ErrSaveInUse):
		msg = fmt.Sprintf("Spielstand „%s“ ist gerade offen", name)
	case errors.Is(err, room.ErrBadRequest), errors.Is(err, store.ErrSlot):
		msg = "Ungültiger Name oder ungültige Optionen (Name: a–z, 0–9, -, höchstens 32 Zeichen)"
	case errors.Is(err, room.ErrRoomFull):
		msg = "Kein Monarch mehr frei"
	case errors.Is(err, room.ErrGameOver):
		msg = "Insel verloren, der Spielstand bleibt wie zuletzt gesichert"
	default:
		msg = fmt.Sprintf("Spielstand „%s“: %v", name, err)
	}
	return userErr(msg, err)
}
