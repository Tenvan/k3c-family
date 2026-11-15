package menu

import (
	"strings"
	"testing"

	"k3c/native/k3c-game/view"
)

// B-944/AC-01: Die Testseite zeigt Tasten, Achsen und die zugeordnete Aktion je Pad; B geht zurück.
func TestPadTestShowsButtonsAxesAction(t *testing.T) {
	old := readPads
	defer func() { readPads = old }()
	readPads = func() []padRaw {
		return []padRaw{{id: 0, name: "Xbox Controller", sdl: "0300", standard: true, buttons: "A LB", axes: "LS 0.50 0 · RS 0 0", raw: "0 4"}}
	}
	sc := newPadTest(newMenus())
	s := &fakeSession{}
	if st := sc.Update(press(pad1, view.ActionConfirm), s); st.Kind != view.StepStay {
		t.Fatalf("A verlässt die Seite: %v", st.Kind)
	}
	sc.fill(view.DefaultLanguage)
	var all []string
	for _, r := range sc.pg.rows[:sc.pg.n] {
		all = append(all, r.label+"|"+r.value)
	}
	text := strings.Join(all, "\n")
	for _, want := range []string{"A LB → Bestätigen", "LS 0.50 0", "Xbox Controller", "roh 0 4"} {
		if !strings.Contains(text, want) {
			t.Errorf("%q fehlt in\n%s", want, text)
		}
	}
	if sc.pg.n > maxRows {
		t.Errorf("%d Zeilen", sc.pg.n)
	}
	if st := sc.Update(press(pad1, view.ActionBack), s); st.Kind != view.StepPop {
		t.Errorf("B: %v, soll zurück", st.Kind)
	}
}

// B-944/AC-01: Testing führt über die Zeile Tasten-Test auf die Seite.
func TestTestingOpensPadTest(t *testing.T) {
	old := readPads
	defer func() { readPads = old }()
	readPads = func() []padRaw { return nil }
	sc := newTesting(newMenus())
	s := &starterSession{}
	sc.Update(view.Input{}, s)
	sc.sel = tstPads
	st := sc.Update(press(pad1, view.ActionConfirm), s)
	if _, ok := st.Next.(*padTestScreen); st.Kind != view.StepPush || !ok {
		t.Fatalf("Schritt %v %T", st.Kind, st.Next)
	}
}
