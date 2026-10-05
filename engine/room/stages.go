package room

import "k3c/engine/sim"

// Stufen der Insel im Raum: Monarch i ist der Insel-Spieler mit Index i; jedes Gerät sieht die Stufe seines ersten
// (kleinster Slot) Monarchen.

// deviceStage ist die Stufe des Geräts: die des Monarchen im kleinsten Slot, ohne Slots die Startstufe des Raums.
func (r *Room) deviceStage(d *device) int {
	first := -1
	for slot := range d.slots {
		if first < 0 || slot < first {
			first = slot
		}
	}
	if first < 0 {
		return r.start
	}
	return r.isl.StageOf(d.slots[first])
}

// pushState schickt dem Gerät den Zustand seiner Stufe, bei Stufenwechsel (oder beim ersten Mal) vorher das Level.
// states sind die in diesem Tick schon gebauten Zustände je Stufe (nil: nur für dieses Gerät bauen).
// true: Das Level wurde gesendet.
func (r *Room) pushState(d *device, states map[int]any) (levelSent bool) {
	s := r.deviceStage(d)
	w := r.isl.Stages[s]
	if s != d.stage {
		d.peer.Level(w.Biome.Depth, w.Level)
		d.stage, levelSent = s, true
	}
	st, ok := states[s]
	if !ok {
		st = r.snapshot(w)
		if states != nil {
			states[s] = st
		}
	}
	d.peer.State(r.tick, st)
	return levelSent
}

// snapshot baut den Zustand der Stufe w über Manager.Snapshot.
func (r *Room) snapshot(w *sim.World) any {
	if r.m.Snapshot == nil {
		return w
	}
	timescale := 0 // ohne Dev-Mode fehlt devTimescale im Zustand
	if r.m.Dev {
		timescale = r.scale()
	}
	return r.m.Snapshot(w, timescale, r.paused)
}

// startStage ist die Startstufe einer geöffneten Insel: die Stufe des ersten Spielers, sonst die mit dieser Tiefe.
func startStage(isl *sim.Island, depth int) int {
	if ps := isl.Players(); len(ps) > 0 {
		return isl.StageOf(ps[0].Index)
	}
	for i, w := range isl.Stages {
		if w.Biome.Depth == depth {
			return i
		}
	}
	return 0
}

// freeMonarchs trägt die Spieler eines geladenen Stands als freie Monarchen ein. Annahme: Indizes 0…n−1 ohne Lücke
// (Stände aus dem Raum sind so); sonst false.
func freeMonarchs(isl *sim.Island) ([]*monarch, bool) {
	ps := isl.Players()
	out := make([]*monarch, len(ps))
	for i, p := range ps {
		if p.Index != i || len(ps) > MaxMonarchs {
			return nil, false
		}
		out[i] = &monarch{state: Free}
	}
	return out, true
}

// depths sind die Stufen aller Monarchen (Index = Slot), zum Erkennen eines Stufenwechsels im Tick.
func (r *Room) depths() []int {
	out := make([]int, len(r.monarchs))
	for i := range r.monarchs {
		out[i] = r.isl.StageOf(i)
	}
	return out
}
