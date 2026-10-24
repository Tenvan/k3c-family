package main

import (
	"errors"
	"fmt"

	"k3c/tools/k3c-workbench/internal/github"
	"k3c/tools/k3c-workbench/internal/gitops"
)

// Seite Git (B-580): Stand des Repos, Branches, Dateien und PRs; Aktionen ohne Force und ohne Shell. Jede Aktion steht
// im Log der Workbench.

// GitView ist alles, was die Seite beim Laden zeichnet; PRs kommen getrennt (gh braucht Netz und ist langsamer).
type GitView struct {
	Status   gitops.Status   `json:"status"`
	Branches []gitops.Branch `json:"branches"`
	Files    gitops.Files    `json:"files"`
}

// GitOverview liest Status, Branches und den Diff zu origin/develop (Binding).
func (a *App) GitOverview() (GitView, error) {
	a.wait()
	st, err := a.git.Status(a.ctx)
	if err != nil {
		return GitView{}, err
	}
	branches, err := a.git.Branches(a.ctx)
	if err != nil {
		return GitView{}, err
	}
	files, err := a.git.BranchFiles(a.ctx)
	if err != nil {
		files = gitops.Files{Base: "origin/develop", Files: []gitops.DiffFile{}} // ohne origin/develop geht der Rest
	}
	return GitView{Status: st, Branches: branches, Files: files}, nil
}

// GitAction führt fetch, pull, push, stash oder stash-pop aus (Binding).
func (a *App) GitAction(action string) (string, error) {
	a.wait()
	msg, err := a.git.Do(a.ctx, action)
	a.gitLog("git "+action, msg, err)
	return msg, err
}

// GitDeleteBranches löscht gemergte lokale Branches, mit remote auch auf origin (Binding).
func (a *App) GitDeleteBranches(names []string, remote bool) (string, error) {
	a.wait()
	msg, err := a.git.Delete(a.ctx, names, remote)
	if err == nil {
		a.log.Info("🧹 Branches gelöscht", "ns", "main", "ergebnis", msg, "remote", remote)
	} else {
		a.gitLog("Branches löschen", msg, err)
	}
	return msg, err
}

// GitCheckMerged prüft Branches gründlich auf Squash-Merges (gemergter PR, merge-tree, Squash-Patch) und liefert die
// Branch-Liste neu (Binding); names leer = alle ungeprüften. Ohne gh prüft git allein.
func (a *App) GitCheckMerged(names []string) ([]gitops.Branch, error) {
	a.wait()
	var prs []gitops.MergedPR
	if heads, err := a.gh.Merged(a.ctx); err == nil {
		for _, h := range heads {
			prs = append(prs, gitops.MergedPR{Number: h.Number, Head: h.Head, SHA: h.SHA})
		}
	} else if a.log != nil {
		a.log.Warn("❌ Gemergte PRs nicht gelesen, prüfe nur mit git", "ns", "main", "fehler", err.Error())
	}
	if err := a.git.Check(a.ctx, names, prs); err != nil {
		return nil, err
	}
	return a.git.Branches(a.ctx)
}

// GitPRs liefert die offenen PRs gegen develop (Binding); ein Fehler von gh steht in Error.
func (a *App) GitPRs() github.PRList {
	a.wait()
	return a.gh.Open(a.ctx)
}

// GitPRCreate legt für den aktuellen Branch einen PR gegen develop an und liefert die Adresse (Binding).
func (a *App) GitPRCreate(title, body string) (string, error) {
	a.wait()
	st, err := a.git.Status(a.ctx)
	if err != nil {
		return "", err
	}
	switch {
	case st.Branch == "" || !gitops.ValidBranch(st.Branch):
		return "", errors.New("kein gültiger Branch ausgecheckt")
	case st.Upstream == "" || st.UpstreamGone:
		return "", errors.New("der Branch ist nicht auf origin: erst pushen")
	case st.Ahead > 0:
		return "", fmt.Errorf("%d Commits nicht gepusht: erst pushen", st.Ahead)
	}
	url, err := a.gh.Create(a.ctx, st.Branch, title, body)
	a.gitLog("PR anlegen", url, err)
	return url, err
}

// GitPRAction mergt (merge, squash) oder schließt (close) PR number (Binding).
func (a *App) GitPRAction(number int, action string) (string, error) {
	a.wait()
	if number <= 0 {
		return "", errors.New("ungültige PR-Nummer")
	}
	var err error
	var msg string
	switch action {
	case "merge", "squash":
		err = a.gh.Merge(a.ctx, number, action == "squash")
		msg = fmt.Sprintf("PR #%d gemergt", number)
	case "close":
		err = a.gh.Close(a.ctx, number)
		msg = fmt.Sprintf("PR #%d geschlossen", number)
	default:
		return "", fmt.Errorf("unbekannte PR-Aktion %q", action)
	}
	a.gitLog(fmt.Sprintf("PR #%d %s", number, action), msg, err)
	return msg, err
}

func (a *App) gitLog(what, msg string, err error) {
	if a.log == nil {
		return
	}
	if err != nil {
		a.log.Warn("❌ "+what+" fehlgeschlagen", "ns", "main", "fehler", err.Error())
		return
	}
	a.log.Info("✅ "+what, "ns", "main", "ergebnis", msg)
}
