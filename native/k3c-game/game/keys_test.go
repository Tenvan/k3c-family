package game

import (
	"slices"
	"strings"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/hud"
	"k3c/native/k3c-game/input"
	"k3c/native/k3c-game/view"
)

// hudKeyName ist der Name einer Ebitengine-Taste in der Schreibweise des HUD (hud.KeyName).
func hudKeyName(k ebiten.Key) string {
	n := strings.ToUpper(k.String())
	switch {
	case n == "ESCAPE":
		return "ESC"
	case strings.HasPrefix(n, "ARROW"):
		return n[len("ARROW"):]
	case strings.HasPrefix(n, "NUMPAD") && len(n) == len("NUMPAD")+1:
		return "NUM" + n[len("NUMPAD"):]
	}
	return n
}

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
