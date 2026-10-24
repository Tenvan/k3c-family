package gitops

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"time"
)

// Aktionen der Seite; jede läuft allein (Repo.mu), nie mit Force.
const (
	ActFetch    = "fetch"
	ActPull     = "pull"
	ActPush     = "push"
	ActStash    = "stash"
	ActStashPop = "stash-pop"
)

// Do führt eine Aktion aus und liefert einen Satz zum Ergebnis.
func (r *Repo) Do(ctx context.Context, action string) (string, error) {
	r.mu.Lock()
	defer r.mu.Unlock()
	switch action {
	case ActFetch:
		_, err := r.git(ctx, "fetch", "--prune", "origin")
		return "Fetch fertig", err
	case ActPull:
		_, err := r.git(ctx, "pull", "--ff-only")
		return "Pull fertig (Fast-Forward)", err
	case ActPush:
		return r.push(ctx)
	case ActStash:
		_, err := r.git(ctx, "stash", "push", "--include-untracked", "-m", "workbench "+time.Now().Format("2006-01-02 15:04"))
		return "Änderungen gestasht", err
	case ActStashPop:
		_, err := r.git(ctx, "stash", "pop")
		return "Letzter Stash zurückgeholt", err
	}
	return "", fmt.Errorf("unbekannte Aktion %q", action)
}

// push schiebt den aktuellen Branch; ohne Upstream mit -u nach origin. develop und main nimmt die Seite nicht.
func (r *Repo) push(ctx context.Context) (string, error) {
	branch, err := r.git(ctx, "branch", "--show-current")
	if err != nil {
		return "", err
	}
	switch {
	case branch == "":
		return "", errors.New("detached HEAD: kein Branch zum Pushen")
	case protected(branch):
		return "", fmt.Errorf("%s pusht die Seite nicht (nur über PR bzw. Release)", branch)
	case !ValidBranch(branch):
		return "", fmt.Errorf("ungewöhnlicher Branch-Name %q", branch)
	}
	// Immer mit ausdrücklichem Ziel gleichen Namens: ein bloßes `git push` ginge an den Upstream, und der könnte (mit
	// push.default=upstream) origin/develop sein.
	refspec := "HEAD:refs/heads/" + branch
	if _, err := r.git(ctx, "rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{upstream}"); err == nil {
		_, err := r.git(ctx, "push", "origin", refspec)
		return "Push fertig", err
	}
	_, err = r.git(ctx, "push", "--set-upstream", "origin", refspec)
	return "Push fertig, Upstream origin/" + branch + " gesetzt", err
}

// Delete löscht lokale Branches, die develop schon enthält; remote löscht dazu den Branch auf origin, falls es ihn
// noch gibt. Die Prüfung läuft hier noch einmal, die Oberfläche könnte einen alten Stand zeigen.
func (r *Repo) Delete(ctx context.Context, names []string, remote bool) (string, error) {
	if len(names) == 0 {
		return "", errors.New("keine Branches gewählt")
	}
	for _, n := range names {
		if !ValidBranch(n) {
			return "", fmt.Errorf("ungültiger Branch-Name %q", n)
		}
	}
	r.mu.Lock()
	defer r.mu.Unlock()
	all, err := r.Branches(ctx)
	if err != nil {
		return "", err
	}
	byName := map[string]Branch{}
	for _, b := range all {
		if !b.Remote {
			byName[b.Name] = b
		}
	}
	var refused []string
	for _, n := range names {
		b, ok := byName[n]
		switch {
		case !ok:
			refused = append(refused, n+": kein lokaler Branch")
		case !b.Deletable:
			refused = append(refused, n+": "+b.Reason)
		}
	}
	if len(refused) > 0 {
		return "", errors.New("nicht gelöscht: " + strings.Join(refused, "; "))
	}
	return r.deleteAll(ctx, names, byName, remote)
}

// deleteAll löscht erst die Remote-Branches (nur, wenn develop auch deren Stand enthält), dann lokal. Scheitert ein
// Remote-Schritt, bleibt der lokale Branch stehen; die übrigen laufen weiter.
func (r *Repo) deleteAll(ctx context.Context, names []string, byName map[string]Branch, remote bool) (string, error) {
	var res result
	for _, n := range names {
		b := byName[n]
		if remote && b.Track != TrackGone && b.Track != TrackNone {
			if err := r.deleteRemote(ctx, b); err != nil {
				res.failed = append(res.failed, n+": "+err.Error())
				continue
			}
			res.remote = append(res.remote, n)
		}
		// -D statt -d: nach einem Squash-Merge hält git den Branch für ungemergt; geprüft hat Delete selbst.
		if _, err := r.git(ctx, "branch", "-D", n); err != nil {
			res.failed = append(res.failed, n+": "+err.Error())
			continue
		}
		res.local = append(res.local, n+" ("+shortHash(b.SHA)+")")
	}
	if len(res.failed) > 0 {
		return res.String(), errors.New(res.String())
	}
	return res.String(), nil
}

// deleteRemote löscht origin/<upstream>, wenn develop dessen Stand enthält (Vorfahre oder Squash); Commits, die nur auf
// origin liegen (anderes Gerät, andere Session), bleiben so erhalten.
func (r *Repo) deleteRemote(ctx context.Context, b Branch) error {
	up, ok := strings.CutPrefix(b.Upstream, "origin/")
	if !ok || !ValidBranch(up) || protected(up) {
		return fmt.Errorf("upstream %q wird nicht gelöscht", b.Upstream)
	}
	ahead, _, measured := r.aheadBehind(ctx, b.Upstream, baseRemote)
	if !measured || (ahead > 0 && !r.squashed(ctx, b.Upstream)) {
		return errors.New("origin hat Commits, die develop fehlen; nichts gelöscht")
	}
	_, err := r.git(ctx, "push", "origin", "--delete", up)
	return err
}

type result struct{ local, remote, failed []string }

// String nennt die gelöschten Branches mit Hash (zum Wiederherstellen: git branch <name> <hash>).
func (r result) String() string {
	s := fmt.Sprintf("%d Branches gelöscht", len(r.local))
	if len(r.remote) > 0 {
		s += fmt.Sprintf(", %d auf origin", len(r.remote))
	}
	if len(r.local) > 0 {
		s += ": " + strings.Join(r.local, ", ")
	}
	if len(r.failed) > 0 {
		s += "; nicht gelöscht: " + strings.Join(r.failed, "; ")
	}
	return s
}
