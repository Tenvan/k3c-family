package mcpsrv

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// repoWurzel ist die Wurzel des eigenen Checkouts (Paketordner tools/k3c-workbench/internal/mcpsrv).
func repoWurzel(t *testing.T) string {
	t.Helper()
	abs, err := filepath.Abs(filepath.Join("..", "..", "..", ".."))
	if err != nil {
		t.Fatal(err)
	}
	return abs
}

func schreibeDatei(t *testing.T, root, rel, text string) {
	t.Helper()
	p := filepath.Join(root, filepath.FromSlash(rel))
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

func kopiereAusRepo(t *testing.T, root, src, dst string) {
	t.Helper()
	b, err := os.ReadFile(filepath.Join(repoWurzel(t), filepath.FromSlash(src)))
	if err != nil {
		t.Fatal(err)
	}
	schreibeDatei(t, root, dst, string(b))
}

func gitIn(t *testing.T, root string, args ...string) {
	t.Helper()
	cmd := exec.Command("git", args...)
	cmd.Dir = root
	cmd.Env = append(os.Environ(), "GIT_AUTHOR_NAME=test", "GIT_AUTHOR_EMAIL=t@example.invalid",
		"GIT_COMMITTER_NAME=test", "GIT_COMMITTER_EMAIL=t@example.invalid")
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("git %v: %v\n%s", args, err, out)
	}
}

// paperRepo ist ein Git-Repo mit Stil-Datei, Schema, Prompt-Vorlagen und den Fixture-Specs monarch, pferd, mauer
// (ein Commit); K3C_PAPER_CLI zeigt auf die Attrappe. Tests mit paperRepo laufen nicht parallel (t.Setenv).
func paperRepo(t *testing.T) string {
	t.Helper()
	if _, err := exec.LookPath("bun"); err != nil {
		t.Fatal("bun fehlt im PATH (requirements.md); die Asset-Tests rufen die CLI-Attrappe mit bun")
	}
	t.Setenv("K3C_PAPER_CLI", filepath.Join(repoWurzel(t), "tools", "paper", "fixtures", "cli-stub.ts"))
	root := t.TempDir()
	for src, dst := range map[string]string{
		"public/grafik/stil.paper.json":                 "public/grafik/stil.paper.json",
		"tools/paper/schema/stil.schema.json":           "tools/paper/schema/stil.schema.json",
		"data/asset-prompts.json":                       "data/asset-prompts.json",
		"tools/paper/fixtures/specs/monarch.paper.json": "public/grafik/figuren/monarch.paper.json",
		"tools/paper/fixtures/specs/pferd.paper.json":   "public/grafik/reittiere/pferd.paper.json",
		"tools/paper/fixtures/specs/mauer.paper.json":   "public/grafik/gebaeude/mauer.paper.json",
	} {
		kopiereAusRepo(t, root, src, dst)
	}
	schreibeDatei(t, root, "public/grafik/figuren/monarch/idle.png", "alt")
	schreibeDatei(t, root, ".gitignore", ".cache/\n")
	gitIn(t, root, "init", "-q", "-b", "develop")
	gitIn(t, root, "add", "-A")
	gitIn(t, root, "commit", "-q", "--no-gpg-sign", "-m", "Basis")
	return root
}

// hashUnter ist ein Hash über alle Dateien unter den Pfaden (relativ zu root).
func hashUnter(t *testing.T, root string, rels ...string) string {
	t.Helper()
	h := sha256.New()
	for _, rel := range rels {
		var dateien []string
		_ = filepath.WalkDir(filepath.Join(root, filepath.FromSlash(rel)), func(p string, d fs.DirEntry, err error) error {
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

// toolSchemas liefert die Eingabe-Schemas aller Tools nach Namen.
func toolSchemas(t *testing.T, cs *mcp.ClientSession) map[string]map[string]any {
	t.Helper()
	res, err := cs.ListTools(context.Background(), nil)
	if err != nil {
		t.Fatal(err)
	}
	out := map[string]map[string]any{}
	for _, tool := range res.Tools {
		m, _ := tool.InputSchema.(map[string]any)
		out[tool.Name] = m
	}
	return out
}
