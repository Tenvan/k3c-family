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
	root := nextRepo(t)
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
	git(t, root, "checkout", "-q", "-b", "tmp-origin")
	text := strings.Replace(string(b), "autonom | in Arbeit |", "autonom | fertig |", 1)
	if err := os.WriteFile(readme, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
	git(t, root, "commit", "-q", "-am", "A1.1 fertig")
	git(t, root, "update-ref", "refs/remotes/origin/sprint/a1", "HEAD")
	git(t, root, "checkout", "-q", "-")
	git(t, root, "branch", "-q", "-D", "tmp-origin")
	return root
}

// AC-01: Der Status auf origin/sprint/<präfix> gilt für Auswahl und Domänen-Sperre.
func TestNextOverlayOrigin(t *testing.T) {
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
	root := originNextRepo(t, true)
	out := must(t)(Next(root, NextFilter{Domain: "SRV"}))
	checkText(t, out, []string{"Nächste Session: A1.2", "A1: Stand von origin/sprint/a1"}, []string{"Keine Session frei"})
}

// AC-02: Ohne Branch auf origin gilt der lokale Stand, die Antwort sagt das.
func TestNextOhneOriginBranch(t *testing.T) {
	root := originNextRepo(t, false)
	out := must(t)(Next(root, NextFilter{Domain: "SRV"}))
	checkText(t, out, []string{"Keine Session frei in Domäne SRV", "A1.2 Domäne SRV gesperrt (A1.1 in Arbeit)",
		"A1: kein origin/sprint/a1, lokaler Stand"}, nil)
}
