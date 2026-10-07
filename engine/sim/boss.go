package sim

import (
	"math"
	"slices"
)

// Bosse (docs/rules/bosse.md § 1): Ein Miniboss kommt mit seiner Welle über das erste Portal der Stufe, nutzt seine
// Fähigkeit, flieht nie und gibt beim Tod seine Belohnung. Ein besiegter Boss steht in Island.DefeatedBosses und kommt
// nie wieder (auch nicht nach einem Burgfall). Bosse verbrauchen beim Erscheinen kein rng; nur die Münzen der
// Belohnung werden wie jeder Drop gestreut. Bosse gibt es nur auf Inseln (der Merker gehört der Insel). Der Endboss
// wartet in seinem Bau (boss_endboss.go).

// triggerBoss: Am Wellenstart erscheint der Miniboss der Stufe ab seiner Welle, solange er nicht besiegt ist und
// kein Boss der Stufe lebt.
func triggerBoss(w *World) {
	isl := w.island
	b := miniBoss(w.Biome.Depth)
	if isl == nil || b == nil || w.Wave < b.Wave || len(w.Portals) == 0 || slices.Contains(isl.DefeatedBosses, b.ID) {
		return
	}
	if slices.ContainsFunc(w.Enemies, func(e *Enemy) bool { return e.Boss }) {
		return
	}
	spawnBoss(w, b, w.Portals[0])
}

// spawnBoss stellt den Boss bei x auf: Werte des Bezugs-Gegners × Boss-Faktoren, mit Tiefen-Skalierung, Grad
// (enemyHp, enemyDamage) und Spielerfaktor der Insel auf die HP; die Wellengröße des Grads gilt nicht. Der
// Spielerfaktor wirkt auf die gerundeten HP, damit 3 Spieler genau doppelt so viele HP ergeben wie einer.
func spawnBoss(w *World, b *BossData, x float64) *Enemy {
	hp, damage, players := 1.0, 1.0, 1
	if isl := w.island; isl != nil {
		g := difficulty[isl.Options.Grade]
		hp, damage, players = g.EnemyHP, g.EnemyDamage, max(len(isl.Players()), 1)
	}
	e := spawnScaled(w, b.Base, x, b.HPFactor*hp, b.DamageFactor*damage)
	e.HP = math.Round(e.HP * (1 + waves.PerExtraPlayer*float64(players-1)))
	e.MaxHP = e.HP
	e.Kind, e.Boss, e.Traits, e.Range = b.ID, true, nil, b.Range
	e.Speed = b.Speed * math.Pow(waves.DepthScaling.Speed, float64(w.Biome.Depth))
	armBoss(e, b.Ability)
	emit(w, "bossSpawned", Event{"boss": b.ID, "x": unitX(x)})
	return e
}

// bossAbility (nur Bosse): Ist die Fähigkeit (beim Endboss die seiner Phase, boss_endboss.go) fällig (Enemy.AoeIn zählt bei Bossen bis zur nächsten), ruft `summon` Gegner an den
// Ort des Bosses; `aoe` schlägt, sobald ein Ziel im Radius steht, und trifft alle Ziele dort je einmal; `flameTrail`
// und `shardThrow` stehen in boss_abilities.go.
func bossAbility(w *World, e *Enemy) {
	if !e.Boss || e.AoeIn > 0 {
		return
	}
	b := bossByID(e.Kind)
	if b == nil {
		return
	}
	a := bossAbilityOf(w, b)
	switch a.Type {
	case "summon":
		e.AoeIn = a.IntervalSeconds
		summon(w, e, a)
	case "flameTrail":
		flameTrail(w, e, a)
	case "shardThrow":
		shardThrow(w, e, a)
	case "aoe":
		targets := aoeTargets(w, e.X, a.Radius)
		if len(targets) == 0 {
			return
		}
		e.AoeIn = a.IntervalSeconds
		emit(w, "strike", Event{"from": e.ID, "x": unitX(e.X)})
		for _, c := range targets {
			applyDamageBy(w, c.id, e.Damage, e.Kind)
			if c.player != nil {
				frostArmorHit(c.player, e)
			}
		}
	}
}

// summon: Count Gegner (ein Schwarm-Gegner als ganzer Schwarm) mit den Wellenfaktoren der Insel am Ort des Bosses;
// sie fliehen zu seinem Portal.
func summon(w *World, boss *Enemy, a BossAbility) {
	hp, damage := 1.0, 1.0
	if w.island != nil {
		_, hp, damage = w.island.waveFactors()
	}
	n := a.Count
	if d := enemyData[a.Enemy]; slices.Contains(d.Traits, "swarm") {
		n *= max(d.SwarmSize, 1)
	}
	for range n {
		spawnScaled(w, a.Enemy, boss.X, hp, damage).HomeX = boss.HomeX
	}
}

// bossDefeated: Belohnung (Gold als Münzen am Ort, Material in den Insel-Vorrat bis zum Lager-Maximum, Skill-Punkte
// in den Fund-Pool), Merker „besiegt“ und Ereignis.
func bossDefeated(w *World, e *Enemy) {
	b := bossByID(e.Kind)
	scatterCoins(w, e.X, b.Reward.Gold+e.CarriedGold)
	res := w.Biome.PrimaryResource
	if taken, ok := addStockCapped(w, res, b.Reward.Material); ok && taken > 0 {
		emit(w, "gathered", Event{"resource": res, "amount": taken})
	}
	points := monarch.SkillPointSources.Miniboss
	if b.Kind == "end" {
		points = monarch.SkillPointSources.Endboss
	}
	addPoolPoints(w, points)
	if isl := w.island; isl != nil && !slices.Contains(isl.DefeatedBosses, b.ID) {
		isl.DefeatedBosses = append(isl.DefeatedBosses, b.ID)
	}
	endbossDefeated(w, b)
	emit(w, "bossDefeated", Event{"boss": b.ID, "x": unitX(e.X)})
}
