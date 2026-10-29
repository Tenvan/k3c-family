package simrun

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
)

// writingRegistry: Bauen ist sofort fertig, Rechnen schreibt body als balance-1.json in den Berichtsordner des Laufs.
func writingRegistry(t *testing.T, body string) *Registry {
	t.Helper()
	g := New(nil)
	g.Exec = func(_ context.Context, p Proc) Result {
		if p.Args[0] == "go" {
			return Result{}
		}
		dir := filepath.Join(p.Dir, p.Args[slices.Index(p.Args, "--report-dir")+1])
		if err := os.MkdirAll(dir, 0o755); err != nil {
			t.Error(err)
		}
		if err := os.WriteFile(filepath.Join(dir, "balance-1.json"), []byte(body), 0o644); err != nil {
			t.Error(err)
		}
		return Result{}
	}
	return g
}

func findReport(t *testing.T, root, path string) ReportInfo {
	t.Helper()
	list, err := Reports(root)
	if err != nil {
		t.Fatal(err)
	}
	for _, r := range list {
		if r.Path == path {
			return r
		}
	}
	t.Fatalf("%s fehlt in der Liste: %+v", path, list)
	return ReportInfo{}
}

// brief: Label, Lauf-ID, Ziele und Auftrag eines Eintrags der Liste in einer Zeile.
func brief(r ReportInfo) string {
	spec, _ := json.Marshal(r.Spec)
	return fmt.Sprintf("%s|%s|%d/%d|%s", r.Label, r.RunID, r.TargetsPass, r.TargetsTotal, spec)
}

// B-662/AC-01: Ein Lauf schreibt Label, Auftrag und Lauf-ID in den Bericht; Reports liefert sie und x/y Ziele.
func TestBerichtTraegtAuftrag(t *testing.T) {
	root := t.TempDir()
	target := func(name string, pass bool) string {
		return fmt.Sprintf(`{"name":%q,"kind":"share","value":80,"lower":75,"upper":90,"pass":%t}`, name, pass)
	}
	body := `{"version":1,"seeds":2,"pass":false,"targets":[` + target("A", true) + "," + target("B", false) + "," +
		target("C", true) + `],"neuesFeld":{"x":[1,2]}}`
	g := writingRegistry(t, body)
	depth := 0
	spec := Spec{Seeds: 2, Bot: "coop2", Days: 40, Depth: &depth, Label: "Insel 1 solo · ohne Hilfen"}
	info, err := g.StartOffline(root, "Repo-Wurzel", spec)
	if err != nil {
		t.Fatal(err)
	}
	done := waitFor(t, g, root, info.ID, func(i Info) bool { return i.State != Running && i.State != Waiting && i.Report != "" })
	if done.State != Done || done.Verdict != "Fail" {
		t.Fatalf("lauf: %+v", done)
	}
	rel := "reports/simtest-" + info.ID + "/balance-1.json"
	raw, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(rel)))
	if err != nil {
		t.Fatal(err)
	}
	head := "{\n  \"label\": \"Insel 1 solo · ohne Hilfen\",\n  \"runId\": \"" + info.ID + "\",\n  \"spec\": {"
	if !json.Valid(raw) || !strings.HasPrefix(string(raw), head) || !strings.Contains(string(raw), `"neuesFeld": {`) {
		t.Errorf("Bericht nicht vorn ergänzt oder Felder verloren:\n%s", raw)
	}
	want, _ := json.Marshal(spec)
	if got := brief(findReport(t, root, rel)); got != spec.Label+"|"+info.ID+"|2/3|"+string(want) {
		t.Errorf("Eintrag: %s", got)
	}
}

// B-662/AC-01: Ein alter Bericht ohne Auftrag liefert leere Felder ohne Fehler, ein tuning-*.json 0/0 Ziele.
func TestAlterBerichtOhneAuftrag(t *testing.T) {
	root := t.TempDir()
	old := "reports/simtest-run-9/balance-1.json"
	writeReport(t, root, old, `{"seeds":1,"pass":true,"targets":[{"name":"A","pass":true}]}`)
	writeReport(t, root, "reports/tuning-1.json", `{"version":1,"mode":"sweep","best":{"pass":true}}`)
	writeReport(t, root, "reports/balance-2.json", `{"seeds":1,"pass":true,"spec":"kaputt","label":"x"}`)
	for path, want := range map[string]string{old: "||1/1|null", "reports/tuning-1.json": "||0/0|null", "reports/balance-2.json": "x||0/0|null"} {
		if got := brief(findReport(t, root, path)); got != want {
			t.Errorf("%s: %s, erwartet %s", path, got, want)
		}
	}
}

// Stempeln: ein schon gestempelter Bericht bleibt, ein leeres Objekt bekommt die Felder, Kaputtes wird abgelehnt.
func TestStempel(t *testing.T) {
	dir := t.TempDir()
	write := func(name, body string) string {
		p := filepath.Join(dir, name)
		if err := os.WriteFile(p, []byte(body), 0o644); err != nil {
			t.Fatal(err)
		}
		return p
	}
	st := Stamp{Label: "x", RunID: "run-1", Spec: &Spec{Seeds: 1}}
	empty := write("leer.json", "{}")
	if err := stamp(empty, st); err != nil {
		t.Fatal(err)
	}
	raw, _ := os.ReadFile(empty)
	var back Stamp
	if err := json.Unmarshal(raw, &back); err != nil || back.RunID != "run-1" || back.Spec.Seeds != 1 {
		t.Errorf("leeres Objekt: %s %v", raw, err)
	}
	if err := stamp(empty, Stamp{Label: "y", RunID: "run-2"}); err != nil {
		t.Fatal(err)
	}
	if again, _ := os.ReadFile(empty); !bytes.Equal(again, raw) {
		t.Errorf("zweiter Stempel änderte den Bericht:\n%s", again)
	}
	for _, bad := range []string{"[1]", "kaputt", ""} {
		if err := stamp(write("bad.json", bad), st); err == nil {
			t.Errorf("%q gestempelt", bad)
		}
	}
}
