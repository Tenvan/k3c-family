package game

// Schiffsfahrt beim Inselwechsel (B-977): Meldet die Simulation den Wechsel (Ereignis islandSwitch), zeigt die Schleife
// statt des Ladebildschirms die Seekarte im Vollbild (view.VoyageDrawer, hud). Start ist die Insel, die der Wechselpunkt
// im Frame davor als aktuelle nannte (letzte der Abgeschlossenen), Ziel das Ziel des Ereignisses. Während der Fahrt
// taktet die Simulation nicht und keine Spiel-Eingabe wirkt (kein Befehl, kein Overlay, keine Pause, kein Beitritt; nur
// der Vollbild-Wechsel bleibt). Die Zeit zählt in Takten der Schleife; der letzte Frame samt seiner Uhr steht, damit
// Hinweise und Banner nicht hinter der Karte ablaufen (die Ankunft blendet erst nach der Fahrt ein, R6). Die Fahrt endet frühestens nach voyageSeconds und erst, wenn die Grafik der
// Zielinsel geladen ist. Ohne Zeichner, ohne Assets, im Aufnahme- und im Messmodus bleibt der Ladebildschirm. Laden
// eines Spielstands und neues Spiel melden kein islandSwitch und zeigen ihn wie bisher. Die Schleife rechnet dabei
// keine Regel: Wann gewechselt wird und wohin, entscheidet die Simulation.

import (
	"log/slog"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// voyageSeconds ist die Fahrzeit (B-977/R3: Startwert, vorläufig bis zur Abstimmung am TV).
const voyageSeconds = 3.0

// voyage ist der Zustand der Fahrt. done, open und choice merkt note vor jedem Takt aus dem Wechselpunkt (Puffer, keine
// Allokation je Takt); v.Done und v.Open teilen die Puffer, solange die Fahrt läuft (note ruht dann).
type voyage struct {
	on     bool
	v      view.Voyage
	ticks  int // Takte seit dem Ablegen
	done   []string
	open   []string
	choice string
}

// note merkt den Stand des Wechselpunkts im Frame f: die Abgeschlossenen (die aktuelle Insel zuletzt), die Ziele und
// das Ziel der Fahrt (Wahl, sonst das erste der Liste).
func (y *voyage) note(f *view.Frame) {
	y.done, y.open, y.choice = y.done[:0], y.open[:0], ""
	for i := range f.Stages {
		w := f.Stages[i].World
		if w == nil || w.IslandSwitch == nil {
			continue
		}
		sw := w.IslandSwitch
		y.done, y.choice = append(y.done, sw.Completed...), sw.Choice
		for i := range sw.Targets {
			y.open = append(y.open, sw.Targets[i].ID)
		}
		if y.choice == "" && len(sw.Targets) > 0 {
			y.choice = sw.Targets[0].ID
		}
		return
	}
}

// switchTarget liefert das Ziel des Ereignisses islandSwitch im Frame f; ok = der Frame trägt das Ereignis.
func switchTarget(f *view.Frame) (string, bool) {
	for i := range f.Events {
		if f.Events[i].Type() == "islandSwitch" {
			to, _ := f.Events[i].Data["target"].(string)
			return to, true
		}
	}
	return "", false
}

// stillAt meldet, ob der Frame f weiter den Wechselpunkt der Insel from zeigt: Der Raum hat die Insel nicht getauscht
// (Fehler beim Bau der Zielinsel), es gibt keine Fahrt.
func stillAt(f *view.Frame, from string) bool {
	for i := range f.Stages {
		if w := f.Stages[i].World; w != nil && w.IslandSwitch != nil {
			c := w.IslandSwitch.Completed
			return len(c) > 0 && c[len(c)-1] == from
		}
	}
	return false
}

// embark beginnt die Fahrt, wenn der Frame dieses Takts den Inselwechsel meldet und sie zu zeichnen ist.
func (g *Game) embark() {
	to, ok := switchTarget(&g.frame)
	if !ok {
		return
	}
	y := &g.voy
	if to == "" {
		to = y.choice
	}
	vd := g.parts.Voyage
	if vd == nil || !g.deferLoad() || len(y.done) == 0 || to == "" || stillAt(&g.frame, y.done[len(y.done)-1]) {
		return
	}
	y.v = view.Voyage{From: y.done[len(y.done)-1], To: to, Done: y.done, Open: y.open, Duration: voyageSeconds}
	if !vd.VoyageReady(&g.frame, &y.v) {
		return // Assets fehlen (R5): Ladebildschirm wie bisher
	}
	y.on, y.ticks = true, 0
	slog.Info("🚀 Schiffsfahrt", "von", y.v.From, "nach", y.v.To)
}

// sail ist der Takt der Schleife während der Fahrt: Die Simulation steht, der letzte Frame gilt samt Uhr weiter;
// Ereignisse und Cues des Takts davor sind gemeldet und entfallen (Geräte-Cues dieses Takts hängt advance an). Von der
// Eingabe wirkt nur der Vollbild-Wechsel (kein Spielzug); die Einstellungen des Frames folgen ihm sofort.
func (g *Game) sail(in view.Input) {
	g.voy.ticks++
	g.frame.Events, g.frame.Cues, g.frame.StepMs = nil, g.frame.Cues[:0], 0
	if _, ok := in.Has(view.ActionFullscreen); ok {
		g.toggleWindow(in)
		g.frame.Settings = g.sim.Settings()
	}
}

// arrive beendet die Fahrt nach der Fahrzeit, sobald nichts mehr lädt; der nächste Takt rechnet die Zielinsel.
func (g *Game) arrive() {
	y := &g.voy
	if !y.on || float64(y.ticks)/TPS < y.v.Duration || g.loading {
		return
	}
	y.on = false
	slog.Info("✅ Schiffsfahrt beendet", "nach", y.v.To, "takte", y.ticks)
}

// drawVoyage zeichnet die Fahrt zur Zeit des Bilds (Takte seit dem Ablegen plus der Anteil bis zum nächsten Takt) und
// merkt wie die Ladeanzeige, dass ein Bild zu sehen war (erst danach lädt das nächste Update). Zeichnet der Zeichner
// nichts, erscheint der Ladebildschirm.
func (g *Game) drawVoyage(screen *ebiten.Image) {
	y := &g.voy
	y.v.Elapsed = max(0, (float64(y.ticks)-1+g.blend.alpha())/TPS)
	if !g.parts.Voyage.DrawVoyage(screen, &g.frame, &y.v) {
		if g.loading && g.loadMsg != nil {
			g.drawLoading(screen)
		} else {
			screen.Fill(clearColor)
		}
	}
	g.shown = true
}
