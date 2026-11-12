package review

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"testing"
)

// repoRoot ist die Wurzel des K3C-Repos (dort liegen tools/paper/ und die Fixtures).
var repoRoot = func() string {
	p, err := filepath.Abs("../../../..")
	if err != nil {
		panic(err)
	}
	return p
}()

// langsam überspringt mit -short Tests, die bun, die CLI von tools/paper oder ein echtes Git-Repo starten: unter
// Windows kostet jeder Start Sekunden, die Tests liefen mit -short 3–20 s (B-619, B-787). check:dev, Review und CI
// fahren sie ohne -short.
func langsam(t *testing.T) {
	t.Helper()
	if testing.Short() {
		t.Skip("startet bun bzw. git, läuft ohne -short (B-619, B-787)")
	}
}

// bun führt bun in der Repo-Wurzel aus; ohne bun scheitert der Test (requirements.md), mit -short wird er übersprungen.
func bun(t *testing.T, args ...string) string {
	t.Helper()
	langsam(t)
	if _, err := exec.LookPath("bun"); err != nil {
		t.Fatalf("bun fehlt (requirements.md): %v", err)
	}
	cmd := exec.Command("bun", args...)
	cmd.Dir = repoRoot
	out, err := cmd.Output()
	if err != nil {
		var stderr string
		if ee, ok := err.(*exec.ExitError); ok {
			stderr = string(ee.Stderr)
		}
		t.Fatalf("bun %s: %v %s", strings.Join(args, " "), err, stderr)
	}
	return strings.TrimSpace(string(out))
}

var (
	vorlageOnce sync.Once
	vorlage     string // einmal erzeugtes Test-Repo, jeder Test bekommt eine Kopie
)

func TestMain(m *testing.M) {
	code := m.Run()
	if vorlage != "" {
		_ = os.RemoveAll(filepath.Dir(vorlage))
	}
	os.Exit(code)
}

// testRepo liefert eine eigene Kopie des Review-Test-Repos (tools/paper/fixtures/review-repo/erzeuge.ts); erzeugt
// wird es nur einmal je Testlauf, weil git unter Windows langsam startet.
func testRepo(t *testing.T) string {
	t.Helper()
	langsam(t)
	vorlageOnce.Do(func() {
		dir, err := os.MkdirTemp("", "rv1-")
		if err != nil {
			t.Fatal(err)
		}
		vorlage = filepath.Join(dir, "repo")
		bun(t, "tools/paper/fixtures/review-repo/erzeuge.ts", vorlage)
	})
	if vorlage == "" {
		t.Fatal("Test-Repo fehlt")
	}
	ziel := filepath.Join(t.TempDir(), "repo")
	if err := os.CopyFS(ziel, os.DirFS(vorlage)); err != nil {
		t.Fatal(err)
	}
	return ziel
}

// gitIn führt git im Ordner aus (Testaufbau).
func gitIn(t *testing.T, dir string, args ...string) string {
	t.Helper()
	cmd := exec.Command("git", append([]string{"-c", "core.autocrlf=false"}, args...)...)
	cmd.Dir = dir
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git %s: %v %s", strings.Join(args, " "), err, out)
	}
	return strings.TrimSpace(string(out))
}

// lies liest eine Datei; ohne t (Attrappe) panic statt Fatal.
func lies(t *testing.T, pfad string) []byte {
	raw, err := os.ReadFile(pfad)
	if err != nil {
		if t == nil {
			panic(err)
		}
		t.Fatal(err)
	}
	return raw
}

func fixture(name string) string { return filepath.Join(repoRoot, "tools", "paper", "fixtures", name) }
