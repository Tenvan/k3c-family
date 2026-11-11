package hud

import (
	"slices"
	"testing"

	"k3c/native/k3c-game/view"
)

var (
	pad   = view.DeviceID{Kind: view.DeviceGamepad, Index: 3}
	keys1 = view.DeviceID{Kind: view.DeviceKeyboard, Index: 0}
	keys2 = view.DeviceID{Kind: view.DeviceKeyboard, Index: 1}
)

// glyphOf: Aktion + Gerät → Glyph-Schlüssel (glyphs.test.ts, S6.2 AC-04, B-149 AC-01); nur Xbox-Glyphen am Pad.
func TestGlyphSchluessel(t *testing.T) {
	cases := []struct {
		dev  view.DeviceID
		keys []string
	}{
		{pad, []string{"pad:A", "pad:MENU", "", "pad:X", "pad:LB", "pad:RB", "pad:LT", "pad:UP", "pad:DOWN"}},
		{keys1, []string{"key:SPACE", "key:ESC", "key:F", "key:E", "key:Q", "key:R", "key:T", "key:Z", "key:K"}},
		{keys2, []string{"key:ENTER", "key:BACKSPACE", "", "key:NUM0", "key:NUM1", "key:NUM2", "key:NUM3", "key:NUM4", "key:NUM5"}},
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
		{"Leertaste", glyphOf("de", "confirm", keys1).label, "Leertaste"},
		{"Space", glyphOf("en", "confirm", keys1).label, "Space"},
		{"Steuerkreuz", glyphOf("de", "skillMenu", pad).label, "D-Pad ↓"},
		{"Ziffernblock", glyphOf("de", "skill2", keys2).label, "Num 2"},
		{"rechter Stick", glyphOf("de", "fullscreen", pad).label, "RS"},
		{"unbekannt", glyphOf("de", "dance", pad).label, "dance"},
	})
	if got := slotLabels("de", keys1); got != [6]string{"E", "Q", "R", "T", "Z", "K"} {
		t.Errorf("Slots Tastatur: %v", got)
	}
	if got := slotLabels("de", pad); got != [6]string{"X", "LB", "RB", "LT", "D-Pad ↑", "D-Pad ↓"} {
		t.Errorf("Slots Pad: %v", got)
	}
}

func TestBUndViewNieAlsGlyph(t *testing.T) {
	for _, d := range []view.DeviceID{pad, keys1, keys2} {
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
		{"Tastatur und Pad", joinFromKey(newJoinKey("de", []view.DeviceID{keys2, pad})),
			"Enter drücken zum Beitreten  ·  A drücken zum Beitreten"},
		{"englisch", joinFromKey(newJoinKey("en", []view.DeviceID{keys1})), "Press Space to join"},
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

// Bedienhinweis des Skill-Menüs mit den Tasten des Geräts: rechte Tastatur-Hälfte nennt Pfeile und Enter.
func TestMenuFooterPerDevice(t *testing.T) {
	cases := []struct {
		dev  view.DeviceID
		want string
	}{
		{view.DeviceID{Kind: view.DeviceKeyboard}, "A / D wählen · Leertaste bestätigen · K schließen"},
		{view.DeviceID{Kind: view.DeviceKeyboard, Index: 1}, "Pfeil links / Pfeil rechts wählen · Enter bestätigen · Num 5 schließen"},
		{view.DeviceID{Kind: view.DeviceGamepad}, "◀ ▶ wählen · A bestätigen · B schließen"},
	}
	for _, c := range cases {
		label := slotLabels("de", c.dev)[5]
		if got := menuFooter("de", c.dev, label); got != c.want {
			t.Errorf("%v: %q, erwartet %q", c.dev, got, c.want)
		}
	}
}
