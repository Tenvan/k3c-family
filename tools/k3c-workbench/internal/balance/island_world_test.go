package balance

import (
	"strings"
	"testing"

	"k3c/engine/sim"
)

// BS11.1/AC-01 (B-642/AC-01): Mit World hat die Insel alle Tiefen der ersten Insel, ohne World wie bisher eine Stufe.
func TestWorldBautAlleTiefen(t *testing.T) {
	isl, err := newIsland(Scenario{Seed: "1", Players: 2, Days: 1, World: "standard"})
	if err != nil {
		t.Fatal(err)
	}
	want := sim.StandardWorld().Islands[0].Depths
	if len(isl.Stages) != 5 || len(isl.Stages) != len(want) {
		t.Fatalf("Stufen %d, Tiefen der Welt %v", len(isl.Stages), want)
	}
	for i, w := range isl.Stages {
		if w.Biome.Depth != want[i] {
			t.Errorf("Stufe %d hat Tiefe %d, erwartet %d", i, w.Biome.Depth, want[i])
		}
	}
	if len(isl.Stages[0].Players) != 2 {
		t.Errorf("Monarchen auf Stufe 0: %d", len(isl.Stages[0].Players))
	}
	old, err := newIsland(Scenario{Seed: "1", Players: 2, Days: 1})
	if err != nil || len(old.Stages) != 1 {
		t.Fatalf("ohne World: %v, %d Stufen", err, len(old.Stages))
	}
	if _, err := newIsland(Scenario{Seed: "1", Players: 1, Days: 1, World: "gibtsnicht"}); err == nil {
		t.Error("unbekannte Welt muss ein Fehler sein")
	}
}

// Ein Lauf ohne World schreibt keine Insel-Felder; mit World stehen die Stufen im Ergebnis.
func TestInselLaufMetriken(t *testing.T) {
	r := mustRun(t, Matrix{Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"saver"}, Depths: []int{0}, Days: 1, World: "standard"})
	m := r.Results[0].Metrics
	if !r.Results[0].Valid || len(m.Stages) != 5 {
		t.Fatalf("Lauf %+v", r.Results[0])
	}
	if s := m.Stages[0]; s.ArrivalTick == nil || *s.ArrivalTick != 0 || *s.ArrivalDay != 1 {
		t.Errorf("Stufe 0 beginnt mit Ankunft Tag 1: %+v", s)
	}
	if m.Stages[1].ArrivalTick != nil { // saver steigt nicht ab
		t.Errorf("Stufe 1 ohne Spieler: %+v", m.Stages[1])
	}
	plain := mustRun(t, Matrix{Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"saver"}, Depths: []int{0}, Days: 1})
	b, _ := plain.JSON()
	for _, k := range []string{"stages", "bosses", "endbossDay", `"world"`} {
		if strings.Contains(string(b), k) {
			t.Errorf("Lauf ohne World enthält %s", k)
		}
	}
}

// BS11.1/AC-02: Der Sammler führt Ankunft, Bosse, Endboss und Burgfall je Stufe nach festem Ereignisverlauf.
func TestSammlerInselEreignisse(t *testing.T) {
	isl, err := newIsland(Scenario{Seed: "1", Players: 1, Days: 1, World: "standard"})
	if err != nil {
		t.Fatal(err)
	}
	c := newCollector(isl, 0)
	sim.AddIslandPlayer(isl, 2) // ein zweiter Monarch steht auf Stufe 2
	c.observeIsland(100)
	c.islandEvent(isl.Stages[2], sim.Event{"type": "bossDefeated", "boss": "mini2"}, 120)
	c.islandEvent(isl.Stages[2], sim.Event{"type": "castleFallen"}, 130)
	isl.EndbossDefeated = true
	c.observeIsland(200)
	m := c.result()
	if s := m.Stages[2]; s.ArrivalTick == nil || *s.ArrivalTick != 100 || s.CastleFallTick == nil || *s.CastleFallTick != 130 {
		t.Errorf("Stufe 2: %+v", s)
	}
	if len(m.Bosses) != 1 || m.Bosses[0].Boss != "mini2" || m.Bosses[0].Tick != 120 {
		t.Errorf("Bosse %+v", m.Bosses)
	}
	if m.EndbossTick == nil || *m.EndbossTick != 200 {
		t.Errorf("Endboss %v", m.EndbossTick)
	}
	checkInselBericht(t, m)
}

// checkInselBericht: Kennzahlen und Markdown zeigen Tiefen, Bosse und Endboss.
func checkInselBericht(t *testing.T, m *Metrics) {
	t.Helper()
	f := FiguresOf(Report{Results: []Result{{Valid: true, Metrics: m}}})
	if f == nil || f.Island == nil || len(f.Island.Depths) != 5 || f.Island.Endboss.Defeated != 100 || len(f.Island.Bosses) != 1 {
		t.Fatalf("Kennzahlen %+v", f)
	}
	var b strings.Builder
	f.Island.markdown(&b)
	if !strings.Contains(b.String(), "| mini2 | 100") || !strings.Contains(b.String(), "| 4 |") {
		t.Errorf("Markdown ohne Tiefen und Bosse: %s", b.String())
	}
}

// BS11.4: Scope.World lässt alle Ziele die ganze Insel spielen; jede andere Einschränkung ohne World entfernt die Insel-Ziele.
func TestScopeWorld(t *testing.T) {
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	days := 40
	w, err := ts.Restrict(Scope{Days: &days, Bot: "coop2", World: "standard"})
	if err != nil {
		t.Fatal(err)
	}
	if len(w.Targets) != len(ts.Targets) {
		t.Errorf("%d statt %d Ziele", len(w.Targets), len(ts.Targets))
	}
	for _, g := range w.Targets {
		if g.World != "standard" || g.Days != 40 || g.Bot != "coop2" {
			t.Fatalf("Ziel %q: %+v", g.Name, g)
		}
	}
	one, err := ts.Restrict(Scope{Days: &days})
	if err != nil {
		t.Fatal(err)
	}
	for _, g := range one.Targets {
		if g.World != "" {
			t.Errorf("Ziel %q behält World", g.Name)
		}
	}
}
