package gitops

import (
	"context"
	"strconv"
	"strings"
	"sync"
)

// Zustände eines Branches gegenüber develop.
const (
	StateCurrent  = "aktuell"   // ausgecheckt in der Wurzel
	StateWorktree = "worktree"  // ausgecheckt in einem anderen Worktree
	StateBase     = "basis"     // develop oder main
	StateMerged   = "gemergt"   // Vorfahre von origin/develop
	StateSquash   = "squash"    // Inhalt per Squash-Merge in origin/develop
	StateOpen     = "ungemergt" // eigene Commits, die develop fehlen
)

// Remote-Zustände eines Branches.
const (
	TrackSync   = "synchron"
	TrackAhead  = "voraus"
	TrackBehind = "zurück"
	TrackBoth   = "auseinander"
	TrackGone   = "gone"
	TrackNone   = "fehlt" // kein Upstream
	TrackRemote = "nur remote"
)

// Branch ist ein lokaler oder nur entfernter Branch.
type Branch struct {
	Name       string `json:"name"` // ohne origin/ auch bei nur entfernten
	Remote     bool   `json:"remote"`
	Upstream   string `json:"upstream"`
	Track      string `json:"track"`
	Ahead      int    `json:"ahead"`
	Behind     int    `json:"behind"`
	BaseAhead  int    `json:"baseAhead"`
	BaseBehind int    `json:"baseBehind"`
	State      string `json:"state"`
	Checked    bool   `json:"checked"` // Merge-Zustand sicher (Vorfahre oder geprüft); falsch: ungemergt nur vermutet
	PR         int    `json:"pr"`      // gemergter PR, an dem der Branch hängt (Check)
	SHA        string `json:"sha"`
	Date       int64  `json:"date"` // Unix-Sekunden des letzten Commits
	Subject    string `json:"subject"`
	Deletable  bool   `json:"deletable"`
	Reason     string `json:"reason"` // warum nicht löschbar
}

type ref struct {
	name, upstream, track, sha, subject string
	date                                int64
	remote                              bool
}

// Branches liest alle lokalen und nur entfernten Branches mit Zustand; der Merge-Zustand ist je Commit-Paar gemerkt.
func (r *Repo) Branches(ctx context.Context) ([]Branch, error) {
	out, err := r.git(ctx, "for-each-ref",
		"--format=%(refname)%09%(upstream:short)%09%(upstream:track)%09%(objectname)%09%(committerdate:unix)%09%(subject)",
		"refs/heads", "refs/remotes/origin")
	if err != nil {
		return nil, err
	}
	refs := parseRefs(out)
	// Ohne aktuellen Branch, Worktrees oder Basis wäre der Schutz in deletable blind: dann lieber gar keine Liste.
	current, err := r.git(ctx, "branch", "--show-current")
	if err != nil {
		return nil, err
	}
	wtOut, err := r.git(ctx, "worktree", "list", "--porcelain")
	if err != nil {
		return nil, err
	}
	wt := map[string]bool{}
	for _, w := range parseWorktrees(wtOut) {
		wt[w.Branch] = w.Branch != ""
	}
	base, _ := r.git(ctx, "rev-parse", "--verify", "--quiet", baseRemote) // leer: kein origin/develop, alles bleibt ungemergt
	list := make([]Branch, len(refs))
	var wg sync.WaitGroup
	sem := make(chan struct{}, 6)
	for i, f := range refs {
		wg.Add(1)
		go func() {
			defer wg.Done()
			sem <- struct{}{}
			defer func() { <-sem }()
			list[i] = r.branch(ctx, f, base, current, wt)
		}()
	}
	wg.Wait()
	return list, nil
}

func (r *Repo) branch(ctx context.Context, f ref, base, current string, wt map[string]bool) Branch {
	b := Branch{Name: f.name, Remote: f.remote, Upstream: f.upstream, SHA: f.sha, Date: f.date, Subject: f.subject}
	b.Track, b.Ahead, b.Behind = parseTrack(f)
	target := f.name
	if f.remote {
		target = "origin/" + f.name
	}
	switch {
	case protected(f.name):
		b.State = StateBase
	case !f.remote && f.name == current:
		b.State = StateCurrent
	case !f.remote && wt[f.name]:
		b.State = StateWorktree
	}
	ahead, behind, ok := r.aheadBehind(ctx, target, baseRemote)
	b.BaseAhead, b.BaseBehind = ahead, behind
	switch {
	case b.State != "":
	case !ok || base == "":
		b.State = StateOpen
		b.Deletable, b.Reason = false, "Stand zu origin/develop unbekannt"
		return b
	default:
		b.State, b.PR, b.Checked = r.mergedState(f.sha, base, ahead)
	}
	b.Deletable, b.Reason = deletable(b)
	return b
}

// deletable: nur lokale Branches, die develop schon enthält und die nirgends ausgecheckt sind.
func deletable(b Branch) (bool, string) {
	switch {
	case b.Remote:
		return false, "nur remote"
	case b.State == StateBase:
		return false, "develop und main bleiben"
	case b.State == StateCurrent:
		return false, "ausgecheckt"
	case b.State == StateWorktree:
		return false, "in einem Worktree ausgecheckt"
	case b.State == StateOpen && !b.Checked:
		return false, "ungeprüft: erst „Prüfen“"
	case b.State == StateOpen:
		return false, "nicht in develop"
	}
	return true, ""
}

// parseRefs liest for-each-ref; origin/HEAD und entfernte Branches mit lokalem Gegenstück fallen weg.
func parseRefs(out string) []ref {
	var local, remote []ref
	tracked := map[string]bool{}
	for _, l := range lines(out) {
		f := strings.SplitN(l, "\t", 6)
		if len(f) < 6 {
			continue
		}
		date, _ := strconv.ParseInt(f[4], 10, 64)
		x := ref{upstream: f[1], track: f[2], sha: f[3], date: date, subject: f[5]}
		if name, ok := strings.CutPrefix(f[0], "refs/heads/"); ok {
			x.name = name
			local = append(local, x)
			tracked[x.upstream] = true
		} else if name, ok := strings.CutPrefix(f[0], "refs/remotes/origin/"); ok && name != "HEAD" {
			x.name, x.remote = name, true
			remote = append(remote, x)
		}
	}
	for _, x := range remote {
		if !tracked["origin/"+x.name] {
			local = append(local, x)
		}
	}
	return local
}

// parseTrack deutet %(upstream:track): "", "[gone]", "[ahead 1]", "[behind 2]", "[ahead 1, behind 2]".
func parseTrack(f ref) (string, int, int) {
	if f.remote {
		return TrackRemote, 0, 0
	}
	if f.upstream == "" {
		return TrackNone, 0, 0
	}
	t := strings.Trim(f.track, "[]")
	if t == "gone" {
		return TrackGone, 0, 0
	}
	var ahead, behind int
	for _, part := range strings.Split(t, ", ") {
		if n, ok := strings.CutPrefix(part, "ahead "); ok {
			ahead, _ = strconv.Atoi(n)
		}
		if n, ok := strings.CutPrefix(part, "behind "); ok {
			behind, _ = strconv.Atoi(n)
		}
	}
	switch {
	case ahead > 0 && behind > 0:
		return TrackBoth, ahead, behind
	case ahead > 0:
		return TrackAhead, ahead, 0
	case behind > 0:
		return TrackBehind, 0, behind
	}
	return TrackSync, 0, 0
}
