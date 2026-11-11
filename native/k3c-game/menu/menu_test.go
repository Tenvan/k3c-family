package menu

import (
	"errors"
	"testing"

	"k3c/native/k3c-game/view"
)

// fakeSession erfüllt view.Session für die Tests; nur New zählt.
type fakeSession struct {
	err  error
	news []view.NewGame
}

func (s *fakeSession) New(g view.NewGame) error        { s.news = append(s.news, g); return s.err }
func (s *fakeSession) Continue() error                 { return nil }
func (s *fakeSession) Load(string) error               { return nil }
func (s *fakeSession) List() ([]view.SaveInfo, error)  { return nil, nil }
func (s *fakeSession) Delete(string) error             { return nil }
func (s *fakeSession) Save() error                     { return nil }
func (s *fakeSession) Close() error                    { return nil }
func (s *fakeSession) Active() bool                    { return len(s.news) > 0 }
func (s *fakeSession) Pause(bool)                      {}
func (s *fakeSession) Join(view.DeviceID) (int, error) { return 0, nil }
func (s *fakeSession) Leave(int) error                 { return nil }

func press(d view.DeviceID, a ...view.Action) view.Input {
	return view.Input{Devices: []view.DeviceInput{{Device: d, Actions: a}}}
}

func TestMainStartsGameWithConfirmingDevice(t *testing.T) {
	pad := view.DeviceID{Kind: view.DeviceGamepad, Index: 1}
	s := &fakeSession{}
	m := New().Main()
	if st := m.Update(press(pad, view.ActionDown), s); st.Kind != view.StepStay || len(s.news) != 0 {
		t.Fatalf("ohne Bestätigen: %+v", st)
	}
	st := m.Update(press(pad, view.ActionConfirm), s)
	if st.Kind != view.StepPlay || len(s.news) != 1 || s.news[0].Devices[0] != pad {
		t.Fatalf("Bestätigen: %+v, %+v", st, s.news)
	}
}

func TestMainStaysOnError(t *testing.T) {
	s := &fakeSession{err: errors.New("Simulation fehlt")}
	st := New().Main().Update(press(view.DeviceID{}, view.ActionConfirm), s)
	if st.Kind != view.StepStay || len(st.Cues) != 1 || st.Cues[0].Kind != view.CueDenied || st.Cues[0].Text != "Simulation fehlt" {
		t.Errorf("Fehler: %+v", st)
	}
}

func TestPauseClosesOnBack(t *testing.T) {
	p := New().Pause()
	if st := p.Update(press(view.DeviceID{}, view.ActionDown), nil); st.Kind != view.StepStay {
		t.Errorf("Pause schließt ohne Taste: %+v", st)
	}
	if st := p.Update(press(view.DeviceID{}, view.ActionBack), nil); st.Kind != view.StepPop {
		t.Errorf("Pause schließt nicht: %+v", st)
	}
}
