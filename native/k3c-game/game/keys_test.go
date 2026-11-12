package game

import (
	"slices"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/hud"
	"k3c/native/k3c-game/input"
	"k3c/native/k3c-game/view"
)

// keysOf liefert die Tasten der Hälfte half für eine Aktion des HUD (Menü-Aktion oder gehaltener Befehl).
func keysOf(half int, action string) []ebiten.Key {
	switch action {
	case "confirm":
		return input.ActionKeys(half, view.ActionConfirm)
	case "pause":
		return input.ActionKeys(half, view.ActionMenu)
	case "fullscreen":
		return input.ActionKeys(half, view.ActionFullscreen)
	case "skillMenu":
		return input.ActionKeys(half, view.ActionSkillMenu)
	}
	return input.CommandKeys(half, action)
}

// Die Tasten-Texte des HUD je Tastatur-Hälfte stimmen mit der Belegung in input überein (hud darf input nicht
// importieren, game kennt beide); nicht belegte Aktionen haben auf beiden Seiten keine Taste.
func TestHudKeyNamesMatchInput(t *testing.T) {
	for half := range 2 {
		for _, a := range hud.KeyActions {
			name, keys := hud.KeyName(half, a), keysOf(half, a)
			names := make([]string, len(keys))
			for i, k := range keys {
				names[i] = hudKeyName(k)
			}
			if (name == "") != (len(keys) == 0) || (name != "" && !slices.Contains(names, name)) {
				t.Errorf("Hälfte %d, %s: HUD %q, input %v", half, a, name, names)
			}
		}
	}
}

// Tastenbelegung für das Pause-Menü (B-807): eine Zeile je view.KeyActions, Tastatur aus input mit den Beschriftungen
// des HUD, Gamepad mit der Standardbelegung; nicht belegt (Vollbild rechts) bleibt leer.
func TestKeyRows(t *testing.T) {
	rows := KeyRows("de")
	if len(rows) != len(view.KeyActions) {
		t.Fatalf("%d Zeilen, erwartet %d", len(rows), len(view.KeyActions))
	}
	want := map[string]view.KeyRow{
		view.KeyMove:       {Left: "A / D", Right: "Pfeil links / Pfeil rechts", Pad: "Stick / Steuerkreuz"},
		view.KeySprint:     {Left: "Shift links", Right: "Strg rechts", Pad: "RT"},
		view.KeyPay:        {Left: "Leertaste", Right: "Enter", Pad: "A"},
		view.KeySkill4:     {Left: "Z", Right: "Num 4", Pad: "D-Pad ↑"},
		view.KeyPause:      {Left: "Esc", Right: "Backspace", Pad: "Menu"},
		view.KeyFullscreen: {Left: "F", Right: "", Pad: "RS"},
	}
	for i, r := range rows {
		if r.Action != view.KeyActions[i] || r.Left == "" || r.Pad == "" {
			t.Errorf("Zeile %d: %+v", i, r)
		}
		if (r.Right == "") != (len(keysFor(1, r.Action)) == 0) {
			t.Errorf("%s rechts: %q gegen input %v", r.Action, r.Right, keysFor(1, r.Action))
		}
		if w, ok := want[r.Action]; ok && (r.Left != w.Left || r.Right != w.Right || r.Pad != w.Pad) {
			t.Errorf("%s: %+v, erwartet %+v", r.Action, r, w)
		}
	}
	if en := KeyRows("en"); en[2].Left != "Space" {
		t.Errorf("englisch: %+v", en[2])
	}
}
