package sim

import "testing"

// B-922/AC-01: Geldsack in drei Größen als Skill-Stufen: ohne Skill Größe 1 (Grundwert), purseMedium Größe 2,
// purseLarge Größe 3 und erst nach purseMedium; jede Größe ist die Münz-Obergrenze des Spielers.
func TestPurseSizes(t *testing.T) {
	isl := mustIsland(t, "geldsack", []int{0})
	w := isl.Stages[0]
	p, q := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 0)
	addPoolPoints(w, 5)
	base := economy.Purse.MaxGold
	if PurseSize(w, p) != 1 || PurseMaxOf(w, p) != base {
		t.Fatalf("ohne Skill: Größe %d, Grenze %d", PurseSize(w, p), PurseMaxOf(w, p))
	}
	if err := LearnSkill(w, p, "purseLarge"); err == nil {
		t.Fatal("purseLarge ohne purseMedium gelernt")
	}
	for i, id := range []string{"purseMedium", "purseLarge"} {
		if err := LearnSkill(w, p, id); err != nil {
			t.Fatal(err)
		}
		want := w.values().skillRef(id).Effect.MaxGold
		if PurseSize(w, p) != i+2 || PurseMaxOf(w, p) != want || want <= base {
			t.Fatalf("%s: Größe %d, Grenze %d, erwartet %d", id, PurseSize(w, p), PurseMaxOf(w, p), want)
		}
	}
	if PurseSize(w, q) != 1 || PurseMaxOf(w, q) != base {
		t.Fatalf("Mitspieler ohne Skill: Größe %d, Grenze %d", PurseSize(w, q), PurseMaxOf(w, q))
	}
	p.Gold, q.Gold = base, base
	giveGold(w, p, 1000, p.X)
	giveGold(w, q, 1000, q.X)
	if p.Gold != PurseMaxOf(w, p) || q.Gold != base {
		t.Fatalf("Gold p %d, q %d", p.Gold, q.Gold)
	}
	// Review FB2: nach Respec liegt der Beutel über der Grenze; neues Gold fällt zu Boden, der Beutel bleibt.
	full, coins := p.Gold, len(w.Coins)
	if err := Respec(w, p); err != nil {
		t.Fatal(err)
	}
	giveGold(w, p, 5, p.X)
	if p.Gold != full || len(w.Coins) != coins+5 {
		t.Fatalf("nach Respec: Gold %d (vorher %d), Münzen +%d", p.Gold, full, len(w.Coins)-coins)
	}
}
