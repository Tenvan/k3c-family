package props

// Referenz für den Rust-Port (RU16.6, B-1014): Fünf feste Zustände einer Basis (ausgebaute Basis am Tag, Baustelle,
// Mauer mit Schaden und Ruine, Wald mit Plantage, Kampfvariante) als Sim-Welt samt Biom und Level in
// testdata/rust/bild/props-<name>.state.json.gz, das Bild, das Boden (world, ohne Fluss und Licht) und dieses Paket
// daraus zeichnen, als props-<name>.go.png und die Antworten der Sim-Funktionen, die Rust noch nicht rechnet, als
// props-<name>.simview.json (sim.EconomyOf, SiteGoldLeft, SitePayable, SiteNeedsClearing, ShieldRack, CombatSpots,
// SiteVisible, CombatSpot.StateFor, AtLair). Das Rust-Werkzeug `k3c-game still --props` zeichnet denselben Zustand mit diesen
// Antworten (FixedSimView); sein Test vergleicht beide Bilder (rust/crates/k3c-game/tests/still_props.rs).
//
// Schreiben (braucht ein Fenster und die gerenderte Grafik, `task gen:paper`; Ordner der Grafik: K3C_GRAFIK_DIR):
//
//	go test ./props/ -run TestRustRef -write     (im Ordner native/k3c-game)
//
// Zwei weitere Stände (Glut, Kristall) zeigen Glut und Schein, deren Deckkraft gegen Go zu prüfen ist.
// Das Erzeugen der Zustände ist ein kurzer Sim-Aufbau (kein Balancing, Ausnahme der Sim-Sperre aus B-1014). Ebitengine
// braucht den Hauptfaden: Das Schreiben läuft in init und beendet das Programm vor den Tests (wie world/rustref_test.go).
// Ohne -write prüft TestRustRef nur, dass die eingecheckten Dateien vollständig und lesbar sind.

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
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
	"k3c/native/k3c-game/world"
)

const (
	refDir       = "../../../testdata/rust/bild"
	refW, refH   = 960, 540
	refClock     = 12.5
	refSeed      = "rustref"
	refGrafE     = "K3C_GRAFIK_DIR"
	refViewUnits = 420.0 / 32 // Welt-Höhe der Zelle in Units (game/layout.go)
)

// refCase ist ein Stand: Hub-Stufe, Kamera je Zelle und der Aufbau der Welt.
type refCase struct {
	name     string
	traits   []string // Natur-Eigenschaften aus Eisenstollen (glow) und Kristall (brightness), die dem Wald fehlen
	hubLevel int
	cams     [2]float64
	setup    func(w *sim.World)
}

var refCases = []refCase{
	{"basis-tag", nil, 3, [2]float64{440, 472}, setupBasis},
	{"baustelle", nil, 2, [2]float64{426, 478}, setupSites},
	{"mauer-ruine", nil, 2, [2]float64{540, 585}, setupDamage},
	{"wald-plantage", nil, 1, [2]float64{175, 578}, setupForest},
	{"kampf", nil, 3, [2]float64{452, 496}, setupCombat},
	// Glut (Eigenschaft glow) und Schein der Kristall-Adern (brightness): Deckkraft gegen Go. Der Wald bekommt die
	// Eigenschaft und Adern, damit die Kulissen gleich bleiben (Höhlen-Biome zeichnen Rust und Go dort verschieden).
	{"glut", []string{"glow"}, 1, [2]float64{440, 472}, func(w *sim.World) { w.Level.Nature.Traits = map[string]int{"glow": 3} }},
	{"kristall", []string{"brightness"}, 1, [2]float64{440, 472}, setupCrystal},
}

type refSeat struct {
	Player  int `json:"player"`
	Monarch int `json:"monarch"`
	Stage   int `json:"stage"`
}

// refState ist das Dateiformat des Zustands (Rust: still/state.rs). HubLevel, HubStep und die Varianten der Knoten
// und die Natur-Werte stehen sonst nicht im JSON der Welt (`json:"-"`), die Zeichner brauchen sie.
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
	Nature       level.NatureValues `json:"nature"` // geltende Natur-Werte der Stufe (`Layout.Nature` ist `json:"-"`)
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
	fmt.Println("💾 rustref: Zustände, Bilder und Sim-Antworten nach", refDir)
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
	b, biome, err := biomeWith(c.traits)
	if err != nil {
		return nil, err
	}
	w, err := sim.CreateWorld(b, refSeed, sim.Options{Time: 180})
	if err != nil {
		return nil, err
	}
	sim.AddPlayer(w)
	sim.AddPlayer(w)
	w.HubLevel, w.HubStep = c.hubLevel, 1
	w.Players[0].X, w.Players[1].X = c.cams[0], c.cams[1]
	c.setup(w)
	layout, err := json.Marshal(w.Level)
	if err != nil {
		return nil, err
	}
	s := &refState{
		Name: "props-" + c.name, Level: b.ID, Size: [2]int{refW, refH}, Clock: refClock,
		Cams: c.cams[:], Seats: []refSeat{{0, 0, 0}, {1, 1, 0}}, Biome: biome, Layout: layout,
		HubLevel: w.HubLevel, HubStep: w.HubStep, Nature: w.Level.Nature, World: w,
	}
	for _, n := range w.Nodes {
		s.NodeVariants = append(s.NodeVariants, n.Variant)
	}
	return s, nil
}

// frameOfState baut den Frame und die zwei Zellen (übereinander) wie game.
func frameOfState(s *refState) (*view.Frame, []view.Cell) {
	f := &view.Frame{Clock: s.Clock, Stages: []view.Stage{{Index: 0, Level: s.Level, World: s.World}}}
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

func refGrafik() string {
	if d := os.Getenv(refGrafE); d != "" {
		return d
	}
	return "../../../public/grafik"
}

// render zeichnet Boden (Ebenen bis LayerGround der Welt) und die Bauten; Fluss und Licht fehlen (wie im Rust-Bild).
func render(lib *art.Library, drawers []view.Drawer, s *refState, loaded *bool) (*image.RGBA, error) {
	if !*loaded {
		var assets []string
		for _, d := range drawers {
			if src, ok := d.(view.AssetSource); ok {
				assets = append(assets, src.Assets(s.Level, s.World)...)
			}
		}
		*loaded = true
		if err := lib.SetLevels(map[string][]string{s.Level: assets}); err != nil {
			return nil, err
		}
	}
	f, cells := frameOfState(s)
	for _, d := range drawers {
		if l, ok := d.(view.Listener); ok {
			l.Listen(f)
		}
	}
	screen := ebiten.NewImage(s.Size[0], s.Size[1])
	defer screen.Deallocate()
	screen.Fill(color.RGBA{0, 0, 0, 255})
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
	ebiten.SetWindowTitle("K3C Rust-Referenz Bauten")
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
	var drawers []view.Drawer
	for _, d := range world.New(lib) {
		if d.Layer() <= view.LayerGround {
			drawers = append(drawers, d)
		}
	}
	drawers = view.Sorted(append(drawers, New(lib)))
	if err := os.MkdirAll(refDir, 0o755); err != nil {
		return err
	}
	loaded := false
	for _, c := range refCases {
		s, err := buildState(c)
		if err != nil {
			return err
		}
		if err := writeGz(filepath.Join(refDir, s.Name+".state.json.gz"), s); err != nil {
			return err
		}
		if err := writeJSON(filepath.Join(refDir, s.Name+".simview.json"), simView(s)); err != nil {
			return err
		}
		img, err := render(lib, drawers, s, &loaded)
		if err != nil {
			return err
		}
		if err := writePNG(filepath.Join(refDir, s.Name+".go.png"), img); err != nil {
			return err
		}
	}
	return nil
}

// TestRustRef prüft ohne Fenster, dass die eingecheckten Zustände, Bilder und Sim-Antworten vollständig und lesbar
// sind und die Stände zeigen, was ihr Name sagt (Burg-Stufe, Gerüst, Ruine, Plantage, Zahlstellen).
func TestRustRef(t *testing.T) {
	for _, c := range refCases {
		name := "props-" + c.name
		raw, err := os.ReadFile(filepath.Join(refDir, name+".state.json.gz"))
		if err != nil {
			t.Fatalf("%s: Zustand fehlt (go test ./props/ -run TestRustRef -write): %v", name, err)
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
		if s.World == nil || len(s.World.Players) != 2 || s.HubLevel != c.hubLevel {
			t.Errorf("%s: erwartet zwei Spieler und Hub-Stufe %d", name, c.hubLevel)
		}
		if len(s.NodeVariants) != len(s.World.Nodes) {
			t.Errorf("%s: Varianten %d, Knoten %d", name, len(s.NodeVariants), len(s.World.Nodes))
		}
		checkView(t, name, c, &s)
		checkPNG(t, filepath.Join(refDir, name+".go.png"), s.Size)
	}
}

func checkView(t *testing.T, name string, c refCase, s *refState) {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(refDir, name+".simview.json"))
	if err != nil {
		t.Errorf("%s: Sim-Antworten fehlen: %v", name, err)
		return
	}
	var v refView
	if err := json.Unmarshal(raw, &v); err != nil {
		t.Errorf("%s: %v", name, err)
		return
	}
	if len(v.Sites) != len(s.World.Sites) {
		t.Errorf("%s: Antworten für %d Plätze, Welt hat %d", name, len(v.Sites), len(s.World.Sites))
	}
	if c.name == "kampf" && (len(v.CombatSpots) != 3 || !v.ShieldRack.Visible) {
		t.Errorf("%s: erwartet Schild-Zahlstelle, zwei Auslöser und sichtbares Gestell: %+v", name, v)
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
		t.Errorf("%s: %d KB über dem Ziel von 300 KB", path, st.Size()>>10)
	}
}
