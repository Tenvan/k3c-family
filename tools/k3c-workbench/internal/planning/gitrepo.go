package planning

import (
	"os"
	"path/filepath"
)

// inRepo: Liegt dir in einem Git-Checkout (`.git` als Ordner oder Datei in dir oder darüber)? Ohne `.git` scheitert jeder
// git-Aufruf dort ohnehin; die Prüfung spart den Prozess (unter Windows samt Virenscanner gut 100 ms, B-830). Im echten
// Checkout gibt es `.git` immer, dann fragt der Aufrufer git wie bisher. Im Zweifel (Pfad nicht auflösbar) true.
func inRepo(dir string) bool {
	abs, err := filepath.Abs(dir)
	if err != nil {
		return true
	}
	for {
		if _, err := os.Stat(filepath.Join(abs, ".git")); err == nil {
			return true
		}
		parent := filepath.Dir(abs)
		if parent == abs {
			return false
		}
		abs = parent
	}
}

// hasGitEntry: dir selbst trägt `.git` (Wurzel eines Repos oder Worktrees).
func hasGitEntry(dir string) bool {
	_, err := os.Stat(filepath.Join(dir, ".git"))
	return err == nil
}
