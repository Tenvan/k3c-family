package hud

// Diorama der Seekarte (B-964, GS2.2): Aussehen je Insel, Zustände, Rückfall auf Formen, Lage in der Zelle. Die
// Zeichenbefehle selbst misst die Aufnahme im Fenster (seekarte_shot_test.go), vor ebiten.RunGame zählen sie nicht.

import (
	"slices"
	"testing"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

const repoGrafik = "../../../public/grafik"

// Welt wie seaRoute, mit Aussehen: a klein mit 1 Stufe, b mittel mit 3, c sehr groß mit 9, d ohne Angaben.
var paperRoute = []view.RouteIsland{
	{ID: "a", Name: "A", Etappe: 1, World: "sengoku", Stages: 1, Chunks: 16},
	{ID: "b", Name: "B", Etappe: 2, World: "sengoku", Stages: 3, Chunks: 40},
	{ID: "c", Name: "C", Etappe: 2, World: "sengoku", Stages: 9, Chunks: 100},
	{ID: "d", Name: "D", Etappe: 3, World: "gibt-es-nicht"},
}

func nodeOf(t *testing.T, v *seaView, id string) seaNode {
	t.Helper()
	for _, r := range v.rows {
		for _, n := range r.nodes {
			if n.id == id {
				return n
			}
		}
	}
	t.Fatalf("Insel %s fehlt", id)
	return seaNode{}
}

// AC-02 (B-964/R2): Größe aus der Inselgröße, Schichten aus der Stufenzahl, Aufsatz aus dem Hash der ID; bei beiden
// Spielern und nach jedem Neubau gleich.
func TestSeekarteAussehenJeInsel(t *testing.T) {
	var p0, p1 seaView
	p0.refresh("de", view.ProfileWASD, openSwitch(""), paperRoute)
	p1.refresh("en", view.ProfileGamepad, openSwitch("c"), paperRoute)
	for _, isl := range paperRoute {
		a, b := nodeOf(t, &p0, isl.ID), nodeOf(t, &p1, isl.ID)
		if a.look != b.look || a.look != lookOf(isl) || a.look.world != isl.World {
			t.Errorf("%s: Spieler 0 %+v, Spieler 1 %+v", isl.ID, a.look, b.look)
		}
	}
}

// AC-02 (B-964/R2): Werte des Aussehens: Maßstab zwischen kleinster und größter Inselgröße, Schichten je Stufe
// (begrenzt), fester Hash, alle Aufsätze kommen vor.
func TestSeekarteAussehenWerte(t *testing.T) {
	a, b, c, d := lookOf(paperRoute[0]), lookOf(paperRoute[1]), lookOf(paperRoute[2]), lookOf(paperRoute[3])
	if a.scale != seaScaleMin || c.scale != seaScaleMax || a.scale >= b.scale || b.scale >= c.scale {
		t.Errorf("Maßstab a %v, b %v, c %v", a.scale, b.scale, c.scale)
	}
	if d.scale != (seaScaleMin+seaScaleMax)/2 || seaScale(8) != seaScaleMin || seaScale(500) != seaScaleMax {
		t.Errorf("Maßstab ohne Angabe %v, zu klein %v, zu groß %v", d.scale, seaScale(8), seaScale(500))
	}
	if a.layers != 1 || b.layers != 3 || c.layers != seaLayersMax || d.layers != 1 {
		t.Errorf("Schichten a %d, b %d, c %d, d %d", a.layers, b.layers, c.layers, d.layers)
	}
}

// AC-02 (B-964/R2): Der Aufsatz hängt an einem festen Hash der Insel-ID (FNV-1a), kein Zufall; alle Aufsätze kommen vor.
func TestSeekarteAufsatzAusHash(t *testing.T) {
	if seaSeed("a") != 0xe40c292c || seaSeed("") != 2166136261 || seaSeed("a") == seaSeed("b") {
		t.Errorf("Hash a %#x, leer %#x", seaSeed("a"), seaSeed(""))
	}
	m := &art.SeaMap{Toppers: []string{"turm", "baum", "fels"}}
	seen := map[string]bool{}
	for _, id := range []string{"a", "b", "c", "d", "kyoto", "theben", "avalon", "hafen"} {
		seen[m.Topper(seaSeed(id))] = true
	}
	if len(seen) != 3 {
		t.Errorf("Aufsätze über acht Inseln: %v", seen)
	}
}

// AC-02 (B-964/R3): Zustände aus dem Wechselzustand: Schiff an der aktuellen Insel (die Simulation nennt sie als letzte
// der Abgeschlossenen), abgeschlossen, wählbar, gesperrt; Cursor nur auf Zielen.
func TestSeekarteZustaendeDiorama(t *testing.T) {
	var v seaView
	sw := openSwitch("c")
	sw.Completed = []string{"x", "a"}
	v.refresh("de", view.ProfileWASD, sw, paperRoute)
	for id, want := range map[string]seaNode{
		"a": {kind: seaDone, current: true}, "b": {kind: seaTarget}, "c": {kind: seaTarget, chosen: true, cursor: true},
		"d": {kind: seaLocked},
	} {
		n := nodeOf(t, &v, id)
		if n.kind != want.kind || n.current != want.current || n.chosen != want.chosen || n.cursor != want.cursor {
			t.Errorf("%s: %+v", id, n)
		}
	}
	sw.Completed = nil // ohne Abgeschlossene liegt das Schiff nirgends
	v.refresh("de", view.ProfileWASD, sw, paperRoute)
	if slices.ContainsFunc(v.rows, func(r seaRow) bool {
		return slices.ContainsFunc(r.nodes, func(n seaNode) bool { return n.current })
	}) {
		t.Error("Schiff ohne aktuelle Insel")
	}
}

// AC-03 (B-964/R5): Ohne Grafik, ohne Zuordnung oder mit einem nicht ladbaren Asset bleibt die Karte aus Formen und das
// HUD meldet nur die Icons.
func TestSeekarteRueckfallOhneAssets(t *testing.T) {
	var v seaView
	v.refresh("de", view.ProfileWASD, openSwitch(""), paperRoute)
	m := art.DataSeaMap()
	if m == nil {
		t.Fatal("data/paper.json › seekarte nicht lesbar")
	}
	h := &hud{}
	if _, ok := h.seaArtOf(&v); ok {
		t.Error("Diorama ohne Zuordnung")
	}
	h.useSeaMap(nil, func(string) bool { return true })
	h.useSeaMap(m, func(a string) bool { return a != m.Ship })
	if h.seaMap != nil || !slices.Equal(h.Assets("forest", nil), hudAssets) {
		t.Errorf("Asset nicht ladbar: Zuordnung %v, Assets %v", h.seaMap != nil, h.Assets("forest", nil))
	}
	h.useSeaMap(m, func(string) bool { return true })
	got := h.Assets("forest", nil)
	if h.seaMap != m || !slices.IsSorted(got) || len(got) != len(hudAssets)+len(m.Assets()) || !slices.Contains(got, m.Ship) {
		t.Errorf("ladbar: Assets %v", got)
	}
	if _, ok := h.seaArtOf(&v); ok { // Zuordnung da, aber keine Bibliothek: Formen
		t.Error("Diorama ohne Bibliothek")
	}
	if New(nil).(*hud).seaMap != nil {
		t.Error("New ohne Grafik nimmt die Seekarten-Assets")
	}
}

// seaLib: Bibliothek mit den geladenen Assets des HUD aus dem Repo; ohne gerenderte Grafik übersprungen.
func seaLib(tb testing.TB) *hud {
	tb.Helper()
	lib, err := art.New(art.Config{GrafikDir: repoGrafik})
	if err != nil {
		tb.Fatal(err)
	}
	h := New(lib).(*hud)
	if h.seaMap == nil {
		tb.Skip("Grafik fehlt (Sheets erst mit `task gen:paper`)")
	}
	if err := lib.SetLevels(map[string][]string{"forest": h.Assets("forest", nil)}); err != nil {
		tb.Skipf("Grafik fehlt (Sheets erst mit `task gen:paper`): %v", err)
	}
	return h
}

// AC-02, AC-03 (B-964/R5, R6): Mit geladenen Assets gilt das Diorama, auch für eine Welt ohne eigene Miniatur; fehlt ein
// Sheet (anderes Level), gelten die Formen. Auswahl und Lage rechnen ohne Allokation.
func TestSeekarteDioramaMitAssets(t *testing.T) {
	h := seaLib(t)
	v := &h.seats[0].sea
	v.refresh("de", view.ProfileWASD, openSwitch(""), paperRoute)
	a, ok := h.seaArtOf(v)
	if !ok || a.back == a.mid || a.mid == a.front || a.ship == nil || a.flag == nil || a.fog == nil {
		t.Fatalf("Diorama nicht gewählt: %v %+v", ok, a)
	}
	if std, own := h.seaSheet(h.seaMap.Island("gibt-es-nicht")), h.seaSheet(h.seaMap.Island("sengoku")); std == nil || std == own ||
		std != h.seaSheet(h.seaMap.Island(art.StandardWorld)) {
		t.Error("Welt ohne Eintrag nutzt nicht die Miniatur von standard")
	}
	cell := rect{0, 540, 1920, 540}
	if n := testing.AllocsPerRun(50, func() {
		_, _ = h.seaArtOf(v)
		_ = seaPaperPlan(v.sizes, len(v.rows), cell, 1)
	}); n != 0 {
		t.Errorf("%v Allokationen je Bild", n)
	}
	if err := h.lib.SetLevels(map[string][]string{"forest": hudAssets}); err != nil {
		t.Fatal(err)
	}
	if _, ok := h.seaArtOf(v); ok {
		t.Error("Diorama ohne geladene Sheets")
	}
}

// AC-02 (Split-Screen): Das Diorama bleibt in jeder Zelle, alle Inseln und Texte liegen in der Tafel, Etappen stehen
// von links nach rechts, Reihen von hinten nach vorn.
func TestSeekarteDioramaLage(t *testing.T) {
	m := seaSizes{labW: 120, nameW: 200, titleW: 180, hintW: 520, maxNodes: 4}
	for _, c := range []struct {
		cell rect
		k    float64
	}{{rect{0, 0, 1920, 1080}, 1}, {rect{0, 0, 1920, 540}, 1}, {rect{0, 540, 1920, 540}, 1}, {rect{0, 360, 1280, 360}, 2. / 3}, {rect{0, 0, 640, 360}, 1. / 3}} {
		p := seaPaperPlan(m, 3, c.cell, c.k)
		if !p.box.inside(c.cell) || p.u <= 0 || p.u > c.k {
			t.Errorf("Zelle %+v: Tafel %+v, u %v", c.cell, p.box, p.u)
		}
		if p.front+seaLayerH*p.u > p.box.Y+p.box.H+0.01 || p.water-seaTopH*p.u < p.box.Y+2.5*p.b {
			t.Errorf("Zelle %+v: Meer ragt aus der Tafel (%v … %v in %+v)", c.cell, p.water, p.front, p.box)
		}
		for col := range 3 {
			for lane := range 4 {
				x, half := p.nodeX(col, lane, 4), 120*seaScaleMax*p.u
				if x-half < p.box.X || x+half > p.box.X+p.box.W {
					t.Errorf("Zelle %+v: Insel %d/%d bei %v außerhalb", c.cell, col, lane, x)
				}
				if col > 0 && p.nodeX(col, lane, 4) <= p.nodeX(col-1, lane, 4) {
					t.Errorf("Etappen nicht von links nach rechts")
				}
			}
		}
		if p.waterOf(1) <= p.waterOf(0) || p.front <= p.waterOf(3) {
			t.Errorf("Reihen nicht von hinten nach vorn")
		}
	}
}

// AC-03 (B-964/R6): Neubau, Cursor und Eingabe der Karte mit Aussehen bleiben ohne Allokation je Bild.
func TestSeekarteDioramaOhneAllokationen(t *testing.T) {
	var v seaView
	sw := openSwitch("b")
	v.refresh("de", view.ProfileWASD, sw, paperRoute)
	acts := []view.Action{view.ActionRight}
	if n := testing.AllocsPerRun(50, func() {
		v.refresh("de", view.ProfileWASD, sw, paperRoute)
		v.step(acts)
		_ = lookOf(paperRoute[2])
	}); n != 0 {
		t.Errorf("%v Allokationen je Bild", n)
	}
}
