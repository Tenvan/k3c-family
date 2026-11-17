package world

// Referenz für den Rust-Port (RU13.6, B-1014): Fünf feste Zustände (Morgen, Mittag, Abend, Nacht, Nacht mit Gegnern und
// zwei Spielern) als Sim-Welt samt Biom und Level in testdata/rust/bild/<name>.state.json.gz und das Bild, das dieses
// Paket (Welt) und die Figuren (actors) daraus zeichnen, als <name>.go.png. Das Rust-Werkzeug `k3c-game still` zeichnet
// denselben Zustand; sein Test vergleicht beide Bilder (rust/crates/k3c-game/tests/still.rs).
//
// Schreiben (braucht ein Fenster und die gerenderte Grafik, `task gen:paper`; Ordner der Grafik: K3C_GRAFIK_DIR):
//
//	go test ./world/ -run TestRustRef -write     (im Ordner native/k3c-game)
//
// Ebitengine braucht den Hauptfaden: Das Schreiben läuft darum in init (Paket-Initialisierung läuft auf dem
// Hauptfaden) und beendet das Programm, bevor TestMain und die Tests starten. Ohne -write prüft TestRustRef nur, dass
// die eingecheckten Dateien vollständig und lesbar sind.
//
// Das Licht (drawLight) multipliziert die Szene; der Rust-Rasterer kennt kein Multiplizieren (B-1056). Wirkt das Licht
// in einem Stand, liegt deshalb zusätzlich <name>.go-nolight.png (ohne Lichtkarte, mit Fackeln) für den Pixelvergleich bei. Der
// Fluss steht auf Kacheln (Settings.River), der Shader gehört nicht zum Vergleich (B-1055).

import (
	"bytes"
	"compress/gzip"
	"encoding/json"
	"errors"
	"fmt"
	"image"
	"image/color"
	"image/png"
	"io"
	"os"
	"path/filepath"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/level"
	"k3c/engine/sim"
	"k3c/native/k3c-game/actors"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

const (
	refDir   = "../../../testdata/rust/bild"
	refW     = 960
	refH     = 540
	refClock = 12.5 // Frame.Clock der Bilder (Sekunden)
	refSeed  = "rustref"
	refGrafE = "K3C_GRAFIK_DIR"
	// Welt-Höhe der Zelle in Units (game/layout.go: 420/32)
	refViewUnits = 420.0 / 32
)

// refCase ist ein Stand: Sekunde im Tag-Nacht-Zyklus (Tag 0–360, Abend –480, Nacht –720, Morgen –840) und Gegner.
type refCase struct {
	name    string
	time    float64
	enemies bool
}

var refCases = []refCase{
	{"morgen", 780, false},
	{"mittag", 180, false},
	{"abend", 440, false},
	{"nacht", 600, false},
	{"nacht-gegner", 560, true},
}

// refSeat und refState sind das Dateiformat des Zustands (Rust: still/state.rs).
type refSeat struct {
	Player  int `json:"player"`
	Monarch int `json:"monarch"`
	Stage   int `json:"stage"`
}

type refState struct {
	Name   string          `json:"name"`
	Level  string          `json:"level"`
	Size   [2]int          `json:"size"`
	Clock  float64         `json:"clock"`
	Cams   []float64       `json:"cams"`
	Seats  []refSeat       `json:"seats"`
	Biome  json.RawMessage `json:"biome"`  // data/biomes/<level>.json
	Layout json.RawMessage `json:"layout"` // level.Layout der Welt (Form des Level-Golden)
	World  *sim.World      `json:"world"`
}

func init() {
	if !refWriteRequested() {
		return
	}
	if err := writeRefs(); err != nil {
		fmt.Println("❌ rustref:", err)
		os.Exit(1)
	}
	fmt.Println("💾 rustref: Zustände und Bilder nach", refDir)
	os.Exit(0)
}

// refWriteRequested sucht -write in den Argumenten (die Flags sind in init noch nicht ausgewertet).
func refWriteRequested() bool {
	for _, a := range os.Args[1:] {
		if a == "-write" || a == "--write" || a == "-write=true" || a == "-test.write" {
			return true
		}
	}
	return false
}

// buildState legt die Welt eines Stands an: Wald, zwei Monarchen, Truppen rund um die Burg, bei Bedarf Gegner rechts.
func buildState(c refCase) (*refState, error) {
	b, err := level.LoadBiome("forest")
	if err != nil {
		return nil, err
	}
	w, err := sim.CreateWorld(b, refSeed, sim.Options{Time: c.time})
	if err != nil {
		return nil, err
	}
	p0, p1 := sim.AddPlayer(w), sim.AddPlayer(w)
	p0.VX, p0.Facing = 5, 1
	p1.VX, p1.Facing = 0, -1
	for i, k := range []string{"warrior", "archer", "warrior", "eliteArcher", "archer", "vagrant"} {
		w.Troops = append(w.Troops, &sim.Troop{ID: w.NextID, Kind: k, X: w.HubX - 14 + float64(i)*5, HP: 40, MaxHP: 40})
		w.NextID++
	}
	if c.enemies {
		for i, k := range []string{"wolf", "goblin", "skeleton", "wolf", "goblinArcher", "zombie"} {
			w.Enemies = append(w.Enemies, &sim.Enemy{ID: w.NextID, Kind: k, X: w.HubX + 12 + float64(i)*4, HP: 30, MaxHP: 40, Traits: []string{}})
			w.NextID++
		}
	}
	biome, err := os.ReadFile("../../../data/biomes/" + b.ID + ".json")
	if err != nil {
		return nil, err
	}
	layout, err := json.Marshal(w.Level)
	if err != nil {
		return nil, err
	}
	return &refState{
		Name: c.name, Level: b.ID, Size: [2]int{refW, refH}, Clock: refClock,
		Cams:  []float64{w.HubX - 8, w.HubX + 10},
		Seats: []refSeat{{0, 0, 0}, {1, 1, 0}},
		Biome: biome, Layout: layout, World: w,
	}, nil
}

// frameOfState baut den Frame und die zwei Zellen (übereinander) wie game; Fluss auf Kacheln.
func frameOfState(s *refState) (*view.Frame, []view.Cell) {
	f := &view.Frame{
		Clock:    s.Clock,
		Stages:   []view.Stage{{Index: 0, Level: s.Level, World: s.World}},
		Settings: view.Settings{River: view.RiverTiles},
	}
	cells := make([]view.Cell, len(s.Seats))
	for i, st := range s.Seats {
		f.Seats = append(f.Seats, view.Seat{Player: st.Player, Monarch: st.Monarch, Stage: st.Stage})
		r := image.Rect(0, i*s.Size[1]/2, s.Size[0], (i+1)*s.Size[1]/2)
		c := view.Cell{Index: i, Player: st.Player, Rect: r}
		c.Camera = view.NewCamera(r, s.Cams[i], float64(c.WorldRect().Dy())/refViewUnits)
		cells[i] = c
	}
	f.Cells = cells
	return f, cells
}

// refGrafik ist der Ordner der gerenderten Grafik: K3C_GRAFIK_DIR, sonst public/grafik des Repos.
func refGrafik() string {
	if d := os.Getenv(refGrafE); d != "" {
		return d
	}
	return "../../../public/grafik"
}

// refRenderer zeichnet Stände im Fenster-Lauf von Ebitengine.
type refRenderer struct {
	lib    *art.Library
	full   []view.Drawer // alle Drawer nach Ebene (Welt samt Licht, Figuren)
	plain  []view.Drawer // dasselbe ohne Lichtkarte
	act    view.Listener
	loaded bool
}

func newRefRenderer() (*refRenderer, error) {
	lib, err := art.New(art.Config{
		GrafikDir: refGrafik(), RigDir: "../../k3c-rig/assets", FontPath: "../../../public/schrift/k3c.ttf",
		StrictFont: true, Scales: art.DataScales(),
	})
	if err != nil {
		return nil, err
	}
	ds := New(lib)
	a := actors.New(lib)
	r := &refRenderer{lib: lib}
	// der Licht-Drawer ist der letzte der Welt (New); ohne Lichtkarte bleiben Fackeln und Übergang (wie in Rust)
	w := ds[0].(far).w
	noMap := layerDrawer{w, view.LayerRiver, func(dst *ebiten.Image, f *view.Frame, c view.Cell) {
		st := stageOf(f, c)
		if st == nil {
			return
		}
		w.drawTorches(dst, c, st.World)
		if a := w.cams.fade(c.Player, f.Clock); a > 0 {
			w.fill(dst, float64(c.Rect.Min.X), float64(c.Rect.Min.Y), float64(c.Rect.Dx()), float64(c.Rect.Dy()), black, 1, a)
		}
	}}
	r.full = view.Sorted(append(append([]view.Drawer{}, ds...), a))
	r.plain = view.Sorted(append(append([]view.Drawer{}, ds[:len(ds)-1]...), a, noMap))
	r.act, _ = a.(view.Listener)
	return r, nil
}

// load lädt die Grafik des Levels (Assets aller Quellen) beim ersten Stand.
func (r *refRenderer) load(s *refState) error {
	if r.loaded {
		return nil
	}
	var assets []string
	for _, d := range r.full {
		if src, ok := d.(view.AssetSource); ok {
			assets = append(assets, src.Assets(s.Level, s.World)...)
		}
	}
	r.loaded = true
	return r.lib.SetLevels(map[string][]string{s.Level: assets})
}

// render zeichnet den Stand (withLight=false: ohne Lichtkarte) und liefert das Bild.
func (r *refRenderer) render(s *refState, withLight bool) (*image.RGBA, error) {
	if err := r.load(s); err != nil {
		return nil, err
	}
	f, cells := frameOfState(s)
	if r.act != nil {
		r.act.Listen(f)
	}
	screen := ebiten.NewImage(s.Size[0], s.Size[1])
	defer screen.Deallocate()
	screen.Fill(color.RGBA{0, 0, 0, 255})
	drawers := r.full
	if !withLight {
		drawers = r.plain
	}
	for _, c := range cells {
		dst := screen.SubImage(c.Rect).(*ebiten.Image)
		for _, d := range drawers {
			d.Draw(dst, f, c)
		}
	}
	img := image.NewRGBA(image.Rect(0, 0, s.Size[0], s.Size[1]))
	screen.ReadPixels(img.Pix)
	return img, nil
}

type refGame struct {
	err error
}

func (g *refGame) Update() error {
	g.err = writeRefsInLoop()
	return ebiten.Termination
}
func (g *refGame) Draw(*ebiten.Image)         {}
func (g *refGame) Layout(int, int) (int, int) { return refW, refH }

func writeRefs() error {
	g := &refGame{}
	ebiten.SetWindowSize(refW/2, refH/2)
	ebiten.SetWindowTitle("K3C Rust-Referenz")
	if err := ebiten.RunGame(g); err != nil && !errors.Is(err, ebiten.Termination) {
		return err
	}
	return g.err
}

func writeRefsInLoop() error {
	r, err := newRefRenderer()
	if err != nil {
		return err
	}
	if err := os.MkdirAll(refDir, 0o755); err != nil {
		return err
	}
	for _, c := range refCases {
		s, err := buildState(c)
		if err != nil {
			return err
		}
		if err := writeStateFile(s); err != nil {
			return err
		}
		full, err := r.render(s, true)
		if err != nil {
			return err
		}
		if err := writePNG(filepath.Join(refDir, c.name+".go.png"), full); err != nil {
			return err
		}
		plain, err := r.render(s, false)
		if err != nil {
			return err
		}
		nolight := filepath.Join(refDir, c.name+".go-nolight.png")
		if bytes.Equal(full.Pix, plain.Pix) {
			_ = os.Remove(nolight)
			continue
		}
		if err := writePNG(nolight, plain); err != nil {
			return err
		}
	}
	return nil
}

func writeStateFile(s *refState) error {
	raw, err := json.Marshal(s)
	if err != nil {
		return err
	}
	var buf bytes.Buffer
	zw, _ := gzip.NewWriterLevel(&buf, gzip.BestCompression)
	if _, err := zw.Write(raw); err != nil {
		return err
	}
	if err := zw.Close(); err != nil {
		return err
	}
	return os.WriteFile(filepath.Join(refDir, s.Name+".state.json.gz"), buf.Bytes(), 0o644)
}

func writePNG(path string, img image.Image) error {
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	enc := png.Encoder{CompressionLevel: png.BestCompression}
	if err := enc.Encode(f, img); err != nil {
		_ = f.Close()
		return err
	}
	return f.Close()
}

// TestRustRef prüft ohne Fenster, dass die eingecheckten Zustände und Bilder vollständig, lesbar und klein genug sind
// und die Stände zeigen, was ihr Name sagt (Zeit im Zyklus, Gegner nur nachts mit Gegnern).
func TestRustRef(t *testing.T) {
	phases := map[string]string{"morgen": "dawn", "mittag": "day", "abend": "dusk", "nacht": "night", "nacht-gegner": "night"}
	for _, c := range refCases {
		raw, err := os.ReadFile(filepath.Join(refDir, c.name+".state.json.gz"))
		if err != nil {
			t.Fatalf("%s: Zustand fehlt (go test ./world/ -run TestRustRef -write): %v", c.name, err)
		}
		zr, err := gzip.NewReader(bytes.NewReader(raw))
		if err != nil {
			t.Fatal(err)
		}
		plain, _ := io.ReadAll(zr)
		var s refState
		if err := json.Unmarshal(plain, &s); err != nil {
			t.Fatalf("%s: %v", c.name, err)
		}
		switch {
		case s.World == nil || len(s.World.Players) != 2:
			t.Errorf("%s: erwartet zwei Spieler", c.name)
		case s.World.Cycle.Phase != phases[c.name]:
			t.Errorf("%s: Phase %q, erwartet %q", c.name, s.World.Cycle.Phase, phases[c.name])
		case (len(s.World.Enemies) > 0) != c.enemies:
			t.Errorf("%s: Gegner %d, erwartet %v", c.name, len(s.World.Enemies), c.enemies)
		}
		checkPNG(t, filepath.Join(refDir, c.name+".go.png"), s.Size)
		if _, err := os.Stat(filepath.Join(refDir, c.name+".go-nolight.png")); err == nil {
			checkPNG(t, filepath.Join(refDir, c.name+".go-nolight.png"), s.Size)
		}
	}
}

func checkPNG(t *testing.T, path string, size [2]int) {
	t.Helper()
	f, err := os.Open(path)
	if err != nil {
		t.Errorf("Bild fehlt: %v", err)
		return
	}
	defer func() { _ = f.Close() }()
	cfg, err := png.DecodeConfig(f)
	if err != nil {
		t.Errorf("%s: %v", path, err)
		return
	}
	if cfg.Width != size[0] || cfg.Height != size[1] || cfg.Width > 1280 || cfg.Height > 720 {
		t.Errorf("%s: Größe %dx%d, erwartet %dx%d (höchstens 1280x720)", path, cfg.Width, cfg.Height, size[0], size[1])
	}
	if st, err := os.Stat(path); err == nil && st.Size() > 300<<10 {
		t.Logf("%s: %d KB über dem Ziel von 300 KB", path, st.Size()>>10)
	}
}
