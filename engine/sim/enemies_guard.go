package sim

import (
	"maps"
	"math"
	"slices"

	"k3c/engine/level"
)

// Gegnerlager (docs/rules/geplante-chunks.md § 9, B-517): Jede Wache der Planung (Layout.Guards) ist ein Gegner mit
// Kennung Guard. Sie steht an ihrem Heimatpunkt (HomeX), greift Spieler und Truppen im Aggro-Radius um den Heimatpunkt an,
// kehrt danach zurück und ist besiegbar; sie flieht nie, klaut kein Gold, gehört zu keiner Welle (kein Beitrag zur
// Gefahr der Bauern, zum Aggressionspool und zum Tagesanbruch) und kommt nach dem Tod nie wieder (guardsCleared, Spielstand).

// guardKey ist die Kennung einer Wache für Spielstand und Räumung: Art und Heimatpunkt (kind@x wie bei Plätzen).
func guardKey(e *Enemy) string { return key(e.Kind, e.HomeX) }

// spawnGuards stellt die Wachen der Stufe beim ersten Tick auf, ohne die geräumten. Erst dann stehen Insel und Skalierung
// fest (Laden setzt sie nach dem Bau der Stufe); ohne Planung gibt es keine Wachen und nichts ändert sich.
func spawnGuards(w *World) {
	if w.guardsSpawned {
		return
	}
	w.guardsSpawned = true
	for _, g := range w.Level.Guards {
		e := &Enemy{Kind: g.Kind, HomeX: g.X}
		if w.guardsCleared[guardKey(e)] {
			continue
		}
		spawnScaled(w, g.Kind, g.X, 1, 1).Guard = true
	}
}

// stepGuard: Die Wache kämpft gegen das nächste Ziel im Aggro-Radius oder kehrt zum Heimatpunkt zurück. Sie nimmt nicht
// am Marsch der Wellen teil.
func stepGuard(w *World, e *Enemy, dt float64) {
	e.Fleeing = false
	foe := guardFoe(w, e)
	if foe == nil {
		e.X += float64(sign(e.HomeX-e.X) * math.Min(math.Abs(e.HomeX-e.X), e.speed()*dt))
		return
	}
	dir := sign(foe.x - e.X)
	if math.Abs(foe.x-e.X) <= e.Range+body {
		if e.Cooldown <= 0 {
			strike(w, e, foe)
		}
		return
	}
	e.X += float64(dir * math.Min(math.Abs(foe.x-e.X), e.speed()*dt))
}

// guardFoe: das nächste Ziel (Spieler, Truppen ohne Landstreicher) im Aggro-Radius um den Heimatpunkt; nil ohne Ziel.
func guardFoe(w *World, e *Enemy) *target {
	radius := level.DefaultGuardRules().AggroRadiusUnits
	var best *target
	consider := func(t target) {
		if math.Abs(t.x-e.HomeX) <= radius && (best == nil || math.Abs(t.x-e.X) < math.Abs(best.x-e.X)) {
			best = &t
		}
	}
	for _, p := range w.Players {
		if isAlive(p) && !shielded(p) {
			consider(target{p.ID, p.X, "player", p})
		}
	}
	for _, t := range w.Troops {
		if t.Kind != "vagrant" && (e.has("ranged") || !isOnTower(w, t)) {
			consider(target{t.ID, t.X, "troop", nil})
		}
	}
	return best
}

// isWaveEnemy: Gegner, die zu einer Welle gehören (alle außer den Wachen eines Lagers).
func isWaveEnemy(e *Enemy) bool { return !e.Guard }

// keepGuards: nach dem Burgfall verschwinden die Gegner der Welle, die Wachen bleiben.
func keepGuards(enemies []*Enemy) []*Enemy {
	out := []*Enemy{}
	for _, e := range enemies {
		if e.Guard {
			out = append(out, e)
		}
	}
	return out
}

// markCleared trägt eine besiegte Wache als geräumt ein (kein Respawn, auch nicht nach Laden und Burgfall).
func markCleared(w *World, e *Enemy) {
	if w.guardsCleared == nil {
		w.guardsCleared = map[string]bool{}
	}
	w.guardsCleared[guardKey(e)] = true
}

// guardsClearedKeys sind die geräumten Lager für den Spielstand, sortiert (stabile Datei).
func guardsClearedKeys(w *World) []string {
	if len(w.guardsCleared) == 0 {
		return nil
	}
	return slices.Sorted(maps.Keys(w.guardsCleared))
}

// applyGuards übernimmt die geräumten Lager aus dem Spielstand und entfernt bereits aufgestellte Wachen dieser Lager.
func applyGuards(w *World, h HubSave) {
	w.guardsCleared = nil
	for _, k := range h.GuardsCleared {
		if w.guardsCleared == nil {
			w.guardsCleared = map[string]bool{}
		}
		w.guardsCleared[k] = true
	}
	w.Enemies = slices.DeleteFunc(w.Enemies, func(e *Enemy) bool { return e.Guard && w.guardsCleared[guardKey(e)] })
}
