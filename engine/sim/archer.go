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

// freeTower ist der nächste gebaute Turm auf der Seite des Schützen mit freiem Platz.
func freeTower(w *World, t *Troop) *Site {
	var best *Site
	for _, s := range w.Sites {
		if s.Kind != "tower" || s.State != "built" || sign(s.X-w.HubX) != sign(t.AnchorX-w.HubX) {
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

// stepArcher: Posten ist ein freier Platz auf einem Turm, sonst hinter der äußersten Mauer seiner Seite.
func stepArcher(w *World, t *Troop, dt float64) {
	if t.TowerID == nil {
		if tower := freeTower(w, t); tower != nil {
			t.TowerID = intPtr(tower.ID)
		}
	}
	var tower *Site
	if t.TowerID != nil {
		tower = siteByID(w, *t.TowerID)
	}
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
	w.Projectiles = append(w.Projectiles, &Projectile{ID: w.newID(), X: t.X, TargetID: target.ID, Team: "player", Damage: a.Damage, Speed: arrowSpeed})
	t.Cooldown = 1 / a.AttacksPerSecond
}
