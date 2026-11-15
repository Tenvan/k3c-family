package sim

import (
	"strings"
	"testing"
)

// B-961 AC-01: Feld `etappe` je Insel, Standardwert Listenplatz + 1, Prüfung lückenlos und nicht fallend.

const harborPlan = `,"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}`

func etappeWorld(fields ...string) []byte {
	var islands []string
	for i, f := range fields {
		islands = append(islands, testIsland(string(rune('a'+i)), harborPlan+f))
	}
	return testWorld(strings.Join(islands, ","))
}

func TestEtappeStandardIstListenplatz(t *testing.T) {
	w, err := ParseWorld(etappeWorld("", "", ""), "test")
	if err != nil {
		t.Fatal(err)
	}
	for i, d := range w.Islands {
		if d.Etappe != i+1 || w.etappeOf(i) != i+1 {
			t.Errorf("Insel %d: Etappe %d", i, d.Etappe)
		}
	}
}

func TestEtappeGeladenUndGeprueft(t *testing.T) {
	w, err := ParseWorld(etappeWorld("", `,"etappe":2`, `,"etappe":2`), "test")
	if err != nil {
		t.Fatal(err)
	}
	if got := []int{w.Islands[0].Etappe, w.Islands[1].Etappe, w.Islands[2].Etappe}; got[0] != 1 || got[1] != 2 || got[2] != 2 {
		t.Errorf("Etappen %v", got)
	}
	cases := []struct {
		want   string
		fields []string
	}{
		{"Insel b: etappe 3 lässt Etappe 2 aus", []string{"", `,"etappe":3`}},
		{"Insel a: etappe 2 lässt Etappe 1 aus", []string{`,"etappe":2`, `,"etappe":2`}},
		{"Insel c: etappe 1 fällt unter die Etappe 2", []string{"", `,"etappe":2`, `,"etappe":1`}},
		{"Insel a: etappe 0 ungültig", []string{`,"etappe":0`}},
	}
	for _, c := range cases {
		if _, err := ParseWorld(etappeWorld(c.fields...), "test"); err == nil || !strings.Contains(err.Error(), c.want) {
			t.Errorf("%v: erwartet %q, Fehler %v", c.fields, c.want, err)
		}
	}
}

// Jede Insel außer der einzigen der letzten Etappe braucht einen Hafen.
func TestEtappeHafenBeiMehrerenInselnDerLetztenEtappe(t *testing.T) {
	raw := testWorld(testIsland("a", harborPlan) + "," + testIsland("b", harborPlan+`,"etappe":2`) + "," + testIsland("c", `,"etappe":2`))
	if _, err := ParseWorld(raw, "test"); err == nil || !strings.Contains(err.Error(), "Insel c: kein Hafen") {
		t.Errorf("letzte Etappe mit zwei Inseln: %v", err)
	}
}
