package game

// Fehlerzustand (B-810): Ein Fehler beim Start oder fehlende Grafik beendet das Programm nicht still, sondern zeigt die
// Meldung im Fenster (Fault); Enter, Esc, Leertaste oder A bzw. B am Pad beenden. main reicht den Fehler danach an
// stderr und den Exit-Code weiter (Game.Fault, Fault.Err).

import (
	"errors"
	"fmt"
	"log/slog"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"

	"k3c/native/k3c-game/art"
)

// Fault ist ein ebiten.Game, das nur eine Fehlermeldung zeigt (Fehler vor dem Spiel) und auch dem Game als Zustand dient.
type Fault struct {
	err error
	n   *notice
	ids []ebiten.GamepadID
}

// NewFault baut die Meldung zu err: Titel („Grafik fehlt“ bei art.ErrMissing, sonst „Fehler“), der Text des Fehlers
// (mit Pfad) und der Hinweis zum Beenden.
func NewFault(err error) *Fault {
	title := "Fehler: Das Spiel kann nicht weiterlaufen"
	if errors.Is(err, art.ErrMissing) {
		title = "Grafik fehlt"
	}
	first, more := splitErr(err)
	paras := []string{title, "", first}
	if more > 0 {
		paras = append(paras, fmt.Sprintf("(und %d weitere, alle im Log)", more))
	}
	n := newNotice(paras...)
	n.lines = append(capLines(n.lines, faultMaxLines), "", "Enter, Esc oder A beendet das Spiel.", "Die Meldung steht auch im Log (stderr).")
	return &Fault{err: err, n: n}
}

// faultMaxLines sind die Zeilen, die Titel und Fehlertext höchstens belegen (der Rest steht im Log).
const faultMaxLines = 12

// splitErr trennt einen zusammengesetzten Fehler (errors.Join: eine Meldung je Zeile, z. B. eine je fehlendem Sheet) in
// die erste Meldung und die Zahl der weiteren.
func splitErr(err error) (first string, more int) {
	var lines []string
	for l := range strings.SplitSeq(err.Error(), "\n") {
		if strings.TrimSpace(l) != "" {
			lines = append(lines, l)
		}
	}
	if len(lines) == 0 {
		return err.Error(), 0
	}
	return lines[0], len(lines) - 1
}

// Err liefert den Fehler hinter der Meldung.
func (f *Fault) Err() error { return f.err }

// Lines liefert die Zeilen der Meldung (Tests).
func (f *Fault) Lines() []string { return f.n.lines }

// Update beendet das Programm bei einer Beenden-Taste.
func (f *Fault) Update() error {
	if f.exitPressed() {
		return ebiten.Termination
	}
	return nil
}

// Draw zeichnet die Meldung.
func (f *Fault) Draw(screen *ebiten.Image) { f.n.draw(screen) }

// Layout: Zeichenfläche = Fenster.
func (f *Fault) Layout(w, h int) (int, int) { return w, h }

// exitPressed meldet eine frische Beenden-Taste (Tastatur: Enter, Esc, Leertaste; Pad: A oder B).
func (f *Fault) exitPressed() bool {
	for _, k := range [...]ebiten.Key{ebiten.KeyEnter, ebiten.KeyNumpadEnter, ebiten.KeyEscape, ebiten.KeySpace} {
		if inpututil.IsKeyJustPressed(k) {
			return true
		}
	}
	f.ids = ebiten.AppendGamepadIDs(f.ids[:0])
	for _, id := range f.ids {
		if inpututil.IsStandardGamepadButtonJustPressed(id, ebiten.StandardGamepadButtonRightBottom) ||
			inpututil.IsStandardGamepadButtonJustPressed(id, ebiten.StandardGamepadButtonRightRight) {
			return true
		}
	}
	return false
}

// fail meldet einen Fehler der Schleife: Im Messmodus beendet er das Programm über Update (g.err), sonst zeigt die
// Schleife ihn im Fenster (g.fault; auch im Aufnahme-Modus, damit das Bild die Meldung zeigt, den Exit-Code setzt main
// über Game.Fault). Ein erster Fehler bleibt stehen.
func (g *Game) fail(err error) {
	if g.err != nil || g.fault != nil {
		return
	}
	if g.args.Measure.Seconds > 0 {
		g.err = err
		return
	}
	g.fault = NewFault(err)
	slog.Error("💥 Fehler im Fenster gemeldet", "err", err)
}

// Fault liefert den Fehler, den die Schleife im Fenster gemeldet hat (nil: keiner); main macht daraus stderr und Exit 1.
func (g *Game) Fault() error {
	if g.fault == nil {
		return nil
	}
	return g.fault.err
}

// faultUpdate bedient den Fehlerzustand: Beenden-Taste → Spiel schließen (Session.Close speichert) und beenden.
func (g *Game) faultUpdate() error {
	err := g.fault.Update()
	if err != nil && g.sim.Active() {
		if cerr := g.sim.Close(); cerr != nil {
			slog.Warn("💥 Spiel nicht sauber beendet", "err", cerr)
		}
	}
	return err
}
