package sim

import "testing"

// Grad „leicht“: die ersten protectedNights Nächte ohne Verlust (S6.1, B-148, AC-02).

// protectState ist, was eine Nacht nicht kosten darf: Gold beider Spieler, Vorrat, Gebäude, Truppen.
type protectState struct {
	gold   [2]int
	stock  Stock
	built  int
	troops int
}

func protectSnapshot(w *World) protectState {
	s := protectState{stock: *w.Stock, troops: len(w.Troops)}
	for i, p := range w.Players[:2] {
		s.gold[i] = p.Gold
	}
	for _, site := range w.Sites {
		if site.State == "built" {
			s.built++
		}
	}
	return s
}

// forcedAttack: Insel mit 2 Spielern im Grad grade, Nacht day; Goldraub an beiden Spielern, tödlicher Schaden an einem
// Gebäude, dann fällt die Burg. Liefert den Stand davor und danach.
func forcedAttack(t *testing.T, grade string, day int) (before, after protectState) {
	t.Helper()
	isl := islandWith(t, "protect", grade, 2)
	w := isl.Stages[0]
	w.Cycle = CycleInfo{Phase: "night", Day: day}
	*w.Stock = Stock{Wood: 10, Stone: 8, Copper: 6}
	for _, p := range w.Players {
		p.Gold = 20
	}
	site := buildSite(t, w, "tower")
	w.Troops = append(w.Troops, &Troop{ID: w.newID(), Kind: "archer", X: w.HubX, HP: 10, MaxHP: 10})
	before = protectSnapshot(w)
	for _, p := range w.Players {
		greed := spawnEnemy(w, "greed", p.X)
		attack(w, greed, &target{id: p.ID, x: p.X, kind: "player", player: p})
	}
	applyDamageBy(w, site.ID, site.MaxHP*10, "goblin")
	w.Castle.HP = 0
	castleFallen(w)
	return before, protectSnapshot(w)
}

func TestProtectedNightEasyKeepsEverything(t *testing.T) {
	if difficulty["easy"].ProtectedNights != 1 {
		t.Fatalf("easy: protectedNights %d, erwartet 1 aus data/difficulty.json", difficulty["easy"].ProtectedNights)
	}
	before, after := forcedAttack(t, "easy", 1)
	if after != before {
		t.Errorf("easy, Nacht 1: Verlust %+v → %+v", before, after)
	}
}

func TestProtectedNightNormalLoses(t *testing.T) {
	before, after := forcedAttack(t, "normal", 1)
	assertLost(t, "normal, Nacht 1", before, after)
}

func TestProtectedNightEasySecondNightLoses(t *testing.T) {
	before, after := forcedAttack(t, "easy", 2)
	assertLost(t, "easy, Nacht 2", before, after)
}

func assertLost(t *testing.T, name string, before, after protectState) {
	t.Helper()
	for i := range before.gold {
		if after.gold[i] >= before.gold[i] {
			t.Errorf("%s: Spieler %d behält Gold %d", name, i, after.gold[i])
		}
	}
	if after.stock == before.stock || after.built >= before.built || after.troops >= before.troops {
		t.Errorf("%s: ohne Verlust %+v → %+v", name, before, after)
	}
}
