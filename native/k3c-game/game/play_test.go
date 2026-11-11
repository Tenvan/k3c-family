package game

import (
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// fakeSim erfüllt Sim und schreibt mit, was die Schleife verlangt.
type fakeSim struct {
	cmds    map[int]view.Command
	rebinds []Rebind
	joins   []view.DeviceID
	paused  bool
	learned []string
}

func newFakeSim() *fakeSim { return &fakeSim{cmds: map[int]view.Command{}} }

func (s *fakeSim) New(view.NewGame) error              { return nil }
func (s *fakeSim) Continue([]view.DeviceID) error      { return nil }
func (s *fakeSim) Load(string, []view.DeviceID) error  { return nil }
func (s *fakeSim) List() ([]view.SaveInfo, error)      { return nil, nil }
func (s *fakeSim) Delete(string) error                 { return nil }
func (s *fakeSim) Save() error                         { return nil }
func (s *fakeSim) Close() error                        { return nil }
func (s *fakeSim) Active() bool                        { return true }
func (s *fakeSim) Pause(on bool)                       { s.paused = on }
func (s *fakeSim) Join(d view.DeviceID) (int, error)   { s.joins = append(s.joins, d); return 1, nil }
func (s *fakeSim) Leave(int) error                     { return nil }
func (s *fakeSim) Respec(int) error                    { return nil }
func (s *fakeSim) SetSettings(view.Settings) error     { return nil }
func (s *fakeSim) Step() view.Frame                    { return view.Frame{} }
func (s *fakeSim) Command(p int, c view.Command) error { s.cmds[p] = c; return nil }
func (s *fakeSim) Learn(p int, skill string) error     { s.learned = append(s.learned, skill); return nil }
func (s *fakeSim) Rebind(p int, d view.DeviceID) error {
	s.rebinds = append(s.rebinds, Rebind{p, d})
	return nil
}

// skillMenu ist ein Overlay wie das Skill-Menü: View öffnet bzw. schließt es für den Spieler des Geräts; offen nimmt es
// dessen Eingabe und lernt bei Bestätigen.
type skillMenu struct{ open [view.MaxPlayers]bool }

func (m *skillMenu) Update(in view.Input, f *view.Frame, s view.Session) view.OverlayStep {
	var st view.OverlayStep
	for _, seat := range f.Seats {
		di, _ := in.Of(seat.Device)
		for _, a := range di.Actions {
			switch {
			case a == view.ActionView:
				m.open[seat.Player] = !m.open[seat.Player]
			case a == view.ActionConfirm && m.open[seat.Player]:
				_ = s.Learn(seat.Player, "schild")
				st.Cues = append(st.Cues, view.Cue{Kind: view.CueConfirm, Player: seat.Player})
			}
		}
		st.Consumed[seat.Player] = m.open[seat.Player]
	}
	return st
}

type pauseMenus struct{}

func (pauseMenus) Main() view.Screen  { return nil }
func (pauseMenus) Pause() view.Screen { return pauseScreen{} }

type pauseScreen struct{}

func (pauseScreen) Update(view.Input, view.Session) view.Step  { return view.Step{} }
func (pauseScreen) Draw(*ebiten.Image, *view.Frame, view.Cell) { /* Testattrappe */ }

func playGame(s *fakeSim, o ...view.Overlay) *Game {
	g := &Game{sim: s, mode: ModePlay, parts: Parts{Menus: pauseMenus{}, Overlays: o}}
	g.frame.Seats = []view.Seat{{Player: 0, Device: keys1}, {Player: 1, Device: pad3}}
	return g
}

func press(d view.DeviceID, c view.Command, a ...view.Action) view.DeviceInput {
	return view.DeviceInput{Device: d, Command: c, Actions: a}
}

// Ein offenes Overlay nimmt die Eingabe seines Spielers (leerer Befehl, Menu öffnet keine Pause), der andere spielt
// weiter; das Overlay lernt über die Session, seine Cues kommen in den Frame.
func TestPlayOverlayConsumes(t *testing.T) {
	s := newFakeSim()
	g := playGame(s, &skillMenu{})
	run := view.Command{Move: 1}
	g.play(view.Input{Devices: []view.DeviceInput{press(keys1, run, view.ActionView), press(pad3, run)}})
	if s.cmds[0] != (view.Command{}) || s.cmds[1] != run {
		t.Errorf("Befehle mit offenem Menü: %+v", s.cmds)
	}
	g.play(view.Input{Devices: []view.DeviceInput{press(keys1, run, view.ActionConfirm, view.ActionMenu), press(pad3, run)}})
	if g.mode != ModePlay || s.paused || len(s.learned) != 1 || len(g.cues) != 1 || len(s.joins) != 0 {
		t.Errorf("Menu im Overlay: Modus %v, pausiert %v, gelernt %v, Cues %+v, Beitritte %v", g.mode, s.paused, s.learned, g.cues, s.joins)
	}
	g.play(view.Input{Devices: []view.DeviceInput{press(keys1, run, view.ActionView), press(pad3, run)}})
	if s.cmds[0] != run {
		t.Errorf("nach dem Schließen: %+v", s.cmds[0])
	}
}

// ActionHome öffnet im Spiel das Pause-Menü wie ActionMenu.
func TestPlayHomePauses(t *testing.T) {
	s := newFakeSim()
	g := playGame(s)
	g.play(view.Input{Devices: []view.DeviceInput{press(pad3, view.Command{}, view.ActionHome)}})
	if g.mode != ModeMenu || !s.paused || g.stack.Len() != 1 {
		t.Errorf("Home: Modus %v, pausiert %v, Stapel %d", g.mode, s.paused, g.stack.Len())
	}
}

// Verliert Spieler 2 sein Pad, übernimmt ein neues Pad den Platz (Rebind statt Beitritt); ein weiteres freies Gerät mit
// Bestätigen tritt bei.
func TestPlayRebindsLostPad(t *testing.T) {
	s := newFakeSim()
	g := playGame(s)
	g.devs.observe(devicesIn([]view.DeviceID{keys1, keys2, pad3}), g.frame.Seats, nil)
	in := view.Input{Devices: []view.DeviceInput{press(keys1, view.Command{}), press(keys2, view.Command{}, view.ActionConfirm), press(pad5, view.Command{}, view.ActionConfirm)}}
	g.cues = g.devs.observe(in, g.frame.Seats, g.cues)
	g.play(in)
	if len(s.rebinds) != 1 || s.rebinds[0] != (Rebind{1, keys2}) {
		t.Errorf("Neuzuordnung %+v", s.rebinds)
	}
	if len(s.joins) != 1 || s.joins[0] != pad5 {
		t.Errorf("Beitritte %v", s.joins)
	}
	if len(g.cues) != 2 || g.cues[0].Kind != view.CueDeviceConnected || g.cues[1].Kind != view.CueDeviceLost {
		t.Errorf("Cues %+v", g.cues)
	}
}
