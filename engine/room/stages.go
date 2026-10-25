package room

import (
	"fmt"
	"io/fs"
	"maps"
	"path"
	"slices"
	"strings"
	"sync"

	"k3c/data"
	"k3c/engine/level"
	"k3c/engine/sim"
)

// islandDepths sind die Stufen einer neuen Insel: die Tiefen aller Biome (data/biomes/*.json), lückenlos ab 0 (B-199).
// Eine Lücke ist ein Datenfehler und lässt jedes Anlegen scheitern, nicht erst den Wechsel in die fehlende Stufe.
var islandDepths = sync.OnceValues(func() ([]int, error) {
	files, err := fs.Glob(data.Files, "biomes/*.json")
	if err != nil {
		return nil, err
	}
	depths := make([]int, 0, len(files))
	for _, f := range files {
		b, err := level.LoadBiome(strings.TrimSuffix(path.Base(f), ".json"))
		if err != nil {
			return nil, err
		}
		depths = append(depths, b.Depth)
	}
	slices.Sort(depths)
	for i, d := range depths {
		if d != i {
			return nil, fmt.Errorf("biome: Tiefen %v nicht lückenlos ab 0", depths)
		}
	}
	return depths, nil
})

// Stufen der Insel im Raum: Monarch i ist der Insel-Spieler mit Index i; jedes Gerät bekommt Level und Zustand jeder
// Stufe, in der einer seiner Slots steht (B-176).

// deviceStage ist die Stufe für neue Slots des Geräts: die des Monarchen im kleinsten Slot, ohne Slots die Startstufe
// des Raums.
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

// deviceStages sind die Stufen der Slots des Geräts, aufsteigend und ohne Doppelte; beim Beobachter die aller Monarchen.
func (r *Room) deviceStages(d *device) []int {
	if d.observer {
		return r.observerStages()
	}
	out := []int{}
	for _, idx := range d.slots {
		out = append(out, r.isl.StageOf(idx))
	}
	slices.Sort(out)
	return slices.Compact(out)
}

// pushState schickt dem Gerät den Zustand jeder seiner Stufen (aufsteigend), einer neuen Stufe vorher das Level; Stufen
// ohne Slot fallen aus der Menge, ihr Strom endet. states sind die in diesem Tick schon gebauten Zustände je Stufe
// (nil: nur für dieses Gerät bauen). true: Mindestens ein Level wurde gesendet.
func (r *Room) pushState(d *device, states map[int]any) (levelSent bool) {
	now := r.deviceStages(d)
	maps.DeleteFunc(d.stages, func(s int, _ bool) bool { return !slices.Contains(now, s) })
	for _, s := range now {
		w := r.isl.Stages[s]
		if !d.stages[s] {
			d.peer.Level(s, w.Biome.Depth, w.Level)
			d.stages[s], levelSent = true, true
		}
		st, ok := states[s]
		if !ok {
			st = r.snapshot(w)
			if states != nil {
				states[s] = st
			}
		}
		d.peer.State(r.tick, s, st)
	}
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
	out := r.m.Snapshot(w, timescale, r.paused)
	if s, ok := out.(map[string]any); ok && r.isl != nil && r.isl.Options.Grade != "" {
		s["grade"] = r.isl.Options.Grade // Grad des Raums (B-080, KB2.6); die Dev-Aktion grade ändert ihn ab der nächsten Welle
	}
	return out
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
