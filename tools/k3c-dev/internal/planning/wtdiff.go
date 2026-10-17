package planning

import (
	"context"
	"fmt"
	"path/filepath"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/proc"
)

// Change ist ein Unterschied eines Worktrees zur Wurzel: Kind (neu | geändert), Objekt (Projekt, Sprint, Session,
// Ticket), seine ID und ein kurzer Text („Status aktiv → ruht“).
type Change struct {
	Kind string `json:"kind"` // neu | geändert | entfernt
	Item string `json:"item"`
	ID   string `json:"id"`
	Text string `json:"text"`
}

// WorktreeDiff fasst die Planungsunterschiede eines Worktrees zur Wurzel zusammen.
type WorktreeDiff struct {
	Branch  string   `json:"branch"`
	Path    string   `json:"path"`
	Changes []Change `json:"changes"`
	Error   string   `json:"error,omitempty"`
}

type worktree struct{ path, branch, head string }

// worktreeList liefert Ordner und Branch jedes Worktrees außer der Wurzel selbst (`git worktree list`).
func worktreeList(root string) ([]worktree, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()
	out, err := proc.Command(ctx, []string{"git", "-C", root, "worktree", "list", "--porcelain"}).Output()
	if err != nil {
		return nil, fmt.Errorf("git worktree list: %w", err)
	}
	return parseWorktreePaths(string(out), root), nil
}

func parseWorktreePaths(porcelain, root string) []worktree {
	var list []worktree
	var cur *worktree
	for _, l := range strings.Split(porcelain, "\n") {
		l = strings.TrimSpace(l)
		if p, ok := strings.CutPrefix(l, "worktree "); ok {
			list = append(list, worktree{path: filepath.Clean(p)})
			cur = &list[len(list)-1]
		} else if h, ok := strings.CutPrefix(l, "HEAD "); ok && cur != nil {
			cur.head = h
		} else if b, ok := strings.CutPrefix(l, "branch refs/heads/"); ok && cur != nil {
			cur.branch = b
		}
	}
	var others []worktree
	for _, w := range list {
		if !strings.EqualFold(w.path, filepath.Clean(root)) {
			others = append(others, w)
		}
	}
	return others
}

// CheckWorktrees zeigt je Worktree, was er an der Planung gegenüber der Merge-Base mit der Wurzel geändert hat
// (Commits und Arbeitskopie); der Vorsprung der Wurzel zählt nicht mit. Nur lesend, es wird nichts geschrieben.
func CheckWorktrees(root string) ([]WorktreeDiff, error) {
	list, err := worktreeList(root)
	if err != nil {
		return nil, err
	}
	diffs := []WorktreeDiff{}
	bases := map[string]Data{} // je Merge-Base einmal geladen
	for _, w := range list {
		d := WorktreeDiff{Branch: w.branch, Path: w.path, Changes: []Change{}}
		changes, err := diffWorktree(root, w, bases)
		if err != nil {
			d.Error = err.Error()
		} else {
			d.Changes = changes
		}
		diffs = append(diffs, d)
	}
	return diffs, nil
}

func diffWorktree(root string, w worktree, bases map[string]Data) ([]Change, error) {
	mb, err := mergeBase(root, w.head)
	if err != nil {
		return nil, err
	}
	base, ok := bases[mb]
	if !ok {
		if base, err = loadAt(root, mb); err != nil {
			return nil, err
		}
		bases[mb] = base
	}
	wt, err := loadWithArchive(w.path)
	if err != nil {
		return nil, err
	}
	return diffData(base, wt), nil
}

func diffData(base, wt Data) []Change {
	var cs []Change
	add := func(kind, item, id, text string) { cs = append(cs, Change{kind, item, id, text}) }
	field := func(item, id, name, a, b string) {
		if a != b {
			add("geändert", item, id, name+" "+a+" → "+b)
		}
	}
	bp := map[string]Project{}
	for _, p := range base.Projects {
		bp[p.ID] = p
	}
	wp := map[string]bool{}
	for _, p := range wt.Projects {
		wp[p.ID] = true
		old, ok := bp[p.ID]
		if !ok {
			add("neu", "Projekt", p.ID, p.Title)
			continue
		}
		field("Projekt", p.ID, "Status", old.Status, p.Status)
		field("Projekt", p.ID, "Rang", old.Rang, p.Rang)
	}
	for _, p := range base.Projects {
		if !wp[p.ID] {
			add("entfernt", "Projekt", p.ID, p.Title)
		}
	}
	bs := map[string]Sprint{}
	for _, s := range base.Sprints {
		bs[s.ID] = s
	}
	ws := map[string]bool{}
	for _, s := range wt.Sprints {
		ws[s.ID] = true
		old, ok := bs[s.ID]
		if !ok {
			add("neu", "Sprint", s.ID, s.Title)
			continue
		}
		field("Sprint", s.ID, "Status", old.Status, s.Status)
		field("Sprint", s.ID, "Projekt", old.Project, s.Project)
		cs = append(cs, diffSessions(old, s)...)
	}
	for _, s := range base.Sprints {
		if !ws[s.ID] {
			add("entfernt", "Sprint", s.ID, s.Title)
		}
	}
	bt := map[string]Ticket{}
	for _, t := range base.Tickets {
		bt[t.Nr] = t
	}
	wtk := map[string]bool{}
	for _, t := range wt.Tickets {
		wtk[t.Nr] = true
		old, ok := bt[t.Nr]
		if !ok {
			add("neu", "Ticket", t.Nr, t.Title)
			continue
		}
		field("Ticket", t.Nr, "Status", old.Status, t.Status)
		field("Ticket", t.Nr, "Prio", old.Prio, t.Prio)
		field("Ticket", t.Nr, "Sprint", old.Sprint, t.Sprint)
	}
	for _, t := range base.Tickets {
		if !wtk[t.Nr] {
			add("entfernt", "Ticket", t.Nr, t.Title)
		}
	}
	return cs
}

func diffSessions(old, now Sprint) []Change {
	var cs []Change
	prev := map[string]Session{}
	for _, s := range old.Sessions {
		prev[s.Nr] = s
	}
	have := map[string]bool{}
	for _, s := range now.Sessions {
		have[s.Nr] = true
		o, ok := prev[s.Nr]
		switch {
		case !ok:
			cs = append(cs, Change{"neu", "Session", s.Nr, s.Titel})
		case o.Status != s.Status:
			cs = append(cs, Change{"geändert", "Session", s.Nr, "Status " + o.Status + " → " + s.Status})
		}
	}
	for _, s := range old.Sessions {
		if !have[s.Nr] {
			cs = append(cs, Change{"entfernt", "Session", s.Nr, s.Titel})
		}
	}
	return cs
}
