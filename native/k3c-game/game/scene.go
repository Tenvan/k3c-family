package game

import (
	"errors"
	"flag"
	"fmt"
	"io"
	"math"
	"regexp"
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

// Hold ist ein Eintrag der festen Eingabe-Folge: Ab Takt Tick (seit Start der Szene) hält Spieler Player den Befehl.
type Hold struct {
	Tick   int
	Player int
	Cmd    view.Command
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
}

// Args sind die Schalter des Programms.
type Args struct {
	Saves  string // Ordner der Spielstände
	Shots  string // Aufnahme-Modus: Ordner der Bilder; leer = Fenster
	Count  int    // Zahl der Bilder
	Every  int    // Takte zwischen zwei Bildern
	W, H   int    // Größe der Zeichenfläche (Aufnahme) bzw. des Fensters
	Scene  Scene
	Script bool // Eingabe aus Scene.Script statt von den Geräten
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
const Usage = "k3c-game [--saves <ordner>] [--seed <s> | --load <name>] [--depth <n>] [--players 1|2] [--cam <x>[,<x>]] " +
	"[--tick <n>] [--input <p@t=LRSPA,…>] [--size <B>x<H>] [--shots <ordner> [--count <n>] [--every <takte>]]"

// ParseArgs wertet die Schalter aus (rein, ohne Dateizugriff); defSaves ist der Standard für --saves.
func ParseArgs(argv []string, defSaves string) (Args, error) {
	a := Args{Saves: defSaves}
	var cams, input, size string
	fs := flag.NewFlagSet("k3c-game", flag.ContinueOnError)
	fs.SetOutput(io.Discard)
	fs.StringVar(&a.Saves, "saves", defSaves, "")
	fs.StringVar(&a.Shots, "shots", "", "")
	fs.IntVar(&a.Count, "count", 8, "")
	fs.IntVar(&a.Every, "every", 3, "")
	fs.StringVar(&size, "size", "1280x720", "")
	fs.StringVar(&a.Scene.Seed, "seed", "", "")
	fs.IntVar(&a.Scene.Depth, "depth", 0, "")
	fs.StringVar(&a.Scene.Load, "load", "", "")
	fs.IntVar(&a.Scene.Players, "players", 2, "")
	fs.StringVar(&cams, "cam", "", "")
	fs.IntVar(&a.Scene.Tick, "tick", 0, "")
	fs.StringVar(&input, "input", "", "")
	if err := fs.Parse(argv); err != nil || fs.NArg() > 0 {
		return Args{}, fmt.Errorf("🚫 ungültige Schalter (%v %v); gültig: %s", err, fs.Args(), Usage)
	}
	if a.Shots != "" && !a.Scene.Direct() {
		a.Scene.Seed = "k3c"
	}
	a.Script = input != "" || a.Shots != ""
	if input == "" && a.Shots != "" {
		input = defScript
	}
	var err error
	if a.W, a.H, err = parseSize(size); err != nil {
		return Args{}, err
	}
	if a.Scene.Cams, err = parseCams(cams); err != nil {
		return Args{}, err
	}
	if a.Scene.Script, err = ParseScript(input); err != nil {
		return Args{}, err
	}
	return a, a.check()
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
	return nil
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

// ParseScript liest die Eingabe-Folge „spieler@takt=befehl,…“, Befehl aus L (links), R (rechts), S (Sprint),
// P (zahlen), A (Schlag), 1–4 (Skill) oder „-“ (nichts). Beispiel: „0@0=R,1@0=LS,0@90=-“.
func ParseScript(s string) ([]Hold, error) {
	if s == "" {
		return nil, nil
	}
	var out []Hold
	for part := range strings.SplitSeq(s, ",") {
		h, err := parseHold(part)
		if err != nil {
			return nil, fmt.Errorf("🚫 --input %q: %v", part, err)
		}
		out = append(out, h)
	}
	return out, nil
}

// parseHold liest einen Eintrag „spieler@takt=befehl“.
func parseHold(s string) (Hold, error) {
	who, cmd, ok := strings.Cut(s, "=")
	ps, ts, ok2 := strings.Cut(who, "@")
	p, errP := strconv.Atoi(ps)
	t, errT := strconv.Atoi(ts)
	if !ok || !ok2 || errP != nil || errT != nil || p < 0 || p >= view.MaxPlayers || t < 0 || t > maxTick {
		return Hold{}, errors.New("erwartet spieler@takt=befehl")
	}
	h := Hold{Tick: t, Player: p}
	if cmd == "-" {
		return h, nil
	}
	for _, r := range cmd {
		if err := apply(&h.Cmd, r); err != nil {
			return Hold{}, err
		}
	}
	if cmd == "" {
		return Hold{}, errors.New("leerer Befehl, „-“ für nichts")
	}
	return h, nil
}

// apply setzt den Buchstaben r im Befehl c.
func apply(c *view.Command, r rune) error {
	switch r {
	case 'L':
		c.Move--
	case 'R':
		c.Move++
	case 'S':
		c.Sprint = true
	case 'P':
		c.Pay = true
	case 'A':
		c.Attack = true
	case '1', '2', '3', '4':
		c.Skill = int(r - '0')
	default:
		return fmt.Errorf("unbekannter Befehl %q (L R S P A 1–4 -)", r)
	}
	return nil
}

// CommandsAt liefert die gehaltenen Befehle aller Spieler im Takt tick: je Spieler der letzte Eintrag mit Tick ≤ tick
// (bei gleichem Takt der spätere in der Folge).
func CommandsAt(script []Hold, tick int) [view.MaxPlayers]view.Command {
	var out [view.MaxPlayers]view.Command
	var best [view.MaxPlayers]int
	for i := range best {
		best[i] = -1
	}
	for _, h := range script {
		if h.Tick <= tick && h.Tick >= best[h.Player] {
			out[h.Player], best[h.Player] = h.Cmd, h.Tick
		}
	}
	return out
}

// scripted ist die Eingabe-Quelle der festen Folge: je Poll ein Takt, Geräte wie Scene.Devices, keine Menü-Aktionen.
type scripted struct {
	script []Hold
	devs   []view.DeviceID
	buf    []view.DeviceInput
	tick   int
}

func newScripted(s Scene) *scripted {
	return &scripted{script: s.Script, devs: s.Devices(), buf: make([]view.DeviceInput, s.Players)}
}

func (s *scripted) Poll() view.Input {
	cmds := CommandsAt(s.script, s.tick)
	s.tick++
	for i, d := range s.devs {
		s.buf[i] = view.DeviceInput{Device: d, Command: cmds[i]}
	}
	return view.Input{Devices: s.buf}
}
