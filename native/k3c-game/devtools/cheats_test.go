package devtools

import (
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

var (
	keys1 = view.DeviceID{Kind: view.DeviceKeyboard, Index: 0}
	keys2 = view.DeviceID{Kind: view.DeviceKeyboard, Index: 1}
	pad1  = view.DeviceID{Kind: view.DeviceGamepad, Index: 0}
)

// fakeSession ist eine Session mit Entwickler-Rolle, die die Dev-Aktionen mitschreibt.
type fakeSession struct {
	view.Session
	players []int
	acts    []view.DevAction
}

func (s *fakeSession) Dev(p int, a view.DevAction) error {
	s.players, s.acts = append(s.players, p), append(s.acts, a)
	return nil
}
func (s *fakeSession) Halt(bool)     {}
func (s *fakeSession) Halted() bool  { return false }
func (s *fakeSession) StepOnce()     {}
func (s *fakeSession) Reload() error { return nil }

func press(d view.DeviceID, a ...view.Action) view.DeviceInput {
	return view.DeviceInput{Device: d, Actions: a}
}

func input(ds ...view.DeviceInput) view.Input { return view.Input{Devices: ds} }

// Jede Gruppe hat Einträge, jede Aktion ist eine Aktion des Raums, die Bosse kommen aus data/bosses.json.
func TestCheatGroups(t *testing.T) {
	known := map[string]bool{"gold": true, "material": true, "maxStock": true, "fillStock": true, "cheapBuild": true,
		"buildAll": true, "castleFall": true, "nextPhase": true, "phase": true, "timescale": true, "timescaleAuto": true,
		"grade": true, "night": true, "wave": true, "boss": true, "bossPhase": true, "event": true}
	names := []string{"Vorrat", "Ausbau", "Zeit", "Grad", "Kampf"}
	if len(cheatGroups) != len(names) {
		t.Fatalf("%d Gruppen", len(cheatGroups))
	}
	bosses := 0
	for i, g := range cheatGroups {
		if g.name != names[i] || len(g.entries) == 0 {
			t.Errorf("Gruppe %d: %s mit %d Einträgen", i, g.name, len(g.entries))
		}
		for _, e := range g.entries {
			if !known[e.act.Action] || e.label == "" {
				t.Errorf("%s: %+v", g.name, e)
			}
			if e.act.Action == "boss" {
				bosses++
			}
		}
	}
	if bosses == 0 {
		t.Error("keine Bosse aus data/bosses.json")
	}
}

// openCheats öffnet mit F9 der linken Tastatur-Hälfte das Menü von Spieler 2 (Spieler 1 am Pad).
func openCheats(t *testing.T) (*overlay, *fakeSession, *view.Frame) {
	t.Helper()
	o := New(nil)[0].(*overlay)
	s := &fakeSession{}
	f := &view.Frame{Seats: []view.Seat{{Player: 0, Device: pad1}, {Player: 1, Device: keys1}}}
	st := o.Update(input(press(keys1, view.ActionCheats)), f, s)
	if !st.Consumed[1] || st.Consumed[0] || !o.menus[1].open || o.menus[0].open {
		t.Fatalf("F9: verbraucht %v, offen %v/%v", st.Consumed, o.menus[0].open, o.menus[1].open)
	}
	return o, s, f
}

// AC-03 (B-888): F9 öffnet das Menü des Spielers und verbraucht seine Eingabe, der andere spielt weiter; Bestätigen
// löst für diesen Spieler aus, ein Schalter kehrt seinen Stand um, rechts wechselt die Gruppe.
func TestCheatOverlayPerPlayer(t *testing.T) {
	o, s, f := openCheats(t)
	o.Update(input(press(keys1, view.ActionConfirm)), f, s)
	if len(s.acts) != 1 || s.players[0] != 1 || s.acts[0].Action != "gold" || s.acts[0].Amount != 10 {
		t.Fatalf("Gold: %+v an %v", s.acts, s.players)
	}
	o.Update(input(press(keys1, view.ActionUp, view.ActionConfirm)), f, s) // hoch: letzter Eintrag (Baukosten 1)
	if a := s.acts[1]; a.Action != "cheapBuild" || a.On == nil || !*a.On || !o.toggles["cheapBuild"] {
		t.Fatalf("Schalter: %+v", a)
	}
	o.Update(input(press(keys1, view.ActionRight, view.ActionRight, view.ActionDown, view.ActionConfirm)), f, s)
	if a := s.acts[2]; a.Action != "phase" || a.Phase != "day" {
		t.Errorf("Zeit, zweiter Eintrag: %+v", a)
	}
}

// Erneut F9 schließt (das Update ist noch verbraucht), danach geht die Eingabe wieder ans Spiel.
func TestCheatOverlayCloses(t *testing.T) {
	o, s, f := openCheats(t)
	st := o.Update(input(press(keys1, view.ActionCheats)), f, s)
	if o.menus[1].open || !st.Consumed[1] {
		t.Errorf("zweites F9: offen %v, verbraucht %v", o.menus[1].open, st.Consumed)
	}
	if st = o.Update(input(press(keys1, view.ActionConfirm)), f, s); st.Consumed[1] || len(s.acts) != 0 {
		t.Errorf("geschlossen: verbraucht %v, Aktionen %d", st.Consumed, len(s.acts))
	}
}

// Spielt die linke Tastatur-Hälfte nicht, öffnet F9 das Menü des ersten Spielers an einer Tastatur, sonst des ersten.
func TestCheatKeyFallback(t *testing.T) {
	o := New(nil)[0].(*overlay)
	f := &view.Frame{Seats: []view.Seat{{Player: 0, Device: pad1}, {Player: 1, Device: keys2}}}
	o.Update(input(press(keys1, view.ActionCheats)), f, &fakeSession{})
	if !o.menus[1].open || o.menus[0].open {
		t.Fatalf("Tastatur rechts: %v/%v", o.menus[0].open, o.menus[1].open)
	}
	o = New(nil)[0].(*overlay)
	f.Seats = f.Seats[:1]
	o.Update(input(press(keys1, view.ActionCheats)), f, &fakeSession{})
	if !o.menus[0].open {
		t.Error("nur Pad: Menü von Spieler 1 nicht offen")
	}
	f.Recording = true
	o = New(nil)[0].(*overlay)
	o.Update(input(press(pad1, view.ActionCheats)), f, &fakeSession{})
	if o.menus[0].open {
		t.Error("Aufnahme-Modus: Menü offen")
	}
}

// Ohne Entwickler-Rolle meldet das Menü einen Fehler statt auszulösen; Zeichnen ohne Schrift stürzt nicht ab.
func TestCheatWithoutDevSession(t *testing.T) {
	o := New(nil)[0].(*overlay)
	f := &view.Frame{Seats: []view.Seat{{Player: 0, Device: pad1}}}
	o.Update(input(press(pad1, view.ActionCheats)), f, nil)
	st := o.Update(input(press(pad1, view.ActionConfirm)), f, nil)
	if !o.menus[0].bad || len(st.Cues) == 0 || st.Cues[0].Kind != view.CueDenied {
		t.Errorf("Status %q, Cues %+v", o.menus[0].status, st.Cues)
	}
	dst := ebiten.NewImage(320, 360)
	f.Cells = []view.Cell{{Index: 0, Player: 0, Rect: dst.Bounds()}}
	o.Draw(dst, f, view.WindowCell(dst.Bounds()))
	if got := window(20, 30, 9); got != 16 || window(2, 30, 9) != 0 || window(29, 30, 9) != 21 || window(3, 5, 9) != 0 {
		t.Errorf("Fenster der Einträge: %d", got)
	}
}
