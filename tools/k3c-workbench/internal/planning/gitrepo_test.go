package planning

import (
	"os"
	"path/filepath"
	"testing"
)

// B-830: inRepo findet .git (Ordner oder Datei) im Ordner oder darüber; ohne .git kein git-Aufruf.
func TestInRepo(t *testing.T) {
	t.Parallel()
	base := t.TempDir()
	sub := filepath.Join(base, "repo", "docs", "backlog")
	if err := os.MkdirAll(sub, 0o755); err != nil {
		t.Fatal(err)
	}
	if inRepo(sub) {
		t.Skip("Temp-Ordner liegt in einem Git-Checkout (CI: .work/tmp)")
	}
	if hasGitEntry(filepath.Join(base, "repo")) {
		t.Fatal("ohne .git als Repo erkannt")
	}
	if err := os.WriteFile(filepath.Join(base, "repo", ".git"), []byte("gitdir: x\n"), 0o644); err != nil { // Worktree: .git ist eine Datei
		t.Fatal(err)
	}
	if !inRepo(sub) || !hasGitEntry(filepath.Join(base, "repo")) || hasGitEntry(sub) {
		t.Fatal(".git-Datei nicht erkannt")
	}
	if Worktrees(base) != nil {
		t.Fatal("Worktrees ohne Repo")
	}
}
