package balance

import (
	"strings"
	"testing"
)

// wideGold: das Testziel „Gold zur Dämmerung von Tag 1“ (goldTargets) mit weitem Korridor 105–200. Mit Startgold 100
// liegt der Wert bei 95 (verletzt), mit 150 im Korridor, mit 250 darüber.
func wideGold(t *testing.T) *Targets {
	t.Helper()
	ts := goldTargets(t)
	ts.Targets[0].Lower, ts.Targets[0].Upper = f(105), f(200)
	return ts
}

const startGold = "economy.json:purse.startGold=50..250"

// WZ28.6/AC-06: Sweep über 3 Punkte findet einen Punkt mit Score 0, der Vorschlag hat Begründung und Diff mit dem
// neuen Wert; data/ bleibt unverändert.
func TestTuneSweepVorschlagMitDiff(t *testing.T) {
	before := dataSum(t)
	var progress []int
	tn, err := wideGold(t).Tune(TuneOptions{Mode: TuneSweep, Params: []string{startGold}, Steps: 3,
		Progress: func(done, total int) { progress = append(progress, done*10+total) }})
	if err != nil {
		t.Fatal(err)
	}
	if len(tn.Points) != 3 || tn.Points[0].Score == 0 || tn.Points[1].Score != 0 || tn.Points[2].Score == 0 {
		t.Fatalf("punkte: %+v", tn.Points)
	}
	if tn.Best.Values[0] != 150 || !tn.Best.Pass || tn.Params[0].Base != 100 || !tn.Params[0].Int {
		t.Fatalf("bester: %+v, params %+v", tn.Best, tn.Params)
	}
	checkProposal(t, tn)
	if len(progress) != 3 || progress[2] != 33 {
		t.Errorf("fortschritt: %v", progress)
	}
	if dataSum(t) != before {
		t.Error("data/ wurde geändert")
	}
}

// checkProposal: Diff mit altem und neuem Wert, Begründung mit Treffern und Werten.
func checkProposal(t *testing.T, tn Tuning) {
	t.Helper()
	if !strings.Contains(tn.Diff, `+  "purse": { "startGold": 150, "maxGold": 100 },`) ||
		!strings.Contains(tn.Diff, `-  "purse": { "startGold": 100,`) || !strings.Contains(tn.Diff, "--- data/economy.json") {
		t.Errorf("diff:\n%s", tn.Diff)
	}
	if !strings.Contains(tn.Reason, "Alle 1 Korridore getroffen") || !strings.Contains(tn.Reason, "= 150 (bisher 100)") {
		t.Errorf("begründung: %s", tn.Reason)
	}
	t.Logf("reason: %s\ndiff:\n%s", tn.Reason, tn.Diff)
}

// Suche mit Budget 4: höchstens 4 Auswertungen, Abbruch beim ersten Punkt mit Score 0.
func TestTuneSucheBudget(t *testing.T) {
	tn, err := wideGold(t).Tune(TuneOptions{Mode: TuneSearch, Params: []string{startGold}, Budget: 4})
	if err != nil {
		t.Fatal(err)
	}
	if len(tn.Points) > 4 || tn.Best.Score != 0 || tn.Points[len(tn.Points)-1].Score != 0 {
		t.Fatalf("suche: %+v", tn.Points)
	}
	if tn.Points[0].Values[0] != 50 || tn.Points[1].Values[0] != 250 {
		t.Errorf("erst die Enden: %+v", tn.Points)
	}
	// Ohne Treffer bleibt die Begründung bei den verfehlten Korridoren.
	tn, err = wideGold(t).Tune(TuneOptions{Mode: TuneSearch, Params: []string{"economy.json:purse.startGold=10..40"}, Budget: 2})
	if err != nil || len(tn.Points) != 2 || tn.Best.Pass || !strings.Contains(tn.Reason, "Verfehlt: Gold Tag 1") {
		t.Fatalf("%v %+v", err, tn)
	}
}

// Zwei Parameter: Raster und Verfeinerung bleiben im Budget.
func TestTuneZweiParameter(t *testing.T) {
	tn, err := wideGold(t).Tune(TuneOptions{Mode: TuneSearch, Steps: 2, Budget: 6,
		Params: []string{"economy.json:purse.startGold=10..40", "economy.json:dawnGoldPerPlayer=1..9"}})
	if err != nil || len(tn.Points) != 6 || len(tn.Best.Values) != 2 {
		t.Fatalf("%v %+v", err, tn.Points)
	}
}

func TestTuneUngueltig(t *testing.T) {
	bad := []TuneOptions{
		{Mode: "raten", Params: []string{startGold}},
		{Mode: TuneSweep},
		{Mode: TuneSweep, Params: []string{startGold, startGold, startGold}},
		{Mode: TuneSweep, Params: []string{startGold}, Steps: 22},
		{Mode: TuneSweep, Params: []string{"economy.json:purse.startGold=9..1"}},
		{Mode: TuneSweep, Params: []string{"economy.json:purse.startGold=1"}},
		{Mode: TuneSweep, Params: []string{"economy.json:purse=1..2"}},
		{Mode: TuneSweep, Params: []string{"fehlt.json:a=1..2"}},
	}
	for _, o := range bad {
		if _, err := wideGold(t).Tune(o); err == nil {
			t.Errorf("%+v: Fehler erwartet", o)
		}
	}
}

// locateNumber findet Zahlen in Listen und verschachtelten Objekten; der Diff nennt nur geänderte Zeilen.
func TestTuneDiffVerschachtelt(t *testing.T) {
	raw := []byte("{\n  \"a\": [1, {\"b\": 2.5}],\n  \"c\": {\"d\": 3}\n}\n")
	for path, want := range map[string]string{"a.0": "1", "a.1.b": "2.5", "c.d": "3"} {
		s, e, err := locateNumber(raw, strings.Split(path, "."))
		if err != nil || string(raw[s:e]) != want {
			t.Errorf("%s: %q %v", path, raw[s:e], err)
		}
	}
	if _, _, err := locateNumber(raw, []string{"c"}); err == nil {
		t.Error("objekt als Zahl gefunden")
	}
	p, err := ParseTuneParam("economy.json:dawnGoldPerPlayer=1..9")
	if err != nil {
		t.Fatal(err)
	}
	if d, _ := proposalDiff([]TuneParam{p}, []float64{p.Base}); d != "" {
		t.Errorf("diff ohne Änderung: %q", d)
	}
}
