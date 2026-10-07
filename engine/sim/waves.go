package sim

import (
	"math"
	"slices"

	"k3c/engine/level"
	"k3c/engine/rng"
)

// Wellen und Geschosse (Port von src/world/sim/waves.ts und dem Rest von enemies.ts).

type spawnOrder struct {
	kind  string
	x     float64
	delay float64 // Sekunden nach Wellenstart
}

// planWave stellt eine Welle zusammen: Anzahl laut Tabelle, Gegnertypen aus dem Biom (nachts mit Nacht-Gegnern),
// gleichmäßig auf die Portale verteilt und über `spawnSpreadSeconds` gestaffelt.
func planWave(b level.Biome, wave int, r *rng.Rng, portals []float64, night bool, size float64) []spawnOrder {
	if len(portals) == 0 {
		return nil
	}
	row := waves.Table[0]
	for _, t := range waves.Table {
		if wave >= t.FromWave {
			row = t
		}
	}
	kinds := slices.Clone(b.Enemies.Portal)
	if night {
		kinds = append(kinds, b.Enemies.Night...)
	}
	var standard, elite []string
	for _, k := range kinds {
		if d, ok := enemyData[k]; ok && d.Tier == "standard" {
			standard = append(standard, k)
		} else if ok && d.Tier == "elite" {
			elite = append(elite, k)
		}
	}
	var picks []string
	if len(standard) > 0 {
		for n := scaled(r.Int(row.Standard[0], row.Standard[1]), size); n > 0; n-- {
			picks = append(picks, rng.Pick(r, standard))
		}
	}
	if len(elite) > 0 {
		for n := scaled(r.Int(row.Elite[0], row.Elite[1]), size); n > 0; n-- {
			picks = append(picks, rng.Pick(r, elite))
		}
	}
	return spawnOrders(picks, portals, r)
}

// spawnOrders verteilt die Plätze reihum auf die Portale und staffelt sie über `spawnSpreadSeconds`.
func spawnOrders(picks []string, portals []float64, r *rng.Rng) []spawnOrder {
	out := make([]spawnOrder, 0, len(picks))
	for i, kind := range picks {
		delay := float64(float64(i)/math.Max(1, float64(len(picks)))*waves.SpawnSpreadSeconds) + r.Next()
		x := portals[i%len(portals)]
		out = append(out, spawnOrder{kind, x, delay})
		// Schwarm-Platz (`swarm`): swarmSize Gegner am selben Portal, die übrigen je eine Zufalls-Sekunde später.
		for n := 1; slices.Contains(enemyData[kind].Traits, "swarm") && n < enemyData[kind].SwarmSize; n++ {
			out = append(out, spawnOrder{kind, x, delay + r.Next()})
		}
	}
	return out
}

func startWave(w *World) {
	w.Wave++
	size, hp, damage := 1.0, 1.0, 1.0 // nur Inseln skalieren nach Spieleranzahl und Grad
	if w.island != nil {
		size, hp, damage = w.island.waveFactors()
	}
	plan := planWave(w.Biome, w.Wave, w.rng, w.Portals, w.Cycle.Phase == "night", size)
	for _, s := range plan {
		w.SpawnQueue = append(w.SpawnQueue, QueuedSpawn{Kind: s.kind, X: s.x, At: w.Time + s.delay, hpFactor: hp, damageFactor: damage})
	}
	w.Events = append(w.Events, Event{"type": "wave", "wave": w.Wave, "count": len(plan)})
	triggerBoss(w)
}

// targetX ist die Position des Ziels eines Geschosses (Gegner, Spieler, Truppen, Bauplätze, Burg).
func targetX(w *World, id int) (float64, bool) {
	for _, e := range w.Enemies {
		if e.ID == id {
			return e.X, true
		}
	}
	for _, p := range w.Players {
		if p.ID == id {
			return p.X, true
		}
	}
	if t := troopByID(w, id); t != nil {
		return t.X, true
	}
	if s := siteByID(w, id); s != nil {
		return s.X, true
	}
	return w.Castle.X, w.Castle.ID == id
}

func stepProjectiles(w *World, dt float64) {
	kept := w.Projectiles[:0]
	for _, pr := range w.Projectiles {
		x, ok := targetX(w, pr.TargetID)
		if !ok {
			continue
		}
		if math.Abs(x-pr.X) <= pr.Speed*dt {
			if pr.Splash > 0 {
				hitSplash(w, pr, x) // Splitter-Wurf (boss_abilities.go)
			} else {
				applyDamageBy(w, pr.TargetID, pr.Damage, pr.Cause)
			}
			continue
		}
		pr.X += float64(sign(x-pr.X) * pr.Speed * dt)
		kept = append(kept, pr)
	}
	w.Projectiles = kept
}

// removeDeadEnemies: Besiegte Gegner droppen Gold (plus geklautes Gold), manchmal auch die Stufen-Ressource; ein Boss
// gibt stattdessen seine Belohnung (bossDefeated, ohne Würfel für den Drop).
func removeDeadEnemies(w *World) {
	kept := w.Enemies[:0]
	for _, e := range w.Enemies {
		if e.HP > 0 {
			kept = append(kept, e)
			continue
		}
		if e.Boss {
			bossDefeated(w, e)
		} else {
			enemyDrop(w, e)
		}
		if w.Aggression != nil && w.Biome.Cycle.Type == "aggressionPool" {
			*w.Aggression = math.Min(100, *w.Aggression+w.Biome.Cycle.PercentPerKill)
		}
	}
	w.Enemies = kept
}

// enemyDrop: Gold laut Daten plus geklautes Gold als Münzen, mit Chance die Stufen-Ressource in den Vorrat.
func enemyDrop(w *World, e *Enemy) {
	gold := enemyData[e.Kind].Gold
	dropped := w.rng.Int(gold[0], gold[1]) + e.CarriedGold
	scatterCoins(w, e.X, dropped)
	emit(w, "kill", Event{"kind": e.Kind, "x": unitX(e.X), "gold": dropped})
	drop := economy.EnemyResourceDrop
	if w.rng.Next() < drop.Chance && addStock(w, w.Biome.PrimaryResource, drop.Amount) {
		w.Events = append(w.Events, Event{"type": "gathered", "resource": w.Biome.PrimaryResource, "amount": drop.Amount})
	}
}
