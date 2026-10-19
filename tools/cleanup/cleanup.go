package main

import (
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
)

// options bündelt die Umgebung des Laufs; Git und PRMerged sind austauschbar (Tests, kein gh).
type options struct {
	Root     string
	Dry      bool
	Git      func(dir string, args ...string) (string, error)
	PRMerged func(branch string) (bool, error)
	Out      io.Writer
}

type branch struct {
	name     string
	upstream bool // hat einen Upstream (auch wenn er weg ist)
	gone     bool // Upstream ist auf origin gelöscht
	current  bool // im aktuellen Checkout ausgecheckt
	worktree string
}

type worktree struct {
	path     string
	head     string
	branch   string // leer bei detached
	detached bool
}

// action ist die Entscheidung zu einem Branch oder Worktree.
type action struct {
	subject string
	remove  bool
	reason  string
	wtPath  string // Worktree, der vor dem Branch entfernt wird
	brName  string // Branch, der gelöscht wird
}

func run(o options) error {
	base, err := baseRef(o)
	if err != nil {
		return err
	}
	branches, err := listBranches(o)
	if err != nil {
		return err
	}
	trees, err := listWorktrees(o)
	if err != nil {
		return err
	}
	top, err := o.Git(o.Root, "rev-parse", "--show-toplevel")
	if err != nil {
		return err
	}
	acts := judgeBranches(o, base, branches, trees, top)
	acts = append(acts, judgeDetached(o, base, trees, top)...)
	return apply(o, acts)
}

// baseRef ist origin/develop, sonst develop: der Stand, in dem gemergte Arbeit stecken muss.
func baseRef(o options) (string, error) {
	for _, ref := range []string{"origin/develop", "develop"} {
		if _, err := o.Git(o.Root, "rev-parse", "--verify", "--quiet", ref); err == nil {
			return ref, nil
		}
	}
	return "", fmt.Errorf("weder origin/develop noch develop gefunden")
}

func listBranches(o options) ([]branch, error) {
	out, err := o.Git(o.Root, "for-each-ref", "--format=%(refname:short)%09%(upstream)%09%(upstream:track)%09%(HEAD)%09%(worktreepath)", "refs/heads")
	if err != nil {
		return nil, err
	}
	var res []branch
	for _, line := range strings.Split(out, "\n") {
		f := strings.Split(strings.TrimRight(line, "\r"), "\t")
		if len(f) < 5 {
			continue
		}
		res = append(res, branch{name: f[0], upstream: f[1] != "", gone: strings.Contains(f[2], "gone"), current: f[3] == "*", worktree: f[4]})
	}
	return res, nil
}

func listWorktrees(o options) ([]worktree, error) {
	out, err := o.Git(o.Root, "worktree", "list", "--porcelain")
	if err != nil {
		return nil, err
	}
	var res []worktree
	for _, line := range strings.Split(out, "\n") {
		line = strings.TrimRight(line, "\r")
		switch {
		case strings.HasPrefix(line, "worktree "):
			res = append(res, worktree{path: strings.TrimPrefix(line, "worktree ")})
		case len(res) == 0:
		case strings.HasPrefix(line, "HEAD "):
			res[len(res)-1].head = strings.TrimPrefix(line, "HEAD ")
		case strings.HasPrefix(line, "branch refs/heads/"):
			res[len(res)-1].branch = strings.TrimPrefix(line, "branch refs/heads/")
		case line == "detached":
			res[len(res)-1].detached = true
		}
	}
	return res, nil
}

// activeSprints liefert die Branch-Namen aktiver Sprints: `docs/sprints/aktiv/CI1-name` gehört zu `sprint/ci1`.
func activeSprints(root string) map[string]bool {
	res := map[string]bool{}
	entries, err := os.ReadDir(filepath.Join(root, "docs", "sprints", "aktiv"))
	if err != nil {
		return res
	}
	for _, e := range entries {
		if e.IsDir() {
			id, _, _ := strings.Cut(e.Name(), "-")
			res["sprint/"+strings.ToLower(id)] = true
		}
	}
	return res
}

// cherry zählt die Commits des Branchs, die dem Patch nach nicht in base stecken.
func cherry(o options, base, name string) (int, error) {
	out, err := o.Git(o.Root, "cherry", base, name)
	if err != nil {
		return 0, err
	}
	n := 0
	for _, line := range strings.Split(out, "\n") {
		if strings.HasPrefix(line, "+") {
			n++
		}
	}
	return n, nil
}

// judgeBranch entscheidet allein über den Branch; Worktrees prüft judgeBranches danach.
func judgeBranch(o options, base string, b branch, active map[string]bool) (bool, string) {
	switch {
	case b.name == "develop" || b.name == "main":
		return false, "geschützt"
	case b.current:
		return false, "im aktuellen Checkout ausgecheckt"
	case active[b.name]:
		return false, "Branch eines aktiven Sprints"
	case b.upstream && !b.gone:
		return false, "Upstream auf origin vorhanden"
	}
	ahead, err := cherry(o, base, b.name)
	if err != nil {
		return false, "git cherry fehlgeschlagen: " + err.Error()
	}
	if ahead == 0 {
		return true, "alle Commits stecken in " + base
	}
	if !b.gone {
		return false, fmt.Sprintf("nicht gemergt, %d Commits nicht in %s", ahead, base)
	}
	merged, err := o.PRMerged(b.name)
	switch {
	case err != nil:
		return false, "Upstream weg, PR-Status unbekannt: " + err.Error()
	case merged:
		return true, "Upstream weg, Pull Request gemergt"
	}
	return false, fmt.Sprintf("Upstream weg, aber kein gemergter Pull Request, %d Commits nicht in %s", ahead, base)
}

func sameDir(a, b string) bool {
	return strings.EqualFold(filepath.ToSlash(filepath.Clean(a)), filepath.ToSlash(filepath.Clean(b)))
}

func dirty(o options, path string) (bool, error) {
	out, err := o.Git(path, "status", "--porcelain", "--ignored")
	return strings.TrimSpace(out) != "", err
}

func judgeBranches(o options, base string, branches []branch, trees []worktree, top string) []action {
	active := activeSprints(o.Root)
	var acts []action
	for _, b := range branches {
		del, reason := judgeBranch(o, base, b, active)
		a := action{subject: "Branch " + b.name, remove: del, reason: reason, brName: b.name}
		if del && b.worktree != "" && !sameDir(b.worktree, top) {
			a = withWorktree(o, a, b, trees)
		}
		acts = append(acts, a)
	}
	return acts
}

// withWorktree prüft den Worktree eines zu löschenden Branchs: nur ein sauberer, nicht der Hauptcheckout.
func withWorktree(o options, a action, b branch, trees []worktree) action {
	if len(trees) > 0 && sameDir(b.worktree, trees[0].path) {
		return action{subject: a.subject, reason: "im Hauptcheckout ausgecheckt", brName: b.name}
	}
	d, err := dirty(o, b.worktree)
	switch {
	case err != nil:
		return action{subject: a.subject, reason: "Worktree " + b.worktree + " nicht prüfbar: " + err.Error(), brName: b.name}
	case d:
		return action{subject: a.subject, reason: "Worktree " + b.worktree + " hat Änderungen oder ignorierte Dateien", brName: b.name}
	}
	a.wtPath = b.worktree
	a.reason += "; Worktree " + b.worktree + " sauber, wird mit entfernt"
	return a
}

// judgeDetached prüft Worktrees ohne Branch: sauber und auf einem Stand, der in base steckt.
func judgeDetached(o options, base string, trees []worktree, top string) []action {
	var acts []action
	for i, t := range trees {
		if i == 0 || !t.detached || sameDir(t.path, top) {
			continue
		}
		a := action{subject: "Worktree " + t.path, wtPath: t.path}
		d, err := dirty(o, t.path)
		_, anc := o.Git(o.Root, "merge-base", "--is-ancestor", t.head, base)
		switch {
		case err != nil:
			a.reason = "nicht prüfbar: " + err.Error()
		case d:
			a.reason = "hat Änderungen oder ignorierte Dateien"
		case anc != nil:
			a.reason = "detached auf einem Stand, der nicht in " + base + " steckt"
		default:
			a.remove, a.reason = true, "detached auf einem Stand von "+base+", ohne Änderungen"
		}
		if !a.remove {
			a.wtPath = ""
		}
		acts = append(acts, a)
	}
	return acts
}

func apply(o options, acts []action) error {
	removed, kept := 0, 0
	for _, a := range acts {
		verb := "bleibt   "
		if a.remove {
			verb = "löschen  "
			if o.Dry {
				verb = "würde löschen  "
			}
		}
		if a.remove && !o.Dry {
			if err := erase(o, a); err != nil {
				say(o, "Fehler    %s: %v\n", a.subject, err)
				kept++
				continue
			}
		}
		if a.remove {
			removed++
		} else {
			kept++
		}
		say(o, "%s%s – %s\n", verb, a.subject, a.reason)
	}
	say(o, "aufraeumen: %d %s, %d bleiben\n", removed, map[bool]string{true: "würden gelöscht", false: "gelöscht"}[o.Dry], kept)
	return nil
}

func erase(o options, a action) error {
	if a.wtPath != "" {
		if _, err := o.Git(o.Root, "worktree", "remove", a.wtPath); err != nil {
			return err
		}
	}
	if a.brName != "" {
		if _, err := o.Git(o.Root, "branch", "-D", a.brName); err != nil {
			return err
		}
	}
	return nil
}

// say schreibt eine Zeile in die Ausgabe; ein Schreibfehler der Konsole ändert nichts am Aufräumen.
func say(o options, format string, args ...any) {
	_, _ = fmt.Fprintf(o.Out, format, args...)
}
