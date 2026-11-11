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

// without liefert die Beiträge ohne die Rolle, die drop erkennt (Teile bleiben sonst unverändert).
func without(ps []Part, drop func(Part) bool) []Part {
	return slices.DeleteFunc(slices.Clone(ps), drop)
}

// partWith liefert den ersten Beitrag, auf den has zutrifft.
func partWith(t *testing.T, ps []Part, has func(Part) bool) Part {
	t.Helper()
	i := slices.IndexFunc(ps, has)
	if i < 0 {
		t.Fatal("kein Beitrag mit dieser Rolle")
	}
	return ps[i]
}

func hasMenus(p Part) bool  { return p.Menus != nil }
func hasSource(p Part) bool { return p.Source != nil }

// Assemble lehnt ab: ungültige oder fehlende Ebene, zweite Eingabe, zweites Menü, fehlende Eingabe oder fehlendes
// Menü, zweite Kamera-Regel. Ein Drawer, der Listen kann, hört mit (genau ein Listener mehr).
func TestAssembleRejects(t *testing.T) {
	base := Packages(nil)
	p0, err := Assemble(base)
	if err != nil {
		t.Fatal(err)
	}
	rejects := map[string][]Part{
		"ungültige Ebene": append(slices.Clone(base), Part{Name: "kaputt", Drawers: []view.Drawer{badDrawer{}}}),
		"Drawer nil":      append(slices.Clone(base), Part{Name: "leer", Drawers: []view.Drawer{nil}}),
		"zweites Menü":    append(slices.Clone(base), partWith(t, base, hasMenus)),
		"zweite Eingabe":  append(slices.Clone(base), partWith(t, base, hasSource)),
		"ohne Menü":       without(base, hasMenus),
		"ohne Eingabe":    without(base, hasSource),
		"zweite Kamera":   append(slices.Clone(base), Part{Name: "welt2", Listener: &rolePlayer{}}),
	}
	for name, ps := range rejects {
		if _, err := Assemble(ps); err == nil {
			t.Errorf("%s angenommen", name)
		}
	}
	p, err := Assemble(append(slices.Clone(base), Part{Name: "fx2", Drawers: []view.Drawer{listeningDrawer{}}}))
	if err != nil || len(p.Listeners) != len(p0.Listeners)+1 {
		t.Errorf("Drawer mit Listen hört nicht mit: %d Listener statt %d, %v", len(p.Listeners), len(p0.Listeners)+1, err)
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

// contributions liefert alle Beiträge der Pakete (Drawer, Listener, Source, Menus) für die Rollen-Prüfung.
func contributions(ps []Part) []any {
	var out []any
	for _, p := range ps {
		for _, d := range p.Drawers {
			out = append(out, d)
		}
		for _, x := range []any{p.Listener, p.Source, p.Menus} {
			if x != nil {
				out = append(out, x)
			}
		}
	}
	return out
}

// countRole zählt die Beiträge, die die Rolle T erfüllen.
func countRole[T any](ps []Part) int {
	n := 0
	for _, x := range contributions(ps) {
		if _, ok := x.(T); ok {
			n++
		}
	}
	return n
}

// Genau ein Beitrag der echten Pakete führt die Kamera (world); jeder Beitrag wird für jede Rolle angemeldet, die er
// erfüllt. Ohne die Kamera-Regel von world übernimmt die eines anderen Pakets.
func TestAssembleRoles(t *testing.T) {
	base := Packages(nil)
	if n := countRole[view.CameraRule](base); n != 1 {
		t.Fatalf("%d Kamera-Regeln in den Paketen, erwartet genau eine", n)
	}
	p, err := Assemble(base)
	if err != nil || p.Camera == nil {
		t.Fatalf("Kamera %v, %v", p.Camera, err)
	}
	assets, overlays := countRole[view.AssetSource](base), countRole[view.Overlay](base)
	if len(p.Assets) != assets || len(p.Overlays) != overlays {
		t.Errorf("Rollen: Assets %d statt %d, Overlays %d statt %d", len(p.Assets), assets, len(p.Overlays), overlays)
	}
	for i := range base {
		base[i].Drawers = slices.DeleteFunc(slices.Clone(base[i].Drawers), func(d view.Drawer) bool {
			_, ok := d.(view.CameraRule)
			return ok
		})
	}
	extra := append(base, Part{Name: "hud2", Listener: &rolePlayer{}})
	p, err = Assemble(extra)
	if err != nil {
		t.Fatal(err)
	}
	if len(p.Assets) != countRole[view.AssetSource](extra) || len(p.Overlays) != countRole[view.Overlay](extra) {
		t.Errorf("Rollen mit rolePlayer: Assets %d, Overlays %d", len(p.Assets), len(p.Overlays))
	}
	if _, ok := p.Camera.(*rolePlayer); !ok {
		t.Errorf("Kamera-Regel %T, erwartet *rolePlayer", p.Camera)
	}
}
