package simrun

import (
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"
)

// B-632/AC-05: sim_test status nennt die Kurzauswertung und den Pfad des HTML-Berichts vor den Zeilen je Ziel.
func TestStatusMitKurzauswertung(t *testing.T) {
	dir := filepath.Join(t.TempDir(), "reports", "simtest-run-3")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	raw, err := os.ReadFile("../balance/testdata/insights-run7.json")
	if err != nil {
		t.Fatal(err)
	}
	stem := filepath.Join(dir, "balance-20261025-062117")
	if err := os.WriteFile(stem+".json", raw, 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(stem+".html", []byte("<html></html>"), 0o644); err != nil {
		t.Fatal(err)
	}
	verdict, lines, err := readResult(dir, time.Now().Add(-time.Minute), false)
	if err != nil || verdict != "Fail" {
		t.Fatalf("%s %v", verdict, err)
	}
	if len(lines) < 5 || !strings.HasPrefix(lines[0], "4 von 13 Zielen verletzt") {
		t.Errorf("Kurzauswertung fehlt: %q", lines)
	}
	if !slices.Contains(lines, "HTML-Bericht: reports/simtest-run-3/balance-20261025-062117.html") {
		t.Errorf("HTML-Pfad fehlt: %q", lines)
	}
	info := Info{ID: "run-3", Lines: lines, Report: "reports/simtest-run-3"}
	if st := info.Status(time.Now(), 10); strings.Count(st, "\n") > 9 || !strings.Contains(st, "HTML-Bericht:") {
		t.Errorf("status:\n%s", st)
	}
}
