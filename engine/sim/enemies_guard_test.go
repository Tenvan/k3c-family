package sim

import (
	"bytes"
	"math"
	"testing"

	"k3c/engine/level"
)

// Gegnerlager (WZ27.1, B-517, docs/rules/geplante-chunks.md § 9), mit 2 Spielern über CreateIsland.

// guardPlan: Wald, Platz +4 (Hub + 200 … + 250), Wachen bei +20 (Goblin) und +40 (Greed, Gold-Dieb).
func guardPlan() []level.PlannedChunk {
	return []level.PlannedChunk{{Slot: 4, Kind: "forest", Enemies: []level.PlannedEnemy{{Kind: "goblin", At: 20}, {Kind: "greed", At: 40}}}}
}

func guardIsland(t *testing.T) (*Island, *World) {
	t.Helper()
	isl, w := ruinIsland(t, guardPlan())
	Step(w, nil, dt) // erster Tick: Wachen stehen
	return isl, w
}

func guardsOf(w *World) []*Enemy {
	var out []*Enemy
	for _, e := range w.Enemies {
		if e.Guard {
			out = append(out, e)
		}
	}
	return out
}

func guardSteps(w *World, seconds float64) {
	for tm := 0.0; tm < seconds; tm += dt {
		Step(w, nil, dt)
	}
}

func TestLagerStelltWachenAuf(t *testing.T) {
	_, w := guardIsland(t)
	gs := guardsOf(w)
	if len(gs) != 2 || len(w.Level.Guards) != 2 {
		t.Fatalf("%d Wachen, Level %+v", len(gs), w.Level.Guards)
	}
	start := w.HubX + 50 + 3*50
	if gs[0].Kind != "goblin" || gs[0].X != start+20 || gs[0].HomeX != start+20 || gs[1].Kind != "greed" || gs[1].X != start+40 {
		t.Errorf("Wachen %+v / %+v", gs[0], gs[1])
	}
	for _, g := range gs {
		if g.HP <= 0 || g.HP != g.MaxHP {
			t.Errorf("%s: HP %v/%v", g.Kind, g.HP, g.MaxHP)
		}
	}
}

func TestOhneLagerKeineWachen(t *testing.T) {
	_, w := ruinIsland(t, []level.PlannedChunk{{Slot: 4, Kind: "forest"}})
	guardSteps(w, 2)
	if len(w.Enemies) != 0 || len(w.Level.Guards) != 0 {
		t.Errorf("%d Gegner ohne Lager", len(w.Enemies))
	}
}

// Wachen stehen still, solange niemand im Radius ist, und gehören zu keiner Welle.
func TestWachenStehenUndZaehlenNichtAlsWelle(t *testing.T) {
	_, w := guardIsland(t)
	before := []float64{w.Enemies[0].X, w.Enemies[1].X}
	if isDangerous(w) && w.Cycle.Phase != "night" {
		t.Error("Wachen machen den Tag gefährlich")
	}
	sendEnemiesHome(w)
	guardSteps(w, 5)
	if len(guardsOf(w)) != 2 || w.Enemies[0].X != before[0] || w.Enemies[1].X != before[1] || w.Enemies[0].Fleeing {
		t.Errorf("Wachen haben sich bewegt oder fliehen: %+v", w.Enemies)
	}
}

// Ziel im Radius: die Wache greift an (Spieler verliert HP), das Ziel außerhalb des Radius bleibt unbehelligt.
func TestWacheGreiftImRadiusAnUndKehrtZurueck(t *testing.T) {
	_, w := guardIsland(t)
	g := guardsOf(w)[0]
	home := g.HomeX
	far, near := w.Players[0], w.Players[1]
	far.X, near.X = home-30, home-6 // 30 Units: außerhalb des Radius (10), 6: darin
	gold := near.Gold
	maxHP := near.HP
	guardSteps(w, 6)
	if far.HP != maxHP && far.HP != far.MaxHP {
		t.Errorf("Spieler außerhalb des Radius getroffen: HP %v", far.HP)
	}
	if near.HP >= maxHP && near.Gold == gold {
		t.Errorf("Spieler im Radius nicht angegriffen: HP %v", near.HP)
	}
	if g.X == home {
		t.Error("die Wache hat sich nicht auf das Ziel zubewegt")
	}
	// Ziel weg: die Wache kehrt zum Lager zurück.
	near.X = home - 30
	guardSteps(w, 6)
	if math.Abs(g.X-home) > 1e-9 || g.Fleeing {
		t.Errorf("Wache bei %v, Lager bei %v", g.X, home)
	}
}

// Der Dieb-Trait gilt nicht für Wachen: kein Goldklau, keine Flucht.
func TestWacheKlautKeinGold(t *testing.T) {
	_, w := guardIsland(t)
	thief := guardsOf(w)[1]
	p := w.Players[0]
	p.X, p.Gold = thief.HomeX-2, 50
	guardSteps(w, 5)
	if p.Gold != 50 || thief.Fleeing || thief.CarriedGold != 0 {
		t.Errorf("Gold %d, flieht %v, trägt %d", p.Gold, thief.Fleeing, thief.CarriedGold)
	}
}

func TestBesiegteWacheIstGeraeumtOhneAggression(t *testing.T) {
	isl, err := CreateIsland("lager-1", []int{1}, islandSpeed, CreateOptions{Planned: map[int][]level.PlannedChunk{1: {{Slot: 3, Kind: "cavern", Enemies: []level.PlannedEnemy{{Kind: "bat", At: 20}}}}}})
	if err != nil {
		t.Fatal(err)
	}
	w := isl.Stages[0]
	Step(w, nil, dt)
	g := guardsOf(w)
	if len(g) != 1 || w.Aggression == nil {
		t.Fatalf("%d Wachen, Aggression %v", len(g), w.Aggression)
	}
	*w.Aggression = 0
	g[0].HP = 0
	removeDeadEnemies(w)
	if *w.Aggression != 0 || len(w.Enemies) != 0 || !w.guardsCleared[key("bat", g[0].HomeX)] {
		t.Errorf("Aggression %v, Gegner %d, geräumt %v", *w.Aggression, len(w.Enemies), w.guardsCleared)
	}
	guardSteps(w, 3)
	if len(guardsOf(w)) != 0 {
		t.Error("geräumte Wache kommt wieder")
	}
}

func TestGeraeumtesLagerBleibtImSpielstand(t *testing.T) {
	isl, w := guardIsland(t)
	guardsOf(w)[0].HP = 0
	Step(w, nil, dt)
	if len(guardsOf(w)) != 1 {
		t.Fatalf("%d Wachen nach dem Sieg über eine", len(guardsOf(w)))
	}
	save := isl.ToSave("2026-10-22T10:00:00Z")
	if len(save.Stages[0].GuardsCleared) != 1 {
		t.Fatalf("geräumt im Stand: %v", save.Stages[0].GuardsCleared)
	}
	parsed, err := ParseIslandSave(saveJSON(t, save))
	if err != nil {
		t.Fatal(err)
	}
	loaded, err := FromIslandSave(parsed, islandSpeed)
	if err != nil {
		t.Fatal(err)
	}
	// Speichern, Laden, Speichern ist gleich.
	if !bytes.Equal(saveJSON(t, loaded.ToSave(save.SavedAt)), saveJSON(t, save)) {
		t.Error("Stand ändert sich beim Neuladen")
	}
	lw := loaded.Stages[0]
	guardSteps(lw, 1)
	if gs := guardsOf(lw); len(gs) != 1 || gs[0].Kind != "greed" {
		t.Errorf("nach dem Laden %+v, erwartet nur die zweite Wache", gs)
	}
}

// Burgfall: die Gegner der Welle verschwinden, die Wachen bleiben.
func TestBurgfallLaesstWachenStehen(t *testing.T) {
	_, w := guardIsland(t)
	spawnEnemy(w, "goblin", w.HubX+30)
	w.Castle.HP = 0
	Step(w, nil, dt)
	if len(w.Enemies) != 2 || len(guardsOf(w)) != 2 {
		t.Errorf("%d Gegner, %d Wachen nach dem Burgfall", len(w.Enemies), len(guardsOf(w)))
	}
}
