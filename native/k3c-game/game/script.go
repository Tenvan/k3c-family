package game

// Feste Eingabe-Folge des Aufnahme-Modus (--input): gehaltene Befehle und Menü-Aktionen je Spieler und Takt.

import (
	"errors"
	"fmt"
	"strconv"
	"strings"

	"k3c/native/k3c-game/view"
)

// actionNames sind die Menü-Aktionen der Eingabe-Folge („a:<name>“).
var actionNames = map[string]view.Action{
	"up": view.ActionUp, "down": view.ActionDown, "left": view.ActionLeft, "right": view.ActionRight,
	"confirm": view.ActionConfirm, "back": view.ActionBack, "menu": view.ActionMenu, "view": view.ActionView,
	"prev": view.ActionPrevPage, "next": view.ActionNextPage, "home": view.ActionHome,
	"skillmenu": view.ActionSkillMenu, "fullscreen": view.ActionFullscreen,
}

// ParseScript liest die Eingabe-Folge „spieler@takt=befehl,…“. Befehl (gehalten ab dem Takt) aus L (links),
// R (rechts), S (Sprint), P (zahlen), A (Schlag), 1–4 (Skill) oder „-“ (nichts); oder „a:<aktion>“: Menü-Aktion genau
// in diesem Takt (up, down, left, right, confirm, back, menu, view, prev, next, home, skillmenu, fullscreen), der
// gehaltene Befehl bleibt. Mehrere Aktionen im selben Takt kommen in der Reihenfolge der Folge.
// Beispiel: „0@0=R,1@0=LS,0@90=-,0@95=a:skillmenu,0@100=a:right“.
func ParseScript(s string) ([]Hold, error) {
	if s == "" {
		return nil, nil
	}
	var out []Hold
	for part := range strings.SplitSeq(s, ",") {
		h, err := parseHold(part)
		if err != nil {
			return nil, fmt.Errorf("🚫 --input %q: %v", part, err)
		}
		out = append(out, h)
	}
	return out, nil
}

// parseHold liest einen Eintrag „spieler@takt=befehl“ bzw. „spieler@takt=a:aktion“.
func parseHold(s string) (Hold, error) {
	who, cmd, ok := strings.Cut(s, "=")
	ps, ts, ok2 := strings.Cut(who, "@")
	p, errP := strconv.Atoi(ps)
	t, errT := strconv.Atoi(ts)
	if !ok || !ok2 || errP != nil || errT != nil || p < 0 || p >= view.MaxPlayers || t < 0 || t > maxTick {
		return Hold{}, errors.New("erwartet spieler@takt=befehl")
	}
	h := Hold{Tick: t, Player: p}
	if name, isAction := strings.CutPrefix(cmd, "a:"); isAction {
		a, known := actionNames[name]
		if !known {
			return Hold{}, fmt.Errorf("unbekannte Aktion %q (up down left right confirm back menu view prev next home skillmenu fullscreen)", name)
		}
		h.Press, h.Action = true, a
		return h, nil
	}
	if cmd == "-" {
		return h, nil
	}
	for _, r := range cmd {
		if err := apply(&h.Cmd, r); err != nil {
			return Hold{}, err
		}
	}
	if cmd == "" {
		return Hold{}, errors.New("leerer Befehl, „-“ für nichts")
	}
	return h, nil
}

// apply setzt den Buchstaben r im Befehl c.
func apply(c *view.Command, r rune) error {
	switch r {
	case 'L':
		c.Move--
	case 'R':
		c.Move++
	case 'S':
		c.Sprint = true
	case 'P':
		c.Pay = true
	case 'A':
		c.Attack = true
	case '1', '2', '3', '4':
		c.Skill = int(r - '0')
	default:
		return fmt.Errorf("unbekannter Befehl %q (L R S P A 1–4 - oder a:aktion)", r)
	}
	return nil
}

// CommandsAt liefert die gehaltenen Befehle aller Spieler im Takt tick: je Spieler der letzte Befehls-Eintrag mit
// Tick ≤ tick (bei gleichem Takt der spätere in der Folge); Aktionen zählen nicht.
func CommandsAt(script []Hold, tick int) [view.MaxPlayers]view.Command {
	var out [view.MaxPlayers]view.Command
	var best [view.MaxPlayers]int
	for i := range best {
		best[i] = -1
	}
	for _, h := range script {
		if !h.Press && h.Tick <= tick && h.Tick >= best[h.Player] {
			out[h.Player], best[h.Player] = h.Cmd, h.Tick
		}
	}
	return out
}

// ActionsAt hängt die Aktionen des Spielers player im Takt tick in Reihenfolge der Folge an dst.
func ActionsAt(dst []view.Action, script []Hold, player, tick int) []view.Action {
	for _, h := range script {
		if h.Press && h.Player == player && h.Tick == tick {
			dst = append(dst, h.Action)
		}
	}
	return dst
}

// scripted ist die Eingabe-Quelle der festen Folge: je Poll ein Takt (ab dem Start, auch im Vorlauf), Geräte wie
// Scene.Devices.
type scripted struct {
	script []Hold
	devs   []view.DeviceID
	buf    []view.DeviceInput
	acts   [view.MaxPlayers][]view.Action
	tick   int
}

func newScripted(s Scene) *scripted {
	return &scripted{script: s.Script, devs: s.Devices(), buf: make([]view.DeviceInput, s.Players)}
}

func (s *scripted) Poll() view.Input {
	cmds := CommandsAt(s.script, s.tick)
	for i, d := range s.devs {
		s.acts[i] = ActionsAt(s.acts[i][:0], s.script, i, s.tick)
		s.buf[i] = view.DeviceInput{Device: d, Command: cmds[i], Actions: s.acts[i]}
	}
	s.tick++
	return view.Input{Devices: s.buf}
}
