package hud

import (
	"slices"
	"strings"
	"testing"

	"k3c/native/k3c-game/view"
)

func never(string) bool { return false }

func only(ids ...string) func(string) bool {
	return func(id string) bool { return slices.Contains(ids, id) }
}

// menuEntries (skillMenuLogic.test.ts): alle Skills nach Linie und Tier; lernbar nur mit freien Punkten und laut
// Simulation; Respec nur, wenn er geht, als letzter Eintrag.
func TestMenuEintraege(t *testing.T) {
	none := menuEntries(nil, nil, 0, only("fireball"), false)
	if len(none) == 0 || none[0].line != "tank" || none[0].tier != 1 {
		t.Fatalf("erster Eintrag Tank Tier 1: %+v", none)
	}
	for _, e := range none {
		if e.learnable {
			t.Errorf("ohne Punkte nichts lernbar: %s", e.id)
		}
	}
	var mage []int
	for _, e := range none {
		if e.line == "mage" {
			mage = append(mage, e.tier)
		}
	}
	if !slices.IsSorted(mage) {
		t.Errorf("Tiers je Linie sortiert: %v", mage)
	}
}

func TestMenuLernbar(t *testing.T) {
	es := menuEntries(nil, []string{"taunt"}, 2, only("shieldBash", "fireball", "taunt"), false)
	var learnable []string
	for _, e := range es {
		if e.id == "taunt" && (!e.learned || e.learnable) {
			t.Error("gelernt, nicht lernbar")
		}
		if e.learnable {
			learnable = append(learnable, e.id)
		}
	}
	if slices.Sort(learnable); !slices.Equal(learnable, []string{"fireball", "shieldBash"}) {
		t.Errorf("lernbar nur laut Simulation: %v", learnable)
	}
	withRespec := menuEntries(nil, nil, 0, never, true)
	if last := withRespec[len(withRespec)-1]; last.id != respecID || !last.learnable {
		t.Errorf("Respec zuletzt: %+v", last)
	}
	if slices.ContainsFunc(es, func(e menuEntry) bool { return e.id == respecID }) {
		t.Error("ohne Respec kein Eintrag")
	}
}

func TestSlotViews(t *testing.T) {
	if got := slotViews(nil, nil); got != [4]slotView{} {
		t.Errorf("fehlende Felder: frei und bereit, %v", got)
	}
	got := slotViews([]string{"taunt", "", "", ""}, []float64{12.5, -1})
	if got[0] != (slotView{"taunt", 12.5}) || got[1].cooldown != 0 {
		t.Errorf("Slot 1: %+v", got)
	}
}

func acts(a ...view.Action) []view.Action { return a }

// step: öffnen, ein Schritt je Flanke, links vom ersten zum letzten, bestätigen lernt, B schließt; Steuerkreuz ↓ bei
// offenem Menü wählt weiter.
func TestMenuSchritte(t *testing.T) {
	es := menuEntries(nil, nil, 1, only("shieldBash", "taunt"), true)
	var m menuState
	if r := m.step(acts(view.ActionDown, view.ActionSkillMenu), es); !r.opened || m.cursor != 0 {
		t.Fatalf("Steuerkreuz ↓ öffnet, Cursor bleibt: %+v %d", r, m.cursor)
	}
	if m.step(acts(view.ActionRight), es); m.cursor != 1 {
		t.Errorf("rechts: Cursor %d", m.cursor)
	}
	if r := m.step(acts(view.ActionConfirm), es); r.learn != es[1].id {
		t.Errorf("bestätigen lernt Eintrag 1: %+v", r)
	}
	if m.step(acts(view.ActionDown, view.ActionSkillMenu), es); !m.open || m.cursor != 2 {
		t.Errorf("Steuerkreuz ↓ bei offenem Menü wählt weiter: %v %d", m.open, m.cursor)
	}
}

func TestMenuRespecUndSchliessen(t *testing.T) {
	es := menuEntries(nil, nil, 1, only("shieldBash", "taunt"), true)
	m := menuState{open: true}
	if m.step(acts(view.ActionLeft), es); m.cursor != len(es)-1 {
		t.Errorf("links vom ersten: zum letzten, %d", m.cursor)
	}
	if r := m.step(acts(view.ActionConfirm), es); !r.respec {
		t.Error("Respec bestätigt")
	}
	m.cursor = 3
	if r := m.step(acts(view.ActionConfirm), es); !r.denied || r.learn != "" {
		t.Errorf("nicht lernbar: abgelehnt, %+v", r)
	}
	if r := m.step(acts(view.ActionBack), es); !r.closed || m.open {
		t.Error("B schließt")
	}
	m.step(acts(view.ActionSkillMenu), es)
	if r := m.step(acts(view.ActionSkillMenu), es); !r.closed {
		t.Error("K schließt")
	}
	if r := m.step(acts(view.ActionConfirm, view.ActionLeft), es); r != (menuResult{}) || m.open {
		t.Error("geschlossen: nichts")
	}
}

func TestSkillTexte(t *testing.T) {
	e := menuEntry{id: "fireball", line: "mage", tier: 1, learnable: true}
	checkTexts(t, []textCase{
		{"Name", skillName("shieldBash"), "Shield Bash"},
		{"Zeile", entryText("de", e, false, ""), "   Zauberer I · Fireball  ·  lernen"},
		{"Zeile mit Taste", entryText("de", e, true, "Q"), "▶ [Q] Zauberer I · Fireball  ·  lernen"},
		{"passiv gelernt", entryText("de", menuEntry{id: "thickSkin", line: "tank", tier: 2, passive: true, learned: true}, false, ""),
			"   Tank II · Thick Skin (passiv)  ·  gelernt"},
		{"Respec", entryText("en", menuEntry{id: respecID}, true, ""), "▶ Reset all skills (respec)"},
		{"Leiste", slotBar(newBarKey("de", slotViews([]string{"fireball", "taunt"}, []float64{0, 4.2}))),
			"Fireball bereit  ·  Taunt 5 s  ·  –  ·  –"},
	})
}

func TestMenuText(t *testing.T) {
	es := menuEntries(nil, []string{"taunt", "fireball"}, 1, only("heal"), false)
	slots := slotViews([]string{"fireball", "taunt", "", ""}, nil)
	cursor := slices.IndexFunc(es, func(e menuEntry) bool { return e.id == "fireball" })
	kb := buildMenu(newMenuKey("de", keys1, cursor, 1, es, slots), es)
	if kb.title != "Skills  ·  1 Punkte frei" || kb.n != menuWindow {
		t.Errorf("Titel und Fenster: %q %d", kb.title, kb.n)
	}
	sel := slices.IndexFunc(kb.rows[:], func(r menuRow) bool { return r.selected })
	if sel < 0 || !strings.HasPrefix(kb.rows[sel].text, "▶ [Q] ") || kb.rows[sel].glyph.key != "" {
		t.Errorf("Tastatur: Taste als Text vor dem Skill: %+v", kb.rows[sel])
	}
	if kb.footer != "A / D wählen · Leertaste bestätigen · K schließen" {
		t.Errorf("Hinweis Tastatur: %q", kb.footer)
	}
	gp := buildMenu(newMenuKey("de", pad, cursor, 1, es, slots), es)
	if r := gp.rows[sel]; r.glyph.key != "pad:LB" || strings.Contains(r.text, "[") {
		t.Errorf("Pad: Xbox-Glyph statt Text: %+v", r)
	}
	if gp.footer != "◀ ▶ wählen · A bestätigen · B schließen" {
		t.Errorf("Hinweis Pad: %q", gp.footer)
	}
	last := buildMenu(newMenuKey("de", pad, len(es)-1, 1, es, slots), es)
	if !last.rows[menuWindow-1].selected {
		t.Error("am Ende steht der Cursor in der letzten Zeile")
	}
}
