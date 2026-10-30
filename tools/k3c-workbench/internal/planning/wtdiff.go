package planning

import (
	"context"
	"fmt"
	"path/filepath"
	"strings"
	"time"

	"k3c/tools/k3c-workbench/internal/proc"
)

// Change ist ein Unterschied eines Worktrees zur Wurzel: Kind (neu | geändert), Objekt (Projekt, Sprint, Session,
// Ticket), seine ID und ein kurzer Text („Status aktiv → ruht“).
type Change struct {
	Kind string `json:"kind"` // neu | geändert | entfernt
	Item string `json:"item"`
	ID   string `json:"id"`
	Text string `json:"text"`
	// Project und Sprint der neuen Einträge (Sprint, Ticket), damit die Ansicht sie unter ihrem Projekt zeigt
	Project string `json:"project,omitempty"`
	Sprint  string `json:"sprint,omitempty"`
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
		if !samePath(w.path, root) {
			others = append(others, w)
		}
	}
	return others
}

// CheckWorktrees zeigt je Worktree, was er an der Planung gegenüber der Merge-Base mit der Wurzel geändert hat
// (Commits und Arbeitskopie); der Vorsprung der Wurzel zählt nicht mit. Nur lesend, es wird nichts geschrieben.
//
// Gleichzeitige Aufrufe teilen sich einen Lauf (flight), die Stände der Merge-Bases bleiben zwischen den Läufen im Cache:
// Die Oberfläche ruft bei jedem Ereignis und jedem Fokus, ohne das würden sich die Läufe samt `git archive` stauen.
func CheckWorktrees(root string) ([]WorktreeDiff, error) {
	arrived := time.Now()
	return flightFor(root).run(arrived, func(bases map[string]Data) ([]WorktreeDiff, error) {
		return checkWorktrees(root, bases)
	})
}

func checkWorktrees(root string, bases map[string]Data) ([]WorktreeDiff, error) {
	list, err := worktreeList(root)
	if err != nil {
		return nil, err
	}
	diffs := []WorktreeDiff{}
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
	cs := diffByID("Projekt", base.Projects, wt.Projects, func(p Project) (string, string) { return p.ID, p.Title },
		func(old, p Project) []Change {
			return append(changed("Projekt", p.ID, "Status", old.Status, p.Status), changed("Projekt", p.ID, "Rang", old.Rang, p.Rang)...)
		})
	cs = append(cs, withNewSessions(diffByID("Sprint", base.Sprints, wt.Sprints, func(s Sprint) (string, string) { return s.ID, s.Title },
		func(old, s Sprint) []Change {
			out := append(changed("Sprint", s.ID, "Status", old.Status, s.Status), changed("Sprint", s.ID, "Projekt", old.Project, s.Project)...)
			return append(out, diffSessions(old, s)...)
		}), wt.Sprints)...)
	cs = append(cs, diffByID("Ticket", base.Tickets, wt.Tickets, func(t Ticket) (string, string) { return t.Nr, t.Title },
		func(old, t Ticket) []Change {
			out := append(changed("Ticket", t.Nr, "Status", old.Status, t.Status), changed("Ticket", t.Nr, "Prio", old.Prio, t.Prio)...)
			return append(out, changed("Ticket", t.Nr, "Sprint", old.Sprint, t.Sprint)...)
		})...)
	return withPlacement(cs, wt)
}

// withNewSessions hängt hinter jeden neuen Sprint seine Sessions als neu an (B-484); die Wurzel kennt keine davon.
func withNewSessions(cs []Change, sprints []Sprint) []Change {
	out := make([]Change, 0, len(cs))
	for _, c := range cs {
		out = append(out, c)
		if c.Kind != "neu" {
			continue
		}
		for _, s := range sprints {
			if s.ID == c.ID {
				out = append(out, diffSessions(Sprint{}, s)...)
			}
		}
	}
	return out
}

// withPlacement trägt bei neuen Sprints und Tickets Projekt (und Sprint des Tickets) ein, wie der Worktree sie führt.
func withPlacement(cs []Change, wt Data) []Change {
	for i, c := range cs {
		if c.Kind != "neu" {
			continue
		}
		switch c.Item {
		case "Sprint":
			for _, s := range wt.Sprints {
				if s.ID == c.ID {
					cs[i].Project = s.Project
				}
			}
		case "Ticket":
			for _, t := range wt.Tickets {
				if t.Nr == c.ID {
					cs[i].Project, cs[i].Sprint = t.Project, t.Sprint
				}
			}
		}
	}
	return cs
}

// diffByID vergleicht zwei Listen über die ID: in Reihenfolge des Worktrees neu oder die Feldänderungen aus fields,
// danach die in base vorhandenen, im Worktree fehlenden Einträge als entfernt.
func diffByID[T any](item string, base, wt []T, key func(T) (id, title string), fields func(old, now T) []Change) []Change {
	var cs []Change
	old := map[string]T{}
	for _, b := range base {
		id, _ := key(b)
		old[id] = b
	}
	seen := map[string]bool{}
	for _, w := range wt {
		id, title := key(w)
		seen[id] = true
		if o, ok := old[id]; ok {
			cs = append(cs, fields(o, w)...)
		} else {
			cs = append(cs, Change{Kind: "neu", Item: item, ID: id, Text: title})
		}
	}
	for _, b := range base {
		if id, title := key(b); !seen[id] {
			cs = append(cs, Change{Kind: "entfernt", Item: item, ID: id, Text: title})
		}
	}
	return cs
}

// changed ist eine Änderung „name a → b“, wenn sich das Feld unterscheidet, sonst nichts.
func changed(item, id, name, a, b string) []Change {
	if a == b {
		return nil
	}
	return []Change{{Kind: "geändert", Item: item, ID: id, Text: name + " " + a + " → " + b}}
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
			cs = append(cs, Change{Kind: "neu", Item: "Session", ID: s.Nr, Text: s.Titel})
		case o.Status != s.Status:
			cs = append(cs, Change{Kind: "geändert", Item: "Session", ID: s.Nr, Text: "Status " + o.Status + " → " + s.Status})
		}
	}
	for _, s := range old.Sessions {
		if !have[s.Nr] {
			cs = append(cs, Change{Kind: "entfernt", Item: "Session", ID: s.Nr, Text: s.Titel})
		}
	}
	return cs
}
