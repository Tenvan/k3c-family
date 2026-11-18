package hud

// Referenz für den Rust-Port (RU20.7, B-1014): Vier feste Stände des HUD als Sim-Welt samt Biom und Level in
// testdata/rust/bild/hud-<name>.state.json.gz, dazu je Stand
//   - hud-<name>.frame.json   Sitze, Geräte, Sprache, Eingabe-Profile, Takt, Meldungen, freie Geräte (Frame-Felder, die Listen liest),
//   - hud-<name>.simview.json Antworten der Sim-Funktionen (EconomyOf, SitePayable, AtLair), die Rust noch nicht rechnet (FixedSimView),
//   - hud-<name>.layout.json  die Plätze der Elemente je Zelle (`placed`, Fenster-Pixel) als Orakel des Platz-Tests,
//   - hud-<name>.keys.json    Glyph-Schlüssel je Sitz und Aktion (glyphOf) für den Test der Xbox-Symbole und Tastenkappen,
//   - hud-<name>.go.png       das Bild von `drawItems` auf farbigem Grund (backdrop).
//
// Das Bild zeichnet nur drawItems (ohne drawWorldTexts, Seekarte, Menü und Boss-Leiste; die liefert RU23). Radar und
// Geschichtszeile zeichnet Rust bis RU23 nicht: Die Referenz nimmt die Radar-Breite weg (widthUnits = 0), eine
// Geschichtszeile entsteht in diesen Ständen nicht (kein Wendepunkt); ebenso fehlt der Text der Skill-Leiste (skilltext.go,
// RU23), die Referenz leert ihn (barS = ""). Das Rust-Werkzeug `k3c-game still --hud` zeichnet
// denselben Stand; sein Test vergleicht beide Bilder (rust/crates/k3c-game/tests/still_hud.rs).
//
// Schreiben (braucht ein Fenster und die gerenderte Grafik, `task gen:paper`; Ordner der Grafik: K3C_GRAFIK_DIR):
//
//	go test ./hud/ -run TestRustRef -write     (im Ordner native/k3c-game)
//
// Das Erzeugen der Zustände ist ein kurzer Sim-Aufbau (kein Balancing, Ausnahme der Sim-Sperre aus B-1014). Ebitengine
// braucht den Hauptfaden: Das Schreiben läuft in init und beendet das Programm vor den Tests (wie props/rustref_test.go).
// Ohne -write prüft TestRustRef nur, dass die eingecheckten Dateien vollständig und lesbar sind.

import (
	"bytes"
	"compress/gzip"
	"encoding/json"
	"errors"
	"fmt"
	"image"
	"image/color"
	"io"
	"os"
	"path/filepath"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/level"
	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

const (
	refDir       = "../../../testdata/rust/bild"
	refW, refH   = 1280, 720
	refClock     = 12.5
	refSeed      = "rustref"
	refGrafE     = "K3C_GRAFIK_DIR"
	refViewUnits = 420.0 / 32 // Welt-Höhe der Zelle in Units (game/layout.go)
)

// backdrop ist der Grund hinter den Anzeigen (Rust: still::compose::HUD_BACKDROP): Er macht die halbtransparenten
// Flächen sichtbar, schwarz auf Schwarz zeigte der Bildvergleich sie nicht.
var backdrop = color.RGBA{60, 90, 130, 255}

var (
	refKeys = view.DeviceID{Kind: view.DeviceKeyboard}
	refPad  = view.DeviceID{Kind: view.DeviceGamepad, Index: 0}
)

// refCase ist ein Stand: Plätze mit Gerät, Sprache, Eingabe-Profile, freie Geräte, Meldung und der Aufbau der Welt.
type refCase struct {
	name     string
	seats    []view.DeviceID // ein Platz je Eintrag (Spieler 0, 1 …)
	lang     string
	controls [view.MaxPlayers]string
	unseated []view.DeviceID
	cue      string // Meldung an alle (Ablehnung), "" = keine
	setup    func(w *sim.World)
}

func cycle(phase string, day int, progress, left float64) sim.CycleInfo {
	return sim.CycleInfo{Phase: phase, Day: day, Progress: progress, SecondsLeft: left}
}

var refCases = []refCase{
	{name: "tag-allein", seats: []view.DeviceID{refKeys}, lang: view.DefaultLanguage, setup: func(w *sim.World) {
		w.Cycle, w.Players[0].Gold = cycle("day", 2, 0.4, 95), 37
		w.Aggression = new(float64)
		*w.Aggression = 35
		w.Stock.Wood, w.Stock.Stone, w.SkillPoints = 14, 6, 0
	}},
	{name: "nacht-banner", seats: []view.DeviceID{refKeys, refPad}, lang: view.DefaultLanguage, cue: "Nicht genug Gold für den Ausbau, die Nacht kommt", setup: func(w *sim.World) {
		w.Cycle, w.Wave = cycle("night", 3, 0.3, 41), 3
		w.Players[0].Gold, w.Players[1].Gold = 120, 8
		w.Stock.Wood, w.Stock.Stone, w.Stock.Copper = 30, 12, 2
	}},
	{name: "pad-tastatur", seats: []view.DeviceID{refKeys, refPad}, lang: view.DefaultLanguage,
		controls: [view.MaxPlayers]string{view.ProfileArrows, view.ProfileGamepad},
		unseated: []view.DeviceID{{Kind: view.DeviceGamepad, Index: 4}}, setup: func(w *sim.World) {
			w.Cycle, w.SkillPoints = cycle("dusk", 4, 0.5, 20), 2
			w.Players[0].Gold, w.Players[1].Gold = 55, 90
			w.Stock.Wood, w.Stock.Stone = 9, 3
		}},
	{name: "englisch-hub", seats: []view.DeviceID{refKeys, refPad}, lang: view.LanguageEnglish, setup: func(w *sim.World) {
		w.HubLevel, w.HubStep = 3, 2
		w.Cycle, w.Wave = cycle("dawn", 6, 0.8, 12), 5
		w.Players[0].Gold, w.Players[1].Gold = 210, 64
		w.Stock.Wood, w.Stock.Stone, w.Stock.Copper = 40, 22, 9
	}},
}

type refSeat struct {
	Player  int `json:"player"`
	Monarch int `json:"monarch"`
	Stage   int `json:"stage"`
}

// refState ist das Dateiformat des Zustands (Rust: still/state.rs), wie in props/rustref_test.go.
type refState struct {
	Name         string             `json:"name"`
	Level        string             `json:"level"`
	Size         [2]int             `json:"size"`
	Clock        float64            `json:"clock"`
	Cams         []float64          `json:"cams"`
	Seats        []refSeat          `json:"seats"`
	Biome        json.RawMessage    `json:"biome"`
	Layout       json.RawMessage    `json:"layout"`
	HubLevel     int                `json:"hubLevel"`
	HubStep      int                `json:"hubStep"`
	NodeVariants []string           `json:"nodeVariants"`
	Nature       level.NatureValues `json:"nature"`
	World        *sim.World         `json:"world"`
}

func init() {
	if !refWriteRequested() {
		return
	}
	if err := writeRefs(); err != nil {
		fmt.Println("❌ rustref:", err)
		os.Exit(1)
	}
	fmt.Println("💾 rustref: Zustände, Bilder, Frames, Sim-Antworten und Plätze nach", refDir)
	os.Exit(0)
}

func refWriteRequested() bool {
	for _, a := range os.Args[1:] {
		if a == "-write" || a == "--write" || a == "-write=true" || a == "-test.write" {
			return true
		}
	}
	return false
}

func buildState(c refCase) (*refState, error) {
	b, err := level.LoadBiome("forest")
	if err != nil {
		return nil, err
	}
	biome, err := os.ReadFile("../../../data/biomes/forest.json")
	if err != nil {
		return nil, err
	}
	w, err := sim.CreateWorld(b, refSeed, sim.Options{Time: 180})
	if err != nil {
		return nil, err
	}
	s := &refState{Name: "hud-" + c.name, Level: b.ID, Size: [2]int{refW, refH}, Clock: refClock, Biome: biome, World: w}
	for i := range c.seats {
		sim.AddPlayer(w)
		s.Seats = append(s.Seats, refSeat{i, i, 0})
		s.Cams = append(s.Cams, w.Players[i].X)
	}
	c.setup(w)
	if s.Layout, err = json.Marshal(w.Level); err != nil {
		return nil, err
	}
	s.HubLevel, s.HubStep, s.Nature = w.HubLevel, w.HubStep, w.Level.Nature
	for _, n := range w.Nodes {
		s.NodeVariants = append(s.NodeVariants, n.Variant)
	}
	return s, nil
}

// settingsOf sind die Einstellungen des Standes (Sprache, Eingabe-Profile je Platz).
func settingsOf(c refCase) view.Settings {
	s := view.DefaultSettings()
	s.Language = c.lang
	if c.controls[0] != "" {
		s.Controls = c.controls
	}
	return s
}

// frameOf baut den Frame und die Zellen (übereinander) wie game und die Datei des Frames für Rust.
func frameOf(c refCase, s *refState) (*view.Frame, []view.Cell, refFrame) {
	set := settingsOf(c)
	f := &view.Frame{Clock: s.Clock, Settings: set, Stages: []view.Stage{{Index: 0, Level: s.Level, World: s.World}}}
	rf := refFrame{Clock: s.Clock, Language: set.Language, Controls: set.Controls, Cues: []refCue{}, Unseated: []refDevice{}}
	if c.cue != "" {
		f.Cues = []view.Cue{{Kind: view.CueDenied, Player: -1, Text: c.cue}}
		rf.Cues = []refCue{{"denied", -1, c.cue}}
	}
	for _, d := range c.unseated {
		f.Unseated = append(f.Unseated, d)
		rf.Unseated = append(rf.Unseated, devOf(d))
	}
	cells := make([]view.Cell, len(s.Seats))
	for i, st := range s.Seats {
		f.Seats = append(f.Seats, view.Seat{Player: st.Player, Monarch: st.Monarch, Stage: st.Stage, Device: c.seats[i]})
		rf.Seats = append(rf.Seats, refFrameSeat{st.Player, st.Monarch, st.Stage, devOf(c.seats[i])})
		n := len(s.Seats)
		r := image.Rect(0, i*s.Size[1]/n, s.Size[0], (i+1)*s.Size[1]/n)
		cell := view.Cell{Index: i, Player: st.Player, Rect: r}
		cell.Camera = view.NewCamera(r, s.Cams[i], float64(cell.WorldRect().Dy())/refViewUnits)
		cells[i] = cell
	}
	f.Cells = cells
	return f, cells, rf
}

func refGrafik() string {
	if d := os.Getenv(refGrafE); d != "" {
		return d
	}
	return "../../../public/grafik"
}

// refLayout ist das Dateiformat der Plätze: je Zelle die Rechtecke (x, y, w, h) der gezeichneten Elemente nach Index.
type refLayout struct {
	Cells []refLayoutCell `json:"cells"`
}

type refLayoutCell struct {
	Index int                `json:"index"`
	Items map[int][4]float64 `json:"items"`
}

// render zeichnet drawItems je Zelle auf schwarzem Grund und liefert Bild, Plätze und Texte.
func render(h *hud, c refCase, s *refState) (*image.RGBA, refLayout, error) {
	f, cells, _ := frameOf(c, s)
	h.Listen(f)
	screen := ebiten.NewImage(s.Size[0], s.Size[1])
	defer screen.Deallocate()
	screen.Fill(backdrop)
	var lay refLayout
	for _, cell := range cells {
		st := h.seat(cell.Player)
		if st == nil {
			return nil, lay, fmt.Errorf("%s: Platz %d ohne Zustand", s.Name, cell.Player)
		}
		st.widthUnits = 0 // kein Radar: Rust zeichnet ihn erst mit RU23
		st.barS = ""      // Skill-Leiste: Text kommt aus skilltext.go (RU23), in Rust noch leer
		h.paint.stencils()
		h.drawItems(screen.SubImage(cell.Rect).(*ebiten.Image), st, cell, scaleOf(cell, h.cells))
		lc := refLayoutCell{Index: cell.Index, Items: map[int][4]float64{}}
		for id, ok := range h.lay[cell.Player].out.ok {
			if r := h.lay[cell.Player].out.at[id]; ok {
				lc.Items[id] = [4]float64{r.X, r.Y, r.W, r.H}
			}
		}
		lay.Cells = append(lay.Cells, lc)
	}
	img := image.NewRGBA(image.Rect(0, 0, s.Size[0], s.Size[1]))
	screen.ReadPixels(img.Pix)
	return img, lay, nil
}

type refGame struct{ err error }

func (g *refGame) Update() error {
	g.err = writeRefsInLoop()
	return ebiten.Termination
}
func (g *refGame) Draw(*ebiten.Image)         {}
func (g *refGame) Layout(int, int) (int, int) { return refW, refH }

func writeRefs() error {
	g := &refGame{}
	ebiten.SetWindowSize(refW/2, refH/2)
	ebiten.SetWindowTitle("K3C Rust-Referenz HUD")
	if err := ebiten.RunGame(g); err != nil && !errors.Is(err, ebiten.Termination) {
		return err
	}
	return g.err
}

func writeRefsInLoop() error {
	lib, err := art.New(art.Config{
		GrafikDir: refGrafik(), RigDir: "../../k3c-rig/assets", FontPath: "../../../public/schrift/k3c.ttf",
		StrictFont: true, Scales: art.DataScales(),
	})
	if err != nil {
		return err
	}
	h := New(lib).(*hud)
	if err := lib.SetLevels(map[string][]string{"forest": h.Assets("forest", nil)}); err != nil {
		return err
	}
	if err := os.MkdirAll(refDir, 0o755); err != nil {
		return err
	}
	for _, c := range refCases {
		// frischer Zustand je Stand (Meldungen, Merker, Layout-Cache); die Bibliothek mit den Bildern bleibt
		if err := writeCase(New(lib).(*hud), c); err != nil {
			return err
		}
	}
	return nil
}

func writeCase(h *hud, c refCase) error {
	s, err := buildState(c)
	if err != nil {
		return err
	}
	base := filepath.Join(refDir, s.Name)
	if err := writeGz(base+".state.json.gz", s); err != nil {
		return err
	}
	_, _, rf := frameOf(c, s)
	if err := writeJSON(base+".frame.json", rf); err != nil {
		return err
	}
	if err := writeJSON(base+".simview.json", simView(s)); err != nil {
		return err
	}
	img, lay, err := render(h, c, s)
	if err != nil {
		return err
	}
	if err := writeJSON(base+".layout.json", lay); err != nil {
		return err
	}
	if err := writeJSON(base+".keys.json", keysOf(h, c)); err != nil {
		return err
	}
	return writePNG(base+".go.png", img)
}

// TestRustRef prüft ohne Fenster, dass die eingecheckten Dateien vollständig und lesbar sind und die Stände zeigen,
// was ihr Name sagt (Plätze, Sprache, Profile, Meldung, freies Gerät).
func TestRustRef(t *testing.T) {
	for _, c := range refCases {
		name := "hud-" + c.name
		raw, err := os.ReadFile(filepath.Join(refDir, name+".state.json.gz"))
		if err != nil {
			t.Fatalf("%s: Zustand fehlt (go test ./hud/ -run TestRustRef -write): %v", name, err)
		}
		zr, err := gzip.NewReader(bytes.NewReader(raw))
		if err != nil {
			t.Fatal(err)
		}
		plain, _ := io.ReadAll(zr)
		var s refState
		if err := json.Unmarshal(plain, &s); err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		if s.World == nil || len(s.World.Players) != len(c.seats) || len(s.Cams) != len(c.seats) {
			t.Errorf("%s: erwartet %d Spieler", name, len(c.seats))
		}
		checkFrame(t, name, c)
		checkPNG(t, filepath.Join(refDir, name+".go.png"), s.Size)
	}
}
