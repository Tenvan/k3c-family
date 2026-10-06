package sim

import "testing"

// K1.2 (B-128/AC-03, AC-04, Sprint K1 AC-03, AC-04): Tor und übrige Gebäude als Ziele der Gegner (gegner.md § 4),
// `kill` je Tod. (a) Tor hält auf, wird angegriffen, nach der Zerstörung läuft der Gegner weiter, und (b) zwei Spieler
// und ein Bauer laufen hindurch: gate_test.go (W3.1, TestTorHaeltGegnerAufBisZurZerstoerung, TestEigenePassierenDasTor).

// workshopAt baut die Werkstatt und stellt sie nach x.
func workshopAt(t *testing.T, w *World, x float64) *Site {
	t.Helper()
	s := builtSite(t, w, "workshop")
	built(s)
	s.X = x
	return s
}

// targetOf ist das Ziel, das e jetzt wählt (mit Mauer oder Tor wie in stepEnemies), oder 0.
func targetOf(w *World, e *Enemy) int {
	dir := sign(w.HubX - e.X)
	var wall *Site
	if !e.has("ignoresWalls") {
		wall = blockingWall(w, e, dir)
	}
	if c := chooseTarget(w, e, dir, wall); c != nil {
		return c.id
	}
	return 0
}

// (c) Gegner mit `ignoresWalls` (auch mit Phasen) laufen am gebauten Tor vorbei, ohne es anzugreifen.
func TestGegnerIgnoresWallsLaeuftAmTorVorbei(t *testing.T) {
	for _, kind := range []string{"bat", "mineGhost"} {
		w := dayWorld(t)
		gate := sturdy(lineSite(t, w, "gate", -1, 1))
		e := spawnEnemy(w, kind, gate.X-4)
		run(w, 3)
		if e.X <= gate.X || gate.HP < gate.MaxHP {
			t.Errorf("%s bei %v, Tor bei %v mit HP %v: aufgehalten oder angegriffen", kind, e.X, gate.X, gate.HP)
		}
	}
}

// (d) `prefersBuildings` (Höhlentroll) nimmt die Werkstatt vor einem Krieger im selben Abstand; ein Gegner ohne die
// Eigenschaft nimmt den Krieger und die Werkstatt erst, wenn nichts anderes in Reichweite ist.
func TestGegnerGebaeudeAlsZiel(t *testing.T) {
	w := dayWorld(t)
	w.Troops = nil
	shop := workshopAt(t, w, w.HubX-20)
	warrior := &Troop{ID: w.newID(), Kind: "warrior", X: shop.X - 2, HP: 50, MaxHP: 50}
	w.Troops = append(w.Troops, warrior)
	troll := spawnEnemy(w, "caveTroll", shop.X-1)
	skeleton := spawnEnemy(w, "skeleton", shop.X-1)
	if got := targetOf(w, troll); got != shop.ID {
		t.Errorf("Höhlentroll greift %d an, erwartet Werkstatt %d", got, shop.ID)
	}
	if got := targetOf(w, skeleton); got != warrior.ID {
		t.Errorf("Skelett greift %d an, erwartet Krieger %d", got, warrior.ID)
	}
	w.Troops = nil
	if got := targetOf(w, skeleton); got != shop.ID {
		t.Errorf("Skelett ohne andere Ziele greift %d an, erwartet Werkstatt %d", got, shop.ID)
	}
}

// (e) Nahkämpfer erreichen kein Gebäude hinter dem Tor, auch nicht mit `prefersBuildings`; sie greifen das Tor an.
func TestGegnerNahkampfErreichtKeinGebaeudeHinterDemTor(t *testing.T) {
	w := dayWorld(t)
	gate := sturdy(lineSite(t, w, "gate", -1, 1))
	shop := workshopAt(t, w, gate.X+0.5)
	for _, kind := range []string{"skeleton", "caveTroll"} {
		e := spawnEnemy(w, kind, gate.X-body)
		if got := targetOf(w, e); got != gate.ID {
			t.Errorf("%s greift %d an, erwartet Tor %d (Werkstatt %d dahinter)", kind, got, gate.ID, shop.ID)
		}
	}
}

// (f) Jeder Tod eines Gegners meldet genau ein `kill` mit Art und Ort, auch fünf in einem Tick und an der Obergrenze.
func TestGegnerKillJeTod(t *testing.T) {
	w := quietWorld(t)
	var dying []*Enemy
	for i := range 5 {
		e := spawnEnemy(w, "skeleton", w.HubX-20-float64(i)*2)
		e.HP = 0
		dying = append(dying, e)
	}
	kills := eventsOf(w, "kill", 1, nil)
	dead := map[float64]bool{} // Ort beim Tod: Sie laufen in diesem Tick noch, bevor sie entfernt werden.
	for _, e := range dying {
		dead[unitX(e.X)] = true
	}
	if len(kills) != 5 {
		t.Fatalf("%d kill in einem Tick, erwartet 5: %v", len(kills), kills)
	}
	for _, k := range kills {
		if k["kind"] != "skeleton" || !dead[k["x"].(float64)] {
			t.Errorf("kill ohne Art oder Ort: %v", k)
		}
	}
	for i := range 5 {
		spawnEnemy(w, "skeleton", w.HubX-20-float64(i)).HP = 0
	}
	w.Events = nil
	for range maxEventsPerTick {
		hitEvent(w, "enemy", 0, 0, 1)
	}
	removeDeadEnemies(w)
	capEvents(w)
	got := 0
	for _, ev := range w.Events {
		if ev["type"] == "kill" {
			got++
		}
	}
	if got != 5 || w.EventsDropped == 0 {
		t.Errorf("an der Obergrenze %d kill (verworfen %d), erwartet 5 und Treffer verworfen", got, w.EventsDropped)
	}
}
