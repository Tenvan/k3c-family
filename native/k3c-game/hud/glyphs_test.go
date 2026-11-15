package hud

import (
	"slices"
	"testing"

	"k3c/native/k3c-game/view"
)

var (
	pad   = view.DeviceID{Kind: view.DeviceGamepad, Index: 3}
	keys1 = view.DeviceID{Kind: view.DeviceKeyboard, Index: 0}
)

// Eingabe-Profile als Kurznamen der Tests.
const (
	wasd   = view.ProfileWASD
	arrows = view.ProfileArrows
	gp     = view.ProfileGamepad
)

// B-894/AC-04: glyphOf: Aktion + Profil → Glyph-Schlüssel (glyphs.test.ts, S6.2 AC-04, B-149 AC-01); jede Aktion hat in
// jedem Profil ein Icon, nur Xbox-Glyphen am Pad.
func TestGlyphSchluessel(t *testing.T) {
	cases := []struct {
		dev  string
		keys []string
	}{
		{gp, []string{"pad:A", "pad:MENU", "pad:RS", "pad:X", "pad:LB", "pad:RB", "pad:LT", "pad:UP", "pad:DOWN"}},
		{wasd, []string{"key:SPACE", "key:ESC", "key:F11", "key:E", "key:Q", "key:R", "key:T", "key:Z", "key:K"}},
		{arrows, []string{"key:ENTER", "key:ESC", "key:F11", "key:CONTROLRIGHT", "key:INSERT", "key:DELETE", "key:END", "key:PAGEDOWN", "key:DOWN"}},
	}
	for _, c := range cases {
		var got []string
		for _, a := range glyphActions {
			g := glyphOf("de", a, c.dev)
			got = append(got, g.key)
			if g.label == "" {
				t.Errorf("%v %s ohne Beschriftung", c.dev, a)
			}
		}
		if !slices.Equal(got, c.keys) {
			t.Errorf("%v: %v, erwartet %v", c.dev, got, c.keys)
		}
	}
}

func TestGlyphBeschriftung(t *testing.T) {
	checkTexts(t, []textCase{
		{"Leertaste", glyphOf("de", "confirm", wasd).label, "Leertaste"},
		{"Space", glyphOf("en", "confirm", wasd).label, "Space"},
		{"Steuerkreuz", glyphOf("de", "skillMenu", gp).label, "D-Pad ↓"},
		{"Entf", glyphOf("de", "skill2", arrows).label, "Entf"},
		{"Bild ab englisch", glyphOf("en", "skill4", arrows).label, "Page Down"},
		{"rechter Stick", glyphOf("de", "fullscreen", gp).label, "RS"},
		{"unbekannt", glyphOf("de", "dance", gp).label, "dance"},
	})
	if got := slotLabels("de", wasd); got != [6]string{"E", "Q", "R", "T", "Z", "K"} {
		t.Errorf("Slots Tastatur: %v", got)
	}
	if got := slotLabels("de", gp); got != [6]string{"X", "LB", "RB", "LT", "D-Pad ↑", "D-Pad ↓"} {
		t.Errorf("Slots Pad: %v", got)
	}
}

func TestBUndViewNieAlsGlyph(t *testing.T) {
	for _, d := range []string{gp, wasd, arrows} {
		for _, a := range glyphActions {
			if k := glyphOf("de", a, d).key; k == "pad:B" || k == "pad:VIEW" {
				t.Errorf("%s auf %v: %s", a, d, k)
			}
		}
	}
}

func TestGlyphFarbenKontrast(t *testing.T) {
	for name, c := range glyphColors {
		if r := contrastRatio(c[0], c[1]); r < 4.5 {
			t.Errorf("%s: Kontrast %.2f < 4,5", name, r)
		}
	}
}

func TestBeitrittsHinweis(t *testing.T) {
	checkTexts(t, []textCase{
		{"freies Pad", joinFromKey(newJoinKey("de", []view.DeviceID{pad})), "A drücken zum Beitreten"},
		{"zwei Pads einmal", joinFromKey(newJoinKey("de", []view.DeviceID{pad, {Kind: view.DeviceGamepad, Index: 7}})),
			"A drücken zum Beitreten"},
		{"Tastatur (Pfeile) und Pad", joinFromKey(newJoinKey("de", []view.DeviceID{keys1, pad})),
			"Enter drücken zum Beitreten  ·  A drücken zum Beitreten"},
		{"Tastatur (B-946: Enter statt Leertaste)", joinFromKey(newJoinKey("de", []view.DeviceID{keys1})), "Enter drücken zum Beitreten"},
		{"englisch", joinFromKey(newJoinKey("en", []view.DeviceID{keys1})), "Press Enter to join"},
		{"alle beigetreten", joinFromKey(newJoinKey("de", nil)), ""},
	})
}

// Schriftrollen (fontRules.ts, Q03): Pflicht-Info mindestens 28 px, Nebeninfo 24 px bei 1–2 Spielern; Kontrast der
// Farben zur schwarzen Kontur ≥ 4,5.
func TestSchriftrollen(t *testing.T) {
	if minFontPx(2, true, artInfo) != 28 || minFontPx(2, false, artInfo) != 24 || minFontPx(2, true, artSide) != 24 ||
		minFontPx(4, false, artSide) != 20 {
		t.Error("Mindestgrößen wie bedienung.md § 2")
	}
	roles := []fontRole{rolePlayer, roleShared, roleFight, roleBanner, roleBoss, roleTravel, roleJoin, roleHint, roleRoomInfo, roleCitizen}
	for _, r := range roles {
		if fontPx(r, 2) < minFontPx(2, true, r.art) {
			t.Errorf("Rolle %+v unter der Mindestgröße", r)
		}
		if contrastRatio(r.clr, black) < 4.5 {
			t.Errorf("Rolle %+v: Kontrast zur Kontur zu klein", r)
		}
	}
	if roleHint.art != artSide || roleHint.px != 24 {
		t.Error("Aktions-Hinweis ist Nebeninfo in 24 px")
	}
}

// Bedienhinweis des Skill-Menüs mit den Tasten des Profils: Profil pfeile nennt Pfeile und Enter.
func TestMenuFooterPerDevice(t *testing.T) {
	cases := []struct {
		dev  string
		want string
	}{
		{wasd, "A / D wählen · Leertaste bestätigen · K schließen"},
		{arrows, "Pfeil links / Pfeil rechts wählen · Enter bestätigen · Pfeil unten schließen"},
		{gp, "◄ ► wählen · A bestätigen · B schließen"},
	}
	for _, c := range cases {
		if got := menuLine("de", c.dev).plain; got != c.want {
			t.Errorf("%v: %q, erwartet %q", c.dev, got, c.want)
		}
	}
}
