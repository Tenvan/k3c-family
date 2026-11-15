package sim

import (
	"math"
	"slices"

	"k3c/engine/rng"
)

// Portale (docs/rules/kampf.md § 2, B-927): Jedes Portal hat HP (data/biomes/*.json › portals.hp). Nur Truppen im Angriff
// (Troop.attacking, gesetzt vom Angriff der Kampfvariante, KS4) beschädigen es, Gegner, Türme, Mauern und der Monarch nie.
// Solange Truppen ein Portal angreifen, spawnt es laufend Verteidiger (portals.defense). Zerstörte Portale stehen in
// World.PortalsDown und im Spielstand (HubSave.PortalsDown); sind alle weg, ist die Stufe dauerhaft sicher.

// Safe: Alle Portale der Stufe sind zerstört (keine Wellen, kein Miniboss, kein Raub).
func (w *World) Safe() bool { return len(w.Portals) == 0 && len(w.PortalsDown) > 0 }

// syncPortals hält HP und Verteidiger-Takt gleich lang wie Portals (Portale kommen aus dem Level, Tests setzen sie auch von Hand).
func syncPortals(w *World) {
	if w.PortalMaxHP == 0 {
		w.PortalMaxHP = w.Biome.Portals.HP
	}
	for len(w.PortalHP) < len(w.Portals) {
		w.PortalHP = append(w.PortalHP, w.PortalMaxHP)
	}
	for len(w.portalDefense) < len(w.Portals) {
		w.portalDefense = append(w.portalDefense, 0)
	}
}

// portalInReach: Index des Portals in Schlagreichweite der Truppe t (wie ein Krieger an einer Mauer), sonst -1.
func portalInReach(w *World, t *Troop) int {
	reach := w.values().Troops[t.Kind].Range + body + 0.1
	best := -1
	for i, x := range w.Portals {
		if math.Abs(x-t.X) <= reach && (best < 0 || math.Abs(x-t.X) < math.Abs(w.Portals[best]-t.X)) {
			best = i
		}
	}
	return best
}

// stepPortals: Truppen im Angriff schlagen das Portal in Reichweite, angegriffene Portale wehren sich in Wellen, zerstörte
// verschwinden.
func stepPortals(w *World, dt float64) {
	if len(w.Portals) == 0 {
		return
	}
	syncPortals(w)
	attacked := make([]bool, len(w.Portals))
	for _, t := range w.Troops {
		t.siegeCD = math.Max(0, t.siegeCD-dt)
		if !t.attacking || !isFighter(w.values().Troops, t.Kind) {
			continue
		}
		i := portalInReach(w, t)
		if i < 0 {
			continue
		}
		attacked[i] = true
		if t.siegeCD <= 0 {
			d := w.values().Troops[t.Kind]
			w.PortalHP[i] -= d.Damage
			t.siegeCD = 1 / d.AttacksPerSecond
		}
	}
	for i := range w.Portals {
		if attacked[i] && w.PortalHP[i] > 0 {
			if w.portalDefense[i] -= dt; w.portalDefense[i] <= 0 {
				defendPortal(w, w.Portals[i])
				w.portalDefense[i] = w.Biome.Portals.Defense.IntervalSeconds
			}
		}
	}
	for i := len(w.Portals) - 1; i >= 0; i-- {
		if w.PortalHP[i] <= 0 {
			destroyPortal(w, i)
		}
	}
}

// defendPortal stellt eine Verteidiger-Welle am Portal x in die Warteschlange (Standard-Gegner des Portal-Pools, Größe je
// Tiefe, bei Inseln nach Spielerzahl und Grad skaliert).
func defendPortal(w *World, x float64) {
	var kinds []string
	for _, k := range w.Biome.Enemies.Portal {
		if d, ok := w.values().Enemies[k]; ok && d.Tier == "standard" {
			kinds = append(kinds, k)
		}
	}
	if len(kinds) == 0 {
		return
	}
	size, hp, damage := 1.0, 1.0, 1.0
	if w.island != nil {
		size, hp, damage = w.island.waveFactors()
	}
	var picks []string
	for n := scaled(w.Biome.Portals.Defense.Size, size); n > 0; n-- {
		picks = append(picks, rng.Pick(w.rng, kinds))
	}
	queueWave(w, spawnOrders(w.values(), picks, []float64{x}, w.rng), hp, damage)
	emit(w, "portalDefense", Event{"x": unitX(x), "count": len(picks)})
}

// destroyPortal entfernt das Portal i dauerhaft. Gegner mit diesem Portal als Ziel fliehen zum nächsten stehenden; das letzte
// Portal macht die Stufe sicher.
func destroyPortal(w *World, i int) {
	x := w.Portals[i]
	w.PortalsDown = append(w.PortalsDown, x)
	w.Portals = slices.Delete(w.Portals, i, i+1)
	w.PortalHP = slices.Delete(w.PortalHP, i, i+1)
	w.portalDefense = slices.Delete(w.portalDefense, i, i+1)
	w.SpawnQueue = slices.DeleteFunc(w.SpawnQueue, func(s QueuedSpawn) bool { return s.X == x })
	emit(w, "portalDestroyed", Event{"x": unitX(x), "left": len(w.Portals)})
	for _, e := range w.Enemies {
		if e.HomeX == x && len(w.Portals) > 0 {
			e.HomeX = standingPortalNear(w, e.X)
		}
	}
	if len(w.Portals) == 0 {
		secureStage(w)
	}
}

// standingPortalNear ist das stehende Portal nächst an x (Aufrufer prüfen, dass eins steht).
func standingPortalNear(w *World, x float64) float64 {
	best := w.Portals[0]
	for _, p := range w.Portals[1:] {
		if math.Abs(p-x) < math.Abs(best-x) {
			best = p
		}
	}
	return best
}

// secureStage: Das letzte Portal ist gefallen. Keine Wellen mehr, der Skill-Punkt des Miniboss geht in den Fund-Pool, getragene
// Beute fällt zu Boden (ohne Portal gibt es keinen Raub).
func secureStage(w *World) {
	w.SpawnQueue = []QueuedSpawn{}
	addPoolPoints(w, w.values().Monarch.SkillPointSources.Miniboss)
	for _, e := range w.Enemies {
		dropLoot(w, e)
	}
	emit(w, "stageSafe", Event{"depth": w.Biome.Depth})
}

// dropLoot lässt einen Gegner geraubtes Gold und getragene Ausrüstung fallen; er flieht nicht mehr (kein Portal).
func dropLoot(w *World, e *Enemy) {
	if e.CarriedGold > 0 {
		scatterCoins(w, e.X, e.CarriedGold)
		e.CarriedGold = 0
	}
	dropCarried(w, e)
	e.Fleeing = false
}

// applyPortals übernimmt die zerstörten Portale aus dem Spielstand (nach dem Anlegen aus dem Level).
func applyPortals(w *World, h HubSave) {
	w.PortalsDown = slices.Clone(h.PortalsDown)
	w.Portals = slices.DeleteFunc(w.Portals, func(p float64) bool {
		return slices.ContainsFunc(h.PortalsDown, func(d float64) bool { return math.Abs(d-p) < 0.01 })
	})
	w.PortalHP, w.portalDefense = nil, nil
	syncPortals(w)
}
