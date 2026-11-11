package game

import (
	"slices"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

func TestEveryPackageRegistered(t *testing.T) {
	p, err := Assemble(Packages())
	if err != nil {
		t.Fatal(err)
	}
	names := slices.Sorted(slices.Values(p.Names))
	if !slices.Equal(names, fachpakete) {
		t.Errorf("Pakete %v, erwartet %v", names, fachpakete)
	}
	if p.Input == nil || p.Menus == nil || len(p.Listeners) == 0 {
		t.Error("Eingabe, Menü oder Listener fehlt")
	}
	for i := 1; i < len(p.Drawers); i++ {
		if p.Drawers[i-1].Layer() > p.Drawers[i].Layer() {
			t.Fatalf("Drawer nicht nach Ebene sortiert: %s vor %s", p.Drawers[i-1].Layer(), p.Drawers[i].Layer())
		}
	}
	// Die Platzhalter decken alle Zellen-Ebenen ab (Overlay gehört vorerst der Schleife: FPS in NG1.4).
	for _, l := range view.Layers() {
		if l.PerCell() && !slices.ContainsFunc(p.Drawers, func(d view.Drawer) bool { return d.Layer() == l }) {
			t.Errorf("keine Platzhalter auf Ebene %s", l)
		}
	}
}

type badDrawer struct{}

func (badDrawer) Layer() view.Layer                                { return view.Layer(77) }
func (badDrawer) Draw(_ *ebiten.Image, _ *view.Frame, _ view.Cell) {}

type listeningDrawer struct{ badDrawer }

func (listeningDrawer) Layer() view.Layer  { return view.LayerEffects }
func (listeningDrawer) Listen(*view.Frame) {}

func TestAssembleRejects(t *testing.T) {
	base := Packages()
	if _, err := Assemble(append(slices.Clone(base), Part{Name: "kaputt", Drawers: []view.Drawer{badDrawer{}}})); err == nil {
		t.Error("ungültige Ebene angenommen")
	}
	if _, err := Assemble(append(slices.Clone(base), base[len(base)-2])); err == nil {
		t.Error("zweites Menü angenommen")
	}
	if _, err := Assemble(base[:5]); err == nil {
		t.Error("ohne Eingabe und Menü angenommen")
	}
	p, err := Assemble(append(slices.Clone(base), Part{Name: "fx2", Drawers: []view.Drawer{listeningDrawer{}}}))
	if err != nil || len(p.Listeners) != 2 {
		t.Errorf("Drawer mit Listen hört nicht mit: %d Listener, %v", len(p.Listeners), err)
	}
}
