package balance

import (
	"encoding/json"
	"slices"

	"k3c/data"
	"k3c/engine/sim"
)

// Abstieg (BS11.2, B-642, Regel nach B-643): Im Insel-Lauf steigen die Bots über den Tiefen-Eingang am Levelende ab, sobald
// ihre Stufe sicher steht: die Mauern der Linie 1 stehen auf beiden Seiten, es ist heller Tag und mindestens Tag
// minDescentDay. Der Hub-Ausbau gehört nicht zur Regel: Den Stein für Hub-Stufe 2 liefert die Höhle (Adern, Regel
// materialien-gebaeude.md § 1), der Weg nach unten kommt also vor dem Ausbau. Jeder Spieler wechselt allein
// (Einzelwechsel, stufen.md § 1). Mit mehreren Monarchen steigen zuerst nur die mit ungeradem Index ab (Rollen wie bei coop2:
// Monarch 2 holt das Material, Monarch 1 hält den Hub), sonst fiele der verlassene Hub schon in Nacht 3 (BS3.4, run-3);
// der Hub-Halter folgt erst, wenn sein Heer steht (mayDescend, B-651).
// Auf der neuen Stufe spielt der Bot sein Profil weiter: Mauern und Bauten gelten je Stufe.

// minDescentDay ist der früheste Tag des Abstiegs (Tag 1 gehört dem Aufbau der ersten Mauern).
const minDescentDay = 2

// descentWood ist das Holz, das im Insel-Vorrat liegen soll, bevor der Bot absteigt: Unter Tage wächst kein Holz, Mauern, Türme und
// Werkstatt der tiefen Stufe brauchen aber welches; ohne Reserve warten sie ewig auf Material (B-652, Höhle).
const descentWood = 400

// withDescent legt den Abstieg über ein Profil.
func withDescent(inner Bot, isl *sim.Island, deepest int) Bot {
	return func(w *sim.World, p *sim.Player) sim.PlayerCommand {
		if x, ok := descentX(w, deepest); ok && mayDescend(isl, w, p) && stageSafe(w) {
			if miniDefeated(isl, w.Biome.Depth) && (w.Stock.Wood >= descentWood || w.Biome.Depth > 0) {
				return goTo(p, x, false)
			}
			// Unter Tage kommt der Miniboss mit der Welle, und die füllt der Aggressionspool. Erst die Armee: Ein Miniboss mit
			// Tausenden HP trifft sonst auf einen Hub ohne Kämpfer (BS3.4, Eisenstollen: Lava-Golem gegen Monarch und einen Bauern).
			if n := pumpNode(w, p); n != nil && sim.EconomyOf(w).Fighters >= fighterWant {
				return goTo(p, n.X, true)
			}
		}
		return inner(w, p)
	}
}

// pumpNode: ein unmarkierter Knoten (Baum, Fels, Ader), den der Bot markiert, damit die Bauern sammeln: Unter Tage füllt
// jede gesammelte Ressource den Aggressionspool (stufen.md § 2), bei vollem Pool kommt die Welle mit dem Miniboss. Nur dort
// (Stufen mit Aggressionspool); im Wald kommt die Welle mit der Nacht von selbst.
func pumpNode(w *sim.World, p *sim.Player) *sim.ResourceNode {
	if w.Aggression == nil || w.Cycle.Phase != "day" {
		return nil
	}
	return nodeFor(w, p, sim.Cost{Wood: 1 << 20, Stone: 1 << 20, Copper: 1 << 20, Iron: 1 << 20, Crystal: 1 << 20})
}

// miniDefeated: Der Miniboss der Stufe ist besiegt (oder die Stufe hat keinen). Wer vorher geht, lässt die Stufe mit ihrer
// Welle allein, der Boss kommt und der Hub fällt (BS3.4, Test TestFullGame3AlleMinibosse).
func miniDefeated(isl *sim.Island, depth int) bool {
	id := miniBossID(depth)
	return id == "" || slices.Contains(isl.DefeatedBosses, id)
}

// miniBossID ist die ID des Minibosses der Tiefe aus data/bosses.json (leer: keiner).
func miniBossID(depth int) string {
	raw, err := data.Files.ReadFile("bosses.json")
	if err != nil {
		return ""
	}
	var f struct {
		Bosses []struct {
			ID    string
			Kind  string
			Depth int
		} `json:"bosses"`
	}
	if json.Unmarshal(raw, &f) != nil {
		return ""
	}
	for _, b := range f.Bosses {
		if b.Kind == "mini" && b.Depth == depth {
			return b.ID
		}
	}
	return ""
}

// mayDescend: ein einzelner Monarch geht immer, bei mehreren nur die mit ungeradem Index.
func mayDescend(isl *sim.Island, w *sim.World, p *sim.Player) bool {
	if len(isl.Players()) == 1 || p.Index%2 == 1 {
		return true
	}
	// Der Hub-Halter folgt, sobald sein Heer den Hub allein hält (B-651): Zwei Monarchen unten verdoppeln das Morgengold der
	// tiefen Stufe, sonst reicht es dort nie für Bögen gegen die Welle.
	e := sim.EconomyOf(w)
	return e.Fighters >= fighterWant && e.HubLevel >= 2
}

// descentX: der Tiefen-Eingang der Stufe, wenn es eine tiefere Stufe der Insel gibt.
func descentX(w *sim.World, deepest int) (float64, bool) {
	if w.Biome.Depth >= deepest {
		return 0, false
	}
	for _, e := range w.Level.Entities {
		if e.Kind == "exit" {
			return e.X, true
		}
	}
	return 0, false
}

// stageSafe: Mauern der innersten Linie auf beiden Seiten gebaut, heller Tag, ab Tag minDescentDay.
func stageSafe(w *sim.World) bool {
	return w.Cycle.Phase == "day" && w.Cycle.Day >= minDescentDay && innerWallsBuilt(w)
}

// innerWallsBuilt: je Seite mit Mauer-Plätzen steht mindestens eine gebaute Mauer der Zone 1 (B-652).
func innerWallsBuilt(w *sim.World) bool {
	built := map[bool]bool{} // Seite (links = true) → in Zone 1 steht eine gebaute Mauer
	sides := map[bool]bool{}
	for _, s := range w.Sites {
		if z, ok := sim.SiteZone(w, s); ok && z == 1 && s.Kind == "wall" {
			left := s.X < w.HubX
			sides[left] = true
			built[left] = built[left] || s.State == "built"
		}
	}
	for side := range sides {
		if !built[side] {
			return false
		}
	}
	return len(sides) > 0
}

// deepestOf ist die tiefste Tiefe der Stufen der Insel.
func deepestOf(isl *sim.Island) int {
	deepest := 0
	for _, w := range isl.Stages {
		deepest = max(deepest, w.Biome.Depth)
	}
	return deepest
}
