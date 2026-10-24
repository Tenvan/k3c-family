package simrun

import (
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"
)

// WZ28.6: Spec.tune wird geprüft wie vary und wird zu --tune, --param, --steps, --budget.
func TestSpecTune(t *testing.T) {
	ok := Spec{Seeds: 1, Tune: &Tune{Mode: "search", Params: []string{"economy.json:purse.startGold=50..250",
		"buildings.json:wall.levels.0.cost.gold=-1.5..2"}, Steps: 3, Budget: 4}}
	if err := ok.Check(); err != nil {
		t.Fatal(err)
	}
	args := strings.Join(ok.args("exe", "dir"), " ")
	want := "--tune search --param economy.json:purse.startGold=50..250 --param buildings.json:wall.levels.0.cost.gold=-1.5..2 --steps 3 --budget 4"
	if !strings.HasSuffix(args, want) || !strings.Contains(ok.Title(), "tune search 2 Parameter") {
		t.Errorf("args %q, titel %q", args, ok.Title())
	}
	p := "economy.json:purse.startGold=1..2"
	for _, tu := range []*Tune{
		{Mode: "raten", Params: []string{p}},
		{Mode: "sweep"},
		{Mode: "sweep", Params: []string{p, p, p}},
		{Mode: "sweep", Params: []string{p}, Steps: 1},
		{Mode: "sweep", Params: []string{p}, Steps: 22},
		{Mode: "sweep", Params: []string{p}, Budget: 1001},
		{Mode: "sweep", Params: []string{"economy.json:purse.startGold=1"}},
		{Mode: "sweep", Params: []string{"economy.json:a..b=1..2"}},
		{Mode: "sweep", Params: []string{"../economy.json:a=1..2"}},
		{Mode: "sweep", Params: []string{"sub/economy.json:a=1..2"}},
		{Mode: "sweep", Params: []string{"economy.json:a=1..2 --out x"}},
	} {
		if err := (Spec{Tune: tu}).Check(); err == nil {
			t.Errorf("%+v angenommen", tu)
		}
	}
}

const tuningBody = `{"version":1,"mode":"sweep","params":[{"file":"economy.json","path":"purse.startGold","min":50,"max":250,"base":100,"int":true}],
"points":[{"values":[150],"score":0,"pass":true,"failed":[]}],"best":{"values":[150],"score":0,"pass":true},"reason":"Alle getroffen.","diff":"+x"}`

// Reports nennt tuning-*.json mit kind, ReadTuning liest sie mit der Pfadprüfung von ReadReport.
func TestTuningBerichte(t *testing.T) {
	root := reportsFixture(t)
	dir := filepath.Join(root, "reports", "simtest-run-4")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, "tuning-1.json"), []byte(tuningBody), 0o644); err != nil {
		t.Fatal(err)
	}
	list, err := Reports(root)
	if err != nil || len(list) != 3 {
		t.Fatalf("liste: %+v, %v", list, err)
	}
	i := slices.IndexFunc(list, func(r ReportInfo) bool { return r.Kind == "tuning" })
	if i < 0 || list[i].Path != "reports/simtest-run-4/tuning-1.json" || !list[i].Pass || list[i].Run != "simtest-run-4" {
		t.Errorf("eintrag: %+v", list)
	}
	if slices.ContainsFunc(list, func(r ReportInfo) bool { return r.Kind != "tuning" && r.Kind != "balance" }) {
		t.Errorf("kind fehlt: %+v", list)
	}
	checkReadTuning(t, root)
	verdict, lines, err := readResult(dir, time.Now().Add(-time.Minute), true)
	if err != nil || verdict != "Pass" || len(lines) != 2 || lines[1] != "Alle getroffen." {
		t.Errorf("ergebnis des Laufs: %s %v %v", verdict, lines, err)
	}
}

// checkReadTuning: ReadTuning liest das Ergebnis und lehnt Pfade außerhalb von reports/ und Unlesbares ab.
func checkReadTuning(t *testing.T, root string) {
	t.Helper()
	tn, err := ReadTuning(root, "reports/simtest-run-4/tuning-1.json")
	if err != nil || tn.Best.Values[0] != 150 || tn.Diff != "+x" || tn.Params[0].Base != 100 {
		t.Errorf("ergebnis: %+v, %v", tn, err)
	}
	for _, bad := range []string{"", "geheim.json", "../geheim.json", "reports/../geheim.json", root + "/reports/simtest-run-4/tuning-1.json",
		"reports/nichtda.json", "reports/simtest-run-4", "reports/kaputt/balance-3.json"} {
		if _, err := ReadTuning(root, bad); err == nil {
			t.Errorf("%q gelesen", bad)
		}
	}
}
