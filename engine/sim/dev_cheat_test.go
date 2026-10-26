package sim

import "testing"

// Dev-Lager: +100 je Aufruf bis zur Obergrenze, Füllen bis zum Maximum.
func TestDevLagerMaximumUndFuellen(t *testing.T) {
	isl := mustIsland(t, "dev-lager", []int{0})
	w := isl.Stages[0]
	base, _ := capacity(w)
	for range 2 {
		if !DevRaiseMaxStock(isl) {
			t.Fatal("Erhöhung abgelehnt")
		}
	}
	if got, _ := capacity(w); got != base+200 {
		t.Fatalf("Maximum %d, erwartet %d", got, base+200)
	}
	DevFillStock(isl)
	s := isl.Stock
	for _, v := range []int{s.Wood, s.Stone, s.Copper, s.Iron, s.Crystal} {
		if v != base+200 {
			t.Fatalf("Vorrat %+v, erwartet überall %d", *s, base+200)
		}
	}
	for isl.Dev.StockBonus < DevStockBonusMax {
		if !DevRaiseMaxStock(isl) {
			t.Fatal("Erhöhung vor der Obergrenze abgelehnt")
		}
	}
	if DevRaiseMaxStock(isl) {
		t.Fatal("Obergrenze überschritten")
	}
}

// Gold-Maximum: ohne Bonus unverändert, mit Bonus +100 je Aufruf (Aufnahme von Münzen und giveGold), Füllen hebt das
// Gold aller Spieler auf das erhöhte Maximum.
func TestDevGoldMaximumUndFuellen(t *testing.T) {
	isl := mustIsland(t, "dev-gold", []int{0})
	w := isl.Stages[0]
	p := AddIslandPlayer(isl, 0)
	if purseMax(w) != economy.Purse.MaxGold || EconomyOf(w).PurseMax != 0 {
		t.Fatalf("Maximum ohne Bonus %d", purseMax(w))
	}
	DevRaiseMaxStock(isl)
	DevRaiseMaxStock(isl)
	want := economy.Purse.MaxGold + 200
	if purseMax(w) != want || EconomyOf(w).PurseMax != want {
		t.Fatalf("Maximum %d, erwartet %d", purseMax(w), want)
	}
	p.Gold = economy.Purse.MaxGold
	giveGold(w, p, 500, p.X)
	if p.Gold != want || len(w.Coins) != 300 {
		t.Fatalf("Gold %d, Münzen am Boden %d", p.Gold, len(w.Coins))
	}
	p.Gold = 5
	DevFillStock(isl)
	if p.Gold != want {
		t.Fatalf("Gold %d nach fillStock, erwartet %d", p.Gold, want)
	}
}

// Dev-Baukosten: jeder Posten höchstens 1, 0 bleibt 0; ein Bauplatz ist mit einer Münze bezahlt und zieht 1 Material.
func TestDevBaukostenEins(t *testing.T) {
	isl := mustIsland(t, "dev-kosten", []int{0})
	w := isl.Stages[0]
	c := Cost{Gold: 20, Wood: 50}
	if cheap(w, c) != c {
		t.Fatal("Preis ohne Schalter verändert")
	}
	DevSetCheapBuild(isl, true)
	if got := cheap(w, c); got != (Cost{Gold: 1, Wood: 1}) {
		t.Fatalf("Preis %+v", got)
	}
	p := AddIslandPlayer(isl, 0)
	var site *Site
	for _, s := range w.Sites {
		if s.Kind == "workshop" {
			site = s
		}
	}
	*isl.Stock = Stock{Wood: 1}
	paySite(w, p, site)
	if site.State != "waitingMaterial" {
		t.Fatalf("Zustand %q nach einer Münze", site.State)
	}
	stepSites(w)
	if site.State != "waitingWorker" || isl.Stock.Wood != 0 {
		t.Fatalf("Zustand %q, Holz %d", site.State, isl.Stock.Wood)
	}
	if next := nextLevel(w, w.hubSite); next == nil || next.Cost.Gold != 1 || next.Cost.Stone != 1 {
		t.Fatalf("Hub-Ausbau %+v", next)
	}
	DevSetCheapBuild(isl, false)
	if next := nextLevel(w, w.hubSite); next.Cost.Gold != 4 {
		t.Fatalf("Hub-Ausbau nach Aus %+v", next.Cost)
	}
}

// checkBuiltTo prüft nach DevBuildAll(level): Tore und Linien über level nicht gebaut, Mauern der Linien bis level
// gebaut mit Stufe level; liefert die Zahl dieser Mauern.
func checkBuiltTo(t *testing.T, w *World, level int) int {
	t.Helper()
	walls := 0
	for _, s := range w.Sites {
		l, onLine := siteLine(w, s)
		if s.State == "built" && (s.Kind == "gate" || (onLine && l.Index > level)) {
			t.Fatalf("%s der Linie %d gebaut", s.Kind, l.Index)
		}
		if s.Kind == "wall" && onLine && l.Index <= level {
			walls++
			if s.State != "built" || s.Level != level {
				t.Fatalf("Mauer Linie %d: %q Stufe %d", l.Index, s.State, s.Level)
			}
		}
	}
	return walls
}

// Dev-Ausbau: Hub-Stufe, Mauern und Türme bis zur Stufe gebaut und ausgebaut, Tore bleiben; ungültige Stufe false.
func TestDevBuildAll(t *testing.T) {
	isl := mustIsland(t, "dev-ausbau", []int{0})
	w := isl.Stages[0]
	if DevBuildAll(isl, 0) || DevBuildAll(isl, 6) {
		t.Fatal("ungültige Stufe angenommen")
	}
	if !DevBuildAll(isl, 3) || w.HubLevel != 3 {
		t.Fatalf("Hub-Stufe %d", w.HubLevel)
	}
	if n := checkBuiltTo(t, w, 3); n != 6 {
		t.Fatalf("%d Mauern, erwartet 6", n)
	}
}

// Nächste Tageszeit: day → dusk → night → dawn → day, die Stufe wechselt im nächsten Schritt.
func TestDevNextPhase(t *testing.T) {
	isl := mustIsland(t, "dev-phase", []int{0})
	w := isl.Stages[0]
	for _, want := range []string{"dusk", "night", "dawn", "day", "dusk"} {
		if got := DevNextPhase(isl); got != want {
			t.Fatalf("Phase %q, erwartet %q", got, want)
		}
		StepIsland(isl, nil, 1.0/30)
		if w.Cycle.Phase != want {
			t.Fatalf("Zyklus %q, erwartet %q", w.Cycle.Phase, want)
		}
	}
}
