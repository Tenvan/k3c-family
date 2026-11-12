package game

import (
	"errors"
	"flag"
	"fmt"
	"io"
	"math"
	"regexp"
	"slices"
	"strconv"
	"strings"

	"k3c/native/k3c-game/view"
)

// Grenzen der Schalter.
const (
	maxShots  = 64
	maxTick   = 30 * 60 * 60 * 4 // vier Stunden Spielzeit
	minWidth  = 320
	maxWidth  = 3840
	defScript = "0@0=R,1@0=LS" // Aufnahme ohne --input: Spieler 1 läuft rechts, Spieler 2 sprintet links
)

// saveName ist ein gültiger Spielstand-Name (wie engine/store).
var saveName = regexp.MustCompile(`^[a-z0-9-]{1,32}$`)

// Hold ist ein Eintrag der festen Eingabe-Folge: Ab Takt Tick (seit Start der Szene) hält Spieler Player den Befehl
// Cmd; mit Press drückt er stattdessen genau in Takt Tick die Menü-Aktion Action (Flanke, der Befehl bleibt).
type Hold struct {
	Tick   int
	Player int
	Cmd    view.Command
	Press  bool
	Action view.Action
}

// Scene ist die gesteuerte Szene: was zu sehen ist, ohne Menü und ohne Zufall von außen.
type Scene struct {
	Seed    string    // neues Spiel mit diesem Seed (leer und ohne Load: Menü)
	Depth   int       // Stufe (Tiefe), in der die Spieler beginnen
	Load    string    // statt Seed: Spielstand aus dem Ordner --saves
	Players int       // 1 oder 2 (Geräte keys1, keys2)
	Cams    []float64 // Kamera-x je Zelle in Units, NaN = folgt dem Monarchen
	Tick    int       // Takte, die die Simulation vor dem ersten Bild vorläuft
	Script  []Hold    // feste Eingabe-Folge; gesetzt: ersetzt Tastatur und Gamepads
	Menu    string    // Aufnahme eines Menüs nach dem Vorlauf: "", main, saves, options, lobby, pause, keys, licenses (menus)
}

// menus sind die Werte von --menu: main, saves, lobby und licenses ohne Spiel (ein Spiel der Szene endet nach dem
// Vorlauf mit Speichern), options und keys über dem Hauptmenü bzw. mit Szene über der Pause, pause über dem
// stehenden Spiel.
var menus = []string{"main", "saves", "options", "lobby", "pause", "keys", "licenses"}

// Args sind die Schalter des Programms.
type Args struct {
	Saves  string // Ordner der Spielstände
	Shots  string // Aufnahme-Modus: Ordner der Bilder; leer = Fenster
	Count  int    // Zahl der Bilder
	Every  int    // Takte zwischen zwei Bildern
	W, H   int    // Größe der Zeichenfläche (Aufnahme) bzw. des Fensters
	Scene  Scene
	Script bool // Eingabe aus Scene.Script statt von den Geräten
	// Flow (--flow): Ablauf ab dem Hauptmenü ohne Szenen-Spiel; die Eingabe-Folge bedient Menüs und Spiel, die Aufnahme
	// läuft auch ohne laufendes Spiel weiter (Ende-zu-Ende, NG9.1).
	Flow bool
	// At (--at): Aufnahme genau nach diesen Takten seit dem Start (aufsteigend); ersetzt --tick, --count und --every.
	At      []int
	Measure Measure // Messmodus (--measure, measure.go)
	Assets  bool    // --assets: Grafik-Ordner aller Level ausgeben und beenden (Paket, assets.go)
	Grafik  string  // --grafik: Grafik-Ordner statt public/grafik (leer = Standard)
	River   string  // --river shader|tiles: Fluss-Fassung in die Einstellungen schreiben
	Stats   bool    // --stats: Leistungsanzeige in die Einstellungen schreiben
	Stall   bool    // --stall: dauerhaft die Ladeanzeige (nur Aufnahme-Modus)
}

// InGame meldet, ob das Menü der Szene über dem laufenden (pausierten) Spiel liegt.
func (s Scene) InGame() bool {
	return s.Menu == "pause" || ((s.Menu == "options" || s.Menu == "keys") && s.Direct())
}

// Direct meldet, ob das Spiel ohne Menü startet.
func (s Scene) Direct() bool { return s.Seed != "" || s.Load != "" }

// Devices sind die Geräte der Spieler der Szene: keys1, dann keys2.
func (s Scene) Devices() []view.DeviceID {
	out := make([]view.DeviceID, s.Players)
	for i := range out {
		out[i] = view.DeviceID{Kind: view.DeviceKeyboard, Index: i}
	}
	return out
}

// Usage nennt die Schalter.
const Usage = "k3c-game [--saves <ordner>] [--seed <s> | --load <name> | --flow] [--depth <n>] [--players 1|2] [--cam <x>[,<x>]] " +
	"[--tick <n>] [--input <p@t=LRSPA1234-|a:aktion,…>] [--menu main|saves|options|lobby|pause|keys|licenses] [--size <B>x<H>] " +
	"[--shots <ordner> [--count <n>] [--every <takte>] [--at <t>,…]] " +
	"[--measure <s> [--warmup <s>] [--report <datei>] [--novsync]] [--assets] " +
	"[--grafik <ordner>] [--river shader|tiles] [--stats] [--stall]"

// rawArgs sind die Schalter, die erst nach dem Einlesen ausgewertet werden.
type rawArgs struct{ cams, input, size, at string }

// newFlagSet bindet alle Schalter an a und r.
func newFlagSet(a *Args, r *rawArgs, defSaves string) *flag.FlagSet {
	fs := flag.NewFlagSet("k3c-game", flag.ContinueOnError)
	fs.SetOutput(io.Discard)
	fs.StringVar(&a.Saves, "saves", defSaves, "")
	fs.StringVar(&a.Shots, "shots", "", "")
	fs.IntVar(&a.Count, "count", 8, "")
	fs.IntVar(&a.Every, "every", 3, "")
	fs.StringVar(&r.size, "size", "1280x720", "")
	fs.StringVar(&a.Scene.Seed, "seed", "", "")
	fs.IntVar(&a.Scene.Depth, "depth", 0, "")
	fs.StringVar(&a.Scene.Load, "load", "", "")
	fs.IntVar(&a.Scene.Players, "players", 2, "")
	fs.StringVar(&r.cams, "cam", "", "")
	fs.IntVar(&a.Scene.Tick, "tick", 0, "")
	fs.StringVar(&r.input, "input", "", "")
	fs.StringVar(&a.Scene.Menu, "menu", "", "")
	addModeFlags(fs, a, r)
	return fs
}

// ParseArgs wertet die Schalter aus (rein, ohne Dateizugriff); defSaves ist der Standard für --saves.
func ParseArgs(argv []string, defSaves string) (Args, error) {
	a := Args{Saves: defSaves}
	var r rawArgs
	fs := newFlagSet(&a, &r, defSaves)
	if err := fs.Parse(argv); err != nil || fs.NArg() > 0 {
		return Args{}, fmt.Errorf("🚫 ungültige Schalter (%v %v); gültig: %s", err, fs.Args(), Usage)
	}
	a.applyDefaults(&r)
	var err error
	if a.W, a.H, err = parseSize(r.size); err != nil {
		return Args{}, err
	}
	if a.Scene.Cams, err = parseCams(r.cams); err != nil {
		return Args{}, err
	}
	if a.Scene.Script, err = ParseScript(r.input); err != nil {
		return Args{}, err
	}
	if err = a.applyAt(r.at); err != nil {
		return Args{}, err
	}
	return a, a.check()
}

// applyDefaults setzt Seed und Eingabe-Folge, die eine Aufnahme ohne eigene Angabe braucht (nicht bei --flow).
func (a *Args) applyDefaults(r *rawArgs) {
	if !a.Scene.Direct() && !a.Flow && ((a.Shots != "" && a.Scene.Menu == "") || a.Scene.Menu == "pause") {
		a.Scene.Seed = "k3c" // Aufnahme einer Szene bzw. der Pause braucht ein Spiel
	}
	a.Script = r.input != "" || a.Shots != ""
	if r.input == "" && a.Shots != "" && a.Scene.Menu == "" && !a.Flow {
		r.input = defScript
	}
}

// check prüft die Grenzen der Schalter.
func (a Args) check() error {
	s := a.Scene
	switch {
	case s.Players < 1 || s.Players > view.MaxPlayers:
		return fmt.Errorf("🚫 --players 1 bis %d, nicht %d", view.MaxPlayers, s.Players)
	case s.Seed != "" && s.Load != "":
		return errors.New("🚫 --seed und --load schließen sich aus")
	case s.Load != "" && !saveName.MatchString(s.Load):
		return fmt.Errorf("🚫 --load %q: Name a–z, 0–9, -, höchstens 32 Zeichen", s.Load)
	case s.Depth < 0 || s.Tick < 0 || s.Tick > maxTick:
		return fmt.Errorf("🚫 --depth ab 0, --tick 0 bis %d", maxTick)
	case len(s.Cams) > s.Players:
		return fmt.Errorf("🚫 --cam: %d Werte für %d Spieler", len(s.Cams), s.Players)
	case s.Menu != "" && !slices.Contains(menus, s.Menu):
		return fmt.Errorf("🚫 --menu %q: einer von %s", s.Menu, strings.Join(menus, ", "))
	}
	return a.checkShots()
}

// checkShots prüft Zahl und Abstand der Bilder und die Spieler der Eingabe-Folge.
func (a Args) checkShots() error {
	if a.Count < 1 || a.Count > maxShots || a.Every < 1 || a.Every > maxTick {
		return fmt.Errorf("🚫 --count 1 bis %d, --every ab 1", maxShots)
	}
	for _, h := range a.Scene.Script {
		if h.Player >= a.Scene.Players {
			return fmt.Errorf("🚫 --input: Spieler %d bei %d Spielern", h.Player, a.Scene.Players)
		}
	}
	return a.checkModes()
}

// parseSize liest „BxH“.
func parseSize(s string) (int, int, error) {
	ws, hs, ok := strings.Cut(s, "x")
	w, errW := strconv.Atoi(ws)
	h, errH := strconv.Atoi(hs)
	if !ok || errW != nil || errH != nil || w < minWidth || w > maxWidth || h < minWidth*9/16 || h > maxWidth {
		return 0, 0, fmt.Errorf("🚫 --size %q: Breite×Höhe wie 1280x720, Breite %d bis %d", s, minWidth, maxWidth)
	}
	return w, h, nil
}

// parseCams liest „x[,x]“; ein leerer Eintrag oder „-“ folgt dem Monarchen.
func parseCams(s string) ([]float64, error) {
	if s == "" {
		return nil, nil
	}
	var out []float64
	for part := range strings.SplitSeq(s, ",") {
		if part == "" || part == "-" {
			out = append(out, math.NaN())
			continue
		}
		x, err := strconv.ParseFloat(part, 64)
		if err != nil || math.IsNaN(x) || math.IsInf(x, 0) {
			return nil, fmt.Errorf("🚫 --cam %q: Welt-x in Units, Einträge durch Komma", s)
		}
		out = append(out, x)
	}
	return out, nil
}
