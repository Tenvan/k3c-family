package sim

import "math"

// Bogenschützen (Port von `stepArcher` und `isOnTower` aus src/world/sim/units.ts).

func isOnTower(w *World, t *Troop) bool {
	if t.TowerID == nil {
		return false
	}
	tower := siteByID(w, *t.TowerID)
	return tower != nil && math.Abs(tower.X-t.X) < arrive
}

// freeTower ist der nächste gebaute Turm auf der Seite des Schützen mit freiem Platz; ein Zaubertum hat keine.
func freeTower(w *World, t *Troop) *Site {
	var best *Site
	for _, s := range w.Sites {
		if s.Kind != "tower" || s.State != "built" || isSpellTower(w, s) || sign(s.X-w.HubX) != sign(t.AnchorX-w.HubX) {
			continue
		}
		used := 0
		for _, o := range w.Troops {
			if o.TowerID != nil && *o.TowerID == s.ID {
				used++
			}
		}
		if used < buildings["tower"].ArcherSlots && (best == nil || math.Abs(s.X-t.X) < math.Abs(best.X-t.X)) {
			best = s
		}
	}
	return best
}

// archerTower ist der Turm des Schützen (einen freien nimmt er sich); nil ohne Turm. Auf einem fertigen Zaubertum
// steigt er ab und nimmt einen freien Posten (Q31).
func archerTower(w *World, t *Troop) *Site {
	if t.TowerID == nil {
		if tower := freeTower(w, t); tower != nil {
			t.TowerID = intPtr(tower.ID)
		}
	}
	if t.TowerID == nil {
		return nil
	}
	tower := siteByID(w, *t.TowerID)
	if tower != nil && isSpellTower(w, tower) {
		t.TowerID, tower = nil, nil
	}
	return tower
}

// stepArcher: Posten ist ein freier Platz auf einem Turm, sonst hinter der äußersten Mauer seiner Seite.
func stepArcher(w *World, t *Troop, dt float64) {
	tower := archerTower(w, t)
	side := 1.0
	if t.AnchorX < w.HubX {
		side = -1
	}
	wall := outerWall(w, side)
	post := w.HubX + float64(side*10)
	if tower != nil {
		post = tower.X
	} else if wall != nil {
		post = wall.X - float64(side*2)
	}
	walkTo(t, post, dt)

	t.Cooldown = math.Max(0, t.Cooldown-dt)
	a := troops["archer"]
	reach := a.Range
	if isOnTower(w, t) {
		reach += float64(buildings["tower"].RangeBonus)
	}
	if t.Cooldown > 0 {
		return
	}
	var target *Enemy
	for _, e := range w.Enemies {
		if math.Abs(e.X-t.X) > reach {
			continue
		}
		if target == nil || math.Abs(e.X-t.X) < math.Abs(target.X-t.X) {
			target = e
		}
	}
	if target == nil {
		return
	}
	p := &Projectile{ID: w.newID(), X: t.X, TargetID: target.ID, Team: "player", Damage: a.Damage, Speed: arrowSpeed}
	w.Projectiles = append(w.Projectiles, p)
	arrowEvent(w, p, t.ID)
	t.Cooldown = 1 / a.AttacksPerSecond
}
