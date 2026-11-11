package game

import (
	"slices"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func TestEveryPackageRegistered(t *testing.T) {
	p, err := Assemble(Packages(nil))
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
	base := Packages(nil)
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
	if _, err := Assemble(append(slices.Clone(base), Part{Name: "welt2", Listener: &rolePlayer{}})); err == nil {
		t.Error("zweite Kamera-Regel angenommen")
	}
}

// rolePlayer erfüllt alle optionalen Rollen (Kamera-Regel, Asset-Quelle, Overlay) und hört mit.
type rolePlayer struct{}

func (*rolePlayer) Listen(*view.Frame)                                           {}
func (*rolePlayer) Follow(_ *view.Frame, c view.Cell, _ view.Camera) view.Camera { return c.Camera }
func (*rolePlayer) Assets(string, *sim.World) []string                           { return nil }
func (*rolePlayer) Update(view.Input, *view.Frame, view.Session) view.OverlayStep {
	return view.OverlayStep{}
}

// Die Kamera-Regel kommt aus world; jeder Beitrag wird für die Rollen angemeldet, die er erfüllt.
func TestAssembleRoles(t *testing.T) {
	base := Packages(nil)
	p, err := Assemble(base)
	if err != nil || p.Camera == nil || len(p.Assets) != 0 || len(p.Overlays) != 0 {
		t.Fatalf("Platzhalter: Kamera %v, Assets %d, Overlays %d, %v", p.Camera, len(p.Assets), len(p.Overlays), err)
	}
	for i := range base {
		if base[i].Name == "world" {
			base[i].Drawers = base[i].Drawers[1:] // ohne die Kamera-Regel
		}
	}
	p, err = Assemble(append(base, Part{Name: "hud2", Listener: &rolePlayer{}}))
	if err != nil || p.Camera == nil || len(p.Assets) != 1 || len(p.Overlays) != 1 {
		t.Errorf("Rollen: Kamera %v, Assets %d, Overlays %d, %v", p.Camera, len(p.Assets), len(p.Overlays), err)
	}
}
