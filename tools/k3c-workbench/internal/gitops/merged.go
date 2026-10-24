package gitops

import (
	"context"
	"strings"
)

// mergedState sagt, ob develop den Branch schon enthält. baseAhead 0 heißt Vorfahre (normaler Merge oder Fast-Forward).
// Sonst prüft ein Squash-Test: Ein Commit mit dem Baum des Branches auf der Merge-Basis ist für `git cherry` gleich
// einem Commit auf develop, wenn ein Squash-Merge genau diese Änderung gebracht hat. Das Ergebnis hängt nur an den
// beiden Commits und bleibt im Speicher.
// Der Aufrufer stellt sicher, dass baseAhead gemessen ist (aheadBehind ok) und base existiert.
func (r *Repo) mergedState(ctx context.Context, target, sha, base string, baseAhead int) string {
	if base == "" || sha == "" {
		return StateOpen
	}
	if baseAhead == 0 {
		return StateMerged
	}
	key := sha + " " + base
	if v, ok := r.memo.Load(key); ok {
		return v.(string)
	}
	state := StateOpen
	if r.squashed(ctx, target) {
		state = StateSquash
	}
	r.memo.Store(key, state)
	return state
}

func (r *Repo) squashed(ctx context.Context, target string) bool {
	mb, err := r.git(ctx, "merge-base", baseRemote, target)
	if err != nil {
		return false
	}
	tree, err := r.git(ctx, "rev-parse", target+"^{tree}")
	if err != nil {
		return false
	}
	// Feste Identität: commit-tree verlangt eine, das Objekt bleibt unerreichbar und verschwindet beim nächsten gc.
	probe, err := r.git(ctx, "-c", "user.name=k3c-workbench", "-c", "user.email=workbench@localhost",
		"commit-tree", tree, "-p", mb, "-m", "squash-probe")
	if err != nil {
		return false
	}
	out, err := r.git(ctx, "cherry", baseRemote, probe)
	return err == nil && strings.HasPrefix(out, "-")
}
