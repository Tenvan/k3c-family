package game

import "k3c/native/k3c-game/view"

// Mode ist der Zustand der Schleife.
type Mode int

const (
	ModeMenu Mode = iota // ein Screen liegt oben, kein Spiel oder Spiel pausiert
	ModePlay             // Spiel läuft, kein Screen
	ModeQuit             // Programm beenden
)

// Stack ist der Menü-Stapel; oben liegt das letzte Element. Reine Logik, ohne Zeichnen.
type Stack struct {
	screens []view.Screen
}

// Top liefert den obersten Screen (nil bei leerem Stapel).
func (s *Stack) Top() view.Screen {
	if len(s.screens) == 0 {
		return nil
	}
	return s.screens[len(s.screens)-1]
}

// Len liefert die Zahl der Screens.
func (s *Stack) Len() int { return len(s.screens) }

// Push legt sc oben auf.
func (s *Stack) Push(sc view.Screen) { s.screens = append(s.screens, sc) }

// Apply wendet das Ergebnis eines Screen.Update an und liefert den neuen Zustand. active: es läuft ein Spiel.
// Ein leerer Stapel ohne Spiel öffnet main (Hauptmenü), damit nie ein leeres Bild ohne Menü bleibt.
func (s *Stack) Apply(st view.Step, active bool, main func() view.Screen) Mode {
	switch st.Kind {
	case view.StepPush:
		if st.Next != nil {
			s.Push(st.Next)
		}
	case view.StepReplace:
		if st.Next != nil && len(s.screens) > 0 {
			s.screens[len(s.screens)-1] = st.Next
		}
	case view.StepPop:
		if len(s.screens) > 0 {
			s.screens = s.screens[:len(s.screens)-1]
		}
	case view.StepPlay:
		s.screens = nil
	case view.StepQuit:
		return ModeQuit
	}
	if len(s.screens) > 0 {
		return ModeMenu
	}
	if active {
		return ModePlay
	}
	s.Push(main())
	return ModeMenu
}
