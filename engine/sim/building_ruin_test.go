package sim

import (
	"encoding/json"
	"math"
	"os"
	"strings"
	"testing"

	"k3c/engine/level"
)

// Ruinen als Plätze (WZ14.3, B-468/AC-03, docs/rules/geplante-chunks.md § 8), mit 2 Spielern über CreateIsland.

const ruinFixturePath = "../../testdata/planned/forest-ruin.json"

// ruinFixturePlan ist die Planung aus testdata/planned/forest-ruin.json (Lager, Schmiede, Kaserne) als JSON.
func ruinFixturePlan(t *testing.T) json.RawMessage {
	t.Helper()
	raw, err := os.ReadFile(ruinFixturePath)
	if err != nil {
		t.Fatal(err)
	}
	var f struct{ Planned json.RawMessage }
	if err := json.Unmarshal(raw, &f); err != nil {
		t.Fatal(err)
	}
	return f.Planned
}

// ruinIsland baut die Insel „ruine-1“ (Tiefe 0) mit der Planung plan und zwei Spielern.
func ruinIsland(t *testing.T, plan []level.PlannedChunk) (*Island, *World) {
	t.Helper()
	isl, err := CreateIsland("ruine-1", []int{0}, islandSpeed, CreateOptions{Planned: map[int][]level.PlannedChunk{0: plan}})
	if err != nil {
		t.Fatal(err)
	}
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	return isl, isl.Stages[0]
}

// fixtureIsland ist ruinIsland mit der Planung der Fixture.
func fixtureIsland(t *testing.T) (*Island, *World) {
	t.Helper()
	var plan []level.PlannedChunk
	if err := json.Unmarshal(ruinFixturePlan(t), &plan); err != nil {
		t.Fatal(err)
	}
	return ruinIsland(t, plan)
}

func ruinOf(t *testing.T, w *World, kind string) *Site {
	t.Helper()
	for _, s := range w.Sites {
		if s.Ruin && s.Kind == kind {
			return s
		}
	}
	t.Fatalf("keine Ruine %s", kind)
	return nil
}

// siteAt ist der Platz kind@x der Stufe; nil, wenn es ihn nicht gibt.
func siteAt(w *World, kind string, x float64) *Site {
	for _, s := range w.Sites {
		if s.Kind == kind && s.X == x {
			return s
		}
	}
	return nil
}

func ruinStartHP(kind string) float64 { return math.Ceil(0.25 * buildings[kind].HP) }

func TestRuineStartetGebautMitKennung(t *testing.T) {
	_, w := fixtureIsland(t)
	if len(w.Level.Ruins) != 3 {
		t.Fatalf("Ruinen im Level: %+v", w.Level.Ruins)
	}
	ruins := 0
	for _, s := range w.Sites {
		if s.Ruin {
			ruins++
		}
	}
	if ruins != 3 {
		t.Errorf("%d Plätze mit Kennung, erwartet 3", ruins)
	}
	for _, r := range w.Level.Ruins {
		s := siteAt(w, r.Kind, r.X)
		if s == nil || !s.Ruin || s.State != "built" || s.BuildProgress != 1 || s.Level != 0 {
			t.Fatalf("Ruine %s@%v: %+v", r.Kind, r.X, s)
		}
		if s.HP != ruinStartHP(r.Kind) || s.HP != level.RuinHP(r.Kind) || s.MaxHP != buildings[r.Kind].HP {
			t.Errorf("Ruine %s: HP %v/%v, erwartet %v/%v", r.Kind, s.HP, s.MaxHP, ruinStartHP(r.Kind), buildings[r.Kind].HP)
		}
	}
	snap, _ := json.Marshal(w.Sites)
	if strings.Count(string(snap), `"ruin":true`) != 3 {
		t.Errorf("Snapshot trägt die Kennung nicht genau dreimal: %s", snap)
	}
}

// Ohne Planung gibt es keine Ruinen und keine Kennung im Snapshot (Goldens byte-gleich).
func TestOhnePlanungKeineRuine(t *testing.T) {
	_, w := ruinIsland(t, nil)
	snap, _ := json.Marshal(w)
	if len(w.Level.Ruins) != 0 || strings.Contains(string(snap), `"ruin"`) {
		t.Error("ohne Planung darf es keine Ruine geben")
	}
}

// Die Ruine wirkt wie ein gebautes Gebäude ihrer Art: Kaserne → Truppenlimit, Lager → Kapazität und Abgabestelle.
func TestRuineWirktWieGebaeude(t *testing.T) {
	isl, w := fixtureIsland(t)
	isl0, w0 := ruinIsland(t, nil)
	if got := troopLimit(w) - troopLimit(w0); got != buildings["barracks"].TroopLimit.PerBuilding {
		t.Errorf("Kaserne als Ruine: Truppenlimit +%d, erwartet +%d", got, buildings["barracks"].TroopLimit.PerBuilding)
	}
	c, _ := capacity(w)
	c0, _ := capacity(w0)
	if c-c0 != economy.Storage.PerStorage {
		t.Errorf("Lager als Ruine: Kapazität +%d, erwartet +%d", c-c0, economy.Storage.PerStorage)
	}
	if *isl.Stock != *isl0.Stock {
		t.Errorf("Lager als Ruine ändert den Bestand: %+v statt %+v", *isl.Stock, *isl0.Stock)
	}
	if s := ruinOf(t, w, "storage"); depositPoint(w, s.X) != s.X {
		t.Error("das Lager als Ruine ist keine Abgabestelle")
	}
}

// Hub-neutral: Hub-Stufe, Bezahlbarkeit und Hub-Stufe der Hub-Plätze und Mauerlinien sind wie ohne Ruinen; der
// Sieg „alles gebaut“ zählt Ruinen nicht.
func TestRuineIstHubNeutral(t *testing.T) {
	isl, w := fixtureIsland(t)
	_, w0 := ruinIsland(t, nil)
	if w.HubLevel != 1 || EconomyOf(w).HubUpgrade == nil || *EconomyOf(w).HubUpgrade != *EconomyOf(w0).HubUpgrade {
		t.Errorf("Hub-Stufe/Hub-Ausbau mit Ruinen: %d, %+v", w.HubLevel, EconomyOf(w).HubUpgrade)
	}
	for _, s := range w.Sites {
		if s.Ruin {
			continue
		}
		s0 := siteAt(w0, s.Kind, s.X)
		if s0 == nil {
			t.Fatalf("Platz %s@%v fehlt ohne Planung", s.Kind, s.X)
		}
		if sitePayable(w, s) != sitePayable(w0, s0) || siteHubLevel(w, s) != siteHubLevel(w0, s0) || lineOpen(w, s) != lineOpen(w0, s0) {
			t.Errorf("Platz %s@%v verhält sich mit Ruinen anders", s.Kind, s.X)
		}
	}
	// Sieg „alles gebaut“ (stufen.md): eine Ruine ist ein Bauplatz wie jeder andere, erst ihr Wiederaufbau zählt.
	for _, s := range w.Sites {
		s.State = "built"
	}
	if !allBuilt(isl) {
		t.Error("alles gebaut inklusive Ruine muss gewinnen")
	}
	destroySite(w, ruinOf(t, w, "smithy"))
	if allBuilt(isl) {
		t.Error("eine zerstörte Ruine muss den Sieg „alles gebaut“ sperren")
	}
}

// Bauern reparieren die Ruine wie jedes beschädigte Gebäude, ohne Gold und Material.
func TestRuineWirdRepariert(t *testing.T) {
	isl, w := fixtureIsland(t)
	s := ruinOf(t, w, "smithy")
	var peasant *Troop
	for _, tr := range w.Troops {
		if tr.Kind == "peasant" {
			peasant = tr
		}
	}
	if peasant == nil || isDangerous(w) {
		t.Fatal("Ausgangslage: Bauer fehlt oder Gefahr")
	}
	peasant.X = s.X
	peasant.Job = repairJob(w, peasant)
	if peasant.Job == nil || peasant.Job.SiteID != s.ID {
		t.Fatalf("Reparatur-Auftrag %+v, erwartet die Schmiede-Ruine %d", peasant.Job, s.ID)
	}
	stock := *isl.Stock
	for i := 0; i < 3000 && peasant.Job != nil; i++ {
		repair(w, peasant, dt)
	}
	if s.HP != s.MaxHP || !s.Ruin || s.State != "built" || *isl.Stock != stock {
		t.Errorf("nach der Reparatur: %+v, Vorrat %+v", s, *isl.Stock)
	}
	if !hasEvent(w.Events, "repaired") {
		t.Error("kein Ereignis repaired")
	}
}

func hasEvent(events []Event, typ string) bool {
	for _, e := range events {
		if e["type"] == typ {
			return true
		}
	}
	return false
}

// Zerstört: unbezahlt mit Kennung; Wiederaufbau erst ab der Hub-Stufe der Art.
func TestRuineZerstoertUndWiederaufbauAbHubStufeDerArt(t *testing.T) {
	_, w := fixtureIsland(t)
	want := map[string]int{"smithy": 3, "barracks": 2, "storage": 2} // hub.json: sites bzw. islandSites
	for kind, from := range want {
		s := ruinOf(t, w, kind)
		applyDamage(w, s.ID, s.HP+1)
		if s.State != "unpaid" || !s.Ruin || s.HP != 0 || s.PaidGold != 0 || s.MaxHP != buildings[kind].HP {
			t.Fatalf("zerstörte Ruine %s: %+v", kind, s)
		}
		for hub := 1; hub <= len(hubOf(w).Levels); hub++ {
			w.HubLevel = hub
			if sitePayable(w, s) != (hub >= from) {
				t.Errorf("%s auf Hub-Stufe %d: bezahlbar %v, erwartet ab %d", kind, hub, sitePayable(w, s), from)
			}
		}
		w.HubLevel = 1
	}
	if !hasEvent(w.Events, "destroyed") {
		t.Error("kein Ereignis destroyed")
	}
}

// Wiederaufbau wie jeder Bau (bezahlen, Bauer baut): volle HP der Art, die Kennung bleibt.
func TestRuineWiederaufbauBehaeltKennung(t *testing.T) {
	_, w := fixtureIsland(t)
	s := ruinOf(t, w, "barracks")
	destroySite(w, s)
	w.HubLevel = 2
	for s.State == "unpaid" {
		paySite(w, w.Players[1], s)
	}
	s.State = "waitingWorker"
	tr := spawnVagrant(w, s.X, s.X)
	tr.Kind, tr.Job = "peasant", &Job{Type: "build", SiteID: s.ID}
	for i := 0; i < 3000 && tr.Job != nil; i++ {
		build(w, tr, dt)
	}
	if s.State != "built" || !s.Ruin || s.HP != buildings["barracks"].HP {
		t.Errorf("wieder aufgebaute Ruine: %+v", s)
	}
}

// Burgfall (castleLosses): Ruinen kehren in den Ausgangszustand zurück, auch nach Zerstörung oder voller Reparatur;
// Hub-Plätze werden leer.
func TestBurgfallSetztRuinenZurueck(t *testing.T) {
	isl, w := fixtureIsland(t)
	isl.Options.Defeat = "stage"
	ids := map[string]int{}
	for _, k := range []string{"smithy", "barracks", "storage"} {
		ids[k] = ruinOf(t, w, k).ID
	}
	destroySite(w, ruinOf(t, w, "smithy"))
	ruinOf(t, w, "barracks").HP = buildings["barracks"].HP
	ruinOf(t, w, "storage").HP = 7
	hubWorkshop := builtSite(t, w, "workshop")
	if w.Cycle.Phase != "day" {
		t.Fatalf("Ausgangslage: Phase %s", w.Cycle.Phase)
	}
	w.Castle.HP = 0
	StepIsland(isl, []PlayerCommand{{}, {}}, dt)
	if !hasEvent(w.Events, "castleFallen") {
		t.Fatal("die Burg ist nicht gefallen")
	}
	for kind, id := range ids {
		s := ruinOf(t, w, kind)
		if s.ID != id || s.State != "built" || s.HP != ruinStartHP(kind) || s.MaxHP != buildings[kind].HP {
			t.Errorf("Ruine %s nach Burgfall: %+v", kind, s)
		}
	}
	if hubWorkshop.State != "unpaid" || hubWorkshop.Ruin {
		t.Errorf("Hub-Platz nach Burgfall: %+v", hubWorkshop)
	}
}
