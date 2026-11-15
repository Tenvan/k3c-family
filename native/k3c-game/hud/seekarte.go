package hud

// Seekarte am Hafen (B-962): Solange der Wechselpunkt der Insel offen ist (sim.World.IslandSwitch), zeigt jede Zelle die
// Etappen der Welt untereinander, die Inseln als Knoten (abgeschlossen, Ziel, gesperrt). Der Cursor je Spieler blättert
// die Ziele (Links/Rechts), Bestätigen sendet die Wahl an die Simulation (Session.ChooseIsland); die gewählte Insel
// (IslandSwitch.Choice) gilt für alle und ist in jeder Zelle markiert. Das HUD rechnet keine Regel: Ziele, Wahl und
// Abgeschlossene kommen aus dem Zustand, die Etappen aus Frame.Route. Reine Logik ohne Ebitengine.

import (
	"slices"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// seaKind ist der Zustand eines Knotens der Seekarte.
type seaKind int

const (
	seaLocked seaKind = iota // noch nicht anfahrbar
	seaDone                  // abgeschlossen
	seaTarget                // wählbares Ziel
)

// seaNode ist eine Insel der Karte: chosen = gewähltes Ziel (für alle), cursor = Cursor dieses Spielers.
type seaNode struct {
	name           string
	kind           seaKind
	chosen, cursor bool
}

// seaRow ist eine Etappe mit ihren Inseln in Listenreihenfolge.
type seaRow struct {
	etappe int
	title  string
	nodes  []seaNode
}

// seaView ist die Seekarte eines Spielers. open: der Wechselpunkt ist offen und es gibt Ziele. ids sind die Ziele in
// Reihenfolge der Simulation, cursor ihr Index; choice ist die zuletzt gelesene Wahl (ändert sie sich, springt der
// Cursor darauf).
type seaView struct {
	open        bool
	ids         []string
	cursor      int
	choice      string
	rows        []seaRow
	title, hint string
}

// refresh liest die Karte aus dem Wechselzustand sw und der Welt route; ohne offenen Wechselpunkt (sw nil) schließt sie.
// Zeilen und Listen werden wiederverwendet.
func (v *seaView) refresh(lang string, sw *sim.SwitchState, route []view.RouteIsland) {
	if sw == nil || !sw.Open || len(sw.Targets) == 0 {
		*v = seaView{rows: v.rows[:0], ids: v.ids[:0]}
		return
	}
	v.open, v.ids = true, v.ids[:0]
	for _, t := range sw.Targets {
		v.ids = append(v.ids, t.ID)
	}
	if v.choice != sw.Choice || v.cursor >= len(v.ids) {
		v.choice, v.cursor = sw.Choice, max(0, slices.Index(v.ids, sw.Choice))
	}
	v.title, v.hint = tr(lang, "sea.title"), tr(lang, "sea.hint")
	if len(v.ids) == 1 {
		v.hint = tr(lang, "sea.hintOne") // ein Ziel: nichts zu blättern, keine Pfeile
	}
	if len(route) == 0 { // ohne Welt-Angaben nur die Ziele
		for _, t := range sw.Targets {
			route = append(route, view.RouteIsland{ID: t.ID, Name: t.Name, Etappe: t.Etappe})
		}
	}
	v.build(lang, sw, route)
}

// build legt die Zeilen nach Etappe (aufsteigend) und die Knoten nach Listenreihenfolge an. Abgeschlossen sind die
// gemeldeten Inseln und alle Nicht-Ziele bis zur Etappe der Ziele (darunter die Insel, von der es ablegt: sie fehlt in
// IslandSwitch.Completed); alles dahinter ist gesperrt.
func (v *seaView) build(lang string, sw *sim.SwitchState, route []view.RouteIsland) {
	v.rows = v.rows[:0]
	upTo := sw.Targets[0].Etappe
	for _, isl := range route {
		i := slices.IndexFunc(v.rows, func(r seaRow) bool { return r.etappe == isl.Etappe })
		if i < 0 {
			v.rows = append(v.rows, seaRow{etappe: isl.Etappe, title: tf(lang, "sea.etappe", "n", isl.Etappe)})
			i = len(v.rows) - 1
		}
		n := seaNode{name: isl.Name, kind: seaLocked, chosen: isl.ID == sw.Choice}
		switch {
		case slices.Contains(v.ids, isl.ID):
			n.kind, n.cursor = seaTarget, isl.ID == v.ids[v.cursor]
		case slices.Contains(sw.Completed, isl.ID) || isl.Etappe <= upTo:
			n.kind = seaDone
		}
		v.rows[i].nodes = append(v.rows[i].nodes, n)
	}
	slices.SortStableFunc(v.rows, func(a, b seaRow) int { return a.etappe - b.etappe })
}

// seaResult ist das Ergebnis eines Eingabeschritts: moved = Cursor bewegt, pick = Insel-ID der bestätigten Wahl.
type seaResult struct {
	moved bool
	pick  string
}

// step wertet die Flanken eines Updates aus: Links/Rechts blättern die Ziele im Kreis (bei einem Ziel nichts),
// Bestätigen wählt das Ziel unter dem Cursor.
func (v *seaView) step(acts []view.Action) seaResult {
	var r seaResult
	if !v.open || len(v.ids) == 0 {
		return r
	}
	n := len(v.ids)
	for _, a := range acts {
		switch {
		case a == view.ActionLeft && n > 1:
			v.cursor, r.moved = (v.cursor+n-1)%n, true
		case a == view.ActionRight && n > 1:
			v.cursor, r.moved = (v.cursor+1)%n, true
		case a == view.ActionConfirm:
			r.pick = v.ids[v.cursor]
		}
	}
	return r
}
