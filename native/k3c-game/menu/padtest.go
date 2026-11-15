package menu

import (
	"fmt"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// Tasten-Test (B-944): Seite unter Testing, zeigt je Gerät live die gedrückten Tasten (Standard-Belegung mit Xbox-Namen
// und roh), die Achsen und die zuletzt ausgelöste Aktion. So lässt sich am Gerät prüfen, ob Taste und Aktion
// zusammenpassen. B geht zurück (wie in jedem Menü). Angezeigt werden die Tastatur und die ersten padShown Pads.

const (
	padShown = 2   // angezeigte Pads (zwei Zeilen je Pad, dazu Tastatur und Zurück: höchstens maxRows)
	holdLast = 45  // Updates (30 Hz), die eine Aktion nach dem Auslösen stehen bleibt
	nameMax  = 22  // Zeichen des Gerätenamens in der Zeile
	axisZero = 0.05 // Achsen darunter gelten als 0
)

// padButtons sind die Tasten der Standard-Belegung mit Xbox-Namen.
var padButtons = [...]struct {
	b    ebiten.StandardGamepadButton
	name string
}{
	{ebiten.StandardGamepadButtonRightBottom, "A"}, {ebiten.StandardGamepadButtonRightRight, "B"},
	{ebiten.StandardGamepadButtonRightLeft, "X"}, {ebiten.StandardGamepadButtonRightTop, "Y"},
	{ebiten.StandardGamepadButtonFrontTopLeft, "LB"}, {ebiten.StandardGamepadButtonFrontTopRight, "RB"},
	{ebiten.StandardGamepadButtonFrontBottomLeft, "LT"}, {ebiten.StandardGamepadButtonFrontBottomRight, "RT"},
	{ebiten.StandardGamepadButtonCenterLeft, "View"}, {ebiten.StandardGamepadButtonCenterRight, "Menu"},
	{ebiten.StandardGamepadButtonLeftStick, "LS"}, {ebiten.StandardGamepadButtonRightStick, "RS"},
	{ebiten.StandardGamepadButtonLeftTop, "↑"}, {ebiten.StandardGamepadButtonLeftBottom, "↓"},
	{ebiten.StandardGamepadButtonLeftLeft, "←"}, {ebiten.StandardGamepadButtonLeftRight, "→"},
	{ebiten.StandardGamepadButtonCenterCenter, "Guide"},
}

// actionNames nennt die Menü-Aktionen (Index view.Action).
var actionNames = [...]string{"Hoch", "Runter", "Links", "Rechts", "Bestätigen", "Zurück", "Menü", "View", "Seite zurück",
	"Seite vor", "Home", "Skill-Menü", "Vollbild", "Cheats"}

// padRaw ist der Rohzustand eines Pads: Gerät, Standard-Tasten, Achsen und rohe Tasten und Achsen.
type padRaw struct {
	id       ebiten.GamepadID
	name     string
	sdl      string
	standard bool
	buttons  string // gedrückte Tasten der Standard-Belegung, Xbox-Namen
	axes     string // Sticks der Standard-Belegung bzw. rohe Achsen
	raw      string // rohe Tasten (Nummern)
}

// readPads liest die verbundenen Pads; Tests ersetzen es.
var readPads = ebitenPads

func ebitenPads() []padRaw {
	var out []padRaw
	for _, id := range ebiten.AppendGamepadIDs(nil) {
		p := padRaw{id: id, name: ebiten.GamepadName(id), sdl: ebiten.GamepadSDLID(id), standard: ebiten.IsStandardGamepadLayoutAvailable(id)}
		var down []string
		if p.standard {
			for _, b := range padButtons {
				if ebiten.IsStandardGamepadButtonPressed(id, b.b) {
					down = append(down, b.name)
				}
			}
			p.axes = fmt.Sprintf("LS %s %s · RS %s %s",
				axis(ebiten.StandardGamepadAxisValue(id, ebiten.StandardGamepadAxisLeftStickHorizontal)),
				axis(ebiten.StandardGamepadAxisValue(id, ebiten.StandardGamepadAxisLeftStickVertical)),
				axis(ebiten.StandardGamepadAxisValue(id, ebiten.StandardGamepadAxisRightStickHorizontal)),
				axis(ebiten.StandardGamepadAxisValue(id, ebiten.StandardGamepadAxisRightStickVertical)))
		}
		p.buttons = joinOr(down, " ")
		var raw []string
		for i := range ebiten.GamepadButtonCount(id) {
			if ebiten.IsGamepadButtonPressed(id, ebiten.GamepadButton(i)) {
				raw = append(raw, fmt.Sprint(i))
			}
		}
		p.raw = joinOr(raw, " ")
		if !p.standard {
			var ax []string
			for i := range ebiten.GamepadAxisCount(id) {
				ax = append(ax, axis(ebiten.GamepadAxisValue(id, ebiten.GamepadAxisType(i))))
			}
			p.axes = "ohne Standard-Belegung, Achsen " + joinOr(ax, " ")
		}
		out = append(out, p)
	}
	return out
}

// axis formatiert einen Achsenwert (Totzone 0).
func axis(v float64) string {
	if v > -axisZero && v < axisZero {
		return "0"
	}
	return fmt.Sprintf("%.2f", v)
}

func joinOr(s []string, sep string) string {
	if len(s) == 0 {
		return "–"
	}
	return strings.Join(s, sep)
}

// padTestScreen ist die Seite Tasten-Test.
type padTestScreen struct {
	base
	m      *menus
	pg     page
	pads   []padRaw
	last   map[view.DeviceID]string // zuletzt ausgelöste Aktion je Gerät
	age    map[view.DeviceID]int    // Updates seit dem Auslösen
	shown  string                   // Text der letzten Anzeige (nur bei Änderung neu bauen)
}

func newPadTest(m *menus) *padTestScreen {
	return &padTestScreen{m: m, last: map[view.DeviceID]string{}, age: map[view.DeviceID]int{}}
}

func (sc *padTestScreen) Update(in view.Input, s view.Session) view.Step {
	sc.pads = readPads()
	for id := range sc.age {
		if sc.age[id]++; sc.age[id] > holdLast {
			delete(sc.age, id)
			delete(sc.last, id)
		}
	}
	for _, di := range in.Devices {
		for _, a := range di.Actions {
			if a >= 0 && int(a) < len(actionNames) {
				sc.last[di.Device], sc.age[di.Device] = actionNames[a], 0
			}
		}
	}
	if text := sc.text(); text != sc.shown {
		sc.shown = text
		sc.ver++
	}
	return sc.run(in, s, sc)
}

// handle: B und A auf Zurück gehen zurück; alle anderen Tasten zeigt die Seite nur an.
func (sc *padTestScreen) handle(_ view.DeviceID, k key, _ view.Session) (view.StepKind, view.Screen) {
	if k == keyBack || k == keyHome {
		sc.cue(view.CueBack, -1, "")
		return view.StepPop, nil
	}
	return view.StepStay, nil
}

// action liefert die zuletzt ausgelöste Aktion des Geräts d ("–" ohne).
func (sc *padTestScreen) action(d view.DeviceID) string {
	if a, ok := sc.last[d]; ok {
		return a
	}
	return "–"
}

// rows liefert die Zeilen (Beschriftung, Wert) der Anzeige.
func (sc *padTestScreen) rows() [][2]string {
	out := [][2]string{{"Tastatur", "→ " + sc.action(view.DeviceID{Kind: view.DeviceKeyboard})}}
	if len(sc.pads) == 0 {
		return append(out, [2]string{"Gamepad", "keines verbunden"})
	}
	for i, p := range sc.pads {
		if i >= padShown {
			break
		}
		d := view.DeviceID{Kind: view.DeviceGamepad, Index: int(p.id)}
		out = append(out,
			[2]string{fmt.Sprintf("Pad %d · %s", int(p.id)+1, clip(p.name, nameMax)), p.buttons + " → " + sc.action(d)},
			[2]string{"  roh " + p.raw, p.axes})
	}
	return out
}

func (sc *padTestScreen) text() string {
	var b strings.Builder
	for _, r := range sc.rows() {
		b.WriteString(r[0] + "|" + r[1] + "\n")
	}
	return b.String()
}

// clip kürzt s auf n Zeichen.
func clip(s string, n int) string {
	if r := []rune(s); len(r) > n {
		return string(r[:n])
	}
	return s
}

func (sc *padTestScreen) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if lang := f.Settings.Language; sc.pg.stale(sc.ver, lang) {
		sc.fill(lang)
	}
	sc.m.paint.page(dst, c, &sc.pg)
}

// fill baut die Zeilen der Seite; die Gerätetexte sind Entwickler-Anzeigen und nur deutsch.
func (sc *padTestScreen) fill(lang string) {
	p := &sc.pg
	p.reset(tr(lang, txtTstPads))
	p.sub = "Tasten, Achsen und Aktion je Gerät live"
	for _, r := range sc.rows() {
		p.add(row{label: r[0], value: r[1], mat: matBlue})
	}
	p.add(row{label: tr(lang, txtBack), mat: matCoral})
	p.sel = p.n - 1
	if len(sc.pads) > 0 {
		p.note = fmt.Sprintf("Pad %d: %s · %s", int(sc.pads[0].id)+1, clip(sc.pads[0].name, 40), clip(sc.pads[0].sdl, 40))
	}
	p.hint = tr(lang, txtPadTestHint)
}
