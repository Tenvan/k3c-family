package input

// Reine Auswertung eines Geräts: Rohzustand (gehaltene Tasten, Stick) → Befehl und Menü-Aktionen. Der tracker hält
// je Gerät, was über die Updates zählt (vorige Tasten, Haltedauer, Hysterese des Sticks, Kombi View + Menu).

import (
	"math"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// KeyState ist der Rohzustand der Tastatur in einem Update: gehaltene Tasten nach ebiten.Key.
type KeyState [ebiten.KeyMax + 1]bool

// PadState ist der Rohzustand eines Gamepads in einem Update.
type PadState struct {
	ID       ebiten.GamepadID
	Standard bool   // Standard-Belegung vorhanden; sonst wird das Pad ignoriert
	Name     string // nur für die Meldung bei fehlender Standard-Belegung
	Buttons  [ebiten.StandardGamepadButtonMax + 1]bool
	X, Y     float64 // linker Stick, -1 … 1 (Y negativ = oben)
}

// Schwellen des Sticks mit Hysterese: an ab der ersten, aus erst unter der zweiten Schwelle.
const (
	moveOn  = 0.25 // laufen (Totzone wie im Browser-Client)
	moveOff = 0.15
	navOn   = 0.5 // Menü-Richtung aus dem Stick
	navOff  = 0.3
)

// Takte bei 30 Updates je Sekunde (game.TPS).
const (
	repeatDelay = 12 // Wiederholung einer gehaltenen Richtung ab 400 ms …
	repeatEvery = 4  // … dann alle 133 ms
	menuShort   = 18 // Menu kurz: losgelassen nach höchstens 600 ms
)

// tracker ist der Zustand eines Geräts über die Updates.
type tracker struct {
	started bool
	prev    [actionCount]bool
	hold    [actionCount]int  // Updates in Folge gehalten, einschließlich des aktuellen
	ignore  [actionCount]bool // beim ersten Update schon gehalten: ohne Wirkung bis zum Loslassen
	chord   bool              // View + Menu kam in diesem Halten: Loslassen löst weder Menu noch View aus
	moving  bool              // Stick außerhalb der Totzone (Hysterese)
	nav     [4]bool           // Stick-Richtungen ↑ ↓ ← → (Hysterese)
}

// hysteresis meldet, ob ein Wert v ≥ 0 aktiv ist: einschalten über on, ausschalten erst unter off.
func hysteresis(active bool, v, on, off float64) bool {
	if active {
		return v > off
	}
	return v > on
}

// stickMove liefert die Laufrichtung aus der Stick-Achse x wie der Browser (src/input/playerInput.ts › moveX): 0 in der
// Totzone, sonst der Rohwert (begrenzt auf -1 … 1). Die Hysterese hält den Rohwert noch bis unter moveOff, damit der
// Monarch an der Schwelle nicht flackert; der Betrag liegt damit nie unter dem des Browsers.
func (t *tracker) stickMove(x float64) float64 {
	if math.IsNaN(x) {
		x = 0
	}
	a := math.Abs(x)
	t.moving = a >= moveOn || hysteresis(t.moving, a, moveOn, moveOff) // ab genau 0,25 wie der Browser
	if !t.moving {
		return 0
	}
	return math.Copysign(min(1, a), x)
}

// stickNav aktualisiert die Menü-Richtungen des Sticks (↑ ↓ ← →) mit Hysterese.
func (t *tracker) stickNav(x, y float64) {
	for i, v := range [4]float64{-y, y, -x, x} {
		if math.IsNaN(v) {
			v = 0
		}
		t.nav[i] = hysteresis(t.nav[i], v, navOn, navOff)
	}
}

// padCommand liefert den Befehl eines Gamepads; das Steuerkreuz geht vor dem Stick, beide zugleich heben sich auf.
func (t *tracker) padCommand(p *PadState) view.Command {
	b := &p.Buttons
	c := view.Command{Move: t.stickMove(p.X), Sprint: b[padRT], Pay: b[padA], Attack: b[padX]}
	switch {
	case b[padLeft] && !b[padRight]:
		c.Move = -1
	case b[padRight] && !b[padLeft]:
		c.Move = 1
	}
	for i, s := range padSkills {
		if b[s] {
			c.Skill = i + 1
			break
		}
	}
	return c
}

// padHeld liefert die gehaltenen Menü-Tasten eines Gamepads; Steuerkreuz und Stick zählen gleich.
func (t *tracker) padHeld(p *PadState) [actionCount]bool {
	t.stickNav(p.X, p.Y)
	var held [actionCount]bool
	for a, b := range padActions {
		if b >= 0 {
			held[a] = p.Buttons[b]
		}
	}
	for i := range t.nav {
		held[i] = held[i] || t.nav[i] // view.ActionUp … ActionRight sind 0 … 3
	}
	return held
}

// anyKey meldet, ob eine der Tasten ks gehalten ist.
func anyKey(k *KeyState, ks []ebiten.Key) bool {
	for _, x := range ks {
		if k[x] {
			return true
		}
	}
	return false
}

// keyCommand liefert den Befehl der Tastatur-Hälfte half; beide Richtungen zugleich heben sich auf.
func keyCommand(half int, k *KeyState) view.Command {
	l := &halves[half]
	var c view.Command
	if anyKey(k, l.left) {
		c.Move--
	}
	if anyKey(k, l.right) {
		c.Move++
	}
	c.Sprint, c.Pay, c.Attack = anyKey(k, l.sprint), anyKey(k, l.pay), anyKey(k, l.attack)
	for i, ks := range l.skills {
		if anyKey(k, ks) {
			c.Skill = i + 1
			break
		}
	}
	return c
}

// keyHeld liefert die gehaltenen Menü-Tasten der Tastatur-Hälfte half.
func keyHeld(half int, k *KeyState) [actionCount]bool {
	var held [actionCount]bool
	for a, ks := range halves[half].actions {
		held[a] = anyKey(k, ks)
	}
	return held
}

// step wertet die gehaltenen Menü-Tasten eines Updates aus und hängt die Aktionen an out: Flanke beim Drücken,
// Wiederholung gehaltener Richtungen. pad schaltet die Kombi View + Menu ein (Menu und View erst beim Loslassen).
// Das erste Update eines Geräts löst nichts aus; dort schon gehaltene Tasten wirken erst nach dem Loslassen.
func (t *tracker) step(held *[actionCount]bool, pad bool, out []view.Action) []view.Action {
	if !t.started {
		t.started, t.prev, t.ignore = true, *held, *held
		return out
	}
	if pad {
		out = t.padChord(held, out)
	}
	for a := range actionCount {
		if t.fires(view.Action(a), held[a], pad) {
			out = append(out, view.Action(a))
		}
	}
	return out
}

// fires aktualisiert Haltedauer und Sperre der Aktion a und meldet, ob sie in diesem Update auslöst.
func (t *tracker) fires(a view.Action, held, pad bool) bool {
	pressed := held && !t.prev[a]
	t.prev[a] = held
	switch {
	case !held:
		t.hold[a], t.ignore[a] = 0, false
		return false
	case pressed:
		t.hold[a] = 1
	default:
		t.hold[a]++
	}
	if t.ignore[a] || (pad && (a == view.ActionMenu || a == view.ActionView)) {
		return false
	}
	if pressed {
		return true
	}
	h := t.hold[a] - repeatDelay - 1
	return a <= view.ActionRight && h >= 0 && h%repeatEvery == 0
}

// padChord wertet View und Menu eines Gamepads aus: beide gehalten und eine gerade gedrückt = ActionHome; sonst Menu
// beim Loslassen nach kurzem Druck, View beim Loslassen. Nach der Kombi lösen beide bis zum Loslassen nichts aus.
func (t *tracker) padChord(held *[actionCount]bool, out []view.Action) []view.Action {
	v, m := held[view.ActionView], held[view.ActionMenu]
	pv, pm := t.prev[view.ActionView], t.prev[view.ActionMenu]
	if HomeChord(v, m, v && !pv, m && !pm) {
		out = append(out, view.ActionHome)
		t.chord = true
	}
	if pm && !m && !t.chord && !t.ignore[view.ActionMenu] && t.hold[view.ActionMenu] <= menuShort {
		out = append(out, view.ActionMenu)
	}
	if pv && !v && !t.chord && !t.ignore[view.ActionView] {
		out = append(out, view.ActionView)
	}
	if !v && !m {
		t.chord = false
	}
	return out
}

// HomeChord meldet „View + Menu gemeinsam“ (ActionHome): beide gehalten und mindestens eine in diesem Update gedrückt,
// damit Halten nicht wiederholt auslöst.
func HomeChord(heldView, heldMenu, justView, justMenu bool) bool {
	return heldView && heldMenu && (justView || justMenu)
}
