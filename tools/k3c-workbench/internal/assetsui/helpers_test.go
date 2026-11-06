package assetsui

import (
	"context"
	"encoding/json"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"testing"

	"k3c/tools/k3c-workbench/internal/review"
)

// root ist die Repo-Wurzel; dort liegen tools/paper/ (CLI, Fixtures) und data/asset-prompts.json.
var root = func() string {
	p, err := filepath.Abs("../../../..")
	if err != nil {
		panic(err)
	}
	return p
}()

var (
	vorlageOnce sync.Once
	vorlage     string
)

func TestMain(m *testing.M) {
	code := m.Run()
	if vorlage != "" {
		_ = os.RemoveAll(filepath.Dir(vorlage))
	}
	os.Exit(code)
}

// testRepo liefert eine Kopie des Review-Test-Repos (tools/paper/fixtures/review-repo/erzeuge.ts), einmal erzeugt.
func testRepo(t *testing.T) string {
	t.Helper()
	vorlageOnce.Do(func() {
		dir, err := os.MkdirTemp("", "rv1-ui-")
		if err != nil {
			t.Fatal(err)
		}
		vorlage = filepath.Join(dir, "repo")
		cmd := exec.Command("bun", "tools/paper/fixtures/review-repo/erzeuge.ts", vorlage)
		cmd.Dir = root
		if out, err := cmd.CombinedOutput(); err != nil {
			t.Fatalf("Test-Repo (bun, requirements.md): %v %s", err, out)
		}
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

// fixture liest eine Beispielantwort aus tools/paper/fixtures/ui/ als generisches JSON.
func fixture(t *testing.T, method string) any {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(root, "tools/paper/fixtures/ui", method+".json"))
	if err != nil {
		t.Fatal(err)
	}
	var v any
	if err := json.Unmarshal(raw, &v); err != nil {
		t.Fatal(err)
	}
	return v
}

// roundtrip macht aus einem typisierten Ergebnis wieder generisches JSON (so sieht es die Oberfläche).
func roundtrip(t *testing.T, v any) any {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	var out any
	if err := json.Unmarshal(raw, &out); err != nil {
		t.Fatal(err)
	}
	return out
}

// fakeFrames: CLI-Attrappe, deren Frame die handgemachten Fixture-PNGs liefert (Basis alt, Arbeitsstand neu), wie
// beispiele.ts die pixelAnteil-Werte erzeugt hat.
type fakeFrames struct{ review.CLI }

var fakeBilder = map[string][2]string{
	"monarch": {"monarch/monarch-galopp-0.png", "monarch/monarch-galopp-0-purpur.png"},
	"mauer":   {"mauer/mauer-1.png", "mauer/mauer-1-stil.png"},
}

func (f fakeFrames) Frame(ctx context.Context, spec, animation string, frame int, variante, out string) (review.FrameAntwort, error) {
	a, err := f.CLI.Frame(ctx, spec, animation, frame, variante, out)
	if err != nil {
		return a, err
	}
	if bilder, ok := fakeBilder[strings.TrimSuffix(filepath.Base(spec), ".paper.json")]; ok {
		i := 1
		if strings.Contains(filepath.ToSlash(spec), "/basis/") {
			i = 0
		}
		raw, err := os.ReadFile(filepath.Join(root, "tools/paper/fixtures/render", bilder[i]))
		if err != nil {
			return a, err
		}
		return a, os.WriteFile(out, raw, 0o644)
	}
	return a, nil
}

func dienst(repo string, fake bool) Dienst {
	var r review.Renderer = review.CLI{Root: root}
	if fake {
		r = fakeFrames{CLI: review.CLI{Root: root}}
	}
	return Dienst{Root: repo, R: r}
}
