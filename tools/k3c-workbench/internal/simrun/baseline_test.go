package simrun

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// passingReport ist ein durchgelaufener Bericht mit gültiger Messung (wie in mcpsrv/simtest_baseline_test.go).
const passingReport = `{"version":1,"seeds":1,"seedMode":"dev","pass":true,"targets":[` +
	`{"name":"Endboss","measure":"endbossDefeated","kind":"share","players":2,"bot":"coop2","depth":0,"days":40,"world":"standard","value":100,"lower":100,"upper":100,"status":"ok","pass":true}]}`

// B-662/AC-03: SaveBaseline sichert den Bericht eines Laufs; der Auftrag kommt vom Lauf oder, bei einem alten Lauf
// ohne Eintrag im Register, aus dem Bericht. Beides meldet „Berichte geändert“.
func TestBaselineAusBericht(t *testing.T) {
	root := t.TempDir()
	g := writingRegistry(t, passingReport)
	reports := make(chan string, 8)
	g.SetNotify(Notify{Reports: func(r string) { reports <- r }})
	info, err := g.StartOffline(root, "Repo-Wurzel", Spec{Seeds: 1, Bot: "coop2", Label: "M1-Lauf"})
	if err != nil {
		t.Fatal(err)
	}
	waitFor(t, g, root, info.ID, func(i Info) bool { return i.State == Done && i.Report != "" })
	bi, err := g.SaveBaseline(root, "reports/simtest-"+info.ID+"/balance-1.json", "M1", false)
	if err != nil || bi.Run != "simtest-"+info.ID || !strings.Contains(string(bi.Spec), `"coop2"`) {
		t.Fatalf("sichern: %+v %v", bi, err)
	}
	for _, f := range []string{"balance.json", "run.json"} {
		if _, err := os.Stat(filepath.Join(root, "reports", "baseline", "M1", f)); err != nil {
			t.Error(err)
		}
	}
	drain(t, reports, 2) // Lauf fertig, Baseline gesichert

	old := "reports/simtest-run-7/balance-1.json"
	writeReport(t, root, old, `{"label":"alt","runId":"run-7","spec":{"bot":"saver","days":30},`+passingReport[1:])
	if bi, err := g.SaveBaseline(root, old, "M2", false); err != nil || !strings.Contains(string(bi.Spec), `"saver"`) {
		t.Errorf("alter Bericht: %+v %v", bi, err)
	}
	var meta map[string]any
	raw, _ := os.ReadFile(filepath.Join(root, "reports", "baseline", "M2", "run.json"))
	if json.Unmarshal(raw, &meta) != nil || meta["run"] != "simtest-run-7" {
		t.Errorf("run.json: %s", raw)
	}
}

// B-662/AC-03: SaveBaseline lehnt fremde Pfade, laufende Läufe, ungültige und vergebene Namen mit Grund ab.
func TestBaselineLehntAb(t *testing.T) {
	root := t.TempDir()
	g := New(nil)
	saved := "reports/simtest-run-7/balance-1.json"
	writeReport(t, root, saved, passingReport)
	if _, err := g.SaveBaseline(root, saved, "M1", false); err != nil {
		t.Fatal(err)
	}
	release := make(chan struct{})
	defer close(release)
	running, err := g.Start(root, "Repo-Wurzel", "x", nil, blocking(g, release), true)
	if err != nil {
		t.Fatal(err)
	}
	writeReport(t, root, "reports/simtest-"+running.ID()+"/balance-1.json", passingReport)
	writeReport(t, root, "reports/balance-1.json", passingReport)
	cases := []struct{ path, name, why string }{
		{saved, "M1", "gibt es schon"},
		{saved, "../M3", "Buchstaben"},
		{"reports/balance-1.json", "M3", "Ordner eines Laufs"},
		{"../x/balance-1.json", "M3", "reports/"},
		{"reports/simtest-run-99/balance-1.json", "M3", "gibt es nicht"},
		{"reports/simtest-" + running.ID() + "/balance-1.json", "M3", "läuft"},
	}
	for _, c := range cases {
		if _, err := g.SaveBaseline(root, c.path, c.name, false); err == nil || !strings.Contains(err.Error(), c.why) {
			t.Errorf("%s als %s: %v, erwartet „%s“", c.path, c.name, err, c.why)
		}
	}
}

// drain wartet auf n Meldungen „Berichte geändert“.
func drain(t *testing.T, ch <-chan string, n int) {
	t.Helper()
	for range n {
		select {
		case <-ch:
		case <-time.After(5 * time.Second):
			t.Fatalf("weniger als %d Meldungen", n)
		}
	}
}
