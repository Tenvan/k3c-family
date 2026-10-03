package sim

import "testing"

// coinsAt zählt die Münzen der Welt genau bei x.
func coinsAt(w *World, x float64) int {
	n := 0
	for _, c := range w.Coins {
		if c.X == x {
			n++
		}
	}
	return n
}

// B-178/AC-02: Dev-Gold fällt beim Spieler und wird bis zum Beutel-Maximum aufgehoben, der Rest bleibt liegen.
func TestDevDropGold(t *testing.T) {
	isl := mustIsland(t, "dev-gold", []int{0})
	w := isl.Stages[0]
	p := AddIslandPlayer(isl, 0)
	p.Gold = economy.Purse.MaxGold - 20
	before := len(w.Coins)
	if !DevDropGold(isl, p.Index, 50) {
		t.Fatal("Spieler nicht gefunden")
	}
	if n := coinsAt(w, p.X); len(w.Coins)-before != 50 || n < 50 {
		t.Fatalf("%d neue Münzen, %d bei X, erwartet 50", len(w.Coins)-before, n)
	}
	x := p.X
	StepIsland(isl, nil, 1.0/30)
	if p.Gold != economy.Purse.MaxGold {
		t.Fatalf("Gold %d, erwartet %d", p.Gold, economy.Purse.MaxGold)
	}
	if n := coinsAt(w, x); n != 30 {
		t.Fatalf("%d Münzen liegen noch, erwartet 30", n)
	}
	if DevDropGold(isl, 7, 1) {
		t.Fatal("unbekannter Spieler angenommen")
	}
}

// B-178/AC-03: Dev-Material füllt den Insel-Vorrat bis zum Lager-Maximum, der Rest wird verworfen.
func TestDevAddStock(t *testing.T) {
	isl := mustIsland(t, "dev-stock", []int{0})
	p := AddIslandPlayer(isl, 0)
	if taken, ok := DevAddStock(isl, p.Index, "wood", 100); !ok || taken != 100 || isl.Stock.Wood != 100 {
		t.Fatalf("aufgenommen %d (%v), Holz %d, erwartet 100", taken, ok, isl.Stock.Wood)
	}
	limit, _ := capacity(isl.Stages[0])
	if taken, ok := DevAddStock(isl, p.Index, "wood", 1000); !ok || taken != limit-100 || isl.Stock.Wood != limit {
		t.Fatalf("aufgenommen %d (%v), Holz %d, erwartet %d", taken, ok, isl.Stock.Wood, limit)
	}
	if _, ok := DevAddStock(isl, p.Index, "gold", 1); ok {
		t.Fatal("gold ist kein Baumaterial")
	}
	if _, ok := DevAddStock(isl, 7, "wood", 1); ok {
		t.Fatal("unbekannter Spieler angenommen")
	}
}
