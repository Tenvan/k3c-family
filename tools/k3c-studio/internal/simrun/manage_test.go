package simrun

import (
	"os"
	"path/filepath"
	"testing"
)

func writeReport(t *testing.T, root, rel, body string) {
	t.Helper()
	p := filepath.Join(root, filepath.FromSlash(rel))
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(p, []byte(body), 0o644); err != nil {
		t.Fatal(err)
	}
}

func rating(t *testing.T, root, path string) string {
	t.Helper()
	list, err := Reports(root)
	if err != nil {
		t.Fatal(err)
	}
	for _, r := range list {
		if r.Path == path {
			return r.Rating
		}
	}
	t.Fatalf("%s fehlt in der Liste", path)
	return ""
}

// WZ28.13/AC-14 (B-612): Bewertung wird gespeichert und in der Liste geliefert; Löschen entfernt JSON, Markdown,
// leeren Lauf-Ordner und Bewertung; fremde Pfade und Bewertungen werden abgelehnt.
func TestBerichteBewertenUndLoeschen(t *testing.T) {
	root := t.TempDir()
	a, b := "reports/simtest-run-1/balance-1.json", "reports/balance-2.json"
	writeReport(t, root, a, `{"seeds":1,"pass":true}`)
	writeReport(t, root, "reports/simtest-run-1/balance-1.md", "# Bericht")
	writeReport(t, root, b, `{"seeds":2,"pass":false}`)

	if err := RateReport(root, a, "gut"); err != nil {
		t.Fatal(err)
	}
	if err := RateReport(root, b, "schlecht"); err != nil {
		t.Fatal(err)
	}
	if rating(t, root, a) != "gut" || rating(t, root, b) != "schlecht" {
		t.Errorf("Bewertungen: %q %q", rating(t, root, a), rating(t, root, b))
	}
	if err := RateReport(root, b, ""); err != nil || rating(t, root, b) != "" {
		t.Errorf("Bewertung löschen: %v", err)
	}
	for _, bad := range [][2]string{{a, "toll"}, {"reports/ratings.json", "gut"}, {"../x/balance-1.json", "gut"}, {"reports/balance-9.json", "gut"}} {
		if err := RateReport(root, bad[0], bad[1]); err == nil {
			t.Errorf("RateReport(%q, %q): Fehler erwartet", bad[0], bad[1])
		}
	}

	if err := DeleteReport(root, a); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(filepath.Join(root, "reports", "simtest-run-1")); !os.IsNotExist(err) {
		t.Errorf("Lauf-Ordner nicht entfernt: %v", err)
	}
	r, _ := loadRatings(root)
	if _, ok := r[a]; ok {
		t.Error("Bewertung des gelöschten Berichts bleibt")
	}
	for _, bad := range []string{a, "reports/ratings.json", "data/economy.json", "reports/../data/balance-1.json"} {
		if err := DeleteReport(root, bad); err == nil {
			t.Errorf("DeleteReport(%q): Fehler erwartet", bad)
		}
	}
	if _, err := os.Stat(filepath.Join(root, filepath.FromSlash(b))); err != nil {
		t.Errorf("anderer Bericht weg: %v", err)
	}
}

// Review WZ28.7: Eine kaputte ratings.json kostet nur die Bewertungen, nicht die Liste und nicht das Löschen.
func TestKaputteBewertungen(t *testing.T) {
	root := t.TempDir()
	writeReport(t, root, "reports/balance-1.json", `{"seeds":1,"pass":true}`)
	writeReport(t, root, "reports/ratings.json", "{kaputt")
	if list, err := Reports(root); err != nil || len(list) != 1 {
		t.Fatalf("Liste: %v %v", list, err)
	}
	if err := DeleteReport(root, "reports/balance-1.json"); err != nil {
		t.Errorf("Löschen: %v", err)
	}
}

// B-662/AC-02: Sammel-Bewerten und Sammel-Löschen in einem Aufruf; ein fremder Pfad lehnt den ganzen Aufruf ab.
func TestSammelBewertenUndLoeschen(t *testing.T) {
	root := t.TempDir()
	a, b, c := "reports/simtest-run-1/balance-1.json", "reports/simtest-run-2/balance-2.json", "reports/tuning-3.json"
	for _, p := range []string{a, b, c} {
		writeReport(t, root, p, `{"seeds":1,"pass":true}`)
	}
	writeReport(t, root, "reports/simtest-run-1/balance-1.md", "# Bericht")
	if err := RateReports(root, []string{a, b, c}, "gut"); err != nil {
		t.Fatal(err)
	}
	for _, bad := range [][]string{{a, "data/economy.json"}, {b, "../x/balance-1.json"}, {a, "reports/balance-9.json"}, {}} {
		if err := RateReports(root, bad, "schlecht"); err == nil {
			t.Errorf("RateReports(%q): Fehler erwartet", bad)
		}
		if err := DeleteReports(root, bad); err == nil {
			t.Errorf("DeleteReports(%q): Fehler erwartet", bad)
		}
	}
	for _, p := range []string{a, b, c} {
		if got := rating(t, root, p); got != "gut" {
			t.Errorf("%s: Bewertung %q nach abgelehntem Aufruf", p, got)
		}
	}
	if err := DeleteReports(root, []string{a, b, a}); err != nil {
		t.Fatal(err)
	}
	for _, gone := range []string{a, "reports/simtest-run-1/balance-1.md", "reports/simtest-run-1", "reports/simtest-run-2"} {
		if _, err := os.Stat(filepath.Join(root, filepath.FromSlash(gone))); !os.IsNotExist(err) {
			t.Errorf("%s bleibt: %v", gone, err)
		}
	}
	r, _ := loadRatings(root)
	if len(r) != 1 || r[c] != "gut" {
		t.Errorf("Bewertungen nach dem Löschen: %v", r)
	}
}
