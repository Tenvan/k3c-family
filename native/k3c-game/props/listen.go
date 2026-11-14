package props

// Zustand je Update (Listener): was die Simulation über Plätze und Burg nur abgeleitet liefert (bezahlbar, roden,
// sichtbar, Hub-Ausbau, Gefahr), die stehenden Preisschilder und die Truhen, die sich gerade öffnen. Einmal je Frame
// gerechnet, Draw liest nur. Puffer werden wiederverwendet; Zeiger in die Welt bleiben nicht über das Update hinaus.

import (
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// siteFlags ist der abgeleitete Zustand eines Platzes (Index wie World.Sites).
type siteFlags struct {
	visible, payable, clearing bool
	goldLeft                   int // Gold bis zum vollen Preis, wenn der Platz Münzen nimmt (sim.SiteGoldLeft)
}

// hubState ist der Hub-Ausbau einer Stufe (sim.EconomyOf).
type hubState struct {
	has               bool
	gold, paid, steps int
	state             string
	material          [5]int
}

// stageInfo ist der abgeleitete Zustand einer Stufe.
type stageInfo struct {
	index    int
	danger   bool
	hubLevel int
	hubStep  int
	hub      hubState
	stock    [5]int
	flags    []siteFlags
	tags     []int // Nummern der Plätze bzw. der Burg mit stehendem Preisschild
	cands    []candidate
	players  []float64
	chests   []candidate // Truhen des Frames (Nummer, Ort), für das Öffnen
	prev     []candidate // Truhen des vorigen Frames
	openings []opening
}

// opening ist eine aufgehobene Truhe: kurz offen, dann ausgeblendet (worldRenderer.ts › openChest).
type opening struct {
	x     float64
	start float64 // Frame.Clock beim Aufheben
}

// Dauer der offenen Truhe: steht chestHold Sekunden, blendet bis chestGone aus.
const (
	chestHold = 0.3
	chestGone = 0.8
)

// Listen rechnet den Zustand aller Stufen des Frames (view.Listener).
func (d *drawer) Listen(f *view.Frame) {
	keep := d.infos[:0]
	for i := range f.Stages {
		st := &f.Stages[i]
		if st.World == nil {
			continue
		}
		info := d.infoFor(st.Index)
		info.index = st.Index
		info.update(st.World, f.Clock)
		keep = append(keep, info)
	}
	d.infos = keep
}

// infoFor liefert den Zustand der Stufe i aus dem letzten Frame oder einen neuen. Kommt eine Stufe nach einer Lücke
// wieder (aus spare), beginnt der Truhen-Vergleich neu: Truhen, die inzwischen verschwunden sind, öffnen sich nicht.
func (d *drawer) infoFor(i int) *stageInfo {
	for _, s := range d.infos {
		if s.index == i {
			return s
		}
	}
	for _, s := range d.spare {
		if s.index == i {
			s.chests, s.openings = s.chests[:0], s.openings[:0]
			return s
		}
	}
	s := &stageInfo{index: i}
	d.spare = append(d.spare, s)
	return s
}

// info liefert den Zustand der Stufe i des laufenden Frames (nil, wenn Listen sie nicht kennt).
func (d *drawer) info(i int) *stageInfo {
	for _, s := range d.infos {
		if s.index == i {
			return s
		}
	}
	return nil
}

// update liest die Welt w der Stufe.
func (s *stageInfo) update(w *sim.World, clock float64) {
	e := sim.EconomyOf(w)
	s.danger, s.hubLevel, s.hubStep = e.Danger, max(1, w.HubLevel), max(1, w.HubStep)
	s.hub = hubState{}
	if up := e.HubUpgrade; up != nil {
		m := up.Material
		s.hub = hubState{has: true, gold: up.Gold, paid: up.Paid, steps: up.Steps, state: up.State,
			material: [5]int{m.Wood, m.Stone, m.Copper, m.Iron, m.Crystal}}
	}
	if st := w.Stock; st != nil {
		s.stock = [5]int{st.Wood, st.Stone, st.Copper, st.Iron, st.Crystal}
	}
	s.flags = s.flags[:0]
	for _, site := range w.Sites {
		left, payable := sim.SiteGoldLeft(w, site)
		s.flags = append(s.flags, siteFlags{visible: sim.SiteVisible(w, site), payable: payable,
			clearing: !payable && sim.SiteNeedsClearing(w, site), goldLeft: left})
	}
	s.priceTags(w)
	s.chestsOf(w, clock)
}

// priceTags: je lebendem Monarchen das nächste Preisschild (Plätze mit offersPrice, Burg mit offenem Ausbau).
func (s *stageInfo) priceTags(w *sim.World) {
	s.cands, s.players, s.tags = s.cands[:0], s.players[:0], s.tags[:0]
	for i, site := range w.Sites {
		if s.flags[i].visible && offersPrice(site.Kind, site.State, s.flags[i].payable, site.Bows) {
			s.cands = append(s.cands, candidate{site.ID, site.X})
		}
	}
	if offersHubUpgrade(s.hub.has, s.hub.state, s.hub.gold, s.hub.paid) {
		s.cands = append(s.cands, candidate{w.Castle.ID, w.Castle.X})
	}
	for _, p := range w.Players {
		if p != nil && p.RespawnIn <= 0 {
			s.players = append(s.players, p.X)
		}
	}
	s.tags = nearestTags(s.tags, s.cands, s.players)
}

// chestsOf merkt die Truhen und legt für jede verschwundene eine offene an; abgelaufene fallen weg.
func (s *stageInfo) chestsOf(w *sim.World, clock float64) {
	s.prev, s.chests = s.chests, s.prev[:0]
	for _, p := range w.Pickups {
		if p.Kind == "chest" {
			s.chests = append(s.chests, candidate{p.ID, p.X})
		}
	}
	s.openings = gone(s.openings, s.prev, s.chests, clock)
}

// gone hängt für jede Truhe aus prev, die in now fehlt, ein Öffnen an und entfernt abgelaufene (rein, getestet).
func gone(out []opening, prev, now []candidate, clock float64) []opening {
	keep := out[:0]
	for _, o := range out {
		if clock-o.start < chestGone && clock >= o.start {
			keep = append(keep, o)
		}
	}
	for _, c := range prev {
		if !hasCandidate(now, c.id) {
			keep = append(keep, opening{x: c.x, start: clock})
		}
	}
	return keep
}

func hasCandidate(cs []candidate, id int) bool {
	for _, c := range cs {
		if c.id == id {
			return true
		}
	}
	return false
}

// openAlpha ist die Deckkraft einer offenen Truhe zur Zeit clock.
func openAlpha(o opening, clock float64) float32 {
	t := clock - o.start
	switch {
	case t < chestHold:
		return 1
	case t >= chestGone:
		return 0
	}
	return float32(1 - (t-chestHold)/(chestGone-chestHold))
}

// tagged meldet, ob das Preisschild von id steht.
func (s *stageInfo) tagged(id int) bool { return s != nil && hasID(s.tags, id) }

// flag liefert den Zustand des Platzes i (leer, wenn Listen die Welt nicht kannte).
func (s *stageInfo) flag(i int) siteFlags {
	if s == nil || i >= len(s.flags) {
		return siteFlags{visible: true}
	}
	return s.flags[i]
}
