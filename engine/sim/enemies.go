package sim

import (
	"math"
	"slices"
)

// Gegner (Port von src/world/sim/enemies.ts). Sie laufen vom Portal geradeaus auf die Burg zu. Eine intakte Mauer
// hält sie auf (außer `ignoresWalls`), dann greifen sie an, was in Reichweite ist. `stealsGold`: klaut Gold vom
// Monarchen und flieht damit zum Portal. `fleesAtHalfHp`: flieht bei halber HP.

const body = 0.6

func (e *Enemy) has(trait string) bool { return slices.Contains(e.Traits, trait) }

// speed: Laufgeschwindigkeit, solange Ice Wall wirkt mit Slow verlangsamt (skills_caster.go).
func (e *Enemy) speed() float64 {
	if e.SlowFor > 0 {
		return e.Speed * e.Slow
	}
	return e.Speed
}

func spawnEnemy(w *World, kind string, x float64) *Enemy {
	return spawnScaled(w, kind, x, 1, 1)
}

// spawnScaled wie spawnEnemy, mit den Grad-Faktoren der Insel für HP und Schaden.
func spawnScaled(w *World, kind string, x, hpFactor, damageFactor float64) *Enemy {
	d := enemyData[kind]
	depth := float64(w.Biome.Depth)
	s := waves.DepthScaling
	hp := math.Round(d.HP * math.Pow(s.HP, depth) * hpFactor)
	e := &Enemy{
		ID: w.newID(), Kind: kind, X: x, HP: hp, MaxHP: hp,
		Damage: math.Round(d.Damage * math.Pow(s.Damage, depth) * damageFactor), Speed: d.Speed * math.Pow(s.Speed, depth),
		Range: d.Range, Traits: d.Traits, HomeX: x,
	}
	if e.has("phases") {
		e.Spawned = w.Time
	}
	w.Enemies = append(w.Enemies, e)
	return e
}

func stepSpawns(w *World) {
	due, rest := []QueuedSpawn{}, []QueuedSpawn{}
	for _, s := range w.SpawnQueue {
		if s.At <= w.Time {
			due = append(due, s)
		} else {
			rest = append(rest, s)
		}
	}
	if len(due) == 0 {
		return
	}
	w.SpawnQueue = rest
	for _, s := range due {
		spawnScaled(w, s.Kind, s.X, s.hpFactor, s.damageFactor)
	}
}

// sendEnemiesHome: Bei Tagesanbruch fliehen alle Gegner zurück zu ihren Portalen (K2C), Bosse bleiben.
func sendEnemiesHome(w *World) {
	for _, e := range w.Enemies {
		e.Fleeing = !e.Boss
	}
	w.SpawnQueue = []QueuedSpawn{}
}

type target struct {
	id     int
	x      float64
	kind   string // player, troop, wall (Mauer oder Tor), castle, site (Turm), building (übrige Gebäude)
	player *Player
}

func stepEnemies(w *World, dt float64) {
	for _, e := range w.Enemies {
		e.Cooldown, e.AoeIn = math.Max(0, e.Cooldown-dt), math.Max(0, e.AoeIn-dt)
		if stunned(e, dt) {
			continue
		}
		bossAbility(w, e)
		if e.has("fleesAtHalfHp") && e.HP < e.MaxHP/2 {
			e.Fleeing = true
		}
		speed := e.speed()
		if e.Fleeing {
			e.X += float64(sign(e.HomeX-e.X) * math.Min(math.Abs(e.HomeX-e.X), speed*1.2*dt))
			continue
		}
		dir := sign(w.HubX - e.X)
		if dir == 0 {
			dir = 1
		}
		var wall *Site
		if !e.has("ignoresWalls") {
			wall = blockingWall(w, e, dir)
		}
		t := chooseTarget(w, e, dir, wall)
		if t != nil && e.Cooldown <= 0 {
			strike(w, e, t)
		}
		if !kite(w, e, dt) && t == nil {
			advance(w, e, dir, wall, speed*dt)
		}
	}
	// Geflohene Gegner verschwinden im Portal (mitsamt geklautem Gold).
	kept := w.Enemies[:0]
	for _, e := range w.Enemies {
		if !(e.Fleeing && math.Abs(e.X-e.HomeX) < 0.3) {
			kept = append(kept, e)
		}
	}
	w.Enemies = kept
}

// advance: geradeaus Richtung Burg, höchstens bis zur Mauer oder zum Burgrand.
func advance(w *World, e *Enemy, dir float64, wall *Site, step float64) {
	x := e.X + float64(dir*step)
	stop := w.HubX - float64(dir*hub.CastleRadiusUnits)
	if wall != nil {
		stop = wall.X - float64(dir*body)
	}
	if dir > 0 {
		e.X = math.Min(x, stop)
	} else {
		e.X = math.Max(x, stop)
	}
}

// stunned senkt Betäubung, Verspottung und Verlangsamung (Skills, skills_tank.go, skills_caster.go); true: der Gegner
// ist betäubt und tut nichts.
func stunned(e *Enemy, dt float64) bool {
	if e.TauntFor = math.Max(0, e.TauntFor-dt); e.TauntFor == 0 {
		e.TauntID = 0
	}
	if e.SlowFor = math.Max(0, e.SlowFor-dt); e.SlowFor == 0 {
		e.Slow = 0
	}
	if e.Stun <= 0 {
		return false
	}
	e.Stun = math.Max(0, e.Stun-dt)
	return true
}

// isBarrier: Eine gebaute Mauer oder ein gebautes Tor hält Gegner auf und ist ihr Angriffsziel (Q27). Spieler und
// Truppen kennen keine Hindernisse, sie passieren beides; `outerWall` (Posten der Schützen) bleibt bei der Mauer.
func isBarrier(s *Site) bool { return (s.Kind == "wall" || s.Kind == "gate") && s.State == "built" }

// isBuilding: übrige gebaute Gebäude (Heilplatz, Taverne, Lager, Schmiede, Rüstkammer, Werkstatt, Farm, Kaserne) sind
// Ziele der Gegner (gegner.md § 4). Mauer und Tor halten auf, Türme sind Ziele der Fernkämpfer, Treppen Reisepunkte.
func isBuilding(s *Site) bool {
	return s.State == "built" && !isBarrier(s) && !slices.Contains([]string{"tower", "stairsUp", "stairsDown"}, s.Kind)
}

func blockingWall(w *World, e *Enemy, dir float64) *Site {
	var best *Site
	for _, s := range w.Sites {
		if !isBarrier(s) {
			continue
		}
		ahead := s.X <= e.X+body && s.X > w.HubX
		if dir > 0 {
			ahead = s.X >= e.X-body && s.X < w.HubX
		}
		if ahead && (best == nil || math.Abs(s.X-e.X) < math.Abs(best.X-e.X)) {
			best = s
		}
	}
	return best
}

// reach sagt, ob ein Gegner x angreifen kann. Nahkämpfer erreichen nichts hinter der Mauer.
type reach struct {
	e      *Enemy
	ranged bool
	dir    float64
	wall   *Site
}

func (r reach) reachable(x float64) bool {
	return r.ranged || r.wall == nil || (r.dir > 0 && x <= r.wall.X) || (r.dir <= 0 && x >= r.wall.X)
}

func (r reach) inRange(x float64) bool {
	return math.Abs(x-r.e.X) <= r.e.Range+body && r.reachable(x)
}

// candidates sammelt alle Ziele in Reichweite, in der Reihenfolge von `chooseTarget` in TS:
// Spieler, Truppen, Mauer oder Tor, Türme (nur Fernkämpfer), übrige Gebäude, Burg.
func candidates(w *World, e *Enemy, dir float64, wall *Site) []target {
	r := reach{e, e.has("ranged"), dir, wall}
	out := []target{}
	for _, p := range w.Players {
		if isAlive(p) && r.inRange(p.X) {
			out = append(out, target{p.ID, p.X, "player", p})
		}
	}
	for _, t := range w.Troops {
		if t.Kind != "vagrant" && (r.ranged || !isOnTower(w, t)) && r.inRange(t.X) {
			out = append(out, target{t.ID, t.X, "troop", nil})
		}
	}
	return append(out, buildingTargets(w, r)...)
}

func buildingTargets(w *World, r reach) []target {
	var out []target
	if r.wall != nil && math.Abs(r.wall.X-r.e.X) <= r.e.Range+body+0.1 {
		out = append(out, target{r.wall.ID, r.wall.X, "wall", nil})
	}
	for _, s := range w.Sites {
		switch {
		case r.ranged && s.Kind == "tower" && s.State == "built" && r.inRange(s.X):
			out = append(out, target{s.ID, s.X, "site", nil})
		case isBuilding(s) && r.inRange(s.X):
			out = append(out, target{s.ID, s.X, "building", nil})
		}
	}
	if math.Abs(w.Castle.X-r.e.X) <= r.e.Range+hub.CastleRadiusUnits+0.1 && r.reachable(w.Castle.X) {
		out = append(out, target{w.Castle.ID, w.Castle.X, "castle", nil})
	}
	return out
}

func (e *Enemy) prefers(c target) bool {
	return (e.has("prefersBuildings") && slices.Contains([]string{"wall", "castle", "site", "building"}, c.kind)) ||
		(e.has("prefersTowers") && c.kind == "site") ||
		(e.has("prefersTroops") && c.kind == "troop") ||
		((e.has("prefersMonarch") || e.has("stealsGold")) && c.kind == "player")
}

// chooseTarget: ein verspottender Monarch in Reichweite zuerst (Taunt), sonst bevorzugte Ziele, davon das nächste;
// bei Gleichstand das frühere (wie `reduce` in TS). Ohne bevorzugte Ziele: übrige Gebäude nur, wenn nichts anderes in
// Reichweite ist (gegner.md § 4).
func chooseTarget(w *World, e *Enemy, dir float64, wall *Site) *target {
	all := candidates(w, e, dir, wall)
	for _, c := range all {
		if e.TauntFor > 0 && c.kind == "player" && c.id == e.TauntID {
			return &c
		}
	}
	pool := []target{}
	for _, c := range all {
		if e.prefers(c) {
			pool = append(pool, c)
		}
	}
	if len(pool) == 0 {
		pool = slices.DeleteFunc(slices.Clone(all), func(c target) bool { return c.kind == "building" })
	}
	if len(pool) == 0 {
		pool = all
	}
	if len(pool) == 0 {
		return nil
	}
	best := pool[0]
	for _, c := range pool[1:] {
		if math.Abs(c.x-e.X) < math.Abs(best.x-e.X) {
			best = c
		}
	}
	return &best
}

func attack(w *World, e *Enemy, t *target) {
	e.Cooldown = 1 / attackRate(e.Kind)
	if p := t.player; p != nil && e.has("stealsGold") && p.Gold > 0 && !w.protectedNight() {
		amount := min(p.Gold, waves.StealGold)
		p.Gold -= amount
		e.CarriedGold += amount
		e.Fleeing = true
		w.Events = append(w.Events, Event{"type": "goldStolen", "player": p.Index, "amount": amount})
		return
	}
	if e.has("ranged") {
		p := &Projectile{ID: w.newID(), X: e.X, TargetID: t.id, Team: "enemy", Damage: e.Damage, Speed: 20, Cause: e.Kind}
		w.Projectiles = append(w.Projectiles, p)
		arrowEvent(w, p, e.ID)
	} else {
		emit(w, "strike", Event{"from": e.ID, "x": unitX(e.X)})
		applyDamageBy(w, t.id, e.Damage, e.Kind)
		if t.player != nil {
			frostArmorHit(t.player, e)
		}
	}
}
