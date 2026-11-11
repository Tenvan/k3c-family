package actors

import (
	"errors"
	"image"
	"math"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// Zellen wie im Fenster 1920 × 1080 mit zwei Spielern (game/layout.go: 420/32 Units Welt-Höhe).
const (
	cellW, cellH = 1920, 540
	viewUnits    = 420.0 / 32
)

// libDrawer: Drawer mit der Grafik aus dem Repo (Rigs, dazu die Sheets der genannten Assets).
func libDrawer(tb testing.TB, sheets ...string) *drawer {
	tb.Helper()
	lib, err := art.New(art.Config{GrafikDir: repoGrafik, RigDir: repoRigs})
	if err != nil {
		tb.Fatal(err)
	}
	d, err := withLib(lib, sheets...)
	if err != nil {
		tb.Skipf("Grafik fehlt (Sheets erst mit `task gen:paper`): %v", err)
	}
	return d
}

// withLib baut den Drawer auf lib und lädt die Sheets der requisite-Figuren und der genannten Assets.
func withLib(lib *art.Library, sheets ...string) (*drawer, error) {
	d, ok := New(lib).(*drawer)
	if !ok || d.painter == nil {
		return nil, errors.New("kein Rig-Zeichner")
	}
	return d, lib.SetLevels(map[string][]string{"wald": append(sheets, d.sheets...)})
}

// cells: zwei Zellen übereinander, beide Kameras bei Welt-x x.
func cells(x float64) [2]view.Cell {
	var out [2]view.Cell
	for i := range out {
		c := view.Cell{Index: i, Player: i, Rect: image.Rect(0, i*cellH, cellW, (i+1)*cellH)}
		c.Camera = view.NewCamera(c.Rect, x, float64(c.WorldRect().Dy())/viewUnits)
		out[i] = c
	}
	return out
}

func twoSeats(w *sim.World) *view.Frame {
	f := frameOf(0, w)
	f.Seats = []view.Seat{{Player: 0, Monarch: 0}, {Player: 1, Monarch: 1}}
	return f
}

// Die Rechnung des Zeichenpfads legt nach dem ersten Bild nichts mehr an: Pose (auch mit Umhang und Vierbeiner),
// Größe und Sicht. Die Zeichenbefehle selbst prüft die Messung im Fenster (measure_test.go): Vor ebiten.RunGame
// puffert Ebitengine jeden Befehl als Funktion und legt dafür an.
func TestPoseWithoutAllocations(t *testing.T) {
	d := libDrawer(t)
	tower := 1
	w := &sim.World{
		Sites:   []*sim.Site{{ID: 1, X: 52}},
		Players: []*sim.Player{{ID: 1, Index: 0, X: 50, VX: 5, Facing: 1}, {ID: 2, Index: 1, X: 48, VX: -2, Facing: -1}},
		Troops:  []*sim.Troop{{ID: 3, Kind: "archer", X: 52, TowerID: &tower}, {ID: 4, Kind: "peasant", X: 45}},
		Enemies: []*sim.Enemy{{ID: 5, Kind: "wolf", X: 60}, {ID: 6, Kind: "bat", X: 58}, {ID: 7, Kind: "trollKing", Boss: true, X: 62}},
	}
	f := twoSeats(w)
	d.Listen(f)
	c := cells(55)[0]
	var sink art.M
	step := func() {
		d.now += 1.0 / 60
		for _, k := range []key{{0, classPlayer, 1}, {0, classPlayer, 2}, {0, classTroop, 3}, {0, classTroop, 4}, {0, classEnemy, 5}, {0, classEnemy, 7}} {
			fg := d.figs[k]
			if err := d.advance(fg); err != nil {
				t.Fatal(err)
			}
			size, frame := rigSize(fg)
			sink = art.RigView(c.Camera, 1, 2, size, frame, fg.left)
		}
	}
	step()
	if n := testing.AllocsPerRun(20, step); n != 0 {
		t.Errorf("Pose legt %v Objekte je Bild an", n)
	}
	_ = sink
	if d.figs[key{0, classEnemy, 6}].actor != nil {
		t.Error("Fledermaus (requisite) als Rig")
	}
	dst := ebiten.NewImage(cellW, 2*cellH)
	for _, c := range cells(55) {
		d.Draw(dst, f, c) // läuft durch, ohne Panik
	}
}

// Fußpunkt und Größe: Die Rig-Figur steht mit dem Rahmen auf der Boden-Linie und ist so hoch wie in data/paper.json.
func TestRigFootAndSize(t *testing.T) {
	d := libDrawer(t)
	w := &sim.World{Troops: []*sim.Troop{{ID: 1, Kind: "warrior", X: 10}}}
	d.Listen(twoSeats(w))
	fg := d.figs[key{0, classTroop, 1}]
	size, frame := rigSize(fg)
	c := cells(10)[0]
	m := art.RigView(c.Camera, 10, c.ScreenY(fg.lift), size, frame, false)
	top, foot := m.Apply(art.Point{0, frame[1]}), m.Apply(art.Point{0, 0})
	if foot[1] != float64(c.GroundY()) {
		t.Errorf("Fußpunkt y %v, Boden %d", foot[1], c.GroundY())
	}
	wantPx := 102.5 / unitPx * c.Camera.Scale * (-frame[1] / frame[3])
	if math.Abs((foot[1]-top[1])-wantPx) > 0.01 {
		t.Errorf("Höhe bis Rahmen-Oberkante %v px, erwartet %v", foot[1]-top[1], wantPx)
	}
}

// Die Pose folgt nur dem Takt: Ein Drawer, der zwischen den Frames gar nicht, und einer, der mehrmals und mit der
// Figur mal außerhalb, mal innerhalb der Zelle zeichnet, haben nach denselben Frames dieselbe Pose.
func TestPoseIndependentOfDraws(t *testing.T) {
	a, b := libDrawer(t), libDrawer(t)
	w := &sim.World{Players: []*sim.Player{{ID: 1, Index: 0, X: 50, VX: 5, Facing: 1}}}
	dst := ebiten.NewImage(cellW, 2*cellH)
	for i := range 40 {
		f := twoSeats(w)
		f.Clock = float64(i) / 30
		w.Players[0].X += 0.2
		a.Listen(f)
		b.Listen(f)
		for k := range i % 3 {
			b.Draw(dst, f, cells(float64(50 + k*40))[0]) // k = 1, 2: Figur außerhalb der Zelle
		}
	}
	fa, fb := a.figs[key{0, classPlayer, 1}].actor, b.figs[key{0, classPlayer, 1}].actor
	if fa.T != fb.T || fa.Phase() != fb.Phase() {
		t.Errorf("Pose hängt von Draw ab: T %v / %v, Phase %v / %v", fa.T, fb.T, fa.Phase(), fb.Phase())
	}
}
