//go:build windows

package planning

import (
	"os"
	"path/filepath"
	"strings"
	"syscall"
	"testing"
)

// winPath ruft GetShortPathName oder GetLongPathName auf; schlägt der Aufruf fehl (z. B. 8.3-Namen abgeschaltet), bleibt p.
func winPath(t *testing.T, p string, fn func(*uint16, *uint16, uint32) (uint32, error)) string {
	t.Helper()
	in, err := syscall.UTF16PtrFromString(p)
	if err != nil {
		t.Fatal(err)
	}
	buf := make([]uint16, syscall.MAX_PATH)
	n, err := fn(in, &buf[0], uint32(len(buf)))
	if err != nil || n == 0 || int(n) > len(buf) {
		return p
	}
	return syscall.UTF16ToString(buf[:n])
}

// formen liefert p in Kurz- und Langform (8.3). B-482: Der GitHub-Runner liefert den Temp-Ordner schon als Kurzpfad
// (RUNNER~1), git den Langpfad; der Test muss deshalb beide Formen aus p ableiten und nicht nur verkürzen.
// Fehlen die Kurznamen auf dem Laufwerk, sind beide Formen gleich und der Test wird übersprungen.
func formen(t *testing.T, p string) (short, long string) {
	t.Helper()
	short = winPath(t, p, syscall.GetShortPathName)
	long = winPath(t, short, syscall.GetLongPathName)
	if strings.EqualFold(short, long) {
		t.Skip("keine 8.3-Kurznamen auf diesem Laufwerk")
	}
	return short, long
}

func TestSamePathKurzpfad(t *testing.T) {
	dir := filepath.Join(t.TempDir(), "ein-langer-ordnername")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	short, long := formen(t, dir)
	if !samePath(long, short) || !samePath(short, long) {
		t.Errorf("derselbe Ordner in Lang- und Kurzform: %s / %s", long, short)
	}
	other := filepath.Join(filepath.Dir(dir), "anderer-ordnername")
	if err := os.MkdirAll(other, 0o755); err != nil {
		t.Fatal(err)
	}
	if samePath(short, other) {
		t.Errorf("verschiedene Ordner dürfen nicht gleich sein: %s / %s", short, other)
	}
}

func TestSamePathNichtLesbarVergleichtText(t *testing.T) {
	if !samePath(`C:\gibt\es\nicht`, `c:/gibt/es/nicht`) || samePath(`C:\gibt\es\nicht`, `C:\gibt\es\anders`) {
		t.Error("nicht lesbare Pfade: Textvergleich wie bisher")
	}
}

func TestWurzelInKurzformCheckWorktrees(t *testing.T) {
	root := gitRepo(t)
	short, _ := formen(t, root)
	wt := filepath.Join(t.TempDir(), "wt")
	git(t, root, "worktree", "add", "-q", "-b", "claude/k", wt)
	for _, st := range States { // git legt leere Ordner nicht an
		if err := os.MkdirAll(filepath.Join(wt, "docs", "sprints", st), 0o755); err != nil {
			t.Fatal(err)
		}
	}
	diffs, err := CheckWorktrees(short)
	if err != nil {
		t.Fatal(err)
	}
	if len(diffs) != 1 || diffs[0].Branch != "claude/k" {
		t.Fatalf("nur der Worktree erwartet, die Wurzel nicht: %+v", diffs)
	}
}

func TestWurzelInKurzformIstRepoWurzel(t *testing.T) {
	root := gitRepo(t)
	short, long := formen(t, root)
	wt := filepath.Join(t.TempDir(), "wt")
	git(t, root, "worktree", "add", "-q", "-b", "claude/n", wt)
	addTicket(t, wt, "B-007-im-worktree.md")
	for _, r := range []string{short, long} { // Kurz- wie Langform: dieselbe Wurzel, die Nummer kommt aus dem anderen Worktree
		if !isRepoRoot(r) {
			t.Errorf("Wurzel nicht erkannt: %s", r)
		}
		nr, notes, err := nextTicket(r)
		if err != nil {
			t.Fatal(err)
		}
		text := strings.Join(notes, "\n")
		if nr != "B-008" || strings.Contains(text, "keine Repo-Wurzel") || !strings.Contains(text, "Worktree wt") {
			t.Errorf("%s: Nummer %s, Hinweise %q", r, nr, notes)
		}
	}
}
