package gitops

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// Change ist eine Datei im Working Tree.
type Change struct {
	Path   string `json:"path"`
	Code   string `json:"code"`   // M, A, D, R, C, U (Konflikt), ? (neu, ungetrackt)
	Staged bool   `json:"staged"` // im Index
}

// Stash ist ein Eintrag von git stash list.
type Stash struct {
	Ref     string `json:"ref"` // stash@{0}
	Age     string `json:"age"` // „vor 2 Stunden“
	Subject string `json:"subject"`
}

// Worktree ist ein weiterer Worktree (nicht die Wurzel).
type Worktree struct {
	Path   string `json:"path"`
	Branch string `json:"branch"` // leer bei detached HEAD
	Head   string `json:"head"`   // kurzer Hash
}

// Status ist der Stand des Checkouts für die Statuskarte.
type Status struct {
	Branch       string     `json:"branch"` // leer bei detached HEAD
	Upstream     string     `json:"upstream"`
	UpstreamGone bool       `json:"upstreamGone"`
	Ahead        int        `json:"ahead"`  // zum Upstream
	Behind       int        `json:"behind"` // zum Upstream
	BaseAhead    int        `json:"baseAhead"`
	BaseBehind   int        `json:"baseBehind"`
	MainBehind   int        `json:"mainBehind"` // main hinter origin/main
	Changes      []Change   `json:"changes"`
	Stashes      []Stash    `json:"stashes"`
	Worktrees    []Worktree `json:"worktrees"`
	FetchedAt    int64      `json:"fetchedAt"` // Unix-Millisekunden des letzten Fetch, 0 = unbekannt
	Advice       []string   `json:"advice"`
}

// Status liest den Stand des Checkouts.
func (r *Repo) Status(ctx context.Context) (Status, error) {
	out, err := r.git(ctx, "status", "--porcelain=v2", "--branch", "--untracked-files=all")
	if err != nil {
		return Status{}, err
	}
	s := parseStatus(out)
	s.BaseAhead, s.BaseBehind, _ = r.aheadBehind(ctx, "HEAD", baseRemote)
	_, s.MainBehind, _ = r.aheadBehind(ctx, Main, "origin/"+Main)
	if out, err := r.git(ctx, "stash", "list", "--format=%gd%x09%cr%x09%s"); err == nil {
		s.Stashes = parseStashes(out)
	}
	if out, err := r.git(ctx, "worktree", "list", "--porcelain"); err == nil {
		s.Worktrees = parseWorktrees(out)
	}
	s.FetchedAt = r.fetchedAt(ctx)
	s.Advice = advise(s)
	return s, nil
}

// aheadBehind zählt Commits in a, die b fehlen, und umgekehrt. ok ist falsch, wenn git scheitert (Ref fehlt, Frist,
// Sperre); dann sind die Zahlen bedeutungslos und dürfen nie als „nichts voraus“ gelten.
func (r *Repo) aheadBehind(ctx context.Context, a, b string) (ahead, behind int, ok bool) {
	out, err := r.git(ctx, "rev-list", "--left-right", "--count", a+"..."+b)
	if err != nil {
		return 0, 0, false
	}
	f := strings.Fields(out)
	if len(f) != 2 {
		return 0, 0, false
	}
	x, errX := strconv.Atoi(f[0])
	y, errY := strconv.Atoi(f[1])
	if errX != nil || errY != nil {
		return 0, 0, false
	}
	return x, y, true
}

func (r *Repo) fetchedAt(ctx context.Context) int64 {
	dir, err := r.git(ctx, "rev-parse", "--git-common-dir")
	if err != nil {
		return 0
	}
	if !filepath.IsAbs(dir) {
		dir = filepath.Join(r.root, dir)
	}
	st, err := os.Stat(filepath.Join(dir, "FETCH_HEAD"))
	if err != nil {
		return 0
	}
	return st.ModTime().UnixMilli()
}

// parseStatus wertet `git status --porcelain=v2 --branch` aus.
func parseStatus(out string) Status {
	s := Status{Changes: []Change{}, Stashes: []Stash{}, Worktrees: []Worktree{}}
	hasAB := false
	for _, l := range lines(out) {
		switch {
		case strings.HasPrefix(l, "# branch.head "):
			if h := strings.TrimPrefix(l, "# branch.head "); h != "(detached)" {
				s.Branch = h
			}
		case strings.HasPrefix(l, "# branch.upstream "):
			s.Upstream = strings.TrimPrefix(l, "# branch.upstream ")
		case strings.HasPrefix(l, "# branch.ab "):
			hasAB = true
			_, _ = fmt.Sscanf(strings.TrimPrefix(l, "# branch.ab "), "+%d -%d", &s.Ahead, &s.Behind)
		case strings.HasPrefix(l, "? "):
			s.Changes = append(s.Changes, Change{Path: l[2:], Code: "?"})
		case len(l) > 2 && (l[0] == '1' || l[0] == '2' || l[0] == 'u') && l[1] == ' ':
			s.Changes = append(s.Changes, parseEntry(l)...)
		}
	}
	s.UpstreamGone = s.Upstream != "" && !hasAB
	return s
}

// parseEntry liest eine Zeile 1 (geändert), 2 (umbenannt/kopiert) oder u (Konflikt). Eine Datei mit Änderungen im
// Index (X) und im Working Tree (Y) liefert zwei Einträge: einmal gestaged, einmal nicht.
func parseEntry(l string) []Change {
	fields := map[byte]int{'1': 8, '2': 9, 'u': 10}[l[0]] // Felder vor dem Pfad
	parts := strings.SplitN(l, " ", fields+1)
	path := parts[len(parts)-1]
	if l[0] == '2' {
		path, _, _ = strings.Cut(path, "\t") // neuer Pfad, dann Tab, dann alter
	}
	xy := parts[1]
	if l[0] == 'u' {
		return []Change{{Path: path, Code: "U"}}
	}
	var out []Change
	if xy[0] != '.' {
		out = append(out, Change{Path: path, Code: string(xy[0]), Staged: true})
	}
	if xy[1] != '.' {
		out = append(out, Change{Path: path, Code: string(xy[1])})
	}
	return out
}

// changedFiles zählt Dateien, nicht Einträge (gestaged und ungestaged zählt einmal).
func changedFiles(c []Change) int {
	seen := map[string]bool{}
	for _, x := range c {
		seen[x.Path] = true
	}
	return len(seen)
}

func parseStashes(out string) []Stash {
	list := []Stash{}
	for _, l := range lines(out) {
		f := strings.SplitN(l, "\t", 3)
		if len(f) == 3 {
			list = append(list, Stash{Ref: f[0], Age: f[1], Subject: f[2]})
		}
	}
	return list
}

// parseWorktrees liest `git worktree list --porcelain`; der erste Block ist die Wurzel und fällt weg.
func parseWorktrees(out string) []Worktree {
	list := []Worktree{}
	var cur *Worktree
	first := true
	for _, l := range strings.Split(strings.ReplaceAll(out, "\r", ""), "\n") {
		switch {
		case strings.HasPrefix(l, "worktree "):
			if !first {
				list = append(list, Worktree{Path: strings.TrimPrefix(l, "worktree ")})
				cur = &list[len(list)-1]
			}
			first = false
		case cur != nil && strings.HasPrefix(l, "HEAD "):
			cur.Head = shortHash(strings.TrimPrefix(l, "HEAD "))
		case cur != nil && strings.HasPrefix(l, "branch "):
			cur.Branch = strings.TrimPrefix(l, "branch refs/heads/")
		}
	}
	return list
}

func shortHash(h string) string {
	if len(h) > 8 {
		return h[:8]
	}
	return h
}

// advise leitet die nächsten Handgriffe aus dem Stand ab, passend zu „ein PR je Sprint gegen develop“.
func advise(s Status) []string {
	var a []string
	if len(s.Changes) > 0 {
		a = append(a, fmt.Sprintf("%d geänderte Dateien committen oder stashen", changedFiles(s.Changes)))
	}
	switch {
	case s.Branch == "":
		a = append(a, "Detached HEAD: auf einen Branch wechseln")
	case s.UpstreamGone:
		a = append(a, "Remote-Branch gelöscht (PR gemergt?): auf develop wechseln und den Branch aufräumen")
	case s.Behind > 0:
		a = append(a, fmt.Sprintf("Pull: %d Commits auf dem Remote", s.Behind))
	case s.Ahead > 0:
		a = append(a, fmt.Sprintf("Push: %d Commits nicht gepusht", s.Ahead))
	case s.Upstream == "" && !protected(s.Branch) && s.BaseAhead > 0:
		a = append(a, "Push mit Upstream, dann PR gegen develop öffnen")
	}
	if s.Branch == Base && s.BaseBehind > 0 && s.Behind == 0 {
		a = append(a, fmt.Sprintf("develop liegt %d hinter origin/develop: Fetch und Pull", s.BaseBehind))
	}
	if s.MainBehind > 0 {
		a = append(a, fmt.Sprintf("main liegt %d hinter origin/main (Fast-Forward macht Ralf)", s.MainBehind))
	}
	if a == nil {
		a = []string{}
	}
	return a
}
