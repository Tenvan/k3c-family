package gitops

import (
	"reflect"
	"strings"
	"testing"
)

func TestParseStatus(t *testing.T) {
	out := strings.Join([]string{
		"# branch.oid 73abdd1b",
		"# branch.head feat/git-seite",
		"# branch.upstream origin/feat/git-seite",
		"# branch.ab +2 -1",
		"1 .M N... 100644 100644 100644 aaa bbb src/a b.ts",
		"1 A. N... 000000 100644 100644 000 ccc docs/neu.md",
		"1 MM N... 100644 100644 100644 aaa bbb beides.go",
		"2 R. N... 100644 100644 100644 ddd eee R100 neu.go\talt.go",
		"u UU N... 100644 100644 100644 100644 f1 f2 f3 konflikt.go",
		"? tmp/ü.txt",
	}, "\n")
	s := parseStatus(out)
	if s.Branch != "feat/git-seite" || s.Upstream != "origin/feat/git-seite" || s.Ahead != 2 || s.Behind != 1 || s.UpstreamGone {
		t.Fatalf("Kopf falsch: %+v", s)
	}
	want := []Change{
		{Path: "src/a b.ts", Code: "M"},
		{Path: "docs/neu.md", Code: "A", Staged: true},
		{Path: "beides.go", Code: "M", Staged: true},
		{Path: "beides.go", Code: "M"},
		{Path: "neu.go", Code: "R", Staged: true},
		{Path: "konflikt.go", Code: "U"},
		{Path: "tmp/ü.txt", Code: "?"},
	}
	if !reflect.DeepEqual(s.Changes, want) {
		t.Fatalf("Changes = %+v", s.Changes)
	}
}

func TestParseStatusGoneUndDetached(t *testing.T) {
	s := parseStatus("# branch.head docs/x\n# branch.upstream origin/docs/x\n")
	if !s.UpstreamGone {
		t.Fatal("Upstream ohne branch.ab muss gone sein")
	}
	if d := parseStatus("# branch.head (detached)\n"); d.Branch != "" {
		t.Fatalf("detached: Branch = %q", d.Branch)
	}
}

func TestParseWorktreesOhneWurzel(t *testing.T) {
	out := "worktree C:/k3c\nHEAD 1111111111\nbranch refs/heads/develop\n\n" +
		"worktree C:/k3c/.claude/worktrees/a\nHEAD 4348d4fc99\ndetached\n\n" +
		"worktree C:/k3c/.claude/worktrees/b\nHEAD abcdef0123\nbranch refs/heads/sprint/k5\n"
	got := parseWorktrees(out)
	want := []Worktree{{Path: "C:/k3c/.claude/worktrees/a", Head: "4348d4fc"}, {Path: "C:/k3c/.claude/worktrees/b", Branch: "sprint/k5", Head: "abcdef01"}}
	if !reflect.DeepEqual(got, want) {
		t.Fatalf("Worktrees = %+v", got)
	}
}

func TestParseRefsUndTrack(t *testing.T) {
	out := strings.Join([]string{
		"refs/heads/develop\torigin/develop\t[behind 1]\ts1\t100\tKRT",
		"refs/heads/sprint/bd2\torigin/sprint/bd2\t[gone]\ts2\t90\tReview",
		"refs/heads/lokal\t\t\ts3\t80\tnur hier",
		"refs/heads/beides\torigin/beides\t[ahead 1, behind 2]\ts4\t70\tx",
		"refs/remotes/origin/HEAD\t\t\ts0\t0\t",
		"refs/remotes/origin/develop\t\t\ts1\t100\tKRT",
		"refs/remotes/origin/sprint/k5\t\t\ts5\t60\tK5",
	}, "\n")
	refs := parseRefs(out)
	var names []string
	var tracks []string
	for _, r := range refs {
		names = append(names, r.name)
		tr, _, _ := parseTrack(r)
		tracks = append(tracks, tr)
	}
	if !reflect.DeepEqual(names, []string{"develop", "sprint/bd2", "lokal", "beides", "sprint/k5"}) {
		t.Fatalf("Namen = %v", names)
	}
	if !reflect.DeepEqual(tracks, []string{TrackBehind, TrackGone, TrackNone, TrackBoth, TrackRemote}) {
		t.Fatalf("Track = %v", tracks)
	}
}

func TestParseDiff(t *testing.T) {
	got := parseDiff("A\tneu.go\nM\tsrc/a.ts\nD\tbild.png", "10\t0\tneu.go\n3\t2\tsrc/a.ts\n-\t-\tbild.png")
	want := []DiffFile{{Path: "neu.go", Code: "A", Added: 10}, {Path: "src/a.ts", Code: "M", Added: 3, Deleted: 2},
		{Path: "bild.png", Code: "D", Binary: true}}
	if !reflect.DeepEqual(got, want) {
		t.Fatalf("Diff = %+v", got)
	}
}

func TestValidBranch(t *testing.T) {
	for _, ok := range []string{"develop", "sprint/bd2", "feat/git-seite", "claude/k4-abschluss-review-487fe9", "w11_lauf.3"} {
		if !ValidBranch(ok) {
			t.Errorf("%q sollte gültig sein", ok)
		}
	}
	for _, bad := range []string{"", "-D", "--delete", "a..b", "a b", "a;b", "x/", "a//b", "x.lock", "ä", "a\nb", "$(x)"} {
		if ValidBranch(bad) {
			t.Errorf("%q sollte ungültig sein", bad)
		}
	}
}

func TestAdvise(t *testing.T) {
	cases := []struct {
		s    Status
		want string
	}{
		{Status{Branch: "f", Upstream: "origin/f", Ahead: 1}, "Push: 1 Commits nicht gepusht"},
		{Status{Branch: "f", Upstream: "origin/f", UpstreamGone: true}, "Remote-Branch gelöscht"},
		{Status{Branch: "f", BaseAhead: 3}, "Push mit Upstream"},
		{Status{Branch: "develop", Upstream: "origin/develop", BaseBehind: 2}, "develop liegt 2 hinter"},
		{Status{Branch: "f", MainBehind: 275}, "main liegt 275 hinter"},
		{Status{Changes: []Change{{Path: "a"}}, Branch: "f"}, "1 geänderte Dateien"},
	}
	for _, c := range cases {
		got := strings.Join(advise(c.s), " | ")
		if !strings.Contains(got, c.want) {
			t.Errorf("advise(%+v) = %q, erwartet %q", c.s, got, c.want)
		}
	}
	if got := advise(Status{Branch: "f", Upstream: "origin/f"}); len(got) != 0 {
		t.Errorf("sauberer Stand: %v", got)
	}
}

func TestLastLineUeberspringtHints(t *testing.T) {
	if got := lastLine("fatal: Not possible to fast-forward, aborting.\nhint: x\n"); got != "fatal: Not possible to fast-forward, aborting." {
		t.Fatalf("lastLine = %q", got)
	}
}
