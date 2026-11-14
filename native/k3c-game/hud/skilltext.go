package hud

// Texte der Skill-Leiste und des Skill-Menüs (Spiegel von src/scenes/hud/skillMenuText.ts, B-755): Die Leiste nennt
// keine Tasten, das Menü zeigt die Slot-Taste des Geräts vor dem Skill (Pad: Xbox-Glyph, Tastatur: „[Q] “).

import (
	"math"
	"strconv"
	"strings"
	"unicode"

	"k3c/native/k3c-game/view"
)

// menuWindow: sichtbare Menüzeilen um den Cursor.
const menuWindow = 5

var tierNames = [...]string{"", "I", "II", "III", "IV"}

// skillName macht aus der ID einen Namen („shieldBash“ → „Shield Bash“); die Texte haben keine Skill-Namen.
func skillName(id string) string {
	var b strings.Builder
	for i, r := range id {
		switch {
		case i == 0:
			b.WriteRune(unicode.ToUpper(r))
		case unicode.IsUpper(r):
			b.WriteByte(' ')
			b.WriteRune(r)
		default:
			b.WriteRune(r)
		}
	}
	return b.String()
}

// entryText ist eine Menüzeile: Cursor, Taste (nur belegter Skill, "" = keine), Linie, Tier und Name, dann Zustand.
func entryText(lang string, e menuEntry, selected bool, key string) string {
	mark := "   "
	if selected {
		mark = "► "
	}
	if e.id == respecID {
		return mark + tr(lang, "skill.respec")
	}
	state := tr(lang, "skill.locked")
	switch {
	case e.learned:
		state = tr(lang, "skill.learned")
	case e.learnable:
		state = tr(lang, "skill.learnable")
	}
	passive := ""
	if e.passive {
		passive = " (" + tr(lang, "skill.passive") + ")"
	}
	if key != "" {
		key = "[" + key + "] "
	}
	tier := strconv.Itoa(e.tier)
	if e.tier >= 0 && e.tier < len(tierNames) {
		tier = tierNames[e.tier]
	}
	return mark + key + lineName(e.line) + " " + tier + " · " + skillName(e.id) + passive + sep + state
}

// menuRow ist eine sichtbare Zeile des Menüs: Glyph der Slot-Taste (Xbox-Taste bzw. Tastenkappe, B-894) und Text.
type menuRow struct {
	glyph    glyph
	text     string
	selected bool
}

// menuView ist das gezeichnete Menü: Titel, Zeilen um den Cursor, Bedienhinweis.
type menuView struct {
	title  string
	rows   [menuWindow]menuRow
	n      int
	footer string
}

// menuKey ist alles, wovon der Text des Menüs abhängt.
type menuKey struct {
	lang, profile      string // Sprache, Eingabe-Profil des Spielers (B-894)
	cursor, points, n  int
	learned, learnable uint64
	respec             bool
	slots              [4]string
}

// newMenuKey fasst Einträge, Punkte und Slots zusammen.
func newMenuKey(lang, profile string, cursor, points int, entries []menuEntry, slots [4]slotView) menuKey {
	k := menuKey{lang: lang, profile: profile, cursor: cursor, points: points, n: len(entries)}
	for i, e := range entries {
		switch {
		case e.id == respecID:
			k.respec = true
		case i < 64 && e.learned:
			k.learned |= 1 << i
		}
		if i < 64 && e.learnable {
			k.learnable |= 1 << i
		}
	}
	for i, s := range slots {
		k.slots[i] = s.skill
	}
	return k
}

// buildMenu baut die Zeilen aus den Einträgen (Fenster von menuWindow Zeilen um den Cursor).
func buildMenu(k menuKey, entries []menuEntry) menuView {
	v := menuView{title: tf(k.lang, "skill.title", "n", k.points)}
	labels := slotLabels(k.lang, k.profile)
	from := max(0, min(k.cursor-menuWindow/2, len(entries)-menuWindow))
	for i := from; i < min(len(entries), from+menuWindow); i++ {
		e, row := entries[i], menuRow{selected: i == k.cursor}
		key := ""
		for s, id := range k.slots {
			if id != "" && id == e.id {
				row.glyph = glyphOf(k.lang, glyphActions[4+s], k.profile)
				if row.glyph.key == "" {
					key = labels[s+1] // ohne Icon: Taste als Text
				}
			}
		}
		row.text = entryText(k.lang, e, row.selected, key)
		v.rows[v.n] = row
		v.n++
	}
	v.footer = menuFooter(k.lang, k.profile, labels[5])
	return v
}

// menuFooter ist der Bedienhinweis mit den Tasten des Eingabe-Profils; am Pad schließt B (Steuerkreuz ↓ wählt im
// offenen Menü weiter).
func menuFooter(lang, profile, menuKeyLabel string) string {
	if profile == view.ProfileGamepad {
		return tf(lang, "skill.hint.pad", "menu", "B")
	}
	move := glyphOf(lang, "left", profile).label + " / " + glyphOf(lang, "right", profile).label
	return tf(lang, "skill.hint.keyboard", "move", move, "ok", glyphOf(lang, "confirm", profile).label, "menu", menuKeyLabel)
}

// barKey: Skill und Abklingzeit (ganze Sekunden, aufgerundet) je Slot.
type barKey struct {
	lang   string
	skills [4]string
	cd     [4]int
}

func newBarKey(lang string, slots [4]slotView) barKey {
	k := barKey{lang: lang}
	for i, s := range slots {
		k.skills[i], k.cd[i] = s.skill, int(math.Ceil(s.cooldown))
	}
	return k
}

// slotBar ist die Skill-Leiste: Skill und Abklingzeit je Slot, ohne Taste.
func slotBar(k barKey) string {
	parts := make([]string, 0, 4)
	for i, id := range k.skills {
		switch {
		case id == "":
			parts = append(parts, tr(k.lang, "skill.free"))
		case k.cd[i] > 0:
			parts = append(parts, skillName(id)+" "+strconv.Itoa(k.cd[i])+" s")
		default:
			parts = append(parts, skillName(id)+" "+tr(k.lang, "skill.ready"))
		}
	}
	return strings.Join(parts, sep)
}
