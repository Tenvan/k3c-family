package props

import (
	"image"
	"strings"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// fakeSheets liefert für jedes Asset ein Sheet mit den Maßen aus dem Manifest unter public/grafik (ohne GPU).
type fakeSheets struct {
	dir   string
	cache map[[2]string]*art.Sheet
}

func newFake() fakeSheets { return fakeSheets{dir: grafikDir, cache: map[[2]string]*art.Sheet{}} }

func (f fakeSheets) Sheet(asset, anim string) *art.Sheet {
	key := [2]string{asset, anim}
	if s, ok := f.cache[key]; ok {
		return s
	}
	s := f.read(asset, anim)
	f.cache[key] = s
	return s
}

func (f fakeSheets) read(asset, anim string) *art.Sheet {
	m, err := art.ReadManifest(f.dir, asset)
	if err != nil {
		return nil
	}
	a, ok := m.Anims[anim]
	if !ok {
		return nil
	}
	r := m.Frame
	return &art.Sheet{Asset: asset, Anim: anim, FW: m.FrameW, FH: m.FrameH, FPS: a.Fps,
		OriginX: -r[0] / r[2], OriginY: -r[1] / r[3], UnitsPerPx: r[3] / float64(m.FrameH)}
}

const grafikDir = "../../../public/grafik"

// island baut eine Insel mit einer Stufe (Wald) für Testfälle.
func island(t *testing.T) *sim.World {
	t.Helper()
	isl, err := sim.CreateIsland("wald", []int{0}, 1, sim.CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	return isl.Stages[0]
}

// camera: Zelle 1280 × 360 um x, 27 Pixel je Unit.
func camera(x float64) view.Camera { return view.NewCamera(image.Rect(0, 0, 1280, 360), x, 27) }

// assets der Zeichenliste (Sprites), in Reihenfolge.
func assetsOf(items []item) []string {
	var out []string
	for _, it := range items {
		if it.shape == shapeSprite || it.shape == shapeTiles {
			out = append(out, it.look.asset+":"+it.look.anim)
		}
	}
	return out
}

func TestPlanBurgUndPlaetzeAusDerSimulation(t *testing.T) {
	w := island(t)
	d := New(nil).(*drawer)
	d.plan = newPlanner(newFake())
	info := &stageInfo{}
	info.update(w, 0)
	d.plan.plan(w, info, camera(w.Castle.X), 0)
	got := strings.Join(assetsOf(d.plan.body), " ")
	if !strings.Contains(got, "gebaeude-paper/lagerfeuer:stand") {
		t.Errorf("Burg der Hub-Stufe 1 fehlt: %s", got)
	}
}

func TestPlanGebauteStufenSchadenUndRuine(t *testing.T) {
	w := island(t)
	p := newPlanner(newFake())
	var wall *sim.Site
	for _, s := range w.Sites {
		if s.Kind == "wall" {
			wall = s
			break
		}
	}
	if wall == nil {
		t.Fatal("keine Mauer im Hub")
	}
	*wall = sim.Site{ID: wall.ID, Kind: "wall", X: wall.X, State: "built", Level: 3, HP: 100, MaxHP: 600, Ruin: true}
	p.plan(w, nil, camera(wall.X), 0)
	var sprite *item
	for i, it := range p.body {
		if it.shape == shapeSprite && it.look.asset == "gebaeude-paper/mauer" {
			sprite = &p.body[i]
		}
	}
	if sprite == nil || sprite.look.anim != "stand.s3" || sprite.shade != damageShade[3] {
		t.Fatalf("Mauer Stufe 3, schwer beschädigt: %+v", sprite)
	}
	lines, rects := 0, 0
	for _, it := range p.top {
		switch it.shape {
		case shapeLine:
			lines++
		case shapeRect:
			rects++
		}
	}
	if lines == 0 || rects < len(rubbleStones) {
		t.Errorf("Risse %d, Trümmer und Balken %d", lines, rects)
	}
}

func TestPlanOhneGrafikPlatzhalter(t *testing.T) {
	w := island(t)
	p := newPlanner(nil)
	p.plan(w, nil, camera(w.Castle.X), 0)
	if n := len(assetsOf(p.body)); n != 0 {
		t.Errorf("ohne Grafik %d Bilder", n)
	}
	if len(p.body) == 0 {
		t.Error("keine Platzhalter-Formen")
	}
	p.plan(nil, nil, camera(0), 0)
	if len(p.body)+len(p.top) != 0 {
		t.Error("ohne Welt nichts")
	}
}

func TestPlanNurSichtbares(t *testing.T) {
	w := island(t)
	p := newPlanner(newFake())
	p.plan(w, nil, camera(-10000), 0)
	if len(p.body)+len(p.top) != 0 {
		t.Errorf("weit außerhalb der Welt %d Einträge", len(p.body)+len(p.top))
	}
}

func TestPlanBaustelleMitPreisUndGeruest(t *testing.T) {
	w := island(t)
	p := newPlanner(newFake())
	info := &stageInfo{}
	info.update(w, 0)
	var site *sim.Site
	for i, s := range w.Sites {
		if info.flags[i].payable {
			site = s
			break
		}
	}
	if site == nil {
		t.Fatal("kein bezahlbarer Platz")
	}
	info.tags = []int{site.ID}
	p.plan(w, info, camera(site.X), 0)
	if !hasText(p.top, siteName(site.Kind)) {
		t.Errorf("Preis-Anzeige von %s fehlt", site.Kind)
	}
	site.State, site.BuildProgress = "waitingWorker", 0.5
	info.update(w, 0)
	p.plan(w, info, camera(site.X), 0)
	if !strings.Contains(strings.Join(assetsOf(p.body), " "), "geruest") {
		t.Error("Gerüst im Bau fehlt")
	}
}

func hasText(items []item, part string) bool {
	for _, it := range items {
		if it.shape == shapeText && strings.Contains(it.text, part) {
			return true
		}
	}
	return false
}

func TestPlanOhneAllokationNachDemErstenBild(t *testing.T) {
	w := island(t)
	p := newPlanner(newFake())
	info := &stageInfo{}
	info.update(w, 0)
	k := camera(w.Castle.X)
	p.plan(w, info, k, 0)
	allocs := testing.AllocsPerRun(20, func() { p.plan(w, info, k, 1) })
	if allocs > 0 {
		t.Errorf("%v Allokationen je Bild", allocs)
	}
}

func TestPlanAusbauSchildDerBurg(t *testing.T) {
	w := island(t)
	p := newPlanner(newFake())
	info := &stageInfo{hubLevel: 2, hubStep: 2, stock: [5]int{0, 10, 0, 0, 0},
		hub: hubState{has: true, gold: 50, paid: 10, steps: 3, material: [5]int{0, 100, 0, 0, 0}}}
	info.tags = []int{w.Castle.ID}
	p.plan(w, info, camera(w.Castle.X), 0)
	if !hasText(p.top, "Ausbau 2/3 · 100 Stein") {
		t.Errorf("Angebot fehlt: %v", labelsOf(p.top))
	}
	rings := 0
	for _, it := range p.top {
		if it.shape == shapeRing {
			rings++
		}
	}
	if rings != 50 {
		t.Errorf("50 Münz-Plätze erwartet, %d", rings)
	}
	info.hub.state, info.hub.paid = sim.WaitingNextIsland, 50
	p.plan(w, info, camera(w.Castle.X), 0)
	if !hasText(p.top, "Ausbau 2/3: Tor-Material fehlt (Stein)") {
		t.Errorf("Wartegrund fehlt: %v", labelsOf(p.top))
	}
}

func labelsOf(items []item) []string {
	var out []string
	for _, it := range items {
		if it.shape == shapeText {
			out = append(out, it.text)
		}
	}
	return out
}
