// Package gitops liest und pflegt das Git-Repo der Workbench für die Seite Git (B-580): Status, Branches mit
// Squash-Erkennung, geänderte Dateien, Fetch, Pull, Push, Stash und das Löschen gemergter Branches. Alle Aufrufe laufen
// ohne Shell mit festen Argumenten; Namen aus der Oberfläche prüft ValidBranch, bevor git sie sieht.
package gitops

import (
	"context"
	"errors"
	"os"
	"os/exec"
	"strings"
	"sync"
	"time"

	"k3c/tools/k3c-workbench/internal/proc"
)

// Base ist der Ziel-Branch aller PRs, Main der Release-Branch; beide lassen sich nicht löschen und nicht pushen.
const (
	Base       = "develop"
	Main       = "main"
	baseRemote = "origin/" + Base
	timeout    = 60 * time.Second
)

// Runner führt git mit args (ohne "git") im Repo aus und liefert stdout; ein Fehler trägt die erste Zeile von stderr.
type Runner func(ctx context.Context, args []string) ([]byte, error)

// Repo ist das Repo unter root.
type Repo struct {
	root string
	run  Runner
	mu   sync.Mutex // eine schreibende Aktion zur Zeit
	memo sync.Map   // Merge-Zustand je "branch-sha develop-sha" (merged.go)

	dirOnce sync.Once // gemeinsamer Git-Ordner für Stamp (stamp.go)
	dir     string
	dirErr  error
}

// New baut ein Repo für root mit dem echten git.
func New(root string) *Repo { return &Repo{root: root, run: execRunner(root)} }

// NewWith baut ein Repo mit eigenem Runner (Tests).
func NewWith(root string, run Runner) *Repo { return &Repo{root: root, run: run} }

func execRunner(root string) Runner {
	return func(ctx context.Context, args []string) ([]byte, error) {
		cmd := proc.Command(ctx, append([]string{"git", "-c", "core.quotePath=false"}, args...))
		cmd.Dir = root
		cmd.Env = append(os.Environ(), "GIT_TERMINAL_PROMPT=0", "GIT_OPTIONAL_LOCKS=0")
		out, err := cmd.Output()
		if err != nil {
			return out, gitError(args, err)
		}
		return out, nil
	}
}

// gitError macht aus einem Fehler von git einen Satz für die Oberfläche.
func gitError(args []string, err error) error {
	sub := "git"
	if len(args) > 0 {
		sub = "git " + args[0]
	}
	var exit *exec.ExitError
	switch {
	case errors.Is(err, exec.ErrNotFound):
		return errors.New("git nicht gefunden")
	case errors.Is(err, context.DeadlineExceeded):
		return errors.New(sub + ": keine Antwort (Netz?)")
	case errors.As(err, &exit) && len(exit.Stderr) > 0:
		return errors.New(sub + ": " + lastLine(string(exit.Stderr)))
	}
	return errors.New(sub + ": " + err.Error())
}

// lastLine ist die letzte nicht leere Zeile; bei git steht dort der eigentliche Grund (vor ihr oft „hint:“-Zeilen).
func lastLine(s string) string {
	lines := strings.Split(strings.TrimSpace(s), "\n")
	for i := len(lines) - 1; i >= 0; i-- {
		if l := strings.TrimSpace(lines[i]); l != "" && !strings.HasPrefix(l, "hint:") {
			return l
		}
	}
	return strings.TrimSpace(s)
}

// git ruft git mit Frist auf und liefert stdout ohne Zeilenende am Schluss.
func (r *Repo) git(ctx context.Context, args ...string) (string, error) {
	ctx, cancel := context.WithTimeout(ctx, timeout)
	defer cancel()
	out, err := r.run(ctx, args)
	return strings.TrimRight(string(out), "\r\n"), err
}

// lines teilt eine Ausgabe in Zeilen ohne leere.
func lines(s string) []string {
	var out []string
	for _, l := range strings.Split(s, "\n") {
		if l = strings.TrimRight(l, "\r"); l != "" {
			out = append(out, l)
		}
	}
	return out
}

// ValidBranch prüft einen Branch-Namen aus der Oberfläche: keine Leerzeichen und Steuerzeichen, kein führendes "-",
// kein "..", nur Zeichen, die in diesem Repo vorkommen. Strenger als git check-ref-format, damit nichts als Option gilt.
func ValidBranch(name string) bool {
	if name == "" || len(name) > 200 || strings.HasPrefix(name, "-") || strings.Contains(name, "..") ||
		strings.HasSuffix(name, "/") || strings.HasSuffix(name, ".lock") || strings.Contains(name, "//") {
		return false
	}
	return strings.IndexFunc(name, func(c rune) bool { return !branchRune(c) }) < 0
}

func branchRune(c rune) bool {
	return strings.ContainsRune("/-_.", c) || (c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

// protected sind Branches, die die Seite weder löscht noch pusht.
func protected(name string) bool { return name == Base || name == Main }
