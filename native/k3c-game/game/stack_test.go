package game

import (
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

type named string

func (named) Update(view.Input, view.Session) view.Step  { return view.Step{} }
func (named) Draw(*ebiten.Image, *view.Frame, view.Cell) {}

func mainScreen() view.Screen { return named("main") }

func TestStackApply(t *testing.T) {
	var s Stack
	s.Push(named("main"))
	if m := s.Apply(view.Step{Kind: view.StepPush, Next: named("laden")}, false, mainScreen); m != ModeMenu || s.Top() != named("laden") || s.Len() != 2 {
		t.Fatalf("Push: %v %v", m, s.Top())
	}
	if m := s.Apply(view.Step{Kind: view.StepReplace, Next: named("löschen")}, false, mainScreen); m != ModeMenu || s.Top() != named("löschen") || s.Len() != 2 {
		t.Fatalf("Replace: %v %v", m, s.Top())
	}
	if m := s.Apply(view.Step{Kind: view.StepPop}, false, mainScreen); m != ModeMenu || s.Top() != named("main") {
		t.Fatalf("Pop: %v %v", m, s.Top())
	}
	if m := s.Apply(view.Step{Kind: view.StepPlay}, true, mainScreen); m != ModePlay || s.Len() != 0 {
		t.Fatalf("Play: %v %d", m, s.Len())
	}
	if m := s.Apply(view.Step{Kind: view.StepQuit}, true, mainScreen); m != ModeQuit {
		t.Fatalf("Quit: %v", m)
	}
}

// Ohne laufendes Spiel bleibt nie ein leerer Stapel: Pop des letzten Screens öffnet das Hauptmenü.
func TestStackNeverEmptyWithoutGame(t *testing.T) {
	var s Stack
	s.Push(named("pause"))
	if m := s.Apply(view.Step{Kind: view.StepPop}, false, mainScreen); m != ModeMenu || s.Top() != named("main") {
		t.Errorf("leerer Stapel ohne Spiel: %v %v", m, s.Top())
	}
	if m := s.Apply(view.Step{Kind: view.StepPush}, false, mainScreen); m != ModeMenu || s.Len() != 1 {
		t.Errorf("Push ohne Next: %v %d", m, s.Len())
	}
}
