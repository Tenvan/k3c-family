package game

// Anzeige-Modus (B-826): Fenster mit Rahmen, rahmenloses Fenster in Größe des Monitors oder Vollbild. Die Optionen und
// F bzw. der rechte Stick ändern view.Settings.Window über die Session; applyWindow wendet es im nächsten Schritt an.

import (
	"cmp"
	"log/slog"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// toggledWindow ist der Modus nach F bzw. rechtem Stick: Vollbild wird Fenster, alles andere Vollbild.
func toggledWindow(cur string) string {
	if cur == view.WindowFullscreen {
		return view.WindowFramed
	}
	return view.WindowFullscreen
}

// toggleWindow schaltet bei ActionFullscreen eines Geräts (Menü und Spiel) zwischen Vollbild und Fenster um
// (Session.SetSettings). Nicht im Aufnahme-Modus und nicht vor dem ersten Frame (dessen Settings sind noch leer, sonst
// gingen Lautstärken und Sprache verloren).
func (g *Game) toggleWindow(in view.Input) {
	if g.args.Shots != "" || g.frame.Settings.Language == "" {
		return
	}
	if _, ok := in.Has(view.ActionFullscreen); !ok {
		return
	}
	s := g.frame.Settings
	s.Window = toggledWindow(s.Window)
	if err := g.sim.SetSettings(s); err != nil {
		slog.Warn("💥 Einstellungen nicht gespeichert", "err", err)
	}
}

// applyWindow setzt den Anzeige-Modus nach den Einstellungen, sobald er sich ändert; nicht im Aufnahme-Modus.
func (g *Game) applyWindow() {
	w := g.frame.Settings.Window
	if g.shots != nil || g.args.Shots != "" || w == g.window {
		return
	}
	if w == view.WindowFullscreen {
		g.window = w
		ebiten.SetFullscreen(true)
		slog.Info("🚦 Anzeige", "modus", "vollbild")
		return
	}
	if ebiten.IsFullscreen() {
		ebiten.SetFullscreen(false) // wirkt erst zum nächsten Bild: Größe und Lage setzt das nächste Update
		return
	}
	g.window = w
	if ebiten.IsWindowMaximized() || ebiten.IsWindowMinimized() {
		ebiten.RestoreWindow()
	}
	mw, mh := ebiten.Monitor().Size()
	x, y, ww, wh := windowRect(w, mw, mh, g.args.W, g.args.H)
	ebiten.SetWindowDecorated(w != view.WindowBorderless)
	ebiten.SetWindowSize(ww, wh)
	ebiten.SetWindowPosition(x, y)
	slog.Info("🚦 Anzeige", "modus", cmp.Or(w, "fenster"), "breite", ww, "hoehe", wh)
}

// windowRect liefert Lage und Größe des Fensters auf einem Monitor mw × mh: rahmenlos füllt den Monitor, das Fenster hat
// die Startgröße w × h (höchstens 90 % des Monitors, damit die Titelleiste sichtbar bleibt) und steht mittig.
func windowRect(mode string, mw, mh, w, h int) (x, y, ww, wh int) {
	if mode == view.WindowBorderless {
		return 0, 0, mw, mh
	}
	ww, wh = min(w, mw*9/10), min(h, mh*9/10)
	return (mw - ww) / 2, (mh - wh) / 2, ww, wh
}
