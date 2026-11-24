package gitcommit

import (
	"context"
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

func TestMessage(t *testing.T) {
	m, err := Message{Type: " feat ", Scope: "srv", Subject: " Git-Seite ", Body: "Rumpf"}.Normalize()
	if err != nil || m.Format() != "feat(srv): Git-Seite\n\nRumpf\n" {
		t.Fatalf("%q %v", m.Format(), err)
	}
	for field, bad := range map[string]Message{
		"type":    {Type: "wip", Subject: "x"},
		"scope":   {Type: "fix", Scope: "Srv Neu", Subject: "x"},
		"subject": {Type: "fix", Subject: ""},
	} {
		var fe *FieldError
		if _, err := bad.Normalize(); !errors.As(err, &fe) || fe.Field != field {
			t.Errorf("%s: %v", field, err)
		}
	}
	if (Message{Type: "docs", Subject: "kurz"}).Format() != "docs: kurz\n" {
		t.Error("ohne Scope und Rumpf")
	}
}

func TestParseStatus(t *testing.T) {
	out := "## main...origin/main\x00M  a.go\x00 M b.go\x00MM c.go\x00R  neu.go\x00alt.go\x00?? d.go\x00UU e.go\x00"
	branch, staged, unstaged, err := parseStatus([]byte(out))
	if err != nil || branch != "main" {
		t.Fatalf("%q %v", branch, err)
	}
	if len(staged) != 3 || staged[2].Path != "neu.go" || staged[2].Status != "R" {
		t.Errorf("staged: %+v", staged)
	}
	if len(unstaged) != 4 || unstaged[2].Status != "?" || unstaged[3].Status != "U" {
		t.Errorf("unstaged: %+v", unstaged)
	}
}

func TestCheckPaths(t *testing.T) {
	for _, bad := range []string{"", "../x", "a/../../x", "/abs", "C:/x", "a\nb"} {
		if checkPaths([]string{bad}) == nil {
			t.Errorf("%q muss abgelehnt werden", bad)
		}
	}
	if err := checkPaths([]string{"a/b.go", "dir\\c.go"}); err != nil {
		t.Error(err)
	}
}

func run(root string, args ...string) error {
	_, err := runGit(context.Background(), root, nil, args...)
	return err
}

func must(t *testing.T, err error) {
	t.Helper()
	if err != nil {
		t.Fatal(err)
	}
}

func newRepo(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	for _, args := range [][]string{{"init", "-q"}, {"config", "user.email", "t@t"}, {"config", "user.name", "t"}, {"config", "commit.gpgsign", "false"}} {
		must(t, run(root, args...))
	}
	must(t, os.WriteFile(filepath.Join(root, "a.txt"), []byte("eins\n"), 0o644))
	return root
}

func mustRead(t *testing.T, root string) State {
	t.Helper()
	st, err := Read(context.Background(), root)
	must(t, err)
	return st
}

// TestRepo geht den ganzen Weg in einem echten Repo: untracked → stagen → committen → Index leer.
func TestRepo(t *testing.T) {
	if _, err := exec.LookPath("git"); err != nil {
		t.Skip("git nicht im PATH")
	}
	ctx, root := context.Background(), newRepo(t)
	if _, err := Commit(ctx, root, Message{Type: "chore", Subject: "leer"}); err == nil || !strings.Contains(err.Error(), "nichts gestaged") {
		t.Fatalf("leerer Index: %v", err)
	}
	if st := mustRead(t, root); len(st.Unstaged) != 1 || st.Unstaged[0].Status != "?" {
		t.Fatalf("untracked: %+v", st)
	}
	must(t, Stage(ctx, root, []string{"a.txt"}))
	if st := mustRead(t, root); len(st.Staged) != 1 || st.Staged[0].Status != "A" {
		t.Fatalf("nach Stage: %+v", st)
	}
	must(t, Unstage(ctx, root, []string{"a.txt"})) // vor dem ersten Commit: rm --cached
	if st := mustRead(t, root); len(st.Staged) != 0 {
		t.Fatalf("nach Unstage: %+v", st)
	}
	must(t, Stage(ctx, root, []string{"a.txt"}))
	hash, err := Commit(ctx, root, Message{Type: "feat", Scope: "srv", Subject: "erster", Body: "Rumpf"})
	if err != nil || hash == "" {
		t.Fatalf("%q %v", hash, err)
	}
	st := mustRead(t, root)
	if len(st.Staged)+len(st.Unstaged) != 0 || len(st.Recent) != 1 || !strings.Contains(st.Recent[0], "feat(srv): erster") {
		t.Fatalf("nach Commit: %+v", st)
	}
}
