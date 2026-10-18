package planning

import (
	"context"
	"fmt"
	"maps"
	"os"
	"os/exec"
	"path"
	"path/filepath"
	"regexp"
	"slices"
	"strconv"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/proc"
)

// reTicketFile liest die Nummer aus dem Dateinamen eines Tickets (B-387-name.md).
var reTicketFile = regexp.MustCompile(`^B-(\d+)-.*\.md$`)

// B-387: Die nächste Ticket-Nummer berücksichtigt alle Checkouts des Repos (Wurzel, Git-Worktrees samt Archiv) und die
// lokalen Remote-Refs origin/develop und origin/sprint/* (aus dem lokalen Stand, ohne Fetch).

// highestTicket ist die größte Ticket-Nummer in den Dateinamen, 0 ohne Ticket.
func highestTicket(names []string) int {
	top := 0
	for _, n := range names {
		if m := reTicketFile.FindStringSubmatch(path.Base(n)); m != nil {
			nr, _ := strconv.Atoi(m[1])
			top = max(top, nr)
		}
	}
	return top
}

// checkoutHighest liest backlog/ und backlog/archiv/ eines Checkouts; ohne backlog/ ist er nicht lesbar.
func checkoutHighest(root string) (int, error) {
	var top int
	for _, dir := range []string{"backlog", "backlog/archiv"} {
		entries, err := os.ReadDir(docsPath(root, dir))
		if err != nil {
			if dir == "backlog" {
				return 0, err
			}
			continue
		}
		names := make([]string, 0, len(entries))
		for _, e := range entries {
			names = append(names, e.Name())
		}
		top = max(top, highestTicket(names))
	}
	return top, nil
}

func gitOut(root string, args ...string) (string, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	out, err := proc.Command(ctx, append([]string{"git", "-C", root}, args...)).Output()
	return string(out), err
}

// worktreePaths liefert die Pfade aller Worktrees (`git worktree list`), die Wurzel eingeschlossen.
func worktreePaths(root string) ([]string, error) {
	out, err := gitOut(root, "worktree", "list", "--porcelain")
	if err != nil {
		return nil, err
	}
	var paths []string
	for _, l := range strings.Split(strings.ReplaceAll(out, "\r\n", "\n"), "\n") {
		if p, ok := strings.CutPrefix(l, "worktree "); ok {
			paths = append(paths, p)
		}
	}
	return paths, nil
}

// samePath: derselbe Ordner. Maßgeblich ist die Identität (os.SameFile): git nennt den Langpfad, ein Aufrufer kann einen
// 8.3-Kurzpfad (RUNNER~1 statt runneradmin) oder einen Link übergeben (B-482). Ist ein Pfad nicht lesbar, gilt der Textvergleich.
func samePath(a, b string) bool {
	if strings.EqualFold(filepath.ToSlash(filepath.Clean(a)), filepath.ToSlash(filepath.Clean(b))) {
		return true
	}
	ia, errA := os.Stat(a)
	ib, errB := os.Stat(b)
	return errA == nil && errB == nil && os.SameFile(ia, ib)
}

// remoteHighest liefert je lokalem Remote-Ref (origin/develop, origin/sprint/*) die größte Ticket-Nummer.
func remoteHighest(root string) map[string]int {
	out, err := gitOut(root, "for-each-ref", "--format=%(refname)", "refs/remotes/origin/develop", "refs/remotes/origin/sprint")
	if err != nil {
		return nil
	}
	found := map[string]int{}
	for _, ref := range strings.Fields(out) {
		tree, err := gitOut(root, "ls-tree", "--name-only", ref, "--", "docs/backlog/", "docs/backlog/archiv/")
		if err != nil {
			continue
		}
		found[strings.TrimPrefix(ref, "refs/remotes/")] = highestTicket(strings.Fields(tree))
	}
	return found
}

// nextTicket vergibt die nächste Nummer über alle Checkouts und Remote-Refs; die Hinweise nennen den bestimmenden
// Checkout, übersprungene Worktrees und fehlendes git.
func nextTicket(root string) (string, []string, error) {
	own, err := checkoutHighest(root)
	if err != nil {
		return "", nil, err
	}
	top, from := own, "eigener Checkout"
	var notes []string
	consider := func(n int, name string) {
		if n > top {
			top, from = n, name
		}
	}
	if _, err := exec.LookPath("git"); err != nil {
		notes = append(notes, "Warnung: git nicht gefunden, Nummer nur aus dem eigenen Checkout")
	} else if !isRepoRoot(root) {
		notes = append(notes, "Warnung: Ordner ist keine Repo-Wurzel, Worktrees und Refs nicht gelesen")
	} else {
		paths, err := worktreePaths(root)
		if err != nil {
			notes = append(notes, fmt.Sprintf("Warnung: Worktrees nicht gelesen (%v)", err))
		}
		for _, p := range paths {
			if samePath(p, root) {
				continue
			}
			n, err := checkoutHighest(p)
			if err != nil {
				notes = append(notes, fmt.Sprintf("übersprungen: Worktree %s nicht lesbar (%v)", filepath.Base(p), err))
				continue
			}
			consider(n, "Worktree "+filepath.Base(p))
		}
		refs := remoteHighest(root)
		if refs == nil {
			notes = append(notes, "Warnung: Remote-Refs nicht gelesen")
		}
		for _, ref := range slices.Sorted(maps.Keys(refs)) { // feste Reihenfolge: bei Gleichstand gewinnt der erste Name
			consider(refs[ref], ref)
		}
	}
	nr := fmt.Sprintf("B-%03d", top+1)
	return nr, append(notes, fmt.Sprintf("%s vergeben, höchste bekannte Nummer B-%03d: %s", nr, top, from)), nil
}

// isRepoRoot: git-Auskünfte gelten nur, wenn root selbst die Wurzel eines Worktrees ist; ein Ordner unterhalb eines
// Repos (z. B. ein Test-Verzeichnis) würde sonst die Worktrees und Refs des umgebenden Repos liefern.
func isRepoRoot(root string) bool {
	top, err := gitOut(root, "rev-parse", "--show-toplevel")
	return err == nil && samePath(strings.TrimSpace(top), root)
}
