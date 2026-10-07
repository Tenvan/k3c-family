package sim

import (
	"math"
	"slices"
)

// Endboss (docs/rules/bosse.md § 1 und § 1.1, K2.1c): Er liegt in seinem Bau (Entität `lair` im Level, nur in Biomen
// mit `lair`) und wartet ohne Zeitdruck; solange er wartet, gibt es ihn nicht als Gegner, er ist also unverwundbar
// und tut nichts. Sobald ein lebender Spieler näher als `lair.triggerUnits` am Bau ist, erscheint er dort
// (`bossSpawned`, HP mit dem Spielerfaktor der Insel in diesem Moment) und bleibt aktiv bis zu seinem Tod, auch über
// einen Burgfall hinweg. Er läuft nicht (speed 0), sondern kämpft am Bau; Beschwörungen laufen wie jeder Gegner zum
// Hub. Seine Phasen (`phases`) wechseln an den HP-Schwellen, jeder Wechsel wird einmal gemeldet (`bossPhase`). Der
// Sieg setzt Island.EndbossDefeated; danach bleibt der Bau leer. Kein rng außer den Münzen der Belohnung.

// endbossFight ist der laufende Kampf gegen den Endboss einer Insel; phase ist der Index in BossData.Phases.
type endbossFight struct {
	boss  *Enemy
	phase int
}

// stepEndboss (vor stepEnemies): Auslösung am Bau, Rückkehr nach einem Burgfall, Phasenwechsel und Tempo.
func stepEndboss(w *World, dt float64) {
	isl := w.island
	if isl == nil {
		return
	}
	x, ok := lairX(w)
	if !ok {
		return
	}
	if f := isl.endboss; f != nil {
		if !slices.Contains(w.Enemies, f.boss) { // castleFallen räumt die Gegner ab, der Endboss bleibt
			w.Enemies = append(w.Enemies, f.boss)
		}
		endbossPhase(w, f, dt)
		return
	}
	b := endBoss(w.Biome.Depth)
	if b == nil || isl.EndbossDefeated || slices.Contains(isl.DefeatedBosses, b.ID) || !playerNear(w, x, b.Lair.TriggerUnits) {
		return
	}
	e := spawnBoss(w, b, x)
	armBoss(e, *b.Phases[0].Ability)
	isl.endboss = &endbossFight{boss: e}
}

// lairX: Ort des Baus aus dem Level der Stufe (false: die Stufe hat keinen).
func lairX(w *World) (float64, bool) {
	for _, e := range w.Level.Entities {
		if e.Kind == "lair" {
			return e.X, true
		}
	}
	return 0, false
}

// playerNear: Ein lebender, gesteuerter Spieler ist näher als dist an x.
func playerNear(w *World, x, dist float64) bool {
	return slices.ContainsFunc(w.Players, func(p *Player) bool {
		return !p.Free && p.HP > 0 && p.RespawnIn <= 0 && math.Abs(p.X-x) < dist
	})
}

// endbossPhase wechselt in jede Phase, deren HP-Schwelle unterschritten ist (je einmal, mit Meldung), stellt ihre
// Fähigkeit ein und lässt bei Tempo > 1 Abklingzeit und Fähigkeit entsprechend schneller ablaufen (stepEnemies zieht
// dt ab, hier kommt der Rest dazu).
func endbossPhase(w *World, f *endbossFight, dt float64) {
	e := f.boss
	b := bossByID(e.Kind)
	for f.phase+1 < len(b.Phases) && 100*e.HP < b.Phases[f.phase+1].BelowHPPercent*e.MaxHP {
		f.phase++
		p := b.Phases[f.phase]
		if p.Ability != nil {
			armBoss(e, *p.Ability)
		}
		if p.Tempo > 0 {
			e.Speed *= p.Tempo
		}
		emit(w, "bossPhase", Event{"boss": b.ID, "phase": f.phase + 1})
	}
	if t := phaseTempo(b, f.phase); t > 1 {
		extra := float64((t - 1) * dt)
		e.Cooldown, e.AoeIn = math.Max(0, e.Cooldown-extra), math.Max(0, e.AoeIn-extra)
	}
}

// phaseTempo: Tempo der Phase, ohne Angabe das der Phase davor (1: normal).
func phaseTempo(b *BossData, phase int) float64 {
	for i := phase; i >= 0; i-- {
		if t := b.Phases[i].Tempo; t > 0 {
			return t
		}
	}
	return 1
}

// bossAbilityOf: die Fähigkeit, die der Boss gerade nutzt: beim Endboss die der aktuellen Phase (ohne eigene die der
// Phase davor), sonst seine einzige.
func bossAbilityOf(w *World, b *BossData) BossAbility {
	if b.Kind != "end" || w.island == nil || w.island.endboss == nil {
		return b.Ability
	}
	for i := w.island.endboss.phase; i >= 0; i-- {
		if a := b.Phases[i].Ability; a != nil {
			return *a
		}
	}
	return b.Ability
}

// endbossDefeated: Merker für Siegvariante und Inselwechsel; der Kampf ist vorbei.
func endbossDefeated(w *World, b *BossData) {
	if isl := w.island; isl != nil && b.Kind == "end" {
		isl.EndbossDefeated, isl.endboss = true, nil
	}
}
