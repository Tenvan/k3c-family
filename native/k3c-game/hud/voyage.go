package hud

// Schiffsfahrt beim Inselwechsel (B-977): Die Karte der Fahrt ist dieselbe wie am Hafen (seekarte.go), gebaut aus der
// Route und der Fahrt (view.Voyage): abgeschlossen und Ziel ist, was die Fahrt nennt, gewählt die Zielinsel, kein Cursor, das
// Schiff liegt an keiner Insel (es fährt). Die Lage des Schiffs ist eine reine Funktion der Zeit: gleiche Zeit, gleiche
// Lage, ohne Zustand und ohne Zufall. Reine Logik ohne Ebitengine; das HUD rechnet damit keine Regel.

import (
	"math"
	"slices"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

const (
	voyageArc    = 28.0 // Höhe des Bogens über der geraden Strecke, in Asset-Pixeln
	voyageBob    = 2.5  // Hub des Schiffs auf den Wellen
	voyageBobHz  = 0.7  // Schaukeln je Sekunde
	voyageWave   = 14.0 // Ausschlag der vorderen Welle
	voyageWaveHz = 0.22 // Schwingungen der vorderen Welle je Sekunde
	voyageFlagAt = 0.05 // Anteil der Strecke, ab dem die verlassene Insel ihre Fahne trägt (das Schiff hat abgelegt)
	voyageScale  = 1.5  // Maßstab des Dioramas im Vollbild gegenüber dem HUD (passt es nicht, schrumpft es)
)

// seaSpot ist der Platz einer Insel auf der Karte: Etappen-Spalte col, Reihe lane, n Inseln in der Etappe.
type seaSpot struct{ col, lane, n int }

// voyageView ist die Karte der Fahrt samt dem Stand, aus dem sie gebaut ist (neu nur bei anderer Fahrt, Route oder
// Sprache; sonst keine Allokation). ok: beide Inseln stehen auf der Karte.
type voyageView struct {
	sea      seaView
	sw       sim.SwitchState
	from, to seaSpot
	ok, set  bool
	lang     string
	fromID   string
	toID     string
	done     []string
	route    *view.RouteIsland
	routeN   int
	clip     seaClip
}

// same: Die Karte ist aus dieser Fahrt gebaut.
func (y *voyageView) same(lang string, route []view.RouteIsland, v *view.Voyage) bool {
	return y.set && y.lang == lang && y.fromID == v.From && y.toID == v.To && slices.Equal(y.done, v.Done) &&
		y.routeN == len(route) && (len(route) == 0 || y.route == &route[0])
}

// use baut die Karte der Fahrt v über die Inseln route, wenn sie sich geändert hat; false: Start oder Ziel fehlt auf
// der Karte (keine Route, unbekannte Insel).
func (y *voyageView) use(lang string, route []view.RouteIsland, v *view.Voyage) bool {
	if y.same(lang, route, v) {
		return y.ok
	}
	y.set, y.ok, y.lang, y.fromID, y.toID, y.routeN = true, false, lang, v.From, v.To, len(route)
	y.done = append(y.done[:0], v.Done...)
	y.route = nil
	if len(route) == 0 || v.From == "" || v.To == "" || v.From == v.To || !slices.Contains(v.Done, v.From) {
		return false
	}
	y.route = &route[0]
	y.sw.Open, y.sw.Choice = true, v.To
	y.sw.Completed = append(y.sw.Completed[:0], v.Done...)
	y.sw.Targets = y.sw.Targets[:0]
	for _, id := range v.Open {
		y.sw.Targets = append(y.sw.Targets, sim.SwitchTarget{ID: id})
	}
	if !slices.Contains(v.Open, v.To) {
		y.sw.Targets = append(y.sw.Targets, sim.SwitchTarget{ID: v.To})
	}
	y.sea.shown.set = false
	y.sea.refresh(lang, "", &y.sw, route)
	y.sea.hint, y.sea.line = tr(lang, "sea.voyage"), keyLine{} // auf der Fahrt wählt niemand: kein Bedienhinweis mit Tasten
	var okFrom, okTo bool
	for i := range y.sea.rows {
		nodes := y.sea.rows[i].nodes
		for j := range nodes {
			nodes[j].cursor, nodes[j].current = false, false
			switch nodes[j].id {
			case v.From:
				y.from, okFrom = seaSpot{i, j, len(nodes)}, true
			case v.To:
				y.to, okTo = seaSpot{i, j, len(nodes)}, true
			}
		}
	}
	y.ok = okFrom && okTo
	return y.ok
}

// node liefert die Insel am Platz s.
func (y *voyageView) node(s seaSpot) *seaNode { return &y.sea.rows[s.col].nodes[s.lane] }

// voyageEase ist der Anteil der Strecke zur Zeit t bei Fahrzeit dur (0 … 1): sanft ablegen und anlegen, vor 0 am
// Start, nach dur am Ziel.
func voyageEase(t, dur float64) float64 {
	if dur <= 0 || t >= dur {
		return 1
	}
	s := max(0, t/dur)
	return s * s * (3 - 2*s)
}

// voyagePos ist der Fußpunkt des Schiffs zur Zeit t: auf einem leichten Bogen der Höhe arc von (ax, ay) nach (bx, by),
// dazu das Schaukeln mit Hub bob. Reine Funktion der Zeit.
func voyagePos(ax, ay, bx, by, arc, bob, t, dur float64) (x, y float64) {
	s := voyageEase(t, dur)
	x = ax + (bx-ax)*s
	y = ay + (by-ay)*s - arc*math.Sin(math.Pi*s) + bob*math.Sin(2*math.Pi*voyageBobHz*t)
	return x, y
}

// voyageLane ist die Reihe, in der das Schiff beim Streckenanteil s gezeichnet wird: bis zur Mitte die des Starts,
// danach die des Ziels.
func voyageLane(from, to int, s float64) int {
	if s < 0.5 {
		return from
	}
	return to
}

// voyageShift ist der Versatz der vorderen Welle zur Zeit t beim Ausschlag amp.
func voyageShift(amp, t float64) float64 { return amp * math.Sin(2*math.Pi*voyageWaveHz*t) }
