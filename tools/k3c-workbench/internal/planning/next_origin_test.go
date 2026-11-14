package planning

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// originNextRepo: nextRepo als Git-Repo; origin/sprint/a1 hat A1.1 schon auf fertig (lokal: in Arbeit, sperrt SRV).
func originNextRepo(t *testing.T, withRef bool) string {
	t.Helper()
	if _, err := exec.LookPath("git"); err != nil {
		t.Skip("git nicht verfügbar")
	}
	return originRepoFrom(t, nextRepo(t), withRef)
}

// originRepoFrom macht aus dem nextRepo root ein Git-Repo, wahlweise mit origin/sprint/a1 (A1.1 dort fertig).
func originRepoFrom(t *testing.T, root string, withRef bool) string {
	t.Helper()
	git(t, root, "init", "-q")
	git(t, root, "add", "-A")
	git(t, root, "commit", "-q", "-m", "init")
	if !withRef {
		return root
	}
	readme := filepath.Join(root, "docs", "sprints", "aktiv", "A1-s", "README.md")
	b, err := os.ReadFile(readme)
	if err != nil {
		t.Fatal(err)
	}
	// Commit auf dem Branch, Ref darauf, Branch zurück: drei git-Aufrufe statt fünf (jeder kostet unter Windows gut 100 ms, B-830).
	text := strings.Replace(string(b), "autonom | in Arbeit |", "autonom | fertig |", 1)
	if err := os.WriteFile(readme, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
	git(t, root, "commit", "-q", "-am", "A1.1 fertig")
	git(t, root, "update-ref", "refs/remotes/origin/sprint/a1", "HEAD")
	git(t, root, "reset", "-q", "--hard", "HEAD~1")
	return root
}

// AC-01: Der Status auf origin/sprint/<präfix> gilt für Auswahl und Domänen-Sperre.
func TestNextOverlayOrigin(t *testing.T) {
	t.Parallel()
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
	root := originNextRepo(t, true)
	out := must(t)(Next(root, NextFilter{Domain: "SRV"}))
	checkText(t, out, []string{"Nächste Session: A1.2", "A1: Stand von origin/sprint/a1"}, []string{"Keine Session frei"})
}

// AC-02: Ohne Branch auf origin gilt der lokale Stand, die Antwort sagt das.
func TestNextOhneOriginBranch(t *testing.T) {
	t.Parallel()
	root := originNextRepo(t, false)
	out := must(t)(Next(root, NextFilter{Domain: "SRV"}))
	checkText(t, out, []string{"Keine Session frei in Domäne SRV", "A1.2 Sprint A1 gesperrt (A1.1 in Arbeit)",
		"A1: kein origin/sprint/a1, lokaler Stand"}, nil)
}

// B-782/AC-02: Trägt die Session `Branch: sprint/a1`, liest das Overlay origin/sprint/a1 (nicht origin/sprint/sprint).
func TestNextOverlayBranchMitSprintPraefix(t *testing.T) {
	t.Parallel()
	if _, err := exec.LookPath("git"); err != nil {
		t.Skip("git nicht verfügbar")
	}
	root := nextRepo(t)
	files, err := filepath.Glob(filepath.Join(root, "docs", "sprints", "aktiv", "A1-s", "A1.*-x.md"))
	if err != nil || len(files) == 0 {
		t.Fatalf("Session-Dateien: %v %v", files, err)
	}
	for _, f := range files {
		b, err := os.ReadFile(f)
		if err != nil {
			t.Fatal(err)
		}
		text := strings.Replace(string(b), "**Branch:** a1/1-x", "**Branch:** sprint/a1", 1)
		if err := os.WriteFile(f, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	originRepoFrom(t, root, true)
	d, err := Load(root)
	if err != nil {
		t.Fatal(err)
	}
	notes, _ := originOverlay(root, &d)
	checkText(t, strings.Join(notes, "\n"), []string{"A1: Stand von origin/sprint/a1"}, []string{"sprint/sprint"})
}
