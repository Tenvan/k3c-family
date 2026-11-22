package balance

import (
	"cmp"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
)

// WZ63.7/AC-09: Das Balancing schreibt Quelldateien nur unter data/worlds/, Laufausgaben nur unter reports/.
func TestBalancingWaechterRegeln(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	for _, p := range []string{"data/worlds/standard.json", "reports/simtest-7/simtest.md", "reports/baseline/M1/run.json",
		"reports/ratings.json", "reports/r/result.json", "reports/r/balance-20261122-100000.html"} {
		if err := Guard.Check(root, p); err != nil {
			t.Errorf("%s abgelehnt: %v", p, err)
		}
	}
	outside := filepath.Join(filepath.Dir(root), "fremd.json")
	for _, p := range []string{"data/troops.json", "data/economy.json", "data/paper.json", "engine/sim/x.go",
		"rust/crates/k3c-sim/src/lib.rs", "tools/k3c-studio/app.go", "docs/arbeitsweise.md", "cmd/k3c-sim/main.go",
		"public/grafik/a.png", "saves/a.json", "testdata/balance/baseline.json", "data/worlds/../troops.json", outside} {
		err := Guard.Check(root, p)
		if err == nil {
			t.Errorf("%s erlaubt", p)
		} else if !strings.Contains(err.Error(), "Plan im Chat") {
			t.Errorf("%s: Meldung ohne Hinweis auf den Plan im Chat: %v", p, err)
		}
	}
}

// Die Helfer schreiben nichts, wenn der Wächter ablehnt.
func TestBalancingHelferSchreibenNichtsAusserhalb(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	if err := WriteFile(root, "data/troops.json", []byte("{}")); err == nil {
		t.Error("data/troops.json geschrieben")
	}
	if err := MkdirAll(root, filepath.Join(root, "engine", "sim")); err == nil {
		t.Error("engine/sim angelegt")
	}
	if _, err := os.Stat(filepath.Join(root, "data")); !os.IsNotExist(err) {
		t.Errorf("data/ entstand trotz Ablehnung: %v", err)
	}
	if err := cmp.Or(MkdirAll(root, "reports/r"), WriteFile(root, "reports/r/a.md", []byte("x")),
		Rename(root, filepath.Join(root, "reports", "r", "a.md"), filepath.Join(root, "reports", "r", "b.md")),
		Remove(root, "reports/r/b.md"), RemoveAll(root, "reports/r")); err != nil {
		t.Fatal(err)
	}
}

// rawWrite findet Schreibaufrufe am Wächter vorbei.
var rawWrite = regexp.MustCompile(`\b(os\.(WriteFile|Create|CreateTemp|OpenFile|Rename|Remove|RemoveAll|Mkdir|MkdirAll|MkdirTemp|Truncate|Symlink|Link|Chmod|CopyFS)|ioutil\.WriteFile)\(`)

// rawWriteAllowed sind die Dateien, die roh schreiben dürfen (Pfad ab tools/k3c-studio → Grund).
var rawWriteAllowed = map[string]string{
	"internal/balance/writeguard.go": "die Helfer selbst, jeder Aufruf hinter Guard.Check",
}

// WZ63.7/AC-09: balance, simrun, datadoc, gamedata und die Balance-Bindings schreiben nur über die Helfer mit Wächter.
func TestBalancingSchreibtNurUeberWaechter(t *testing.T) {
	t.Parallel()
	files := map[string]string{}
	var bindings int
	for _, pattern := range []string{"../balance/*.go", "../simrun/*.go", "../datadoc/*.go", "../gamedata/*.go", "../../app_balance*.go"} {
		matches, err := filepath.Glob(pattern)
		if err != nil {
			t.Fatal(err)
		}
		for _, m := range matches {
			if strings.HasSuffix(m, "_test.go") {
				continue
			}
			raw, err := os.ReadFile(m)
			if err != nil {
				t.Fatal(err)
			}
			rel := strings.TrimPrefix(filepath.ToSlash(m), "../../")
			if strings.HasPrefix(rel, "../") {
				rel = "internal/" + strings.TrimPrefix(rel, "../")
			} else {
				bindings++
			}
			files[rel] = string(raw)
		}
	}
	if len(files) < 20 || bindings == 0 || files["internal/balance/writeguard.go"] == "" {
		t.Fatalf("nur %d Dateien, %d Bindings gelesen: Pfade stimmen nicht", len(files), bindings)
	}
	for rel, src := range files {
		if rawWriteAllowed[rel] != "" {
			if !rawWrite.MatchString(src) {
				t.Errorf("%s steht in rawWriteAllowed, schreibt aber nicht mehr roh: Eintrag streichen", rel)
			}
			continue
		}
		for _, m := range rawWrite.FindAllString(src, -1) {
			t.Errorf("%s ruft %s am Wächter vorbei: balance.WriteFile/MkdirAll/Rename/Remove nutzen oder in rawWriteAllowed mit Grund eintragen", rel, m)
		}
	}
}
