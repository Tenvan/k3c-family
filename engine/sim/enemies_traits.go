package sim

import (
	"fmt"
	"io/fs"
	"maps"
	"math"
	"slices"

	"k3c/data"
)

// Traits der Gegner (gegner.md § 2): Flächenschlag (`aoe`), Phasen (`phases`), Kiting (`kiting`) und die Angriffsrate
// je Gegner; der Schwarm (`swarm`) entsteht beim Planen der Welle (waves.go). Parameter stehen in data/enemies.json.

// knownTraits sind alle Traits, die die Simulation kennt; ein anderes Trait in enemies.json scheitert beim Laden (B-013).
var knownTraits = []string{"stealsGold", "prefersTroops", "prefersBuildings", "prefersTowers", "prefersMonarch",
	"fleesAtHalfHp", "ignoresWalls", "ranged", "flying", "swarm", "aoe", "phases", "kiting"}

func loadEnemyData() map[string]EnemyData {
	var v map[string]EnemyData
	if err := loadEnemies(data.Files, &v); err != nil {
		panic(err)
	}
	return v
}

// loadEnemies liest enemies.json aus fsys und prüft die Traits; bei einem Fehler bleibt *dst unverändert.
func loadEnemies(fsys fs.FS, dst *map[string]EnemyData) error {
	var v map[string]EnemyData
	if err := loadFrom(fsys, "enemies.json", &v); err != nil {
		return err
	}
	for _, kind := range slices.Sorted(maps.Keys(v)) {
		for _, t := range v[kind].Traits {
			if !slices.Contains(knownTraits, t) {
				return fmt.Errorf("data/enemies.json: %s: unbekanntes Trait %q", kind, t)
			}
		}
	}
	*dst = v
	return nil
}

// attackRate: Angriffe je Sekunde laut Daten, ohne Eintrag 1.
func attackRate(kind string) float64 {
	if r := enemyData[kind].AttacksPerSecond; r > 0 {
		return r
	}
	return 1
}

// strike: Ist der Flächenschlag fällig (`aoe`), trifft er alle Ziele im Radius um das gewählte Ziel je einmal;
// sonst der normale Angriff. Beide teilen die Abklingzeit der Angriffsrate.
func strike(w *World, e *Enemy, t *target) {
	if !e.has("aoe") || e.AoeIn > 0 {
		attack(w, e, t)
		return
	}
	a := enemyData[e.Kind].Aoe
	e.Cooldown, e.AoeIn = 1/attackRate(e.Kind), a.IntervalSeconds
	emit(w, "strike", Event{"from": e.ID, "x": unitX(e.X)})
	for _, c := range aoeTargets(w, t.x, a.Radius) {
		applyDamageBy(w, c.id, e.Damage, e.Kind)
		if c.player != nil {
			frostArmorHit(c.player, e)
		}
	}
}

// aoeTargets: Spieler, Truppen (ohne Landstreicher und Schützen auf Türmen), gebaute Plätze und die Burg im Radius um x.
func aoeTargets(w *World, x, radius float64) []target {
	in := func(at, extra float64) bool { return math.Abs(at-x) <= radius+extra }
	var out []target
	for _, p := range w.Players {
		if isAlive(p) && in(p.X, 0) {
			out = append(out, target{p.ID, p.X, "player", p})
		}
	}
	for _, t := range w.Troops {
		if t.Kind != "vagrant" && !isOnTower(w, t) && in(t.X, 0) {
			out = append(out, target{t.ID, t.X, "troop", nil})
		}
	}
	for _, s := range w.Sites {
		if s.State == "built" && in(s.X, 0) {
			out = append(out, target{s.ID, s.X, "site", nil})
		}
	}
	if in(w.Castle.X, hub.CastleRadiusUnits) {
		out = append(out, target{w.Castle.ID, w.Castle.X, "castle", nil})
	}
	return out
}

// invulnerable: Ein Gegner mit `phases` ist ab Spawn alle everySeconds für durationSeconds unverwundbar.
func invulnerable(w *World, e *Enemy) bool {
	p := enemyData[e.Kind].Phases
	if !e.has("phases") || p.EverySeconds <= 0 {
		return false
	}
	age := w.Time - e.Spawned
	return age >= p.EverySeconds && math.Mod(age, p.EverySeconds) < p.DurationSeconds
}

// kite: Ein Gegner mit `kiting` hält kiteDistance Abstand zu Spielern und Truppen. Ist einer näher, weicht er Richtung
// Portal aus, nie über HomeX hinaus und nie durch eine Mauer; an Mauer oder Portal bleibt er stehen.
// true: Ein Ziel ist zu nah, der Gegner rückt nicht vor.
func kite(w *World, e *Enemy, dt float64) bool {
	if !e.has("kiting") {
		return false
	}
	threat, ok := nearestThreat(w, e)
	if !ok || math.Abs(threat-e.X) >= enemyData[e.Kind].KiteDistance {
		return false
	}
	back := sign(e.HomeX - e.X)
	if back == 0 || sign(e.X-threat) == -back {
		return true // am Portal, oder der Weg zurück führt zum Ziel hin
	}
	x := approach(e.X, e.HomeX, e.speed(), dt)
	for _, s := range w.Sites {
		if stop := s.X - back*body; isBarrier(s) && (s.X-e.X)*back > 0 && (x-stop)*back > 0 {
			x = stop
		}
	}
	if (x-e.X)*back > 0 {
		e.X = x
	}
	return true
}

// nearestThreat: Ort des nächsten lebenden Spielers oder der nächsten Truppe (ohne Landstreicher).
func nearestThreat(w *World, e *Enemy) (float64, bool) {
	best, ok := 0.0, false
	closer := func(x float64) {
		if !ok || math.Abs(x-e.X) < math.Abs(best-e.X) {
			best, ok = x, true
		}
	}
	for _, p := range w.Players {
		if isAlive(p) {
			closer(p.X)
		}
	}
	for _, t := range w.Troops {
		if t.Kind != "vagrant" {
			closer(t.X)
		}
	}
	return best, ok
}
