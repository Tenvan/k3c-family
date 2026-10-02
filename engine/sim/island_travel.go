package sim

import "math"

// Einzelwechsel der Stufe (Entscheidung 003, B-100): Jeder Spieler wechselt allein, indem er `hub.Travel.Seconds`
// im Bereich `hub.Travel.RangeUnits` eines Reisepunkts seiner Stufe steht (Tiefen-Eingang nach unten, gebaute
// Treppen). Die anderen Spieler bleiben stehen. Tote und freie Monarchen (B-059) wechseln nicht selbst.

// islandTravel ist der Reisefortschritt eines Spielers an einem Punkt.
type islandTravel struct {
	x        float64
	progress float64
}

// pointNear ist der erste Reisepunkt in Reichweite von x (Reihenfolge wie `travelPoints`).
func pointNear(points []travelPoint, x float64) *travelPoint {
	for i := range points {
		if math.Abs(x-points[i].x) <= hub.Travel.RangeUnits {
			return &points[i]
		}
	}
	return nil
}

type islandMove struct {
	player  *Player
	from    int
	toDepth int
}

// stepIslandTravel rechnet den Reisefortschritt aller Spieler und führt fertige Wechsel aus (nach dem Tick der Stufen).
func stepIslandTravel(isl *Island, dt float64) {
	var moves []islandMove
	for si, w := range isl.Stages {
		points := travelPoints(w)
		for _, p := range w.Players {
			var pt *travelPoint
			if isAlive(p) && !p.Free {
				pt = pointNear(points, p.X)
			}
			if pt == nil {
				delete(isl.travel, p.Index)
				continue
			}
			st := isl.travel[p.Index]
			if st == nil || st.x != pt.x {
				st = &islandTravel{x: pt.x}
				isl.travel[p.Index] = st
			}
			st.progress += dt / hub.Travel.Seconds
			if st.progress >= 1 {
				moves = append(moves, islandMove{p, si, pt.toDepth})
			}
		}
	}
	for _, m := range moves {
		delete(isl.travel, m.player.Index)
		moveToStage(isl, m)
	}
}

// stageByDepth ist der Index der Stufe mit dieser Tiefe, -1 wenn die Insel sie nicht hat.
func stageByDepth(isl *Island, depth int) int {
	for i, w := range isl.Stages {
		if w.Biome.Depth == depth {
			return i
		}
	}
	return -1
}

// moveToStage setzt einen Spieler in die Zielstufe: neue ID, Gold und Index bleiben, Position an der Burg.
// Hat die Insel die Zielstufe nicht, bleibt der Spieler stehen.
func moveToStage(isl *Island, m islandMove) {
	to := stageByDepth(isl, m.toDepth)
	if to < 0 {
		return
	}
	from, target := isl.Stages[m.from], isl.Stages[to]
	p := m.player
	refundPending(from, p) // bezahlte, nicht fertige Münzen bleiben in der alten Stufe liegen
	from.Players = removePlayer(from.Players, p)
	q := *p
	q.ID = target.newID() // IDs sind nur innerhalb einer Welt eindeutig
	q.X = target.HubX + 3
	if q.Index%2 == 0 {
		q.X = target.HubX - 3
	}
	q.VX, q.RespawnIn, q.PayCooldown, q.Paying = 0, 0, 0.5, false
	q.PayKey, q.PayAmount = nil, 0
	target.Players = insertPlayer(target.Players, &q)
	target.Events = append(target.Events, Event{"type": "arrived", "depth": m.toDepth, "name": target.Biome.Name, "player": q.Index})
}

func removePlayer(players []*Player, p *Player) []*Player {
	out := make([]*Player, 0, len(players))
	for _, o := range players {
		if o != p {
			out = append(out, o)
		}
	}
	return out
}

// insertPlayer fügt p nach aufsteigendem Spielerindex ein (stabile Reihenfolge).
func insertPlayer(players []*Player, p *Player) []*Player {
	i := len(players)
	for i > 0 && players[i-1].Index > p.Index {
		i--
	}
	players = append(players, nil)
	copy(players[i+1:], players[i:])
	players[i] = p
	return players
}

// StageOf liefert die Stufe eines Spielers der Insel nach Index, -1 wenn es ihn nicht gibt.
func (isl *Island) StageOf(index int) int {
	for i, w := range isl.Stages {
		for _, p := range w.Players {
			if p.Index == index {
				return i
			}
		}
	}
	return -1
}

// Players liefert alle Spieler der Insel nach Index sortiert.
func (isl *Island) Players() []*Player {
	var out []*Player
	for _, w := range isl.Stages {
		for _, p := range w.Players {
			out = insertPlayer(out, p)
		}
	}
	return out
}
