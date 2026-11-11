package hud

// Skill-Menü je Spieler (Spiegel von src/scenes/hud/skillMenuLogic.ts, S3.2, B-124): Einträge aus dem Katalog nach
// Linie und Tier, gelernt und lernbar aus der Simulation (sim.CanLearn, sim.CanRespec), Respec als letzter Eintrag,
// wenn er geht. Eingabe als Flanken (view.Action): Skill-Menü-Taste öffnet, links/rechts (auch hoch/runter) wählt,
// Bestätigen lernt, B/Menu/Esc schließt. Steuerkreuz ↓ liefert ActionDown und ActionSkillMenu zugleich: bei offenem
// Menü zählt es als „weiter“ (view/input.go). Reine Logik ohne Ebitengine.

import (
	"cmp"
	"slices"

	"k3c/native/k3c-game/view"
)

// menuEntry ist ein Eintrag: Skill oder Respec (id respecID).
type menuEntry struct {
	id, line                    string
	tier                        int
	passive, learned, learnable bool
}

// respecID ist die ID des Respec-Eintrags.
const respecID = "respec"

// sortedSkills ist der Katalog nach Linie (Reihenfolge in monarch.json › lines) und Tier, sonst in Datei-Reihenfolge.
var sortedSkills = sortSkills(skillCatalog)

func sortSkills(in []skillInfo) []skillInfo {
	out := slices.Clone(in)
	slices.SortStableFunc(out, func(a, b skillInfo) int {
		return cmp.Or(cmp.Compare(lineIndex(a.Line), lineIndex(b.Line)), cmp.Compare(a.Tier, b.Tier))
	})
	return out
}

// menuEntries schreibt die Einträge in out (geleert): lernbar nur mit freien Punkten und laut canLearn; Respec am Ende,
// wenn respec gilt.
func menuEntries(out []menuEntry, learned []string, points int, canLearn func(string) bool, respec bool) []menuEntry {
	out = out[:0]
	for _, s := range sortedSkills {
		has := slices.Contains(learned, s.ID)
		out = append(out, menuEntry{id: s.ID, line: s.Line, tier: s.Tier, passive: s.Kind == "passive", learned: has,
			learnable: !has && points > 0 && canLearn(s.ID)})
	}
	if respec {
		out = append(out, menuEntry{id: respecID, learnable: true})
	}
	return out
}

// menuState ist das Menü eines Spielers.
type menuState struct {
	open   bool
	cursor int
}

// menuResult ist, was ein Update im Menü auslöst.
type menuResult struct {
	opened, closed, moved, denied bool
	learn                         string // Skill-ID zum Lernen
	respec                        bool
}

// step wertet die Flanken acts eines Updates für das Menü mit den Einträgen entries aus.
func (m *menuState) step(acts []view.Action, entries []menuEntry) menuResult {
	var r menuResult
	if !m.open { // Öffnen verbraucht das Update: Steuerkreuz ↓ bewegt nicht zugleich den Cursor
		if slices.Contains(acts, view.ActionSkillMenu) {
			m.open, m.cursor, r.opened = true, 0, true
		}
		return r
	}
	down := slices.Contains(acts, view.ActionDown)
	for _, a := range acts {
		if m.open {
			m.act(a, down, entries, &r)
		}
	}
	if m.open && len(entries) > 0 {
		m.cursor = min(m.cursor, len(entries)-1)
	}
	return r
}

// act führt eine Aktion bei offenem Menü aus.
func (m *menuState) act(a view.Action, down bool, entries []menuEntry, r *menuResult) {
	n := len(entries)
	switch a {
	case view.ActionSkillMenu:
		if !down { // K bzw. Num 5 schließt; Steuerkreuz ↓ zählt als ActionDown
			m.open, r.closed = false, true
		}
	case view.ActionBack, view.ActionMenu, view.ActionHome:
		m.open, r.closed = false, true
	case view.ActionLeft, view.ActionUp:
		if n > 0 {
			m.cursor, r.moved = (m.cursor-1+n)%n, true
		}
	case view.ActionRight, view.ActionDown:
		if n > 0 {
			m.cursor, r.moved = (m.cursor+1)%n, true
		}
	case view.ActionConfirm:
		m.confirm(entries, r)
	}
}

func (m *menuState) confirm(entries []menuEntry, r *menuResult) {
	if m.cursor >= len(entries) || !entries[m.cursor].learnable {
		r.denied = true
		return
	}
	if e := entries[m.cursor]; e.id == respecID {
		r.respec = true
	} else {
		r.learn = e.id
	}
}

// slotView ist ein Skill-Slot: Skill ("" = frei) und Abklingzeit in Sekunden (0 = bereit).
type slotView struct {
	skill    string
	cooldown float64
}

// slotViews liest die Slots 1–4 (fehlende Felder: frei bzw. bereit).
func slotViews(slots []string, cooldowns []float64) [4]slotView {
	var out [4]slotView
	for i := range out {
		if i < len(slots) {
			out[i].skill = slots[i]
		}
		if i < len(cooldowns) {
			out[i].cooldown = max(0, cooldowns[i])
		}
	}
	return out
}
