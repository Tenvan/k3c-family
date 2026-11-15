package hud

import (
	"slices"
	"strconv"
	"strings"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// B-919/AC-02: Das Geldsack-Icon ist doppelt so groß wie ein Ressourcen-Icon, in jeder Größe 1 bis 3.
func TestPurseSideDoppeltSoGrossWieRessourcenIcon(t *testing.T) {
	for _, px := range []float64{14, 28, 42} {
		if got, res := purseSide(px), px*iconScale; got != 2*res {
			t.Errorf("Schrift %v: Sack %v, Ressourcen-Icon %v", px, got, res)
		}
	}
	if !(purseScale(1) < purseScale(2) && purseScale(2) < purseScale(3) && purseScale(3) <= 1) {
		t.Errorf("drei wachsende Größen: %v", purseFill)
	}
	if purseScale(0) != purseScale(1) || purseScale(9) != purseScale(3) {
		t.Error("Größe außerhalb 1 bis 3 wird begrenzt")
	}
}

// B-919/AC-01: Die Spielerkarte hat weder Lebensbalken noch Spielermarke; sie ist so hoch wie der Sack und trägt das Gold.
func TestCardOhneHealthbarUndMarke(t *testing.T) {
	_, f := twoPlayers(t, 0)
	f.Settings = view.DefaultSettings()
	h := New(nil).(*hud)
	h.Listen(f)
	st := &h.seats[0]
	k := 1.0
	w, ht := h.cardSize(st, nil, 0, k)
	side := purseSide(fontPx(roleShared, h.cells) * k)
	if ht < side || w < side {
		t.Errorf("Karte %v×%v kleiner als der Sack %v", w, ht, side)
	}
	if strings.Contains(st.goldS, "P1") {
		t.Errorf("Goldtext ohne Spielermarke: %q", st.goldS)
	}
}

// B-922/AC-02: Das HUD zeigt die gesetzte Größe und die Münz-Obergrenze des Spielers (sim.PurseMaxOf).
func TestPurseGroesseUndObergrenzeAusDerSim(t *testing.T) {
	w, f := twoPlayers(t, 0)
	f.Settings = view.DefaultSettings()
	h := New(nil).(*hud)
	h.Listen(f)
	p := w.Players[0]
	if st := &h.seats[0]; st.purse != 1 || !strings.HasSuffix(st.goldS, "/"+strconv.Itoa(sim.PurseMaxOf(w, p))) {
		t.Errorf("ohne Skill: Größe %d, Gold %q, Obergrenze %d", st.purse, st.goldS, sim.PurseMaxOf(w, p))
	}
	for i, id := range []string{"purseMedium", "purseLarge"} {
		p.Skills = append(p.Skills, id)
		h.Listen(f)
		st, max := &h.seats[0], sim.PurseMaxOf(w, p)
		if st.purse != i+2 || !strings.HasSuffix(st.goldS, "/"+strconv.Itoa(max)) {
			t.Errorf("%s: Größe %d (erwartet %d), Gold %q, Obergrenze %d", id, st.purse, i+2, st.goldS, max)
		}
	}
	if sim.PurseSize(w, p) != 3 {
		t.Errorf("Größe der Sim %d", sim.PurseSize(w, p))
	}
}

// Das Skill-Menü führt die Linie „Geldsack“ mit beiden Skills hinter den anderen Linien und hat Zeilen dafür.
func TestSkillMenuGeldsackLinie(t *testing.T) {
	if lineIndex("purse") >= len(skillLines) {
		t.Fatal("Linie purse fehlt in monarch.json › lines")
	}
	var ids []string
	for _, e := range menuEntries(nil, nil, 1, func(string) bool { return true }, false) {
		if e.line == "purse" {
			ids = append(ids, e.id)
			if txt := entryText("de", e, false, ""); !strings.Contains(txt, "Geldsack") || !strings.Contains(txt, "Purse") {
				t.Errorf("Zeile %q ohne Linie und Namen", txt)
			}
		}
	}
	if !slices.Equal(ids, []string{"purseMedium", "purseLarge"}) {
		t.Errorf("Skills der Linie purse: %v", ids)
	}
}
