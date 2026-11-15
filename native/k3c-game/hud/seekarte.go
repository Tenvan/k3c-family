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
	id, name       string
	kind           seaKind
	chosen, cursor bool
}

// seaRow ist eine Etappe mit ihren Inseln in Listenreihenfolge.
type seaRow struct {
	etappe int
	title  string
	nodes  []seaNode
}

// seaShown ist der Stand, aus dem Zeilen und Texte zuletzt gebaut wurden; nur bei Änderung wird neu gebaut.
type seaShown struct {
	set             bool
	lang, profile   string
	choice          string
	route           *view.RouteIsland // erster Eintrag und Länge der Route (local hält sie je Spiel stabil)
	routeN          int
	targets, doneID []string
}

// seaView ist die Seekarte eines Spielers. open: der Wechselpunkt ist offen und es gibt Ziele. ids sind die Ziele in
// Reihenfolge der Simulation, cursor ihr Index; choice ist die zuletzt gelesene Wahl (ändert sie sich, springt der
// Cursor darauf). Texte, Zeilen und Messungen (sizes bei Skalierung sizesK) gelten, bis sich shown ändert. hold: der
// Spieler hat am Steg bestätigt und hält A noch; bis zum Loslassen bleibt seine Eingabe verbraucht (seekarte_input.go).
type seaView struct {
	open        bool
	hold        bool
	ids         []string
	cursor      int
	choice      string
	rows        []seaRow
	title, hint string
	shown       seaShown
	sizes       seaSizes
	sizesK      float64
	fallback    []view.RouteIsland
}

// refresh liest die Karte aus dem Wechselzustand sw und der Welt route; ohne offenen Wechselpunkt (sw nil) schließt sie.
// Neu gebaut wird nur bei Änderung von Sprache, Profil, Route, Zielen, Wahl oder Abgeschlossenen; sonst setzt es nur den
// Cursor (keine Allokation je Frame).
func (v *seaView) refresh(lang, profile string, sw *sim.SwitchState, route []view.RouteIsland) {
	if sw == nil || !sw.Open || len(sw.Targets) == 0 {
		sh := v.shown
		sh.set = false
		*v = seaView{rows: v.rows[:0], ids: v.ids[:0], shown: sh, fallback: v.fallback[:0]}
		return
	}
	v.open = true
	if len(route) == 0 { // ohne Welt-Angaben nur die Ziele
		v.fallback = v.fallback[:0]
		for _, t := range sw.Targets {
			v.fallback = append(v.fallback, view.RouteIsland{ID: t.ID, Name: t.Name, Etappe: t.Etappe})
		}
		route = v.fallback
	}
	if !v.same(lang, profile, sw, route) {
		v.remember(lang, profile, sw, route)
		v.ids = v.ids[:0]
		for _, t := range sw.Targets {
			v.ids = append(v.ids, t.ID)
		}
		if v.choice != sw.Choice || v.cursor >= len(v.ids) {
			v.choice, v.cursor = sw.Choice, max(0, slices.Index(v.ids, sw.Choice))
		}
		v.title, v.hint = tr(lang, "sea.title"), seaHint(lang, profile, len(v.ids) > 1)
		v.build(lang, sw, route)
		v.sizesK = 0
	} else if v.choice != sw.Choice {
		v.choice = sw.Choice
	}
	v.markCursor()
}

// same: Der Stand, aus dem die Karte gebaut ist, ist unverändert.
func (v *seaView) same(lang, profile string, sw *sim.SwitchState, route []view.RouteIsland) bool {
	sh := &v.shown
	if !sh.set || sh.lang != lang || sh.profile != profile || sh.choice != sw.Choice || sh.routeN != len(route) ||
		sh.route != &route[0] || len(sh.targets) != len(sw.Targets) || !slices.Equal(sh.doneID, sw.Completed) {
		return false
	}
	for i, t := range sw.Targets {
		if sh.targets[i] != t.ID {
			return false
		}
	}
	return true
}

// remember merkt den Stand der Karte (Slices werden wiederverwendet).
func (v *seaView) remember(lang, profile string, sw *sim.SwitchState, route []view.RouteIsland) {
	sh := &v.shown
	sh.set, sh.lang, sh.profile, sh.choice, sh.route, sh.routeN = true, lang, profile, sw.Choice, &route[0], len(route)
	sh.targets = sh.targets[:0]
	for _, t := range sw.Targets {
		sh.targets = append(sh.targets, t.ID)
	}
	sh.doneID = append(sh.doneID[:0], sw.Completed...)
}

// seaHint ist der Bedienhinweis mit den Tasten des Eingabe-Profils; bei einem Ziel ohne Pfeile (nichts zu blättern).
func seaHint(lang, profile string, several bool) string {
	ok := glyphOf(lang, "confirm", profile).label
	if profile == view.ProfileGamepad {
		if several {
			return tr(lang, "sea.hint.pad")
		}
		return tr(lang, "sea.hintOne.pad")
	}
	if several {
		move := glyphOf(lang, "left", profile).label + " / " + glyphOf(lang, "right", profile).label
		return tf(lang, "sea.hint.keyboard", "move", move, "ok", ok)
	}
	return tf(lang, "sea.hintOne.keyboard", "ok", ok)
}

// build legt die Zeilen nach Etappe (aufsteigend) und die Knoten nach Listenreihenfolge an. Abgeschlossen ist, was
// IslandSwitch.Completed nennt (die Simulation zählt die aktuelle Insel dazu), Ziel, was in Targets steht, alles
// andere ist gesperrt.
func (v *seaView) build(lang string, sw *sim.SwitchState, route []view.RouteIsland) {
	v.rows = v.rows[:0]
	for _, isl := range route {
		i := slices.IndexFunc(v.rows, func(r seaRow) bool { return r.etappe == isl.Etappe })
		if i < 0 {
			v.rows = append(v.rows, seaRow{etappe: isl.Etappe, title: tf(lang, "sea.etappe", "n", isl.Etappe)})
			i = len(v.rows) - 1
		}
		n := seaNode{id: isl.ID, name: isl.Name, kind: seaLocked, chosen: isl.ID == sw.Choice}
		switch {
		case slices.Contains(v.ids, isl.ID):
			n.kind = seaTarget
		case slices.Contains(sw.Completed, isl.ID):
			n.kind = seaDone
		}
		v.rows[i].nodes = append(v.rows[i].nodes, n)
	}
	slices.SortStableFunc(v.rows, func(a, b seaRow) int { return a.etappe - b.etappe })
}

// markCursor setzt den Cursor-Marker auf das Ziel unter dem Cursor und die Wahl-Marker (ohne Allokation).
func (v *seaView) markCursor() {
	for i := range v.rows {
		for j := range v.rows[i].nodes {
			n := &v.rows[i].nodes[j]
			n.cursor = n.kind == seaTarget && n.id == v.ids[v.cursor]
			n.chosen = n.id == v.choice
		}
	}
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
