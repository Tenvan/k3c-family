package mcpsrv

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"sync"
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

// paperRepo ist ein Git-Repo mit Stil-Datei, Schema, Prompt-Vorlagen und den Fixture-Specs monarch, pferd, mauer
// (ein Commit); K3C_PAPER_CLI zeigt auf die Attrappe (gesetzt in TestMain). Das Repo entsteht einmal je Testlauf
// (paperVorlage) und wird je Test kopiert: drei git-Aufrufe kosten unter Windows gut 0,4 s (B-830). Ohne t.Setenv kann
// ein Test mit paperRepo parallel laufen, sofern er keine globalen Zustände ändert (mitFixtureReview).
func paperRepo(t *testing.T) string {
	t.Helper()
	if _, err := exec.LookPath("bun"); err != nil {
		t.Fatal("bun fehlt im PATH (requirements.md); die Asset-Tests rufen die CLI-Attrappe mit bun")
	}
	vorlage := paperVorlage(t)
	root := t.TempDir()
	err := filepath.WalkDir(vorlage, func(p string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		rel, _ := filepath.Rel(vorlage, p)
		ziel := filepath.Join(root, rel)
		if d.IsDir() {
			return os.MkdirAll(ziel, 0o755)
		}
		b, err := os.ReadFile(p)
		if err != nil {
			return err
		}
		return os.WriteFile(ziel, b, 0o644)
	})
	if err != nil {
		t.Fatalf("paperRepo kopieren: %v", err)
	}
	return root
}

var (
	paperOnce   sync.Once
	paperDir    string // Vorlage, TestMain räumt sie auf
	paperDirErr error
)

// paperVorlage legt das Repo von paperRepo einmal je Testlauf an.
func paperVorlage(t *testing.T) string {
	t.Helper()
	paperOnce.Do(func() {
		dir, err := os.MkdirTemp("", "mcpsrv-paper-")
		if err != nil {
			paperDirErr = err
			return
		}
		paperDir = dir
		paperDirErr = fuellePaperVorlage(repoWurzel(t), dir)
	})
	if paperDirErr != nil {
		t.Fatalf("paperRepo: %v", paperDirErr)
	}
	return paperDir
}

func fuellePaperVorlage(wurzel, root string) error {
	for src, dst := range map[string]string{
		"public/grafik/stil.paper.json":                 "public/grafik/stil.paper.json",
		"tools/paper/schema/stil.schema.json":           "tools/paper/schema/stil.schema.json",
		"data/asset-prompts.json":                       "data/asset-prompts.json",
		"tools/paper/fixtures/specs/monarch.paper.json": "public/grafik/figuren/monarch.paper.json",
		"tools/paper/fixtures/specs/pferd.paper.json":   "public/grafik/reittiere/pferd.paper.json",
		"tools/paper/fixtures/specs/mauer.paper.json":   "public/grafik/gebaeude/mauer.paper.json",
	} {
		b, err := os.ReadFile(filepath.Join(wurzel, filepath.FromSlash(src)))
		if err != nil {
			return err
		}
		if err := schreibe(root, dst, b); err != nil {
			return err
		}
	}
	if err := schreibe(root, "public/grafik/figuren/monarch/idle.png", []byte("alt")); err != nil {
		return err
	}
	if err := schreibe(root, ".gitignore", []byte(".cache/\n")); err != nil {
		return err
	}
	for _, args := range [][]string{{"init", "-q", "-b", "main"}, {"add", "-A"}, {"commit", "-q", "--no-gpg-sign", "-m", "Basis"}} {
		cmd := exec.Command("git", args...)
		cmd.Dir = root
		cmd.Env = append(os.Environ(), "GIT_AUTHOR_NAME=test", "GIT_AUTHOR_EMAIL=t@example.invalid",
			"GIT_COMMITTER_NAME=test", "GIT_COMMITTER_EMAIL=t@example.invalid")
		if out, err := cmd.CombinedOutput(); err != nil {
			return fmt.Errorf("git %v: %w\n%s", args, err, out)
		}
	}
	return nil
}

func schreibe(root, rel string, b []byte) error {
	p := filepath.Join(root, filepath.FromSlash(rel))
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		return err
	}
	return os.WriteFile(p, b, 0o644)
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
