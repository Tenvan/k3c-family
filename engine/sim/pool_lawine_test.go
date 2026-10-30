package sim

import "testing"

// B-646/AC-01: Unter Tage löst eine getötete Welle nicht sofort die nächste aus. Der Pool friert ein, solange Gegner der
// Welle leben oder kommen, und wächst nach der letzten Welle erst wieder nach minWaveGapSeconds.
func TestAggressionspoolFriertEin(t *testing.T) {
	w := newWorld(2, "b646", Options{})
	if w.Aggression == nil {
		t.Fatal("Mine ohne Aggressionspool")
	}
	*w.Aggression = 0
	w.Wave = 0
	addPool(w, 5)
	if *w.Aggression != 5 {
		t.Fatalf("freier Pool wächst nicht: %v", *w.Aggression)
	}
	w.Enemies = append(w.Enemies, &Enemy{ID: 900, HP: 10})
	addPool(w, 5)
	if *w.Aggression != 5 {
		t.Errorf("Gegner leben, Pool wuchs auf %v", *w.Aggression)
	}
	w.Enemies[len(w.Enemies)-1].Guard = true // Wachen gehören zu keiner Welle
	addPool(w, 5)
	if *w.Aggression != 10 {
		t.Errorf("Wache friert den Pool ein: %v", *w.Aggression)
	}
	w.Enemies = nil
	w.Wave, w.lastWaveAt, w.Time = 1, 100, 100+minWaveGapSeconds-1
	addPool(w, 5)
	if *w.Aggression != 10 {
		t.Errorf("Mindestabstand nicht beachtet: %v", *w.Aggression)
	}
	w.Time = 100 + minWaveGapSeconds
	addPool(w, 5)
	if *w.Aggression != 15 {
		t.Errorf("nach dem Mindestabstand wächst der Pool wieder: %v", *w.Aggression)
	}
	w.SpawnQueue = append(w.SpawnQueue, QueuedSpawn{Kind: "goblin"})
	addPool(w, 5)
	if *w.Aggression != 15 {
		t.Errorf("Warteschlange einer Welle friert den Pool ein: %v", *w.Aggression)
	}
}

// Wellen-Lawine: Dauerkämpfe in der Mine bringen in zehn Spielminuten höchstens drei Wellen.
func TestKeineWellenLawine(t *testing.T) {
	isl := mustIsland(t, "b646-lawine", []int{0, 1, 2})
	p := AddIslandPlayer(isl, 2)
	w := isl.Stages[2]
	*w.Aggression = 99.9
	dt := 1.0 / 30
	for i := 0; i < 10*60*30; i++ {
		StepIsland(isl, []PlayerCommand{{}}, dt)
		for _, e := range w.Enemies { // jeder Gegner stirbt sofort: je Kill +5 % ohne Einfrieren
			e.HP = 0
		}
		p.HP = p.MaxHP
	}
	if w.Wave > 3 {
		t.Errorf("%d Wellen in 10 Spielminuten (Lawine)", w.Wave)
	}
}
