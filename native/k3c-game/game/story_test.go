package game

import (
	"testing"

	"k3c/native/k3c-game/view"
)

// storyListener merkt je Update, was ein Listener für die Geschichtszeile sieht (B-974).
type storyListener struct {
	clocks   []float64
	loading  []bool
	switches int
}

func (l *storyListener) Listen(f *view.Frame) {
	l.clocks, l.loading = append(l.clocks, f.Clock), append(l.loading, f.Loading)
	for i := range f.Events {
		if f.Events[i].Type() == "islandSwitch" {
			l.switches++
		}
	}
}

// B-974 (Ankunft nach der Fahrt): Listener sehen den Inselwechsel genau einmal; während der Schiffsfahrt steht die Uhr
// des Frames, die Einblendung der Ankunft beginnt also erst danach.
func TestStoryWaitsForVoyage(t *testing.T) {
	g, _, _ := voyGame(&voyDrawer{ready: true}, harbor("b"), switched("b"))
	l := &storyListener{}
	g.parts.Listeners = []view.Listener{l}
	g.advance()
	g.advance()
	if !g.voy.on {
		t.Fatal("keine Fahrt")
	}
	at := g.frame.Clock
	for g.voy.on {
		g.advance()
		if g.voy.on && g.frame.Clock != at {
			t.Fatalf("Uhr läuft während der Fahrt: %v statt %v", g.frame.Clock, at)
		}
	}
	g.advance()
	if l.switches != 1 || g.frame.Clock <= at {
		t.Errorf("Inselwechsel %d Mal gemeldet, Uhr nach der Fahrt %v (davor %v)", l.switches, g.frame.Clock, at)
	}
}

// B-974: Frame.Loading meldet den Listenern die Ladeanzeige (Einblendungen warten so lange).
func TestFrameLoadingForListeners(t *testing.T) {
	g, _ := loadGame()
	src, l := &fixedSource{}, &storyListener{}
	g.src, g.mode, g.parts.Menus, g.parts.Listeners = src, ModePlay, pauseMenus{}, []view.Listener{l}
	g.sim = &voySim{fakeSim: newFakeSim(), frames: []view.Frame{{Seats: []view.Seat{{Player: 0, Device: keys1}}, Stages: stages("wald")}}}
	g.advance() // Stufe neu: Ladeanzeige
	g.shown = true
	g.advance() // lädt
	g.advance()
	if len(l.loading) != 3 || !l.loading[0] || l.loading[2] || g.Loading() {
		t.Errorf("Frame.Loading je Update %v, lädt am Ende %v", l.loading, g.Loading())
	}
}

// holder ist ein Listener, der am Spielende noch etwas zeigt (view.EndHolder), bis left Abfragen verbraucht sind.
type holder struct{ left int }

func (*holder) Listen(*view.Frame) {}

func (h *holder) EndHolding() bool {
	h.left--
	return h.left >= 0
}

// B-974 (Review): Zeigt das HUD am Spielende noch Zeilen der Geschichte, wartet das Hauptmenü darauf, höchstens
// view.EndHoldMax Takte über view.EndDelay hinaus.
func TestEndingWaitsForStory(t *testing.T) {
	for _, c := range []struct{ hold, want int }{{40, view.EndDelay + 40}, {1 << 20, view.EndDelay + view.EndHoldMax}} {
		s := newFakeSim()
		g := playGame(s, &countingOverlay{})
		g.parts.Listeners = []view.Listener{&holder{left: c.hold}}
		endedFrame(g, view.OutcomeWon)
		n := 0
		for s.closed == 0 && n < 10*view.EndHoldMax {
			g.play(view.Input{})
			n++
		}
		if n != c.want || g.mode != ModeMenu {
			t.Errorf("Halten %d: geschlossen nach %d Takten (erwartet %d), Modus %v", c.hold, n, c.want, g.mode)
		}
	}
}
