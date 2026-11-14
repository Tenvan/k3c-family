package menu

import (
	"errors"
	"slices"
	"testing"

	"k3c/native/k3c-game/view"
)

// fakeSession ist eine Schein-view.Session: Spielstände im Speicher, Fehler je Methode setzbar, Aufrufe gezählt.
type fakeSession struct {
	saves    []view.SaveInfo
	listErr  error
	newErr   error
	loadErr  error
	delErr   error
	leaveErr error
	closeErr error
	setErr   error
	active   bool
	news     []view.NewGame
	loads    []string
	devs     [][]view.DeviceID
	conts    int
	deleted  []string
	left     []int
	closed   int
	settings []view.Settings
	cur      view.Settings // aktuelle Einstellungen; Null-Wert = Standard
	seats    []view.Seat
}

var errFake = errors.New("Schein-Fehler")

func (s *fakeSession) New(g view.NewGame) error {
	s.news = append(s.news, g)
	if s.newErr == nil {
		s.active = true
	}
	return s.newErr
}

func (s *fakeSession) Continue(d []view.DeviceID) error {
	s.conts++
	s.devs = append(s.devs, d)
	for _, e := range s.saves {
		if e.Loadable {
			s.active = true
			return nil
		}
	}
	return errors.New("Kein ladbarer Spielstand vorhanden")
}

func (s *fakeSession) Load(name string, d []view.DeviceID) error {
	s.loads = append(s.loads, name)
	s.devs = append(s.devs, d)
	if s.loadErr == nil {
		s.active = true
	}
	return s.loadErr
}

func (s *fakeSession) List() ([]view.SaveInfo, error) { return slices.Clone(s.saves), s.listErr }

func (s *fakeSession) Delete(name string) error {
	if s.delErr != nil {
		return s.delErr
	}
	s.deleted = append(s.deleted, name)
	s.saves = slices.DeleteFunc(s.saves, func(e view.SaveInfo) bool { return e.Name == name })
	return nil
}

func (s *fakeSession) Save() error { return nil }
func (s *fakeSession) Close() error {
	s.closed++
	s.active = false
	return s.closeErr
}
func (s *fakeSession) Active() bool                    { return s.active }
func (s *fakeSession) Pause(bool)                      {}
func (s *fakeSession) Join(view.DeviceID) (int, error) { return 0, nil }
func (s *fakeSession) Leave(p int) error               { s.left = append(s.left, p); return s.leaveErr }
func (s *fakeSession) Rebind(int, view.DeviceID) error { return nil }
func (s *fakeSession) Learn(int, string) error         { return nil }
func (s *fakeSession) Respec(int) error                { return nil }
func (s *fakeSession) SetSettings(v view.Settings) error {
	s.settings = append(s.settings, v)
	s.cur = v
	return s.setErr
}

func (s *fakeSession) Settings() view.Settings {
	if s.cur == (view.Settings{}) {
		return view.DefaultSettings()
	}
	return s.cur
}

func (s *fakeSession) Seats() []view.Seat { return s.seats }

var (
	keys1 = view.DeviceID{Kind: view.DeviceKeyboard}
	pad1  = view.DeviceID{Kind: view.DeviceGamepad, Index: 0}
	pad2  = view.DeviceID{Kind: view.DeviceGamepad, Index: 1}
	pad3  = view.DeviceID{Kind: view.DeviceGamepad, Index: 2}
)

// press ist die Eingabe eines Updates: Gerät d drückt die Aktionen a.
func press(d view.DeviceID, a ...view.Action) view.Input {
	return view.Input{Devices: []view.DeviceInput{{Device: d, Actions: a}}}
}

// stepThrough gibt die Eingaben nacheinander (je ein Update) und liefert den letzten Schritt.
func stepThrough(sc view.Screen, s view.Session, ins ...view.Input) view.Step {
	var st view.Step
	for _, in := range ins {
		st = sc.Update(in, s)
	}
	return st
}

// hasCue meldet, ob der Schritt einen Cue der Art k trägt.
func hasCue(st view.Step, k view.CueKind) bool {
	return slices.ContainsFunc(st.Cues, func(c view.Cue) bool { return c.Kind == k })
}

func save(name string, day int, ok bool) view.SaveInfo {
	e := view.SaveInfo{Name: name, Day: day, Depths: []int{0}, Loadable: ok}
	if !ok {
		e.Problem = "Spielstand nicht ladbar: Version 3"
	}
	return e
}

func newMenus() *menus { return New(nil, nil).(*menus) }

func TestFakeSessionFulfilsContract(t *testing.T) {
	var _ view.Session = (*fakeSession)(nil)
}
