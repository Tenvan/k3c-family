package sim

import "math"

// Stufenwechsel (Port von src/world/sim/travel.ts): Der Tiefen-Eingang am Levelende führt nach unten, gebaute Treppen
// im Hub verbinden die Hubs. Alle gesteuerten, lebenden Monarchen müssen gemeinsam am selben Punkt stehen; freie
// Monarchen (B-059) zählen nicht und reisen mit.

type travelPoint struct {
	x       float64
	toDepth int
	via     string // exit, stairsUp, stairsDown
}

func travelPoints(w *World) []travelPoint {
	depth := w.Biome.Depth
	var points []travelPoint
	if hasDepth(depth + 1) {
		for _, e := range w.Level.Entities {
			if e.Kind == "exit" {
				points = append(points, travelPoint{e.X, depth + 1, "exit"})
			}
		}
	}
	for _, s := range w.Sites {
		if s.State != "built" {
			continue
		}
		if s.Kind == "stairsUp" && hasDepth(depth-1) {
			points = append(points, travelPoint{s.X, depth - 1, "stairsUp"})
		}
		if s.Kind == "stairsDown" && hasDepth(depth+1) {
			points = append(points, travelPoint{s.X, depth + 1, "stairsDown"})
		}
	}
	return points
}

// readyAt: Stehen alle Entscheider am Punkt? Ohne Entscheider gibt es keinen Wechsel.
func readyAt(deciders []*Player, x float64) bool {
	for _, p := range deciders {
		if math.Abs(p.X-x) > hub.Travel.RangeUnits {
			return false
		}
	}
	return len(deciders) > 0
}

func stepTravel(w *World, dt float64) {
	if w.noTravel {
		return
	}
	var deciders []*Player
	for _, p := range w.Players {
		if isAlive(p) && !p.Free {
			deciders = append(deciders, p)
		}
	}
	var point *travelPoint
	for _, pt := range travelPoints(w) {
		if readyAt(deciders, pt.x) {
			point = &pt
			break
		}
	}
	if point == nil {
		w.Travel = nil
		return
	}
	if w.Travel == nil || w.Travel.X != point.x {
		w.Travel = &Travel{X: point.x, ToDepth: point.toDepth, Via: point.via}
	}
	w.Travel.Progress = math.Min(1, w.Travel.Progress+dt/hub.Travel.Seconds)
}
