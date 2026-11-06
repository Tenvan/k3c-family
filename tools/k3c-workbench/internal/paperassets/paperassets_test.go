package paperassets

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
	"testing"
)

// repoWurzel ist die Wurzel des eigenen Checkouts (Paketordner tools/k3c-workbench/internal/paperassets).
func repoWurzel(t *testing.T) string {
	t.Helper()
	abs, err := filepath.Abs(filepath.Join("..", "..", "..", ".."))
	if err != nil {
		t.Fatal(err)
	}
	return abs
}

// stubSkript ist der absolute Pfad der CLI-Attrappe (PV1.4).
func stubSkript(t *testing.T) string {
	t.Helper()
	if _, err := exec.LookPath("bun"); err != nil {
		t.Fatal("bun fehlt im PATH (requirements.md); die Asset-Tests rufen die CLI-Attrappe mit bun")
	}
	return filepath.Join(repoWurzel(t), "tools", "paper", "fixtures", "cli-stub.ts")
}

// kopiere legt src (relativ zur Repo-Wurzel) unter root/dst ab.
func kopiere(t *testing.T, root, src, dst string) {
	t.Helper()
	b, err := os.ReadFile(filepath.Join(repoWurzel(t), filepath.FromSlash(src)))
	if err != nil {
		t.Fatal(err)
	}
	schreibe(t, root, dst, string(b))
}

func schreibe(t *testing.T, root, rel, text string) {
	t.Helper()
	p := filepath.Join(root, filepath.FromSlash(rel))
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

func git(t *testing.T, root string, args ...string) {
	t.Helper()
	cmd := exec.Command("git", args...)
	cmd.Dir = root
	cmd.Env = append(os.Environ(), "GIT_AUTHOR_DATE=2026-11-01T12:00:00Z", "GIT_COMMITTER_DATE=2026-11-01T12:00:00Z",
		"GIT_AUTHOR_NAME=test", "GIT_AUTHOR_EMAIL=t@example.invalid", "GIT_COMMITTER_NAME=test", "GIT_COMMITTER_EMAIL=t@example.invalid")
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("git %v: %v\n%s", args, err, out)
	}
}

// testRepo ist ein Git-Repo mit Stil-Datei und den Fixture-Specs monarch, pferd, mauer (ein Commit).
func testRepo(t *testing.T) (string, CLI) {
	t.Helper()
	root := t.TempDir()
	kopiere(t, root, "public/grafik/stil.paper.json", "public/grafik/stil.paper.json")
	kopiere(t, root, "tools/paper/fixtures/specs/monarch.paper.json", "public/grafik/figuren/monarch.paper.json")
	kopiere(t, root, "tools/paper/fixtures/specs/pferd.paper.json", "public/grafik/reittiere/pferd.paper.json")
	kopiere(t, root, "tools/paper/fixtures/specs/mauer.paper.json", "public/grafik/gebaeude/mauer.paper.json")
	schreibe(t, root, "public/grafik/figuren/monarch/idle.png", "alt")
	schreibe(t, root, ".gitignore", ".cache/\n")
	git(t, root, "init", "-q", "-b", "develop")
	git(t, root, "add", "-A")
	git(t, root, "commit", "-q", "--no-gpg-sign", "-m", "Basis")
	return root, CLI{Root: root, Skript: stubSkript(t)}
}

// hashBaum ist ein Hash über alle Dateien unter den Pfaden (relativ zu root).
func hashBaum(t *testing.T, root string, rels ...string) string {
	t.Helper()
	h := sha256.New()
	for _, rel := range rels {
		start := filepath.Join(root, filepath.FromSlash(rel))
		var dateien []string
		_ = filepath.WalkDir(start, func(p string, d fs.DirEntry, err error) error {
			if err == nil && !d.IsDir() {
				dateien = append(dateien, p)
			}
			return nil
		})
		sort.Strings(dateien)
		for _, p := range dateien {
			b, _ := os.ReadFile(p)
			h.Write([]byte(p))
			h.Write(b)
		}
	}
	return hex.EncodeToString(h.Sum(nil))
}

func leer(t *testing.T, dir string) {
	t.Helper()
	eintraege, err := os.ReadDir(dir)
	if err != nil && !os.IsNotExist(err) {
		t.Fatal(err)
	}
	if len(eintraege) > 0 {
		t.Errorf("%s nicht leer: %v", dir, eintraege)
	}
}

func TestCLIValidate(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	ctx := context.Background()
	if err := cli.Validate(ctx, filepath.Join(root, "public/grafik/figuren/monarch.paper.json")); err != nil {
		t.Fatalf("monarch: %v", err)
	}
	err := cli.Validate(ctx, filepath.Join(repoWurzel(t), "tools/paper/fixtures/specs/ungueltig/fremdes-material.paper.json"))
	ab, ok := AlsAbgelehnt(err)
	if !ok || len(ab.Fehler) != 1 || ab.Fehler[0].Regel != "material-unbekannt" {
		t.Fatalf("fremdes-material: %v", err)
	}
	if _, err := cli.run(ctx, "gibtsnicht"); err == nil || !strings.Contains(err.Error(), "unbekannter Befehl") {
		t.Fatalf("Exit 2: %v", err)
	}
}

func TestCLIPfadeRelativ(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	if got := cli.rel(filepath.Join(root, "public", "grafik", "x.png")); got != "public/grafik/x.png" {
		t.Errorf("im Checkout: %q", got)
	}
	if got := cli.rel("tools/paper/fixtures/render/monarch/manifest.json"); got != "tools/paper/fixtures/render/monarch/manifest.json" {
		t.Errorf("relativ: %q", got)
	}
	fremd := filepath.Join(filepath.Dir(root), "anders", "y.png")
	if got := cli.rel(fremd); got != filepath.ToSlash(fremd) {
		t.Errorf("außerhalb: %q", got)
	}
}

func TestCLIPfadAusUmgebung(t *testing.T) {
	t.Setenv("K3C_PAPER_CLI", "x/y.ts")
	if c := NewCLI("R"); c.Skript != "x/y.ts" || c.Root != "R" {
		t.Errorf("Umgebung: %+v", c)
	}
	t.Setenv("K3C_PAPER_CLI", "")
	if c := NewCLI("R"); c.Skript != filepath.Join("R", "tools", "paper", "bin", "paper.ts") {
		t.Errorf("Standard: %+v", c)
	}
}

// Review MC1.1: ein abgebrochener Kontext kommt als ctx.Err() zurück, nicht als Exit-Fehler.
func TestCLIAbbruch(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	err := cli.Validate(ctx, filepath.Join(root, "public/grafik/figuren/monarch.paper.json"))
	if !errors.Is(err, context.Canceled) {
		t.Errorf("Abbruch: %v", err)
	}
}
