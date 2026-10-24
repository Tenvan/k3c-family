package gitops

import (
	"context"
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// Echte Repos im Temp-Ordner: ein Bare-Repo als origin und ein Klon. So prüft der Test die Squash-Erkennung und das
// Löschen gegen git selbst, nicht gegen erfundene Ausgaben.

func sh(t *testing.T, dir string, args ...string) string {
	t.Helper()
	cmd := exec.Command("git", args...)
	cmd.Dir = dir
	cmd.Env = append(os.Environ(), "GIT_AUTHOR_NAME=t", "GIT_AUTHOR_EMAIL=t@t", "GIT_COMMITTER_NAME=t",
		"GIT_COMMITTER_EMAIL=t@t", "GIT_CONFIG_GLOBAL="+os.DevNull, "GIT_CONFIG_NOSYSTEM=1")
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git %v: %v\n%s", args, err, out)
	}
	return strings.TrimSpace(string(out))
}

func commit(t *testing.T, dir, file, text string) {
	t.Helper()
	if err := os.WriteFile(filepath.Join(dir, file), []byte(text), 0o600); err != nil {
		t.Fatal(err)
	}
	sh(t, dir, "add", file)
	sh(t, dir, "commit", "-q", "-m", file+" "+text)
}

// setup baut origin mit develop und main, einen Klon und drei Branches: squash (per Squash in develop),
// merged (Vorfahre von develop) und offen (eigene Arbeit).
func setup(t *testing.T) (*Repo, string) {
	t.Helper()
	if _, err := exec.LookPath("git"); err != nil {
		t.Skip("git fehlt")
	}
	// Auch der Runner des Repos (Stash braucht eine Identität) läuft ohne die Einstellungen des Rechners.
	for k, v := range map[string]string{"GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@t", "GIT_COMMITTER_NAME": "t",
		"GIT_COMMITTER_EMAIL": "t@t", "GIT_CONFIG_GLOBAL": os.DevNull, "GIT_CONFIG_NOSYSTEM": "1"} {
		t.Setenv(k, v)
	}
	base := t.TempDir()
	origin, work := filepath.Join(base, "origin.git"), filepath.Join(base, "work")
	sh(t, base, "init", "-q", "--bare", "-b", "develop", origin)
	sh(t, base, "clone", "-q", origin, work)
	sh(t, work, "checkout", "-q", "-b", "develop")
	commit(t, work, "a.txt", "1")
	sh(t, work, "push", "-q", "-u", "origin", "develop")
	sh(t, work, "push", "-q", "origin", "develop:main")
	sh(t, work, "branch", "main", "origin/main")

	sh(t, work, "checkout", "-q", "-b", "squash")
	commit(t, work, "s1.txt", "x")
	commit(t, work, "s2.txt", "y")
	sh(t, work, "push", "-q", "-u", "origin", "squash")
	sh(t, work, "checkout", "-q", "-b", "merged", "develop")
	sh(t, work, "checkout", "-q", "-b", "offen", "develop")
	commit(t, work, "o.txt", "z")
	sh(t, work, "checkout", "-q", "develop")
	sh(t, work, "merge", "-q", "--squash", "squash")
	sh(t, work, "commit", "-q", "-m", "Squash von squash (#1)")
	sh(t, work, "push", "-q", "origin", "develop")
	sh(t, work, "push", "-q", "origin", "--delete", "squash") // GitHub löscht nach dem Merge
	sh(t, work, "fetch", "-q", "--prune")
	return New(work), work
}

func byName(t *testing.T, list []Branch) map[string]Branch {
	t.Helper()
	m := map[string]Branch{}
	for _, b := range list {
		m[b.Name] = b
	}
	return m
}

func TestBranchesErkenntSquashUndGone(t *testing.T) {
	r, _ := setup(t)
	ctx := context.Background()
	before, err := r.Branches(ctx)
	if err != nil {
		t.Fatal(err)
	}
	if sq := byName(t, before)["squash"]; sq.State != StateOpen || sq.Checked || sq.Deletable {
		t.Fatalf("ohne Prüfung: %+v", sq) // die teure Prüfung läuft nur auf Knopfdruck
	}
	if err := r.Check(ctx, nil, nil); err != nil {
		t.Fatal(err)
	}
	list, err := r.Branches(ctx)
	if err != nil {
		t.Fatal(err)
	}
	b := byName(t, list)
	checks := []struct {
		name, state, track string
		deletable          bool
	}{
		{"develop", StateBase, TrackSync, false}, // ausgecheckt, aber Basis geht vor
		{"main", StateBase, TrackSync, false},
		{"squash", StateSquash, TrackGone, true},
		{"merged", StateMerged, TrackNone, true},
		{"offen", StateOpen, TrackNone, false},
	}
	for _, c := range checks {
		got := b[c.name]
		if got.State != c.state || got.Track != c.track || got.Deletable != c.deletable {
			t.Errorf("%s: State %q Track %q löschbar %v, erwartet %q %q %v", c.name, got.State, got.Track,
				got.Deletable, c.state, c.track, c.deletable)
		}
	}
	if b["offen"].BaseAhead != 1 || !b["offen"].Checked {
		t.Errorf("offen: %+v", b["offen"])
	}
}

// Sammel-PR: develop bekam den Branch zusammen mit anderem in einem Squash und änderte die Datei danach weiter.
// git allein erkennt das nicht (merge-tree Konflikt, Squash-Patch anders), der gemergte PR mit gleichem Head schon.
func TestCheckErkenntSammelPR(t *testing.T) {
	r, work := setup(t)
	ctx := context.Background()
	sh(t, work, "checkout", "-q", "-b", "viapr")
	commit(t, work, "a.txt", "2")
	sha := sh(t, work, "rev-parse", "HEAD")
	sh(t, work, "checkout", "-q", "develop")
	sh(t, work, "merge", "-q", "--squash", "viapr")
	if err := os.WriteFile(filepath.Join(work, "anderes.txt"), []byte("x"), 0o600); err != nil {
		t.Fatal(err)
	}
	sh(t, work, "add", "anderes.txt")
	sh(t, work, "commit", "-q", "-m", "Sammel-PR (#9)")
	commit(t, work, "a.txt", "3")
	sh(t, work, "push", "-q", "origin", "develop")
	sh(t, work, "fetch", "-q")

	if err := r.Check(ctx, []string{"viapr"}, nil); err != nil {
		t.Fatal(err)
	}
	list, _ := r.Branches(ctx)
	if v := byName(t, list)["viapr"]; v.State != StateOpen || !v.Checked || v.Deletable {
		t.Fatalf("nur git: %+v", v)
	}
	prs := []MergedPR{{Number: 3, Head: "anderer", SHA: sha}, {Number: 9, Head: "viapr", SHA: sha}}
	if err := r.Check(ctx, []string{"viapr"}, prs); err != nil {
		t.Fatal(err)
	}
	list, _ = r.Branches(ctx)
	if v := byName(t, list)["viapr"]; v.State != StateSquash || v.PR != 9 || !v.Deletable {
		t.Fatalf("mit PR: %+v", v)
	}
}

func TestDeleteNurGemergte(t *testing.T) {
	r, work := setup(t)
	ctx := context.Background()
	if _, err := r.Delete(ctx, []string{"squash"}, false); err == nil {
		t.Fatal("ungeprüfter Squash-Branch darf nicht löschbar sein")
	}
	if err := r.Check(ctx, nil, nil); err != nil {
		t.Fatal(err)
	}
	for _, names := range [][]string{{"offen"}, {"develop"}, {"main"}, {"gibtsnicht"}, {"-D"}, {"squash", "offen"}} {
		if _, err := r.Delete(ctx, names, false); err == nil {
			t.Errorf("Delete(%v) muss scheitern", names)
		}
	}
	if !strings.Contains(sh(t, work, "branch", "--list", "squash"), "squash") {
		t.Fatal("abgelehnter Aufruf darf nichts löschen")
	}
	msg, err := r.Delete(ctx, []string{"squash", "merged"}, true)
	if err != nil || !strings.HasPrefix(msg, "2 Branches gelöscht: squash (") || !strings.Contains(msg, "merged (") {
		t.Fatalf("Delete = %q, %v", msg, err)
	}
	if left := sh(t, work, "branch", "--list", "squash", "merged"); left != "" {
		t.Fatalf("noch da: %s", left)
	}
}

// Review B-580 (1): Scheitert die Messung zu develop, darf nichts als gemergt und löschbar gelten.
func TestMessfehlerGiltNieAlsGemergt(t *testing.T) {
	_, work := setup(t)
	real := execRunner(work)
	r := NewWith(work, func(ctx context.Context, args []string) ([]byte, error) {
		if args[0] == "rev-list" {
			return nil, errors.New("index.lock")
		}
		return real(ctx, args)
	})
	list, err := r.Branches(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	for _, b := range list {
		if b.Deletable || b.State == StateMerged || b.State == StateSquash {
			t.Errorf("%s: State %q löschbar %v trotz Messfehler", b.Name, b.State, b.Deletable)
		}
	}
	if _, err := r.Delete(context.Background(), []string{"merged"}, false); err == nil {
		t.Fatal("Delete trotz Messfehler")
	}
}

// Review B-580 (2): Liegen auf origin Commits, die develop fehlen, bleibt der Remote-Branch und auch der lokale.
func TestRemoteMitFremdenCommitsBleibt(t *testing.T) {
	r, work := setup(t)
	sh(t, work, "push", "-q", "-u", "origin", "merged")
	other := filepath.Join(filepath.Dir(work), "other")
	sh(t, filepath.Dir(work), "clone", "-q", "-b", "merged", filepath.Join(filepath.Dir(work), "origin.git"), other)
	commit(t, other, "fremd.txt", "nur auf origin")
	sh(t, other, "push", "-q", "origin", "merged")
	sh(t, work, "fetch", "-q")
	msg, err := r.Delete(context.Background(), []string{"merged"}, true)
	if err == nil || !strings.Contains(msg, "origin hat Commits") {
		t.Fatalf("Delete = %q, %v", msg, err)
	}
	if sh(t, work, "ls-remote", "--heads", "origin", "merged") == "" || sh(t, work, "branch", "--list", "merged") == "" {
		t.Fatal("Remote- und lokaler Branch müssen bleiben")
	}
}

func TestStatusUndAktionen(t *testing.T) {
	r, work := setup(t)
	ctx := context.Background()
	if _, err := r.Do(ctx, ActPush); err == nil || !strings.Contains(err.Error(), "develop") {
		t.Fatalf("Push auf develop muss scheitern: %v", err)
	}
	sh(t, work, "checkout", "-q", "offen")
	if err := os.WriteFile(filepath.Join(work, "neu.txt"), []byte("n"), 0o600); err != nil {
		t.Fatal(err)
	}
	st, err := r.Status(ctx)
	if err != nil || st.Branch != "offen" || len(st.Changes) != 1 || st.BaseAhead != 1 || st.Upstream != "" {
		t.Fatalf("Status = %+v, %v", st, err)
	}
	if _, err := r.Do(ctx, ActStash); err != nil {
		t.Fatal(err)
	}
	if st, _ := r.Status(ctx); len(st.Changes) != 0 || len(st.Stashes) != 1 {
		t.Fatalf("nach Stash: %+v", st)
	}
	if _, err := r.Do(ctx, ActStashPop); err != nil {
		t.Fatal(err)
	}
	pushAndFiles(t, r)
}

func pushAndFiles(t *testing.T, r *Repo) {
	t.Helper()
	ctx := context.Background()
	if msg, err := r.Do(ctx, ActPush); err != nil || !strings.Contains(msg, "Upstream") {
		t.Fatalf("Push = %q, %v", msg, err)
	}
	if st, _ := r.Status(ctx); st.Upstream != "origin/offen" || st.Ahead != 0 {
		t.Fatalf("nach Push: %+v", st)
	}
	files, err := r.BranchFiles(ctx)
	if err != nil || len(files.Files) != 1 || files.Files[0].Path != "o.txt" {
		t.Fatalf("BranchFiles = %+v, %v", files, err)
	}
	if _, err := r.Do(ctx, "rebase"); err == nil {
		t.Fatal("unbekannte Aktion muss scheitern")
	}
}
