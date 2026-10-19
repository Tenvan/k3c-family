package sim

import (
	"encoding/json"
	"maps"
	"reflect"
	"slices"
	"testing"

	"k3c/data"
)

// Siegvarianten (K2.2a, B-102/AC-01): je Variante nicht erfüllt, knapp nicht, erfüllt und danach kein zweites `victory`.

// goalIsland ist eine Insel mit zwei Stufen, einem Spieler je Stufe und dem Ziel goal.
func goalIsland(t *testing.T, goal string) *Island {
	t.Helper()
	isl := mustIsland(t, "k2-sieg-"+goal, []int{0, 1})
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 1)
	o := isl.Options
	o.Goal = goal
	if err := SetOptions(isl, o, false); err != nil {
		t.Fatal(err)
	}
	return isl
}

// victories tickt die Insel einmal und liefert die `victory`-Ereignisse aller Stufen.
func victories(isl *Island) []Event {
	StepIsland(isl, nil, dt)
	var out []Event
	for _, w := range isl.Stages {
		for _, e := range w.Events {
			if e["type"] == "victory" {
				out = append(out, e)
			}
		}
	}
	return out
}

func wantVictories(t *testing.T, isl *Island, n int, step string) {
	t.Helper()
	if got := victories(isl); len(got) != n {
		t.Fatalf("%s: %d victory, erwartet %d (%v)", step, len(got), n, got)
	}
}

// wantOnce: erfüllt → genau ein `victory` mit Variante, danach keines mehr, obwohl das Ziel erfüllt bleibt.
func wantOnce(t *testing.T, isl *Island) {
	t.Helper()
	got := victories(isl)
	if len(got) != 1 || got[0]["goal"] != isl.Options.Goal || got[0]["stage"] != 0 {
		t.Fatalf("erfüllt: victory %v, erwartet eines mit goal %s in Stufe 0", got, isl.Options.Goal)
	}
	if got[0]["day"] != isl.Stages[0].Cycle.Day || !isl.Won {
		t.Fatalf("victory: day %v, Won %v", got[0]["day"], isl.Won)
	}
	wantVictories(t, isl, 0, "danach")
}

func TestSiegEndboss(t *testing.T) {
	isl := goalIsland(t, "endboss")
	wantVictories(t, isl, 0, "Endboss lebt")
	isl.EndbossDefeated = true
	wantOnce(t, isl)
}

func TestSiegGoldSummeAllerSpieler(t *testing.T) {
	isl := goalIsland(t, "gold")
	wantVictories(t, isl, 0, "nichts gesammelt")
	isl.GoldCollected = goals.Gold - 2
	coinAtPlayer(isl, 0) // Spieler 1 hebt auf: 999
	wantVictories(t, isl, 0, "999 Gold")
	if isl.GoldCollected != goals.Gold-1 {
		t.Fatalf("Zähler %d, erwartet %d", isl.GoldCollected, goals.Gold-1)
	}
	coinAtPlayer(isl, 1) // Spieler 2 in der anderen Stufe: 1000
	wantOnce(t, isl)
}

// coinAtPlayer legt eine Münze vor den Spieler der Stufe.
func coinAtPlayer(isl *Island, stage int) {
	w := isl.Stages[stage]
	w.Players[0].Gold = 0
	w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: w.Players[0].X})
}

func TestSiegTageUeberleben(t *testing.T) {
	isl := goalIsland(t, "days")
	wantVictories(t, isl, 0, "Tag 1")
	setDay(isl, goals.Days)
	wantVictories(t, isl, 0, "Tag 20")
	if d := isl.Stages[0].Cycle.Day; d != goals.Days {
		t.Fatalf("Tag %d, erwartet %d", d, goals.Days)
	}
	setDay(isl, goals.Days+1)
	wantOnce(t, isl)
	if d := isl.Stages[0].Cycle.Day; d != goals.Days+1 {
		t.Fatalf("Tag %d, erwartet %d", d, goals.Days+1)
	}
}

// setDay stellt die Uhr aller Stufen auf den Anfang von Tag day (der Zyklus rechnet den Tag aus der Zeit).
func setDay(isl *Island, day int) {
	c := globalDayNight
	length := float64((c.DayMinutes + c.TwilightMinutes + c.NightMinutes) * 60)
	for _, w := range isl.Stages {
		w.Time = float64(day-1)*length/w.CycleSpeed + dt
	}
}

func TestSiegAllesAbbauen(t *testing.T) {
	isl := goalIsland(t, "mineAll")
	vein := veinKind(t)
	for _, w := range isl.Stages { // jede Stufe: eine Ader, die nie zählt
		w.Nodes = append(w.Nodes, &ResourceNode{ID: w.newID(), Kind: vein, X: w.HubX})
	}
	wantVictories(t, isl, 0, "volles Level")
	cave := isl.Stages[1]
	last := slices.IndexFunc(cave.Nodes, func(n *ResourceNode) bool { return !isVein(n) })
	if last < 0 {
		t.Fatal("Höhle ohne endliches Ressourcenobjekt")
	}
	isl.Stages[0].Nodes = onlyVeins(isl.Stages[0].Nodes)
	cave.Nodes = append(onlyVeins(cave.Nodes), cave.Nodes[last])
	wantVictories(t, isl, 0, "ein Objekt übrig")
	cave.Nodes = onlyVeins(cave.Nodes)
	wantOnce(t, isl)
}

func TestSiegAllesAbbauenOhnePlantage(t *testing.T) {
	isl := goalIsland(t, "mineAll")
	wantVictories(t, isl, 0, "volles Level")
	w := isl.Stages[0]
	farm := &Site{ID: w.newID(), Kind: "farm", State: "built"}
	tree := &ResourceNode{ID: w.newID(), Kind: "tree", Marked: true}
	farm.plots = []plot{{tree: tree}}
	for _, s := range isl.Stages {
		s.Nodes = nil
	}
	w.Sites, w.Nodes = append(w.Sites, farm), []*ResourceNode{tree}
	wantOnce(t, isl)
}

func TestSiegAllesAbbauenOhneEndlicheObjekteNie(t *testing.T) {
	isl := goalIsland(t, "mineAll")
	for _, w := range isl.Stages {
		w.Nodes = []*ResourceNode{{ID: w.newID(), Kind: veinKind(t), X: w.HubX}}
	}
	for range 30 {
		wantVictories(t, isl, 0, "ohne endliche Objekte")
	}
}

func veinKind(t *testing.T) string {
	t.Helper()
	kinds := slices.Sorted(maps.Keys(economy.Veins))
	if len(kinds) == 0 {
		t.Fatal("keine Ader in data/economy.json")
	}
	return kinds[0]
}

func onlyVeins(nodes []*ResourceNode) []*ResourceNode {
	var out []*ResourceNode
	for _, n := range nodes {
		if isVein(n) {
			out = append(out, n)
		}
	}
	return out
}

func TestSiegAllesAusbauenUeberAlleStufen(t *testing.T) {
	isl := goalIsland(t, "buildAll")
	wantVictories(t, isl, 0, "nichts gebaut")
	for _, w := range isl.Stages {
		for _, s := range w.Sites {
			s.State, s.HP, s.BuildProgress = "built", s.MaxHP, 1
		}
	}
	cave := isl.Stages[1]
	if len(cave.Sites) == 0 || len(isl.Stages[0].Sites) == 0 {
		t.Fatal("Stufe ohne Bauplatz")
	}
	open := cave.Sites[len(cave.Sites)-1]
	open.State = "unpaid"
	wantVictories(t, isl, 0, "ein Bauplatz offen in der Höhle")
	open.State, open.HP, open.BuildProgress = "built", open.MaxHP, 1
	wantOnce(t, isl)
}

// Mehrere Bedingungen im selben Tick und ein späterer Wechsel des Ziels geben zusammen genau ein `victory`.
func TestSiegZweiBedingungenEinVictory(t *testing.T) {
	isl := goalIsland(t, "gold")
	isl.GoldCollected, isl.EndbossDefeated = goals.Gold, true
	setDay(isl, goals.Days+1)
	wantOnce(t, isl)
	o := isl.Options
	o.Goal = "days"
	if err := SetOptions(isl, o, false); err != nil {
		t.Fatal(err)
	}
	wantVictories(t, isl, 0, "zweites Ziel erfüllt")
}

func TestSiegWerteAusDaten(t *testing.T) {
	raw, err := data.Files.ReadFile("goals.json")
	if err != nil {
		t.Fatal(err)
	}
	var want struct{ Gold, Days int }
	if err := json.Unmarshal(raw, &want); err != nil {
		t.Fatal(err)
	}
	if goals.Gold != want.Gold || goals.Days != want.Days || want.Gold != 1000 || want.Days != 20 {
		t.Fatalf("goals %+v, data/goals.json %+v (Startwerte 1000 Gold, 20 Tage)", goals, want)
	}
}

func TestSiegDeterministisch(t *testing.T) {
	play := func() ([][]Event, []string, int) {
		isl := goalIsland(t, "gold")
		isl.GoldCollected = goals.Gold - 1
		ev := runIsland(isl, nil, 20)
		return ev, islandJSON(t, isl), isl.GoldCollected
	}
	ev1, js1, g1 := play()
	ev2, js2, g2 := play()
	if !reflect.DeepEqual(ev1, ev2) || !slices.Equal(js1, js2) || g1 != g2 {
		t.Fatal("gleicher Seed, unterschiedlicher Verlauf")
	}
}
