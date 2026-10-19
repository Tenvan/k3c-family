package main

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

func git(t *testing.T, dir string, args ...string) string {
	t.Helper()
	full := append([]string{"-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"}, args...)
	cmd := exec.Command("git", full...)
	cmd.Dir = dir
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git %v: %v\n%s", args, err, out)
	}
	return strings.TrimSpace(string(out))
}

func commit(t *testing.T, dir, file string) {
	t.Helper()
	if err := os.WriteFile(filepath.Join(dir, file), []byte(file), 0o644); err != nil {
		t.Fatal(err)
	}
	git(t, dir, "add", file)
	git(t, dir, "commit", "-q", "-m", file)
}

func has(t *testing.T, dir, name string) bool {
	t.Helper()
	return git(t, dir, "branch", "--list", name) != ""
}

// fixture baut ein Repo mit origin und je einem Branch pro Fall; der Checkout steht auf `aktuell`.
//
//	sicher        ohne Upstream, per cherry-pick in develop (Patch gleich)
//	ungemergt     ohne Upstream, Commit fehlt in develop
//	weg-gemergt   Upstream weg, PR gemergt laut Stub, Commit fehlt in develop
//	weg-offen     Upstream weg, kein gemergter PR
//	sprint/xx1    sicher, aber Ordner in docs/sprints/aktiv
//	wt-sauber     sicher, eigener sauberer Worktree
//	wt-schmutzig  sicher, Worktree mit Änderung
//	aktuell       sicher, ausgecheckt
type fix struct{ dir, wtSauber, wtSchmutzig, wtDetached, wtDetachedAlt string }

func fixture(t *testing.T) fix {
	t.Helper()
	root := t.TempDir()
	origin := filepath.Join(root, "origin.git")
	dir := filepath.Join(root, "work")
	git(t, root, "init", "-q", "--bare", "-b", "develop", origin)
	git(t, root, "clone", "-q", origin, dir)
	git(t, dir, "checkout", "-q", "-b", "develop")
	commit(t, dir, "basis")
	git(t, dir, "branch", "main")
	for _, n := range []string{"sicher", "sprint/xx1", "wt-sauber", "wt-schmutzig", "aktuell"} {
		git(t, dir, "checkout", "-q", "-b", n, "develop")
		commit(t, dir, "f-"+strings.ReplaceAll(n, "/", "-"))
		git(t, dir, "checkout", "-q", "develop")
		git(t, dir, "cherry-pick", n)
	}
	git(t, dir, "push", "-q", "origin", "develop")
	for _, n := range []string{"ungemergt", "weg-gemergt", "weg-offen"} {
		git(t, dir, "checkout", "-q", "-b", n, "develop")
		commit(t, dir, "f-"+n)
	}
	git(t, dir, "checkout", "-q", "develop")
	for _, n := range []string{"weg-gemergt", "weg-offen"} {
		git(t, dir, "push", "-q", "-u", "origin", n)
		git(t, dir, "push", "-q", "origin", "--delete", n)
	}
	git(t, dir, "fetch", "-q", "--prune")
	if err := os.MkdirAll(filepath.Join(dir, "docs", "sprints", "aktiv", "XX1-test"), 0o755); err != nil {
		t.Fatal(err)
	}
	f := fix{dir: dir}
	f.wtSauber = filepath.Join(root, "wt-sauber")
	f.wtSchmutzig = filepath.Join(root, "wt-schmutzig")
	f.wtDetached = filepath.Join(root, "wt-detached")
	f.wtDetachedAlt = filepath.Join(root, "wt-detached-alt")
	git(t, dir, "worktree", "add", "-q", f.wtSauber, "wt-sauber")
	git(t, dir, "worktree", "add", "-q", f.wtSchmutzig, "wt-schmutzig")
	if err := os.WriteFile(filepath.Join(f.wtSchmutzig, "lose.txt"), []byte("x"), 0o644); err != nil {
		t.Fatal(err)
	}
	git(t, dir, "worktree", "add", "-q", "--detach", f.wtDetached, "develop")
	git(t, dir, "worktree", "add", "-q", "--detach", f.wtDetachedAlt, "ungemergt")
	git(t, dir, "checkout", "-q", "aktuell")
	return f
}

func opts(dir string, dry bool, pr func(string) (bool, error), out *strings.Builder) options {
	return options{Root: dir, Dry: dry, PRMerged: pr, Out: out, Git: execGit}
}

func prStub(merged ...string) func(string) (bool, error) {
	return func(b string) (bool, error) {
		for _, m := range merged {
			if m == b {
				return true, nil
			}
		}
		return false, nil
	}
}

func TestLoeschtNurSichereFaelle(t *testing.T) {
	f := fixture(t)
	var out strings.Builder
	if err := run(opts(f.dir, false, prStub("weg-gemergt"), &out)); err != nil {
		t.Fatal(err)
	}
	for _, n := range []string{"sicher", "weg-gemergt", "wt-sauber"} {
		if has(t, f.dir, n) {
			t.Errorf("%s sollte gelöscht sein\n%s", n, out.String())
		}
	}
	for _, n := range []string{"develop", "main", "aktuell", "sprint/xx1", "ungemergt", "weg-offen", "wt-schmutzig"} {
		if !has(t, f.dir, n) {
			t.Errorf("%s sollte bleiben\n%s", n, out.String())
		}
	}
	if _, err := os.Stat(f.wtSauber); err == nil {
		t.Error("sauberer Worktree sollte entfernt sein")
	}
	if _, err := os.Stat(filepath.Join(f.wtSchmutzig, "lose.txt")); err != nil {
		t.Error("schmutziger Worktree muss bleiben")
	}
	if _, err := os.Stat(f.wtDetached); err == nil {
		t.Error("sauberer detached Worktree auf develop sollte entfernt sein")
	}
	if _, err := os.Stat(f.wtDetachedAlt); err != nil {
		t.Error("detached Worktree auf ungemergtem Stand muss bleiben")
	}
}

func TestDryAendertNichts(t *testing.T) {
	f := fixture(t)
	before := git(t, f.dir, "branch", "--list") + git(t, f.dir, "worktree", "list")
	var out strings.Builder
	if err := run(opts(f.dir, true, prStub("weg-gemergt"), &out)); err != nil {
		t.Fatal(err)
	}
	after := git(t, f.dir, "branch", "--list") + git(t, f.dir, "worktree", "list")
	if before != after {
		t.Errorf("--dry hat etwas geändert:\n%s\n---\n%s", before, after)
	}
	for _, want := range []string{"würde löschen  Branch sicher – ", "würde löschen  Branch weg-gemergt – Upstream weg, Pull Request gemergt", "bleibt   Branch ungemergt – nicht gemergt, 1 Commits", "bleibt   Branch sprint/xx1 – Branch eines aktiven Sprints", "hat Änderungen oder ignorierte Dateien"} {
		if !strings.Contains(out.String(), want) {
			t.Errorf("Ausgabe nennt %q nicht:\n%s", want, out.String())
		}
	}
}

func TestOhneGhNurCherrySichere(t *testing.T) {
	f := fixture(t)
	var out strings.Builder
	noGh := func(string) (bool, error) { return false, errors.New("gh fehlt") }
	if err := run(opts(f.dir, false, noGh, &out)); err != nil {
		t.Fatal(err)
	}
	if has(t, f.dir, "sicher") {
		t.Error("cherry-sicherer Branch sollte auch ohne gh gelöscht sein")
	}
	for _, n := range []string{"weg-gemergt", "weg-offen"} {
		if !has(t, f.dir, n) {
			t.Errorf("%s darf ohne gh nicht gelöscht werden", n)
		}
	}
	if !strings.Contains(out.String(), "PR-Status unbekannt: gh fehlt") {
		t.Errorf("Ausgabe nennt den Grund nicht:\n%s", out.String())
	}
}
