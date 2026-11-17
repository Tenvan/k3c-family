package paint

// Referenz-Netze für den Rust-Port (rust/crates/k3c-rig/src/paint, RU5.4): `go test ./paint -run RustRef -update`
// schreibt testdata/rust/rig/paint/figures.json. Je Figur und Pose stehen die Zeichen-Aufrufe in Reihenfolge darin:
// Teile und Elemente als Bild mit Matrix, Bänder und Zügel als Dreiecksnetz (Ecken, Indizes). Zustand, Pose, Umhang-
// Physik und Netze rechnen der echte Actor und Painter; nur die Verteilung der Aufrufe (Actor.Draw: drawMount,
// drawRider) ist hier nachgebildet, weil sich der Ziel-Image von Ebitengine nicht abhören lässt, ohne Produktivcode zu
// ändern. Der Go-Code selbst bleibt unverändert.

import (
	"encoding/json"
	"flag"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-rig/rig"
)

var updateRef = flag.Bool("update", false, "Referenz-Netze für Rust nach testdata/rust/rig/paint schreiben")

type rec = map[string]any

// refRecorder ruft Painter und hält fest, was er an Ebitengine gäbe.
type refRecorder struct {
	p     *Painter
	names map[*ebiten.Image]string
	calls []rec
}

func (r *refRecorder) image(img *ebiten.Image, m rig.M, offset rig.Point, density float64) {
	r.p.image(img, m, offset, density)
	g := &r.p.op.GeoM
	r.calls = append(r.calls, rec{"kind": "image", "tex": r.names[img], "w": img.Bounds().Dx(), "h": img.Bounds().Dy(),
		"geom":  []float64{g.Element(0, 0), g.Element(0, 1), g.Element(1, 0), g.Element(1, 1), g.Element(0, 2), g.Element(1, 2)},
		"alpha": r.p.alpha()})
}

func (r *refRecorder) mesh(img *ebiten.Image) {
	verts := make([][8]float32, len(r.p.verts))
	for i, v := range r.p.verts {
		verts[i] = [8]float32{v.DstX, v.DstY, v.SrcX, v.SrcY, v.ColorR, v.ColorG, v.ColorB, v.ColorA}
	}
	r.calls = append(r.calls, rec{"kind": "mesh", "tex": r.names[img], "verts": verts, "idx": append([]uint16{}, r.p.idx...)})
}

func (r *refRecorder) band(img *ebiten.Image, b *rig.Band, pts []rig.Point, m rig.M, density float64) {
	r.p.band(img, b, pts, m, density)
	r.mesh(img)
}

// drawMount und drawRider bilden Actor.drawMount und Actor.drawRider nach (gleiche Reihenfolge und Bedingungen).
func (r *refRecorder) drawMount(a *Actor) {
	m, s := a.Mount, a.mount
	for _, it := range m.Order {
		switch it.Kind {
		case rig.ItemPart:
			r.image(s.parts[it.Index], a.ride.MountWorld[it.Index], m.Parts[it.Index].Offset, m.Density)
		case rig.ItemBand:
			a.legs = m.LegPoints(it.Index, a.ride.Pose.Angles, a.legs)
			r.band(s.bands[it.Index], &m.Bands[it.Index], a.legs, a.ride.MountWorld[s.bandPart[it.Index]], m.Density)
		case rig.ItemSlot:
			if a.Rider != nil && m.Slots[it.Index].Name == "sattel" {
				r.drawRider(a, a.ride.RiderWorld, a.ride.Base, true)
			}
		case rig.ItemReins:
			if re := m.Reins; a.Rider != nil {
				if hand, ctrl, mouth, ok := rig.ReinPoints(m, a.ride.MountWorld, a.Rider, a.ride.RiderWorld, a.ride.Base); ok {
					r.p.curve(hand, ctrl, mouth, re.Width, parseColor(re.Color), re.Opacity)
					r.mesh(r.p.white)
				}
			}
		}
	}
}

func (r *refRecorder) drawRider(a *Actor, world []rig.M, cloakBase rig.M, mounted bool) {
	ri, s := a.Rider, a.skin
	for _, it := range ri.Order {
		switch it.Kind {
		case rig.ItemPart:
			if !mounted || !ri.Hidden(it.Index) {
				r.image(s.parts[it.Index], world[it.Index], ri.Parts[it.Index].Offset, ri.Density)
			}
		case rig.ItemBand:
			if len(a.chain) == len(ri.Bands[it.Index].Rest) {
				r.band(s.bands[it.Index], &ri.Bands[it.Index], a.chain, cloakBase, ri.Density)
			}
		case rig.ItemSlot:
			if e := a.elementAt(ri.Slots[it.Index].Name); e != nil {
				r.image(e.img, ri.SlotWorld(&ri.Slots[it.Index], world), e.Offset, e.Density)
			}
		}
	}
}

// refRigs nennt die Ordner aller Rigs (relativ zu assets/, mit /, sortiert).
func refRigs(t *testing.T) []string {
	var dirs []string
	root := filepath.Join("..", "assets")
	err := filepath.WalkDir(root, func(p string, d fs.DirEntry, err error) error {
		if err == nil && !d.IsDir() && d.Name() == "rig.json" {
			rel, _ := filepath.Rel(root, filepath.Dir(p))
			dirs = append(dirs, filepath.ToSlash(rel))
		}
		return err
	})
	if err != nil || len(dirs) == 0 {
		t.Fatalf("keine Rigs: %v", err)
	}
	sort.Strings(dirs)
	return dirs
}

func sortedKeys[V any](m map[string]V) []string {
	keys := make([]string, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	return keys
}

// TestRustRefPaint: Referenz-Netze je Figur (zu Fuß, Vierbeiner allein, Reiter auf Reittier) und Pose.
func TestRustRefPaint(t *testing.T) {
	if !*updateRef {
		t.Skip("Referenz-Netze schreibt nur -update")
	}
	a, err := LoadAssets(filepath.Join("..", "assets"))
	if err != nil {
		t.Fatal(err)
	}
	dirs := refRigs(t)
	rider, err := a.Rig("figuren/monarch")
	if err != nil {
		t.Fatal(err)
	}
	var out []rec
	run := func(id, riderDir, mountDir, variant string, state State, weapon, armor string) {
		ac := mustActor(t, a, riderDir, mountDir, variant, state)
		ac.State, ac.Weapon, ac.Armor = state, weapon, armor
		for range 24 {
			if err := ac.Update(1.0 / 60); err != nil {
				t.Fatalf("%s: %v", id, err)
			}
		}
		p := NewPainter(a.White)
		p.Dst, p.View, p.Alpha = ebiten.NewImage(8, 8), rig.Mul(rig.Translate(100, 200), rig.Scale(1.5)), 0.8
		r := &refRecorder{p: p, names: map[*ebiten.Image]string{a.White: "white"}}
		for name, img := range a.Images {
			r.names[img] = name
		}
		if ac.riding() {
			r.drawMount(ac)
		} else {
			r.drawRider(ac, ac.world, rig.Identity, false)
		}
		out = append(out, rec{"id": id, "rider": riderDir, "mount": mountDir, "variant": variant, "name": state.Name,
			"gait": state.Gait, "weapon": weapon, "armor": armor, "calls": r.calls})
	}
	for _, dir := range dirs {
		r, _ := a.Rig(dir)
		variants := append([]string{""}, sortedKeys(r.Variants)...)
		for vi, variant := range variants {
			for ai, name := range sortedKeys(r.Animations) {
				if vi > 0 && ai > 0 {
					continue
				}
				armor := ""
				if len(a.Armors) > 1 && r.Slot("ruestung") != nil {
					armor = a.Armors[1]
				}
				id := fmt.Sprintf("%s@%s/%s", dir, variant, name)
				switch r.Kind {
				case "figur":
					run(id, dir, "", variant, State{Name: name}, r.Default["waffe"], armor)
				default:
					if g := r.Animations[name].Gait; g != "" {
						run(id, "", dir, variant, State{Name: name, Gait: g}, "", "")
					}
				}
			}
		}
		if r.Kind == "vierbeiner" && r.Slot("sattel") != nil {
			for _, g := range []string{"stand", "schritt", "trab", "galopp"} {
				run(fmt.Sprintf("%s+monarch/%s", dir, g), "figuren/monarch", dir, "", State{Name: g, Gait: g}, rider.Default["waffe"], "")
			}
		}
	}
	writeRef(t, out)
}

func mustActor(t *testing.T, a *Assets, riderDir, mountDir, variant string, state State) *Actor {
	t.Helper()
	var rr, mm *rig.Rig
	var err error
	if riderDir != "" {
		if rr, err = a.Rig(riderDir); err != nil {
			t.Fatal(err)
		}
	}
	if mountDir != "" {
		if mm, err = a.Rig(mountDir); err != nil {
			t.Fatal(err)
		}
	}
	ac := &Actor{assets: a, Rider: rr, Mount: mm, Variant: variant, Tempo: 1, State: state}
	if err := ac.Refresh(); err != nil {
		t.Fatal(err)
	}
	return ac
}

func writeRef(t *testing.T, v any) {
	t.Helper()
	dir := filepath.Join("..", "..", "..", "testdata", "rust", "rig", "paint")
	text, err := json.Marshal(v)
	if err == nil {
		err = os.MkdirAll(dir, 0o755)
	}
	if err == nil {
		err = os.WriteFile(filepath.Join(dir, "figures.json"), append(text, '\n'), 0o644)
	}
	if err != nil {
		t.Fatal(err)
	}
}
