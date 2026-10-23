package sim

import "testing"

// B-200/AC-02: Mine, zwei besetzte Adern mit je zwei Bauern, keine Kills: Der Aggressionspool ist frühestens nach
// 4 min Spielzeit voll (Werte aus data/biomes/mine.json › cycle und data/economy.json › veins). Gezählt wird der Druck
// aus den Lieferungen, dazu rechnerisch der Grundwert je Minute (die Tageszeit steht im Test still).
func TestAggressionspoolMitAdernNichtVorVierMinuten(t *testing.T) {
	w := newWorld(2, "adern-druck", Options{})
	w.Troops, w.Camps, w.Pickups, w.Enemies, w.CycleSpeed = []*Troop{}, []*Camp{}, []*Pickup{}, nil, 1e-6
	if w.Aggression == nil {
		t.Fatal("Mine ohne Aggressionspool")
	}
	var veins []*ResourceNode
	for _, n := range w.Nodes {
		if isVein(n) && len(veins) < 2 {
			n.X, n.Marked = w.HubX+1+float64(len(veins)), true
			veins = append(veins, n)
		}
	}
	if len(veins) < 2 {
		t.Fatalf("nur %d Adern in der Mine", len(veins))
	}
	for range 4 {
		p := spawnVagrant(w, w.HubX, w.HubX)
		p.Kind, p.HP, p.MaxHP = "peasant", troops["peasant"].HP, troops["peasant"].HP
	}
	*w.Aggression = 0
	const minutes = 4
	peak, busy := 0.0, [2]int{}
	// stepCycle setzt einen vollen Pool auf 0 zurück: deshalb das Maximum je Tick, nicht nur der Endwert.
	runUntil(w, minutes*60, func() bool {
		peak = max(peak, *w.Aggression)
		for i, n := range veins {
			busy[i] = max(busy[i], veinWorkers(w, n))
		}
		return false
	})
	total := peak + minutes*w.Biome.Cycle.PercentPerMinute
	t.Logf("Druck aus Lieferungen nach %d min: %.1f %%, mit Grundwert %.1f %%, Bauern je Ader %v", minutes, peak, total, busy)
	if busy != [2]int{2, 2} {
		t.Fatalf("Adern nicht mit je 2 Bauern besetzt: %v", busy)
	}
	if peak <= 0 {
		t.Fatal("kein Druck aus Lieferungen")
	}
	if total >= 100 {
		t.Fatalf("Pool nach %d min voll (%.1f %%)", minutes, total)
	}
}
