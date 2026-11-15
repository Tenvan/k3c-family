package game

// Ladeanzeige beim Stufenwechsel (B-810): Ändern sich die Level der Frames, lädt die Schleife die Grafik synchron
// (levels.go). Damit das Fenster nicht ohne Anzeige einfriert, setzt ein Update zuerst „lädt“, Draw zeigt dann die
// Anzeige, und erst das folgende Update lädt. Im Aufnahme- und Messmodus lädt die Schleife sofort wie bisher (das Bild
// hängt nur vom Takt ab); --stall hält dort die Anzeige dauerhaft, um sie aufzunehmen.

import (
	"log/slog"
	"slices"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// loadBatch ist die Zahl der Sheets, die ein Update beim schrittweisen Laden hochlädt (B-949): dazwischen zeichnet Draw
// den Fortschritt.
const loadBatch = 4

// stepLoader ist ein Loader, der in Schritten lädt (art.Library); andere Loader laden in einem Zug.
type stepLoader interface {
	BeginLevels(want map[string][]string) (*art.LevelLoad, error)
}

// deferLoad meldet, ob die Anzeige dem Laden vorangeht (nur im Fenster).
func (g *Game) deferLoad() bool { return g.args.Shots == "" && g.args.Measure.Seconds == 0 }

// Loading meldet, dass die Schleife auf das Laden der Grafik wartet und die Ladeanzeige zeigt.
func (g *Game) Loading() bool { return g.loading }

// syncLevels bringt die geladene Grafik auf den Stand der Stufen im Frame. Im Fenster: erstes Update „lädt“ setzen,
// weiter warten, bis Draw die Anzeige gezeichnet hat, dann laden.
func (g *Game) syncLevels() {
	if g.parts.Art != nil && len(g.frame.Stages) == 0 && !g.args.Stall {
		g.loading = false
		g.abortLoad()
		g.loadLevels() // nur die Menü-Illustrationen, klein und einmal (B-818): ohne Ladeanzeige
		return
	}
	if g.parts.Art == nil || (SameLevels(g.levels, g.frame.Stages) && !g.args.Stall) {
		g.loading = false
		g.abortLoad()
		return
	}
	switch {
	case g.args.Stall:
		g.startLoading()
	case !g.deferLoad():
		g.loadLevels()
	case !g.loading:
		g.startLoading()
	case g.lload != nil && staleLoad(g.stepWant, g.frame.Stages):
		g.abortLoad() // die Stufen wechselten während des Ladens: von vorn mit den neuen Leveln
		g.beginStepLoad()
	case g.lload != nil:
		g.stepLoading()
	case g.shown:
		if !g.beginStepLoad() {
			g.loadLevels()
			g.loading = false
		}
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
	g.abortLoad()
	g.loadScr.label = ""
	g.loadMsg = newNotice("Lädt …", "", "Stufe: "+strings.Join(names, ", "))
	slog.Info("⏳ Grafik wird geladen", "level", names)
}

// drawLoading zeichnet die Ladeanzeige und merkt, dass sie zu sehen war (nur dieses Flag ändert Draw: ohne gezeichnete
// Anzeige lädt das nächste Update nicht).
func (g *Game) drawLoading(screen *ebiten.Image) {
	p := 0.0
	if g.lload != nil {
		p = g.lload.Progress()
	}
	lib, _ := g.parts.Art.(*art.Library)
	if !g.loadScr.draw(screen, lib, p) {
		g.loadMsg.draw(screen)
	}
	g.shown = true
}

// abortLoad bricht ein laufendes schrittweises Laden ab und gibt dessen Sheets frei.
func (g *Game) abortLoad() {
	g.lload.Abort()
	g.lload = nil
}

// staleLoad meldet, ob die Level der Stufen nicht mehr die Level des laufenden Ladens want sind.
func staleLoad(want map[string][]string, stages []view.Stage) bool {
	n := 0
	for i, st := range stages {
		if _, ok := want[st.Level]; !ok {
			return true
		}
		if !slices.ContainsFunc(stages[:i], func(o view.Stage) bool { return o.Level == st.Level }) {
			n++
		}
	}
	return n != len(want)
}

// beginStepLoad plant das schrittweise Laden der Level aus den Stufen; false, wenn der Loader nur in einem Zug lädt.
func (g *Game) beginStepLoad() bool {
	sl, ok := g.parts.Art.(stepLoader)
	if !ok {
		return false
	}
	g.stepWant = WantedAssets(g.frame.Stages, g.parts.Assets)
	ld, err := sl.BeginLevels(g.stepWant)
	if err != nil {
		g.loading = false
		g.fail(err)
		return true
	}
	g.lload = ld
	g.stepLoading()
	return true
}

// stepLoading lädt den nächsten Schritt; beim Ende gilt der neue Stand der Level.
func (g *Game) stepLoading() {
	done, err := g.lload.Step(loadBatch)
	switch {
	case err != nil:
		g.lload, g.loading = nil, false
		g.fail(err)
	case done:
		g.lload, g.loading = nil, false
		g.setLevels(g.stepWant)
	}
}
