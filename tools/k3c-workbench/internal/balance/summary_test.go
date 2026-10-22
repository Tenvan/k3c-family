package balance

import (
	"strings"
	"testing"
)

func summarizeWith(t *testing.T, g Target, ms ...*Metrics) Summary {
	t.Helper()
	ts := &Targets{Seeds: seeds(len(ms)), Targets: []Target{g}}
	ts.Margin.SharePp = 5
	return ts.Summarize(Report{Matrix: Matrix{Seeds: ts.Seeds}, Results: results(ms...)})
}

// BAL2/AC-04: Der Bericht nennt je verletztem Ziel Kennzahl (Name), Szenario und die Seeds zum Nachspielen.
func TestBerichtNenntSeeds(t *testing.T) {
	s := summarizeWith(t, shareTarget(), metricsWith(false, 1), metricsWith(true, 1), metricsWith(true, 1), metricsWith(false, 1))
	md := s.Markdown(nil)
	if s.Pass || !strings.Contains(md, "| x | 2 Spieler, saver, Tiefe 0, 5 Tage | 50,0 % | 70–90 % | Fail (verletzt) | 2, 3 |") {
		t.Errorf("Zeile fehlt:\n%s", md)
	}
	if !strings.Contains(md, "Keine Baseline") {
		t.Error("Hinweis auf fehlende Baseline fehlt")
	}
	if b, err := s.JSON(); err != nil || !strings.Contains(string(b), `"failSeeds"`) || !strings.Contains(string(b), `"pass": false`) {
		t.Errorf("JSON: %v %s", err, b)
	}
}

// Ein abgebrochener Lauf erscheint mit seinem Seed als ungültig.
func TestBerichtUngueltigMitSeed(t *testing.T) {
	md := summarizeWith(t, shareTarget(), metricsWith(false, 1), nil).Markdown(nil)
	if !strings.Contains(md, "Fail (ungültig)") || !strings.Contains(md, "ungültig: 2") {
		t.Errorf("ungültiger Lauf fehlt:\n%s", md)
	}
}

// BAL2/AC-05: Eine Wertänderung (hier die Untergrenze 70 → 85 %, als Test-Option statt Änderung an data/) lässt das
// Ziel von ok nach verletzt kippen; der Vergleich nennt es, ein unverändertes Ziel nicht.
func TestVergleichNenntGekippte(t *testing.T) {
	var ms []*Metrics
	for i := range 20 {
		ms = append(ms, metricsWith(i >= 16, 1)) // 80 % halten
	}
	y := shareTarget()
	y.Name = "y"
	base := summarizeWith(t, shareTarget(), ms...)
	base.Targets = append(base.Targets, summarizeWith(t, y, ms...).Targets...)
	if base.Targets[0].Status != StatusOK {
		t.Fatalf("Baseline %s", base.Targets[0].Status)
	}
	g := shareTarget()
	g.Lower, g.Upper = f(85), f(100)
	cur := summarizeWith(t, g, ms...)
	cur.Targets = append(cur.Targets, summarizeWith(t, y, ms...).Targets...)
	ch := Compare(base, cur)
	if len(ch) != 1 || ch[0].Name != "x" || ch[0].Before != StatusOK || ch[0].After != StatusViolated || !ch[0].Tipped {
		t.Fatalf("Änderungen: %+v", ch)
	}
	if md := cur.Markdown(&base); !strings.Contains(md, "- x: ok → verletzt **gekippt**") || strings.Contains(md, "- y:") {
		t.Errorf("Markdown:\n%s", md)
	}
	if len(Compare(base, base)) != 0 {
		t.Error("gleiche Berichte dürfen nichts melden")
	}
}

// Baseline lesen und schreiben; neues bzw. entfallenes Ziel.
func TestBaselineLesen(t *testing.T) {
	s := summarizeWith(t, shareTarget(), metricsWith(false, 1), metricsWith(false, 1))
	raw, _ := s.JSON()
	back, err := ReadSummary(raw)
	if err != nil || len(back.Targets) != 1 || back.Targets[0].Status != s.Targets[0].Status {
		t.Fatalf("%v %+v", err, back)
	}
	if ch := Compare(Summary{}, s); len(ch) != 1 || ch[0].Before != "neu" {
		t.Errorf("neu: %+v", ch)
	}
	if ch := Compare(s, Summary{}); len(ch) != 1 || ch[0].After != "entfällt" {
		t.Errorf("entfällt: %+v", ch)
	}
}
