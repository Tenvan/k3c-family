package sim

import "math"

// Gemeinsame Helfer (Port von src/world/sim/common.ts).

// approach bewegt x mit speed auf target zu, ohne darüber hinauszuschießen.
func approach(x, target, speed, dt float64) float64 {
	step := speed * dt
	if math.Abs(target-x) <= step {
		return target
	}
	return x + float64(sign(target-x)*step)
}

// sign wie Math.sign für endliche Zahlen.
func sign(v float64) float64 {
	switch {
	case v > 0:
		return 1
	case v < 0:
		return -1
	}
	return 0
}

func intPtr(v int) *int { return &v }

func isAlive(p *Player) bool { return p.RespawnIn <= 0 }

// isDangerous: nachts oder solange Gegner in der Welt sind, bleiben Bauern im Hub.
func isDangerous(w *World) bool { return w.Cycle.Phase == "night" || len(w.Enemies) > 0 }

// outerWall ist die intakte Mauer auf der Seite side (-1 links, 1 rechts), die am weitesten außen steht.
func outerWall(w *World, side float64) *Site {
	var best *Site
	for _, s := range w.Sites {
		if s.Kind != "wall" || s.State != "built" || sign(s.X-w.HubX) != side {
			continue
		}
		if best == nil || math.Abs(s.X-w.HubX) > math.Abs(best.X-w.HubX) {
			best = s
		}
	}
	return best
}

func siteByID(w *World, id int) *Site {
	for _, s := range w.Sites {
		if s.ID == id {
			return s
		}
	}
	return nil
}

func troopByID(w *World, id int) *Troop {
	for _, t := range w.Troops {
		if t.ID == id {
			return t
		}
	}
	return nil
}

func nodeByID(w *World, id int) *ResourceNode {
	for _, n := range w.Nodes {
		if n.ID == id {
			return n
		}
	}
	return nil
}

// isWorker: Lebt die Truppe mit dieser ID noch?
func isWorker(w *World, id *int) bool {
	if id == nil {
		return false
	}
	for _, o := range w.Troops {
		if o.ID == *id {
			return true
		}
	}
	return false
}
