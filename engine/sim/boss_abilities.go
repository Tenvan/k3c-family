package sim

import "math"

// Fähigkeiten der Minibosse von Eisenstollen und Kristallhöhle (docs/rules/bosse.md § 1.1, K2.1b):
// Flammenspur (`flameTrail`, Lava-Golem): Der Boss legt alle IntervalSeconds eine Bodenfläche an seinen Ort, solange
// dort noch keine liegt (so entsteht die Spur beim Gehen, ein stehender Boss stapelt keine Flächen). Wer in einer
// Fläche steht (Spieler, Truppen, Gebäude außer der Burg), nimmt einmal je Sekunde den Schaden einer Sekunde; die
// Fläche läuft nach DurationSeconds aus.
// Splitter-Wurf (`shardThrow`, Splitter-Titan): Der Boss kämpft auf Reichweite (`range` aus den Daten, Verhalten wie
// `ranged`); jeder Angriff ist ein Geschoss mit Splash, dessen Einschlag alle Ziele im Radius trifft.
// Beides verbraucht kein rng.

// shardSpeed: Tempo des Splitters in Units/s (wie die Geschosse der Fernkämpfer, enemies.go).
const shardSpeed = 20

// armBoss stellt die Fähigkeit beim Erscheinen ein: Wartezeit bis zum ersten Einsatz; der Splitter-Wurf ist sofort
// bereit und macht den Boss zum Fernkämpfer.
func armBoss(e *Enemy, a BossAbility) {
	e.AoeIn = a.IntervalSeconds
	if a.Type == "shardThrow" {
		e.AoeIn, e.Traits = 0, []string{"ranged"}
	}
}

// flameTrail legt eine Fläche an den Ort des Bosses, wenn dort noch keine liegt.
func flameTrail(w *World, e *Enemy, a BossAbility) {
	e.AoeIn = a.IntervalSeconds
	for _, h := range w.Hazards {
		if math.Abs(h.X-e.X) < h.Radius {
			return
		}
	}
	w.Hazards = append(w.Hazards, &Hazard{
		X: e.X, Radius: a.Radius, DamagePerSecond: a.DamagePerSecond, SecondsLeft: a.DurationSeconds, Cause: e.Kind,
	})
}

// shardThrow wirft einen Splitter auf das Ziel, das der Boss auch sonst wählen würde. Die Abklingzeit des Angriffs
// läuft mit dem Intervall, damit der Boss keinen Angriff ohne Splash macht.
func shardThrow(w *World, e *Enemy, a BossAbility) {
	dir := sign(w.HubX - e.X)
	if dir == 0 {
		dir = 1
	}
	t := chooseTarget(w, e, dir, blockingWall(w, e, dir))
	if t == nil {
		return
	}
	e.AoeIn, e.Cooldown = a.IntervalSeconds, a.IntervalSeconds
	p := &Projectile{
		ID: w.newID(), X: e.X, TargetID: t.id, Team: "enemy", Damage: e.Damage, Speed: shardSpeed, Cause: e.Kind,
		Splash: a.Radius,
	}
	w.Projectiles = append(w.Projectiles, p)
	arrowEvent(w, p, e.ID)
}

// hitSplash: Einschlag eines Geschosses mit Splash bei x: das Ziel und alle übrigen Ziele im Radius je einmal.
func hitSplash(w *World, p *Projectile, x float64) {
	applyDamageBy(w, p.TargetID, p.Damage, p.Cause)
	for _, c := range aoeTargets(w, x, p.Splash) {
		if c.id != p.TargetID {
			applyDamageBy(w, c.id, p.Damage, p.Cause)
		}
	}
}

// stepHazards lässt die Flächen auslaufen und zieht allen Zielen darin einmal je Sekunde den Schaden einer Sekunde
// ab (wie Lava, lava.go); ausgelaufene Flächen fallen in Listen-Reihenfolge heraus.
func stepHazards(w *World, dt float64) {
	if len(w.Hazards) == 0 {
		return
	}
	kept := w.Hazards[:0]
	for _, h := range w.Hazards {
		if h.SecondsLeft -= dt; h.SecondsLeft <= 0 {
			continue
		}
		if h.tickIn -= dt; h.tickIn <= 0 {
			h.tickIn++
			burn(w, h)
		}
		kept = append(kept, h)
	}
	w.Hazards = kept
}

// burn: Schaden an allen Zielen der Fläche außer der Burg und getrennten Monarchen.
func burn(w *World, h *Hazard) {
	for _, c := range aoeTargets(w, h.X, h.Radius) {
		if c.kind == "castle" || (c.player != nil && c.player.Free) {
			continue
		}
		applyDamageBy(w, c.id, h.DamagePerSecond, h.Cause)
	}
}
