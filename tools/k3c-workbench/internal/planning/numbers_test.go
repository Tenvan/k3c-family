package planning

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

func git(t *testing.T, dir string, args ...string) string {
	t.Helper()
	cmd := exec.Command("git", append([]string{"-C", dir}, args...)...)
	cmd.Env = append(os.Environ(), "GIT_AUTHOR_NAME=t", "GIT_AUTHOR_EMAIL=t@t", "GIT_COMMITTER_NAME=t", "GIT_COMMITTER_EMAIL=t@t")
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git %v: %v\n%s", args, err, out)
	}
	return strings.TrimSpace(string(out))
}

// gitRepo macht tempRepo zu einem Git-Repo mit einem Commit; ohne git wird der Test übersprungen.
func gitRepo(t *testing.T) string {
	t.Helper()
	if _, err := exec.LookPath("git"); err != nil {
		t.Skip("git nicht verfügbar")
	}
	root := tempRepo(t)
	git(t, root, "init", "-q")
	git(t, root, "add", "-A")
	git(t, root, "commit", "-q", "-m", "init")
	return root
}

func addTicket(t *testing.T, root, name string) {
	t.Helper()
	p := filepath.Join(root, "docs", "backlog", name)
	if err := os.WriteFile(p, []byte("# "+name+"\n"), 0o644); err != nil {
		t.Fatal(err)
	}
}

func newTicket(t *testing.T, root string) string {
	t.Helper()
	return must(t)(Create(root, NewDoc{Kind: "ticket", Slug: "neu", Title: "Neu",
		Fields: map[string]string{"Domäne": "DEV", "Typ": "Idee", "Prio": "hoch"}}))
}

// B-387/AC-01 und AC-03: höhere Nummer in einem anderen Worktree, Antwort nennt den Checkout.
func TestTicketNummerAusAnderemWorktree(t *testing.T) {
	root := gitRepo(t)
	wt := filepath.Join(t.TempDir(), "wt-zwei")
	git(t, root, "worktree", "add", "-q", wt, "-b", "zwei")
	addTicket(t, wt, "B-005-fremd.md")
	out := newTicket(t, root)
	if !strings.Contains(out, "B-006 angelegt") {
		t.Fatalf("Nummer: %q", out)
	}
	if !strings.Contains(out, "wt-zwei") || !strings.Contains(out, "B-005") {
		t.Fatalf("Antwort nennt den Checkout nicht: %q", out)
	}
}

// B-387/AC-02: ein nicht lesbarer Worktree wird übersprungen und genannt, die Anlage gelingt.
func TestTicketNummerWorktreeNichtLesbar(t *testing.T) {
	root := gitRepo(t)
	wt := filepath.Join(t.TempDir(), "wt-kaputt")
	git(t, root, "worktree", "add", "-q", wt, "-b", "kaputt")
	if err := os.RemoveAll(filepath.Join(wt, "docs", "backlog")); err != nil {
		t.Fatal(err)
	}
	out := newTicket(t, root)
	if !strings.Contains(out, "B-002 angelegt") || !strings.Contains(out, "übersprungen") || !strings.Contains(out, "wt-kaputt") {
		t.Fatalf("Antwort: %q", out)
	}
}

// B-387/AC-04: Ein Remote-Ref ohne Worktree zählt mit, ohne Fetch.
func TestTicketNummerAusRemoteRef(t *testing.T) {
	root := gitRepo(t)
	wt := filepath.Join(t.TempDir(), "wt-ref")
	git(t, root, "worktree", "add", "-q", wt, "-b", "nurref")
	addTicket(t, wt, "B-007-remote.md")
	git(t, wt, "add", "-A")
	git(t, wt, "commit", "-q", "-m", "ticket")
	git(t, root, "update-ref", "refs/remotes/origin/sprint/k9", git(t, wt, "rev-parse", "HEAD"))
	git(t, root, "worktree", "remove", "--force", wt)
	out := newTicket(t, root)
	if !strings.Contains(out, "B-008 angelegt") || !strings.Contains(out, "origin/sprint/k9") {
		t.Fatalf("Antwort: %q", out)
	}
}

// B-403/N1: liegt der Ordner unterhalb eines Repos (keine Wurzel), nennt die Antwort, dass Worktrees und Refs fehlen.
func TestTicketNummerKeineRepoWurzelNennt(t *testing.T) {
	root := gitRepo(t)
	sub := filepath.Join(root, "unterordner")
	if err := os.MkdirAll(filepath.Join(sub, "docs", "backlog"), 0o755); err != nil {
		t.Fatal(err)
	}
	_, notes, err := nextTicket(sub)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(strings.Join(notes, "\n"), "keine Repo-Wurzel") {
		t.Fatalf("Hinweis fehlt: %q", notes)
	}
}

// B-360/AC-01: ein Feld, das in der Datei fehlt, wird abgelehnt, die Datei bleibt unverändert.
func TestFehlendesFeldWirdAbgelehnt(t *testing.T) {
	root := tempRepo(t)
	before := doc(t, root, "backlog/B-001-alt.md")
	_, err := Set(root, "B-001", map[string]string{"Prio": "hoch"})
	if err == nil || !strings.Contains(err.Error(), "Prio") {
		t.Fatalf("erwartet Fehler mit Feldname, bekam %v", err)
	}
	if after := doc(t, root, "backlog/B-001-alt.md"); after != before {
		t.Fatalf("Datei geändert:\n%s", after)
	}
}
