package game

// Tastenbelegung für das Pause-Menü (B-807): game kennt input (Tasten) und hud (Beschriftungen) und baut daraus die
// Zeilen von view.KeyTable; menu bekommt nur die fertigen Zeilen (Importregel, view/keys.go).

import (
	"strings"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/hud"
	"k3c/native/k3c-game/input"
	"k3c/native/k3c-game/view"
)

// keyCommands sind die Namen der Aktionen der Tastenbelegung in input (gehaltene Befehle) bzw. hud (PadLabel); move
// sind zwei Befehle (links, rechts).
var keyCommands = map[string]string{
	view.KeySprint: "sprint", view.KeyPay: "pay", view.KeyAttack: "attack", view.KeySkill1: "skill1",
	view.KeySkill2: "skill2", view.KeySkill3: "skill3", view.KeySkill4: "skill4",
}

// keyActions sind die Aktionen der Tastenbelegung, die in input Menü-Aktionen sind, mit ihrem Namen in hud.
var keyActions = map[string]struct {
	a   view.Action
	hud string
}{
	view.KeySkillMenu: {view.ActionSkillMenu, "skillMenu"}, view.KeyPause: {view.ActionMenu, "pause"},
	view.KeyFullscreen: {view.ActionFullscreen, "fullscreen"},
}

// hudKeyName ist der Name einer Ebitengine-Taste in der Schreibweise des HUD (hud.KeyName, hud.KeyLabel).
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

// keysFor liefert die Tasten der Tastatur-Hälfte half für eine Aktion der Tastenbelegung (move: links, dann rechts).
func keysFor(half int, action string) []ebiten.Key {
	if action == view.KeyMove {
		l, r := input.CommandKeys(half, "left"), input.CommandKeys(half, "right")
		if len(l) == 0 || len(r) == 0 {
			return nil
		}
		return []ebiten.Key{l[0], r[0]}
	}
	if c, ok := keyCommands[action]; ok {
		return input.CommandKeys(half, c)
	}
	return input.ActionKeys(half, keyActions[action].a)
}

// keyText beschriftet die Tasten einer Aktion: bei move beide Richtungen, sonst die erste (weitere sind Ersatz, z. B.
// Y für Z auf QWERTY, Num-Enter); "" = nicht belegt.
func keyText(lang string, keys []ebiten.Key, both bool) string {
	switch {
	case len(keys) == 0:
		return ""
	case both && len(keys) > 1:
		return hud.KeyLabel(lang, hudKeyName(keys[0])) + " / " + hud.KeyLabel(lang, hudKeyName(keys[1]))
	}
	return hud.KeyLabel(lang, hudKeyName(keys[0]))
}

// padAction ist der Name einer Aktion der Tastenbelegung für hud.PadLabel.
func padAction(action string) string {
	switch action {
	case view.KeyPay:
		return "confirm"
	case view.KeyMove, view.KeySprint:
		return action
	}
	if c, ok := keyCommands[action]; ok {
		return c
	}
	return keyActions[action].hud
}

// KeyRows ist die Tastenbelegung für menu (view.KeyTable): eine Zeile je view.KeyActions, Tastatur aus input, Gamepad
// mit der Standardbelegung (auch ohne Pad), beschriftet wie im HUD.
func KeyRows(lang string) []view.KeyRow {
	out := make([]view.KeyRow, 0, len(view.KeyActions))
	for _, a := range view.KeyActions {
		move := a == view.KeyMove
		out = append(out, view.KeyRow{Action: a, Left: keyText(lang, keysFor(0, a), move),
			Right: keyText(lang, keysFor(1, a), move), Pad: hud.PadLabel(lang, padAction(a))})
	}
	return out
}
