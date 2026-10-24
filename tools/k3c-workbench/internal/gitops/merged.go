package gitops

import (
	"context"
	"strings"
	"sync"
)

// MergedPR ist ein gemergter PR (aus gh); Head ist der Branch-Name, SHA der letzte Commit des PRs.
type MergedPR struct {
	Number int
	Head   string
	SHA    string
}

// verdict ist das Ergebnis der Prüfung „steckt der Branch in develop?“, gemerkt je Commit-Paar (Branch, develop).
type verdict struct {
	state string // StateSquash oder StateOpen
	pr    int    // Nummer des gemergten PRs, 0 = über git erkannt oder ungemergt
}

func memoKey(sha, base string) string { return sha + " " + base }

// mergedState: Vorfahre von develop heißt gemergt; sonst gilt das gemerkte Ergebnis von Check. Ohne Prüfung bleibt
// der Branch ungemergt und ungeprüft, die teure Prüfung läuft nur auf Knopfdruck. Der Aufrufer stellt sicher, dass
// baseAhead gemessen ist (aheadBehind ok) und base existiert.
func (r *Repo) mergedState(sha, base string, baseAhead int) (string, int, bool) {
	if base == "" || sha == "" {
		return StateOpen, 0, false
	}
	if baseAhead == 0 {
		return StateMerged, 0, true
	}
	if v, ok := r.memo.Load(memoKey(sha, base)); ok {
		v := v.(verdict)
		return v.state, v.pr, true
	}
	return StateOpen, 0, false
}

// Check prüft die genannten lokalen Branches (leer = alle ungeprüften) gründlich und merkt das Ergebnis:
// 1. ein gemergter PR, dessen Head der Branch ist oder enthält (der übliche Squash-Merge, auch Sammel-PRs),
// 2. ein Merge in develop ändert nichts mehr (merge-tree),
// 3. der Netto-Diff des Branches ist als Commit in develop (Squash-Patch, git cherry).
func (r *Repo) Check(ctx context.Context, names []string, prs []MergedPR) error {
	all, err := r.Branches(ctx)
	if err != nil {
		return err
	}
	base, _ := r.git(ctx, "rev-parse", "--verify", "--quiet", baseRemote)
	if base == "" {
		return nil // ohne origin/develop gibt es nichts zu prüfen
	}
	want := map[string]bool{}
	for _, n := range names {
		want[n] = true
	}
	var wg sync.WaitGroup
	sem := make(chan struct{}, 6)
	for _, b := range all {
		if b.Remote || b.State != StateOpen || (len(names) > 0 && !want[b.Name]) || (len(names) == 0 && b.Checked) {
			continue
		}
		wg.Add(1)
		go func() {
			defer wg.Done()
			sem <- struct{}{}
			defer func() { <-sem }()
			// Nur sichere Antworten merken: ein Abbruch, eine Sperre oder eine Frist hieße sonst dauerhaft „ungemergt“.
			if v, sure := r.verdict(ctx, b, prs); sure && ctx.Err() == nil {
				r.memo.Store(memoKey(b.SHA, base), v)
			}
		}()
	}
	wg.Wait()
	return nil
}

func (r *Repo) verdict(ctx context.Context, b Branch, prs []MergedPR) (verdict, bool) {
	n, sureA := r.prFor(ctx, b, prs)
	if n > 0 {
		return verdict{state: StateSquash, pr: n}, true
	}
	in, sureB := r.squashed(ctx, b.Name)
	if in {
		return verdict{state: StateSquash}, true
	}
	return verdict{state: StateOpen}, sureA && sureB
}

// noAnswer: git ist gescheitert, statt „nein“ zu sagen. `merge-base --is-ancestor` und `merge-tree` melden ein Nein
// (kein Vorfahre, Konflikt) mit Exit 1 ohne Text auf stderr; gitError macht daraus „…: exit status 1“.
func noAnswer(err error) bool { return err != nil && !strings.HasSuffix(err.Error(), "exit status 1") }

// prFor sucht einen nach develop gemergten PR mit dem Branch als Head, über den Upstream-Namen oder den eigenen
// Namen; der Branch muss der Head des PRs sein oder in ihm stecken. sure ist falsch, wenn git nicht antworten konnte.
func (r *Repo) prFor(ctx context.Context, b Branch, prs []MergedPR) (pr int, sure bool) {
	up := strings.TrimPrefix(b.Upstream, "origin/")
	sure = true
	for _, p := range prs {
		if p.Head != b.Name && p.Head != up {
			continue
		}
		if p.SHA == b.SHA {
			return p.Number, true
		}
		_, err := r.git(ctx, "merge-base", "--is-ancestor", b.SHA, p.SHA)
		if err == nil {
			return p.Number, true
		}
		if noAnswer(err) && !strings.Contains(err.Error(), "Not a valid commit") {
			sure = false // fehlt der Head-Commit lokal, ist das ein sicheres Nein; alles andere nicht
		}
	}
	return 0, sure
}

// squashed: develop enthält den Stand von target, wenn ein Merge nichts mehr ändert (merge-tree) oder ein Commit
// mit dem Netto-Diff des Branches für `git cherry` gleich einem Commit auf develop ist (Squash-Merge). Bekannte
// Grenze: hat develop die Änderung später zurückgenommen, gilt der Branch über merge-tree ebenfalls als enthalten.
// sure ist falsch, wenn git nicht antworten konnte.
func (r *Repo) squashed(ctx context.Context, target string) (in, sure bool) {
	tree, err := r.git(ctx, "merge-tree", "--write-tree", baseRemote, target)
	if noAnswer(err) {
		return false, false
	}
	if err == nil {
		if dev, err := r.git(ctx, "rev-parse", baseRemote+"^{tree}"); err == nil && firstLine(tree) == dev {
			return true, true
		}
	}
	return r.squashPatch(ctx, target)
}

// squashPatch: Ein Commit mit dem Baum des Branches auf der Merge-Basis ist für `git cherry` gleich einem Commit auf
// develop, wenn ein Squash-Merge genau diese Änderung gebracht hat.
func (r *Repo) squashPatch(ctx context.Context, target string) (in, sure bool) {
	mb, err := r.git(ctx, "merge-base", baseRemote, target)
	if err != nil {
		return false, false
	}
	tree, err := r.git(ctx, "rev-parse", target+"^{tree}")
	if err != nil {
		return false, false
	}
	// Feste Identität: commit-tree verlangt eine, das Objekt bleibt unerreichbar und verschwindet beim nächsten gc.
	probe, err := r.git(ctx, "-c", "user.name=k3c-workbench", "-c", "user.email=workbench@localhost",
		"commit-tree", tree, "-p", mb, "-m", "squash-probe")
	if err != nil {
		return false, false
	}
	out, err := r.git(ctx, "cherry", baseRemote, probe)
	if err != nil {
		return false, false
	}
	return strings.HasPrefix(out, "-"), true
}

func firstLine(s string) string {
	line, _, _ := strings.Cut(s, "\n")
	return strings.TrimSpace(line)
}
