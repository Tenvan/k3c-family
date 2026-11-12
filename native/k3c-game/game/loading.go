package game

// Ladeanzeige beim Stufenwechsel (B-810): Ändern sich die Level der Frames, lädt die Schleife die Grafik synchron
// (levels.go). Damit das Fenster nicht ohne Anzeige einfriert, setzt ein Update zuerst „lädt“, Draw zeigt dann die
// Anzeige, und erst das folgende Update lädt. Im Aufnahme- und Messmodus lädt die Schleife sofort wie bisher (das Bild
// hängt nur vom Takt ab); --stall hält dort die Anzeige dauerhaft, um sie aufzunehmen.

import (
	"log/slog"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"
)

// deferLoad meldet, ob die Anzeige dem Laden vorangeht (nur im Fenster).
func (g *Game) deferLoad() bool { return g.args.Shots == "" && g.args.Measure.Seconds == 0 }

// Loading meldet, dass die Schleife auf das Laden der Grafik wartet und die Ladeanzeige zeigt.
func (g *Game) Loading() bool { return g.loading }

// syncLevels bringt die geladene Grafik auf den Stand der Stufen im Frame. Im Fenster: erstes Update „lädt“ setzen,
// weiter warten, bis Draw die Anzeige gezeichnet hat, dann laden.
func (g *Game) syncLevels() {
	if g.parts.Art == nil || (SameLevels(g.levels, g.frame.Stages) && !g.args.Stall) {
		g.loading = false
		return
	}
	switch {
	case g.args.Stall:
		g.startLoading()
	case !g.deferLoad():
		g.loadLevels()
	case !g.loading:
		g.startLoading()
	case g.shown:
		g.loadLevels()
		g.loading = false
	}
}

// startLoading setzt „lädt“ und baut den Text der Anzeige (einmal je Wechsel).
func (g *Game) startLoading() {
	if g.loading {
		return
	}
	g.loading, g.shown = true, false
	names := make([]string, 0, len(g.frame.Stages))
	for _, st := range g.frame.Stages {
		names = append(names, st.Level)
	}
	g.loadMsg = newNotice("Lädt …", "", "Stufe: "+strings.Join(names, ", "))
	slog.Info("⏳ Grafik wird geladen", "level", names)
}

// drawLoading zeichnet die Ladeanzeige und merkt, dass sie zu sehen war (nur dieses Flag ändert Draw: ohne gezeichnete
// Anzeige lädt das nächste Update nicht).
func (g *Game) drawLoading(screen *ebiten.Image) {
	g.loadMsg.draw(screen)
	g.shown = true
}
