package sim

import (
	"math"
	"slices"
	"testing"
)

// W4.2 (B-121, Sprint W4 AC-02): Berufe, Ausbildung an den Angebots-Zahlzielen, Herstellungszeit des Bogens.
// Alle Werte aus data/ (troops.json › peasant.professions, buildings.json › offers und craftSeconds).

func profession(t *testing.T, kind string) ProfessionData {
	t.Helper()
	p, ok := professionOf(kind)
	if !ok {
		t.Fatalf("Beruf %s fehlt in den Daten", kind)
	}
	return p
}

func addPeasant(w *World, x float64, prof string) *Troop {
	tr := &Troop{ID: w.newID(), Kind: "peasant", X: x, HP: troops["peasant"].HP, MaxHP: troops["peasant"].HP, AnchorX: w.HubX, Profession: prof}
	w.Troops = append(w.Troops, tr)
	return tr
}

// gatherSeconds: so lange baut ein Bauer mit Beruf prof einen markierten Knoten der Art kind ab (ohne Weg).
func gatherSeconds(t *testing.T, kind, prof string) float64 {
	t.Helper()
	w := quietWorld(t)
	n := &ResourceNode{ID: w.newID(), Kind: kind, X: w.HubX + 30, Marked: true}
	w.Nodes = append(w.Nodes, n)
	tr := addPeasant(w, n.X, prof)
	tr.Job = &Job{Type: "gather", NodeID: n.ID}
	secs := 0.0
	for ; tr.Job.Type == "gather" && secs < 120; secs += dt {
		gather(w, tr, dt)
	}
	return secs
}

func aboutDt(got, want float64) bool { return math.Abs(got-want) <= dt+1e-9 }

// (a) Bergmann: 1/workFactor der Zeit an Fels, Kupfererz und Adern, nicht am Baum.
func TestProfessionBergmannBautSchnellerAb(t *testing.T) {
	miner := profession(t, "miner")
	for _, kind := range []string{"rock", "copperOre", "stoneVein", "copperVein", "tree"} {
		g, _ := gatherOf(kind)
		factor := 1.0
		if kind != "tree" {
			factor = miner.WorkFactor
		}
		if got := gatherSeconds(t, kind, ""); !aboutDt(got, g.WorkSeconds) {
			t.Errorf("%s Bauer: %.2f s, erwartet %.2f s", kind, got, g.WorkSeconds)
		}
		if got := gatherSeconds(t, kind, "miner"); !aboutDt(got, g.WorkSeconds/factor) {
			t.Errorf("%s Bergmann: %.2f s, erwartet %.2f s", kind, got, g.WorkSeconds/factor)
		}
	}
}

// (a) Ein Bergmann zählt an der Ader als einer der höchstens maxWorkers Bauern.
func TestProfessionBergmannZaehltAnDerAder(t *testing.T) {
	w, vein := veinWorld(t, 3)
	vein.Marked = true
	w.Troops[0].Profession = "miner"
	most := 0
	for range 30 * 20 {
		Step(w, nil, dt)
		most = max(most, veinWorkers(w, vein))
	}
	if limit := economy.Veins[vein.Kind].MaxWorkers; most != limit {
		t.Errorf("höchstens %d an der Ader, erwartet %d", most, limit)
	}
}

// (a) Baumeister: Bau in 1/workFactor der Bauzeit.
func TestProfessionBaumeisterBautSchneller(t *testing.T) {
	builder := profession(t, "builder")
	for _, prof := range []string{"", "builder"} {
		w := quietWorld(t)
		s := w.Sites[0]
		s.State = "waitingWorker"
		tr := addPeasant(w, s.X, prof)
		tr.Job = &Job{Type: "build", SiteID: s.ID}
		secs := 0.0
		for ; s.State != "built" && secs < 120; secs += dt {
			build(w, tr, dt)
		}
		want := buildings[s.Kind].BuildSeconds
		if prof != "" {
			want /= builder.WorkFactor
		}
		if !aboutDt(secs, want) {
			t.Errorf("%q baut %s in %.2f s, erwartet %.2f s", prof, s.Kind, secs, want)
		}
	}
}

// offerOf ist das Angebot kind an der gebauten Werkstatt.
func offerOf(t *testing.T, w *World, kind string) *offerTarget {
	t.Helper()
	s := buildSite(t, w, "workshop")
	for i, o := range buildings["workshop"].Offers {
		if o.Kind == kind {
			return &offerTarget{s, i}
		}
	}
	t.Fatalf("kein Angebot %s an der Werkstatt", kind)
	return nil
}

// holdAt lässt Spieler 0 bei x A halten, höchstens seconds lang und nur bis zur ersten Ausbildung, dann loslassen.
func holdAt(w *World, p *Player, x, seconds float64) {
	for range int(seconds / dt) {
		p.X = x
		Step(w, []PlayerCommand{{Pay: true}}, dt)
		if slices.ContainsFunc(w.Events, func(e Event) bool { return e["type"] == "trained" }) {
			break
		}
	}
	Step(w, []PlayerCommand{{}}, dt)
}

// (b) Ausbildung kostet das Gold aus den Daten, trifft den nächsten freien Bauern; Umschulung kostet erneut;
// zu wenig Gold lässt den Beruf unverändert.
func TestProfessionAusbildungUndUmschulung(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	minerOffer, builderOffer := offerOf(t, w, "miner"), offerOf(t, w, "builder")
	s := minerOffer.site
	nearer, farther := addPeasant(w, s.X+1, ""), addPeasant(w, s.X+40, "")
	busy := addPeasant(w, s.X, "")
	far := &ResourceNode{ID: w.newID(), Kind: "rock", X: w.WidthUnits, Marked: true}
	w.Nodes = append(w.Nodes, far)
	busy.Job, far.WorkerID = &Job{Type: "gather", NodeID: far.ID}, &busy.ID // hat einen Auftrag (langer Weg), ist nicht frei
	gold := p.Gold
	holdAt(w, p, minerOffer.x(), 8)
	if cost := profession(t, "miner").Cost.Gold; gold-p.Gold != cost || nearer.Profession != "miner" {
		t.Fatalf("Ausbildung: %d Gold bezahlt (erwartet %d), Beruf %q", gold-p.Gold, cost, nearer.Profession)
	}
	if farther.Profession != "" || busy.Profession != "" {
		t.Errorf("falscher Bauer ausgebildet: weiter %q, beschäftigt %q", farther.Profession, busy.Profession)
	}
	nearer.X, farther.X = builderOffer.site.X+1, builderOffer.site.X+40
	gold = p.Gold
	holdAt(w, p, builderOffer.x(), 8)
	if cost := profession(t, "builder").Cost.Gold; gold-p.Gold != cost || nearer.Profession != "builder" {
		t.Fatalf("Umschulung: %d Gold bezahlt (erwartet %d), Beruf %q", gold-p.Gold, cost, nearer.Profession)
	}
	p.Gold = profession(t, "miner").Cost.Gold / 2
	holdAt(w, p, minerOffer.x(), 8)
	// Nach dem Loslassen ist die Teilzahlung erstattet (Münzen am Boden oder schon wieder aufgehoben).
	if back := p.Gold + len(w.Coins); nearer.Profession != "builder" || farther.Profession != "" || minerOffer.paid() != 0 || back != profession(t, "miner").Cost.Gold/2 {
		t.Errorf("zu wenig Gold: Berufe %q/%q, offen %d, zurück %d", nearer.Profession, farther.Profession, minerOffer.paid(), back)
	}
}

// bowSeconds: so lange dauert ein bezahlter Bogen mit n Handwerkern an der Werkstatt (nur stepSites, ohne Abholer).
func bowSeconds(t *testing.T, n int) float64 {
	t.Helper()
	w := quietWorld(t)
	s := buildSite(t, w, "workshop")
	for range n {
		addPeasant(w, s.X, "craftsman").WorkSite = s.ID
	}
	*w.Stock = Stock{Wood: 1000}
	s.BowPaidGold = troops["archer"].Cost.Gold
	secs := 0.0
	for ; s.Bows == 0 && secs < 60; secs += dt {
		w.Time += dt
		stepSites(w)
	}
	return secs - dt // der erste Takt beginnt die Arbeit erst
}

// (c) Der Bogen liegt nach craftSeconds.bow im Regal, mit einem Handwerker nach 1/workFactor davon; ein dritter
// Handwerker je Gebäude wird nicht ausgebildet.
func TestProfessionHandwerkerUndHerstellungszeit(t *testing.T) {
	craft := profession(t, "craftsman")
	base := buildings["workshop"].CraftSeconds["bow"]
	if got := bowSeconds(t, 0); !aboutDt(got, base) {
		t.Errorf("Bogen ohne Handwerker: %.2f s, erwartet %.2f s", got, base)
	}
	if got, want := bowSeconds(t, 1), base/craft.WorkFactor; !aboutDt(got, want) {
		t.Errorf("Bogen mit Handwerker: %.2f s, erwartet %.2f s", got, want)
	}

	w := quietWorld(t)
	p := AddPlayer(w)
	o := offerOf(t, w, "craftsman")
	for range craft.MaxPerBuilding {
		addPeasant(w, o.site.X, "craftsman").WorkSite = o.site.ID
	}
	third := addPeasant(w, o.site.X+1, "")
	gold, coins := p.Gold, len(w.Coins)
	holdAt(w, p, o.x(), 8)
	if third.Profession != "" || workersAt(w, o.site, "craftsman") != craft.MaxPerBuilding || p.Gold != gold || len(w.Coins) != coins {
		t.Errorf("dritter Handwerker: Beruf %q, Handwerker %d, Gold %d → %d, Münzen %d → %d",
			third.Profession, workersAt(w, o.site, "craftsman"), gold, p.Gold, coins, len(w.Coins))
	}
}
