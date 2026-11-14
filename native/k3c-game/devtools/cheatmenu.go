package devtools

// Bedienung des Cheat-Menüs je Spieler (F9 bzw. View + Y, B-888): links/rechts wechselt die Gruppe, hoch/runter den
// Eintrag, Bestätigen löst aus, B, Menu, Pos1 oder erneut F9 schließt. Reine Logik ohne Ebitengine; die Aktion führt
// das Overlay über die Session aus.

import "k3c/native/k3c-game/view"

// cheatState ist das Menü eines Spielers.
type cheatState struct {
	open       bool
	group, row int
	status     string // Ergebnis der letzten Aktion (Anzeige)
	bad        bool   // status ist ein Fehler
}

// cheatResult ist, was ein Update im Menü auslöst.
type cheatResult struct {
	opened, closed, moved bool
	fire                  *cheatEntry // auszulösender Eintrag
}

// step wertet die Flanken acts eines Updates für das Menü mit den Gruppen groups aus.
func (m *cheatState) step(acts []view.Action, groups []cheatGroup) cheatResult {
	var r cheatResult
	for _, a := range acts {
		if !m.open {
			if a == view.ActionCheats {
				m.open, r.opened, m.status = true, true, ""
				return r // Öffnen verbraucht das Update
			}
			continue
		}
		m.act(a, groups, &r)
	}
	return r
}

// act führt eine Aktion bei offenem Menü aus.
func (m *cheatState) act(a view.Action, groups []cheatGroup, r *cheatResult) {
	n := len(groups)
	switch a {
	case view.ActionCheats, view.ActionBack, view.ActionMenu, view.ActionHome:
		m.open, r.closed = false, true
	case view.ActionLeft, view.ActionPrevPage:
		m.group, m.row, r.moved = (m.group-1+n)%n, 0, true
	case view.ActionRight, view.ActionNextPage:
		m.group, m.row, r.moved = (m.group+1)%n, 0, true
	case view.ActionUp, view.ActionDown:
		rows := len(groups[m.group].entries)
		d := 1
		if a == view.ActionUp {
			d = rows - 1
		}
		m.row, r.moved = (m.row+d)%rows, true
	case view.ActionConfirm:
		r.fire = &groups[m.group].entries[m.row]
	}
}

// toggles ist der Stand der Schalter (raumweit, wie der Raum sie hält; Start aus).
type toggles map[string]bool

// action liefert die Aktion des Eintrags; ein Schalter bekommt den umgekehrten Stand.
func (t toggles) action(e *cheatEntry) view.DevAction {
	a := e.act
	if e.toggle != "" {
		on := !t[e.toggle]
		a.On = &on
	}
	return a
}

// label liefert die Beschriftung des Eintrags, bei Schaltern mit Stand.
func (t toggles) label(e *cheatEntry) string {
	if e.toggle == "" {
		return e.label
	}
	if t[e.toggle] {
		return e.label + ": an"
	}
	return e.label + ": aus"
}
