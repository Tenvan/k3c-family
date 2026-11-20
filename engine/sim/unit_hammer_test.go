package sim

import (
	"math"
	"testing"
)

// WI1 (B-950 bis B-953): Handwerker mit Hammer, Werkstatt und Bogner, Gold aus Bäumen, Truhen-Fontäne. Jede Mechanik
// läuft mit 2 Spielern.

// hammerlessPeasant ist ein Bauer ohne Hammer bei x.
func hammerlessPeasant(w *World, x float64) *Troop { return addPeasant(w, x, "") }

// B-950: Ohne Hammer baut kein Bauer; mit Hammer baut er, während beide Spieler am Bauplatz zahlen.
func TestNurHandwerkerBauen(t *testing.T) {
	w := quietWorld(t)
	w.CycleSpeed = 1e-6
	players := []*Player{AddPlayer(w), AddPlayer(w)}
	site := lineSite(t, w, "wall", 1, 1)
	site.State = "waitingWorker"
	peasant := hammerlessPeasant(w, site.X)
	run(w, 3*buildings["wall"].BuildSeconds)
	if site.State != "waitingWorker" || peasant.Job != nil {
		t.Fatalf("Bauer ohne Hammer: Platz %s, Auftrag %+v", site.State, peasant.Job)
	}
	peasant.Hammer = true
	if runUntil(w, 3*buildings["wall"].BuildSeconds, func() bool { return site.State == "built" }) < 0 {
		t.Fatalf("Handwerker baut nicht: Platz %s", site.State)
	}
	// Reparatur ist ebenfalls Hammer-Arbeit.
	site.HP = site.MaxHP / 2
	other := hammerlessPeasant(w, site.X)
	peasant.Hammer = false
	if j := repairJob(w, other); j != nil {
		t.Errorf("Reparatur ohne Hammer: %+v", j)
	}
	other.Hammer = true
	if j := repairJob(w, other); j == nil || j.SiteID != site.ID {
		t.Errorf("Handwerker repariert nicht: %+v", j)
	}
	_ = players
}

// B-950: Das Welt-Feld startTroops.craftsman bestimmt die Start-Handwerker (Standard 1, mindestens ein Handwerker).
func TestStartHandwerker(t *testing.T) {
	w := newWorld(0, "test", Options{})
	n := 0
	for _, tr := range w.Troops {
		if tr.Kind == "peasant" && tr.Hammer {
			n++
		}
	}
	if want := w.values().Hub.StartTroops.Craftsman; want < 1 || n != want {
		t.Errorf("Start-Handwerker %d, laut Daten %d (mindestens 1)", n, want)
	}
	v := *w.values()
	v.Hub.StartTroops.Craftsman = 3
	w.vals, w.Troops = &v, nil
	spawnStartPeasants(w, w.HubX)
	hammers := 0
	for _, tr := range w.Troops {
		if tr.Hammer {
			hammers++
		}
	}
	if len(w.Troops) != 3+w.values().Hub.StartTroops.Peasant || hammers != 3 {
		t.Errorf("Welt mit 3 Start-Handwerkern: %d Bauern, %d mit Hammer", len(w.Troops), hammers)
	}
}

// B-951: Zwei Spieler zahlen den Hammer der Werkstatt; nach der Herstellung holt ihn ein Bauer ohne Hammer.
func TestWerkstattHammerHerstellenUndHolen(t *testing.T) {
	w := quietWorld(t)
	w.CycleSpeed = 1e-6
	players := []*Player{AddPlayer(w), AddPlayer(w)}
	shop := buildSite(t, w, "workshop")
	cost := buildings["workshop"].HammerCost
	*w.Stock = materialOf(cost)
	if !payAt(w, players, shop.X, 10, func() bool { return shop.BowPaidGold >= cost.Gold || shop.CraftDone > 0 }) {
		t.Fatalf("Hammer nicht bezahlt: %+v", shop)
	}
	peasant := hammerlessPeasant(w, shop.X+6)
	if runUntil(w, 3*buildings["workshop"].CraftSeconds["hammer"]+20, func() bool { return peasant.Hammer }) < 0 {
		t.Fatalf("Bauer holt den Hammer nicht: Regal %d, Auftrag %+v", shop.Hammers, peasant.Job)
	}
	if shop.Hammers != 0 || shop.Bows != 0 {
		t.Errorf("Regal danach: Hämmer %d, Bögen %d", shop.Hammers, shop.Bows)
	}
}

// B-951: Der Bogner stellt Bögen her (Preis des Bogens), die Werkstatt keine; ein Bauer wird Bogenschütze.
func TestBognerStelltBoegenHer(t *testing.T) {
	w := quietWorld(t)
	w.CycleSpeed = 1e-6
	players := []*Player{AddPlayer(w), AddPlayer(w)}
	bowyer := buildSite(t, w, "bowyer")
	shop := buildSite(t, w, "workshop")
	bow := troops["archer"].Cost
	*w.Stock = materialOf(bow)
	if !payAt(w, players, bowyer.X, 10, func() bool { return bowyer.BowPaidGold >= bow.Gold || bowyer.CraftDone > 0 }) {
		t.Fatalf("Bogen nicht bezahlt: %+v", bowyer)
	}
	peasant := hammerlessPeasant(w, bowyer.X+6)
	if runUntil(w, 3*buildings["bowyer"].CraftSeconds["bow"]+20, func() bool { return peasant.Kind == "archer" }) < 0 {
		t.Fatalf("Bauer wird nicht Bogenschütze: Regal %d, Auftrag %+v", bowyer.Bows, peasant.Job)
	}
	if shop.Bows != 0 || shop.CraftDone != 0 || buildings["workshop"].CraftSeconds["bow"] != 0 {
		t.Errorf("Werkstatt stellt Bögen her: Regal %d, Arbeit %v", shop.Bows, shop.CraftDone)
	}
}

// B-950: Fällt der Handwerker, bleibt der Hammer liegen; ein anderer Bauer holt ihn, aber nicht mit einem Gegner in 8 Units.
func TestHammerFaelltUndWirdAufgehoben(t *testing.T) {
	w := quietWorld(t)
	w.CycleSpeed = 1e-6
	AddPlayer(w)
	AddPlayer(w)
	smith := addPeasant(w, w.HubX-5, "")
	smith.Hammer = true
	loseLayer(w, smith)
	if smith.Hammer || len(w.Drops) != 1 || w.Drops[0].Kind != hammerKind {
		t.Fatalf("Hammer fällt nicht: Hammer %v, am Boden %v", smith.Hammer, w.Drops)
	}
	other := hammerlessPeasant(w, w.HubX+10)
	near := spawnEnemy(w, "goblin", w.Drops[0].X+3)
	near.Stun = math.Inf(1)
	if j := weaponToFetch(w, other); j != nil {
		t.Fatalf("Gegner in %v Units: Auftrag %+v", near.X-w.Drops[0].X, j)
	}
	w.Enemies = nil
	if runUntil(w, 30, func() bool { return other.Hammer }) < 0 || len(w.Drops) != 0 {
		t.Errorf("Hammer nicht aufgehoben: Hammer %v, am Boden %d", other.Hammer, len(w.Drops))
	}
}

// B-952: Ein gefällter Baum wirft 1 bis 3 Goldmünzen (aus den Daten), deterministisch für gleichen Seed.
func TestBaumWirftGold(t *testing.T) {
	var counts [2]int
	for i := range 2 {
		w := dayWorld(t)
		AddPlayer(w)
		AddPlayer(w)
		tree := firstNode(t, w, "tree")
		tree.Marked = true
		w.Troops[0].X = tree.X
		before := len(w.Coins)
		if runUntil(w, 60, func() bool { return tree.gone }) < 0 {
			t.Fatal("Baum nicht gefällt")
		}
		counts[i] = len(w.Coins) - before
	}
	tg := economy.TreeGold
	if counts[0] < tg[0] || counts[0] > tg[1] || counts[0] != counts[1] {
		t.Errorf("Münzen je Baum %v, erwartet %d bis %d und gleich", counts, tg[0], tg[1])
	}
}

// B-953: Die Truhe wirft ihr Gold (×10) als Fontäne: Münzen im Flug sind noch nicht aufsammelbar, danach liegen sie
// aufsammelbar am Boden; zwei gleiche Welten werfen gleich.
func TestTruheWirftFontaene(t *testing.T) {
	if economy.ChestGold[0] < 15 {
		t.Fatalf("Truhen-Gold %v, erwartet mindestens 15 (B-1159)", economy.ChestGold)
	}
	var xs [2][]float64
	var w *World
	var players []*Player
	for i := range 2 {
		w = quietWorld(t)
		players = []*Player{AddPlayer(w), AddPlayer(w)}
		w.Pickups = append(w.Pickups, &Pickup{ID: w.newID(), Kind: "chest", X: w.HubX + 40})
		players[0].X, players[1].X = w.HubX+40, w.HubX+40
		gold := players[0].Gold
		Step(w, nil, dt)
		got := len(w.Coins)
		if got < economy.ChestGold[0] || got > economy.ChestGold[1] || players[0].Gold != gold {
			t.Fatalf("Truhe: %d Münzen (erwartet %v), Beutel %d → %d", got, economy.ChestGold, gold, players[0].Gold)
		}
		for _, c := range w.Coins {
			if c.LandsAt <= w.Time || math.Abs(c.X-c.FromX) > economy.ChestFountain.SpreadUnits+1e-9 {
				t.Fatalf("Münze im Flug erwartet: %+v (Zeit %v)", c, w.Time)
			}
			xs[i] = append(xs[i], c.X)
		}
	}
	for i := range xs[0] {
		if xs[0][i] != xs[1][i] {
			t.Fatalf("Fontäne nicht deterministisch: Münze %d bei %v und %v", i, xs[0][i], xs[1][i])
		}
	}
	run(w, 4)
	var last *Coin
	for _, c := range w.Coins {
		last = c
	}
	if last == nil || last.LandsAt > w.Time {
		t.Fatalf("Münzen müssen nach dem Flug liegen: %+v", last)
	}
	gold := players[1].Gold
	players[1].X = last.X
	Step(w, nil, dt)
	if players[1].Gold <= gold {
		t.Errorf("gelandete Münze nicht aufgesammelt: Beutel %d → %d", gold, players[1].Gold)
	}
}

// Review WI1: Hämmer gelten nicht als Kämpfer-Waffe und werden auch bei voller Truppe geholt.
func TestHammerBeiVollerTruppeGeholt(t *testing.T) {
	w := quietWorld(t)
	w.CycleSpeed = 1e-6
	AddPlayer(w)
	AddPlayer(w)
	for range troopLimit(w) {
		makeArcher(w, spawnVagrant(w, w.HubX, w.HubX))
	}
	shop := buildSite(t, w, "workshop")
	shop.Hammers = 1
	peasant := hammerlessPeasant(w, shop.X+4)
	if runUntil(w, 20, func() bool { return peasant.Hammer }) < 0 {
		t.Fatalf("Hammer bei voller Truppe nicht geholt: %+v", peasant.Job)
	}
}

// Review WI1: Ohne Handwerker und Hammer legt die Burg zum Tagesbeginn einen ab; Gegner nehmen keinen mit.
func TestBurgLegtErsatzhammerAb(t *testing.T) {
	w := quietWorld(t)
	AddPlayer(w)
	AddPlayer(w)
	spareHammer(w)
	if len(w.Drops) != 1 || w.Drops[0].Kind != hammerKind || w.Drops[0].X != w.Castle.X {
		t.Fatalf("Ersatzhammer fehlt: %v", w.Drops)
	}
	spareHammer(w)
	if len(w.Drops) != 1 {
		t.Errorf("zweiter Hammer trotz Hammer am Boden: %d", len(w.Drops))
	}
	w.Portals = []float64{w.WidthUnits}
	e := spawnEnemy(w, "goblin", w.Castle.X)
	takeEquipment(w, e)
	if len(w.Drops) != 1 || e.Carried != nil {
		t.Errorf("Gegner trägt den Hammer weg: %v", e.Carried)
	}
	h := hammerlessPeasant(w, w.Castle.X)
	h.Hammer = true
	w.Drops = nil
	spareHammer(w)
	if len(w.Drops) != 0 {
		t.Error("Ersatzhammer trotz Handwerker")
	}
}

// Review WI1: Start-Handwerker mindestens 1; Plantagen-Bäume geben kein Gold; ein Handwerker wird nicht ausgebildet.
func TestHandwerkerMindestensEinerKeinPlantagenGold(t *testing.T) {
	w := newWorld(0, "test", Options{})
	v := *w.values()
	v.Hub.StartTroops.Craftsman = 0
	w.vals, w.Troops = &v, nil
	spawnStartPeasants(w, w.HubX)
	if n := len(w.Troops); n < 1 || !w.Troops[0].Hammer {
		t.Errorf("ohne Start-Handwerker: %d Bauern, Hammer %v", n, n > 0 && w.Troops[0].Hammer)
	}
	smith := w.Troops[0]
	if freePeasant(w, smith.X, "miner") == smith {
		t.Error("freePeasant wählt einen Handwerker")
	}
	w2 := dayWorld(t)
	tree := &ResourceNode{ID: w2.newID(), Kind: "tree", X: w2.HubX + 3, Marked: true, plant: true}
	w2.Nodes = append(w2.Nodes, tree)
	w2.Troops[0].X = tree.X
	before := len(w2.Coins)
	if runUntil(w2, 60, func() bool { return tree.gone }) < 0 {
		t.Fatal("Plantagen-Baum nicht gefällt")
	}
	if len(w2.Coins) != before {
		t.Errorf("Plantagen-Baum warf %d Münzen", len(w2.Coins)-before)
	}
}
