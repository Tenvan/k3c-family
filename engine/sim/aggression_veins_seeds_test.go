package sim

import (
	"os"
	"slices"
	"strconv"
	"testing"
)

// B-565 (B-200/AC-01): Wellenabstand unter Tage über 100 Seeds, Mine, Normal, 2 Spieler, ohne und mit zwei besetzten
// Adern (je 2 Bauern, keine Truppen, keine Kills). Gemessen wird die Zeit der Pool-Wellen (Aggressionspool voll →
// startWave); der Test misst und prüft nur die Grundlage (Adern besetzt, Wellen bei besetzten Adern), den Korridor
// 4–12 min (docs/rules/zielkorridore.md) bewertet der Messbericht in B-565, Werte ändert nur 🧑.

const (
	veinSeeds   = 100
	veinHorizon = 60 * 60.0 // Sekunden Spielzeit je Lauf
)

// poolWaves: Zeitpunkte (Minuten) der Wellen aus dem Pool in `veinHorizon`; ok false: die Mine hat keine zwei Adern.
func poolWaves(seed string, occupied bool) (waves []float64, ok bool) {
	w := newWorld(2, seed, Options{})
	w.Troops, w.Camps, w.Pickups, w.Enemies = []*Troop{}, []*Camp{}, []*Pickup{}, nil
	if w.Aggression == nil {
		return nil, false
	}
	if occupied {
		var veins []*ResourceNode
		for _, n := range w.Nodes {
			if isVein(n) && len(veins) < 2 {
				n.X, n.Marked = w.HubX+1+float64(len(veins)), true
				veins = append(veins, n)
			}
		}
		if len(veins) < 2 {
			return nil, false
		}
		for range 4 {
			p := spawnVagrant(w, w.HubX, w.HubX)
			p.Kind, p.HP, p.MaxHP = "peasant", troops["peasant"].HP, troops["peasant"].HP
		}
	}
	*w.Aggression = 0
	prev := 0.0
	for tm := 0.0; tm < veinHorizon; tm += dt {
		Step(w, nil, dt)
		if prev > 90 && *w.Aggression < 1 {
			waves = append(waves, (tm+dt)/60)
		}
		prev = *w.Aggression
	}
	return waves, true
}

func TestWellenabstandMineMitAdern(t *testing.T) {
	if os.Getenv("K3C_MESSUNG") != "1" {
		t.Skip("Messung über 100 Seeds (18 s), nur mit K3C_MESSUNG=1")
	}
	var first, gaps []float64
	skipped, noWave := 0, 0
	for i := 1; i <= veinSeeds; i++ {
		waves, ok := poolWaves(strconv.Itoa(i), true)
		if !ok {
			skipped++
			continue
		}
		if len(waves) == 0 {
			noWave++
			continue
		}
		first = append(first, waves[0])
		for j := 1; j < len(waves); j++ {
			gaps = append(gaps, waves[j]-waves[j-1])
		}
	}
	t.Logf("mit Adern: %d Seeds übersprungen, %d ohne Welle in %.0f min; erste Welle %s; Abstand folgender Wellen %s",
		skipped, noWave, veinHorizon/60, summary(first), summary(gaps))
	if len(first) == 0 {
		t.Fatal("mit besetzten Adern keine Welle aus dem Pool")
	}
	base, _ := poolWaves("1", false)
	t.Logf("ohne Adern (Seed 1, deterministisch, Horizont %.0f min): Wellen %v", veinHorizon/60, base)
}

// summary: Median und p10 (10. Perzentil, unterer Rang) mit Anzahl.
func summary(v []float64) string {
	if len(v) == 0 {
		return "n=0"
	}
	s := slices.Clone(v)
	slices.Sort(s)
	return "n=" + strconv.Itoa(len(s)) + " Median " + strconv.FormatFloat(s[len(s)/2], 'f', 2, 64) +
		" min, p10 " + strconv.FormatFloat(s[len(s)/10], 'f', 2, 64) + " min, min " + strconv.FormatFloat(s[0], 'f', 2, 64) + " min"
}
