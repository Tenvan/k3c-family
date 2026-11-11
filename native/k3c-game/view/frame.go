package view

import (
	"iter"

	"k3c/engine/sim"
)

// Frame ist der Spielzustand eines Updates, wie ihn die Darstellung sieht. local baut ihn, game ergänzt die Geräte
// (Unseated, Seat.Lost, Geräte-Cues) und reicht ihn an Listener und Drawer. Im Menü ohne laufendes Spiel ist Stages
// leer, Cues, Clock und Settings sind trotzdem gesetzt. Alles im Frame ist nur zu lesen und gilt bis zum nächsten
// Update (keine Zeiger und Slices darüber hinaus halten: local und game verwenden die Puffer wieder).
type Frame struct {
	Tick      uint64     // Takt der Simulation (30 Hz), steht in der Pause
	Clock     float64    // Sekunden für Animationen; Fenster: Spielzeit der Bilder, Aufnahme-Modus: aus dem Takt
	Paused    bool       // Spiel pausiert (Pause-Menü offen); Simulation steht
	Timescale int        // Zeitraffer der Simulation (1 = normal)
	Stages    []Stage    // Stufen, in denen mindestens ein lokaler Spieler steht, nach Stage.Index
	Seats     []Seat     // Plätze der lokalen Spieler, nach Player; Zelle i zeigt Seats[i]
	Events    []Event    // Ereignisse der Simulation seit dem letzten Frame, je genau einmal
	Cues      []Cue      // Ereignisse außerhalb der Simulation seit dem letzten Frame (Menü, Session, Geräte)
	Settings  Settings   // Einstellungen (Session.SetSettings), auch ohne laufendes Spiel
	Unseated  []DeviceID // verbundene Geräte ohne Platz, Reihenfolge wie Input (Beitritts-Hinweis im HUD)
}

// Stage ist eine Stufe der Insel mit ihrer Welt.
type Stage struct {
	Index int        // Index der Stufe in der Insel (wie room.Seat.Stage)
	Depth int        // Tiefe der Stufe (0 = Oberfläche)
	Level string     // Level der Stufe = World.Biome.ID; Schlüssel für AssetSource und art
	World *sim.World // nur lesen
}

// Seat verbindet einen lokalen Spieler mit Gerät, Monarch und Stufe. Player ist der Slot im Raum und bleibt fest,
// solange der Spieler spielt; die Zelle ergibt sich aus der Lage in Frame.Seats (Cell.Index). Geht Spieler 0, steht
// Spieler 1 allein in Zelle 0 (Cell.Player = 1).
type Seat struct {
	Player  int      // lokaler Spieler (Slot) 0 … MaxPlayers−1, nicht der Zellen-Index
	Device  DeviceID // steuerndes Gerät (für Tastensymbole im HUD)
	Monarch int      // sim.Player.Index des Monarchen
	Stage   int      // Stage.Index der Stufe, in der der Monarch steht
	Lost    bool     // Gerät getrennt: Monarch steht, ein freies Gerät übernimmt den Platz (Verbinden oder Bestätigen)
}

// Event ist ein Ereignis der Simulation (sim.World.Events) mit seiner Stufe.
type Event struct {
	Stage int       // Stage.Index
	Tick  uint64    // Takt, in dem es entstand
	Data  sim.Event // {"type": "night", …}, nur lesen
}

// Type liefert den Typ des Ereignisses ("" ohne Typ).
func (e Event) Type() string {
	t, _ := e.Data["type"].(string)
	return t
}

// CueKind ist die Art eines Ereignisses außerhalb der Simulation.
type CueKind int

const (
	CueMove            CueKind = iota // Auswahl im Menü bewegt
	CueConfirm                        // Auswahl bestätigt
	CueBack                           // zurück
	CueDenied                         // nicht möglich (z. B. Spielstand nicht ladbar)
	CuePause                          // Spiel pausiert
	CueResume                         // Spiel läuft weiter
	CueJoined                         // Spieler beigetreten (Player gesetzt)
	CueLeft                           // Spieler gegangen (Player gesetzt)
	CueSaved                          // Spielstand gespeichert
	CueSaveFailed                     // Speichern fehlgeschlagen (Text: Grund)
	CueError                          // Fehler, z. B. Panic im Takt (Text: Meldung)
	CueDeviceConnected                // Gerät verbunden (Device gesetzt; Player, wenn es seinen Platz wieder hat)
	CueDeviceLost                     // Gerät getrennt (Device gesetzt; Player, wenn es einen Platz hatte)
	CueRebound                        // Spieler steuert jetzt ein anderes Gerät (Player und Device gesetzt)
)

// Cue ist ein Ereignis außerhalb der Simulation, für Ton und Hinweise.
type Cue struct {
	Kind   CueKind
	Player int      // betroffener Spieler, -1 = keiner
	Text   string   // Meldung für Hinweise (deutsch), sonst leer
	Device DeviceID // betroffenes Gerät bei CueDeviceConnected, CueDeviceLost, CueRebound
}

// Stage liefert die Stufe mit Index i.
func (f *Frame) Stage(i int) (*Stage, bool) {
	for k := range f.Stages {
		if f.Stages[k].Index == i {
			return &f.Stages[k], true
		}
	}
	return nil, false
}

// Seat liefert den Platz des lokalen Spielers p.
func (f *Frame) Seat(p int) (Seat, bool) {
	for _, s := range f.Seats {
		if s.Player == p {
			return s, true
		}
	}
	return Seat{}, false
}

// World liefert die Welt, in der der Spieler der Zelle c steht (nil ohne Spieler oder Stufe).
func (f *Frame) World(c Cell) *sim.World {
	s, ok := f.Seat(c.Player)
	if !ok {
		return nil
	}
	st, ok := f.Stage(s.Stage)
	if !ok {
		return nil
	}
	return st.World
}

// Monarch liefert den Monarchen des lokalen Spielers p in seiner Welt (nil, wenn es ihn nicht gibt).
func (f *Frame) Monarch(p int) *sim.Player {
	s, ok := f.Seat(p)
	if !ok {
		return nil
	}
	st, ok := f.Stage(s.Stage)
	if !ok || st.World == nil {
		return nil
	}
	for _, m := range st.World.Players {
		if m != nil && m.Index == s.Monarch {
			return m
		}
	}
	return nil
}

// EventsOf liefert die Ereignisse der Stufe i in Entstehungs-Reihenfolge, ohne Slice anzulegen
// (`for e := range f.EventsOf(i)`).
func (f *Frame) EventsOf(i int) iter.Seq[Event] {
	return func(yield func(Event) bool) {
		for _, e := range f.Events {
			if e.Stage == i && !yield(e) {
				return
			}
		}
	}
}
