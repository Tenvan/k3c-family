package balance

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// B-640/AC-02: Baselines sichern, auflisten, nie überschreiben, nur durchgelaufene Läufe.

func writeReport(t *testing.T, root, run, raw string) string {
	t.Helper()
	dir := filepath.Join(root, "reports", run)
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	path := filepath.Join(dir, "balance-20261028-100000.json")
	if err := os.WriteFile(path, []byte(raw), 0o644); err != nil {
		t.Fatal(err)
	}
	return path
}

const passReport = `{"version":1,"seeds":1,"seedMode":"dev","pass":true,"targets":[` +
	`{"name":"Endboss","measure":"endbossDefeated","kind":"share","players":2,"bot":"coop2","depth":0,"days":40,"world":"standard","value":100,"lower":100,"upper":100,"status":"ok","pass":true}]}`

func TestBaselineSichernAuflistenUndLesen(t *testing.T) {
	root := t.TempDir()
	path := writeReport(t, root, "simtest-run-3", passReport)
	info, err := SaveBaseline(root, "M1", path, json.RawMessage(`{"seeds":1,"bot":"coop2","world":"standard","days":40}`))
	if err != nil {
		t.Fatal(err)
	}
	if got, want := fmt.Sprint(info.Name, info.Run, info.Seeds, info.SeedMode, info.Bots, info.Worlds), fmt.Sprint("M1", "simtest-run-3", 1, "dev", []string{"coop2"}, []string{"standard"}); got != want {
		t.Errorf("Konfiguration: %s, erwartet %s", got, want)
	}
	if !HasBaseline(root, "M1") || HasBaseline(root, "M2") {
		t.Error("HasBaseline")
	}
	list, err := ListBaselines(root)
	if err == nil && len(list) == 1 && !strings.Contains(string(list[0].Spec), `"world":"standard"`) {
		t.Errorf("Spec des Laufs fehlt in der Liste: %s", list[0].Spec)
	}
	if err != nil || len(list) != 1 || list[0].Name != "M1" {
		t.Fatalf("Liste: %v %v", list, err)
	}
	base, err := ReadBaseline(filepath.Join(root, filepath.FromSlash(BaselineFile("M1"))))
	if err != nil || base == nil || len(base.Targets) != 1 {
		t.Errorf("gesicherter Bericht lässt sich als Baseline lesen: %v %v", base, err)
	}
}

func TestBaselineLehntAb(t *testing.T) {
	root := t.TempDir()
	pass := writeReport(t, root, "simtest-run-1", passReport)
	fail := writeReport(t, root, "simtest-run-2", strings.Replace(passReport, `"pass":true,"targets"`, `"pass":false,"targets"`, 1))
	if _, err := SaveBaseline(root, "M1", pass, nil); err != nil {
		t.Fatal(err)
	}
	for _, c := range []struct{ name, path, want string }{
		{"M1", pass, "gibt es schon"},
		{"M2", fail, "nicht erreicht"},
		{"../x", pass, "Buchstaben"},
		{"", pass, "Buchstaben"},
		{"M3", filepath.Join(root, "fehlt.json"), ""},
	} {
		_, err := SaveBaseline(root, c.name, c.path, nil)
		if err == nil || !strings.Contains(err.Error(), c.want) {
			t.Errorf("%q: Fehler %v, erwartet %q", c.name, err, c.want)
		}
	}
	if list, _ := ListBaselines(root); len(list) != 1 {
		t.Errorf("abgelehnte Namen legen nichts an: %v", list)
	}
}

func TestListBaselinesOhneOrdnerLeer(t *testing.T) {
	list, err := ListBaselines(t.TempDir())
	if err != nil || list == nil || len(list) != 0 {
		t.Errorf("%#v %v", list, err)
	}
}
