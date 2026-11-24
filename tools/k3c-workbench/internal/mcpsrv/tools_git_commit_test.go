package mcpsrv

import (
	"context"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// gcTestRepo legt ein Test-Repo mit dem Branch branch an (ohne Commit).
func gcTestRepo(t *testing.T, branch string) string {
	t.Helper()
	root := t.TempDir()
	t.Setenv("GIT_AUTHOR_NAME", "t")
	t.Setenv("GIT_AUTHOR_EMAIL", "t@t")
	t.Setenv("GIT_COMMITTER_NAME", "t")
	t.Setenv("GIT_COMMITTER_EMAIL", "t@t")
	t.Setenv("GIT_CONFIG_NOSYSTEM", "1")
	cmd := exec.Command("git", "init", "-q", "-b", branch)
	cmd.Dir = root
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("git init: %v %s", err, out)
	}
	return root
}

// Auf main wird nicht committet: die Ablehnung nennt den Grund und lässt den Index leer.
func TestGitCommitLehntMainAb(t *testing.T) {
	root := gcTestRepo(t, "main")
	if err := os.WriteFile(filepath.Join(root, "a.md"), []byte("x"), 0o644); err != nil {
		t.Fatal(err)
	}
	_, err := commitFiles(context.Background(), root, gitCommitIn{Paths: []string{"a.md"}, Type: "docs", Subject: "Test"})
	if err == nil || !strings.Contains(err.Error(), "auf main") {
		t.Fatalf("main muss abgelehnt werden: %v", err)
	}
}

// Eine fremde gestagte Datei im Index lässt den Commit scheitern, damit keine fremde Arbeit mitgeht.
func TestGitCommitLehntFremdeGestagteDateiAb(t *testing.T) {
	root := gcTestRepo(t, "sprint/test")
	for _, f := range []string{"fremd.md", "mein.md"} {
		if err := os.WriteFile(filepath.Join(root, f), []byte("x"), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	if out, err := exec.Command("git", "-C", root, "add", "fremd.md").CombinedOutput(); err != nil {
		t.Fatalf("add: %v %s", err, out)
	}
	_, err := commitFiles(context.Background(), root, gitCommitIn{Paths: []string{"mein.md"}, Type: "docs", Subject: "Test"})
	if err == nil || !strings.Contains(err.Error(), "fremd.md") {
		t.Fatalf("fremde Datei muss ablehnen: %v", err)
	}
}

// Auf einem Feature-Branch committet das Werkzeug genau die genannte Datei mit dem Format typ(domäne): Betreff.
func TestGitCommitCommitetAufFeatureBranch(t *testing.T) {
	root := gcTestRepo(t, "sprint/test")
	if err := os.WriteFile(filepath.Join(root, "mein.md"), []byte("x"), 0o644); err != nil {
		t.Fatal(err)
	}
	out, err := commitFiles(context.Background(), root, gitCommitIn{Paths: []string{"mein.md"}, Type: "docs", Scope: "planung", Subject: "Test"})
	if err != nil || !strings.Contains(out, "docs(planung): Test") {
		t.Fatalf("Commit: %q %v", out, err)
	}
	log, _ := exec.Command("git", "-C", root, "log", "-1", "--format=%s").CombinedOutput()
	if strings.TrimSpace(string(log)) != "docs(planung): Test" {
		t.Errorf("letzter Commit: %q", log)
	}
}
