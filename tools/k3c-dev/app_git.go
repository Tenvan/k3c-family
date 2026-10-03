package main

import (
	"context"
	"time"

	"k3c/tools/k3c-dev/internal/gitcommit"
)

// gitTimeout begrenzt jeden Git-Aufruf der Oberfläche; die Hooks eines Commits dürfen länger brauchen.
const (
	gitTimeout       = 30 * time.Second
	gitCommitTimeout = 3 * time.Minute
)

// GitView ist der Stand der Git-Seite plus die Auswahllisten des Formulars.
type GitView struct {
	gitcommit.State
	Types   []string `json:"types"`
	Domains []string `json:"domains"`
}

// CommitResult ist die Antwort auf GitCommit: der neue Hash und der Stand danach.
type CommitResult struct {
	Hash string  `json:"hash"`
	View GitView `json:"view"`
}

func (a *App) gitView(ctx context.Context) (GitView, error) {
	st, err := gitcommit.Read(ctx, a.root)
	return GitView{State: st, Types: gitcommit.Types, Domains: gitcommit.Domains}, err
}

// Git liest Index, Arbeitsbaum und die letzten Commits (Binding).
func (a *App) Git() (GitView, error) {
	a.wait()
	ctx, cancel := context.WithTimeout(context.Background(), gitTimeout)
	defer cancel()
	return a.gitView(ctx)
}

// GitStage nimmt Dateien in den Index auf und liefert den neuen Stand (Binding). Staging bleibt bei der Person: es
// gibt dafür kein MCP-Tool.
func (a *App) GitStage(paths []string) (GitView, error) {
	a.wait()
	ctx, cancel := context.WithTimeout(context.Background(), gitTimeout)
	defer cancel()
	if err := gitcommit.Stage(ctx, a.root, paths); err != nil {
		return GitView{}, err
	}
	return a.gitView(ctx)
}

// GitUnstage nimmt Dateien aus dem Index (Binding).
func (a *App) GitUnstage(paths []string) (GitView, error) {
	a.wait()
	ctx, cancel := context.WithTimeout(context.Background(), gitTimeout)
	defer cancel()
	if err := gitcommit.Unstage(ctx, a.root, paths); err != nil {
		return GitView{}, err
	}
	return a.gitView(ctx)
}

// GitCommit committet den Index mit der Nachricht (Binding). Eine verletzte Regel kommt als Fehlertext
// `feld: Grund`, damit die Oberfläche das Feld markiert.
func (a *App) GitCommit(m gitcommit.Message) (CommitResult, error) {
	a.wait()
	ctx, cancel := context.WithTimeout(context.Background(), gitCommitTimeout)
	defer cancel()
	hash, err := gitcommit.Commit(ctx, a.root, m)
	if err != nil {
		return CommitResult{}, err
	}
	a.log.Info("commit angelegt", "ns", "git", "hash", hash)
	view, err := a.gitView(ctx)
	return CommitResult{Hash: hash, View: view}, err
}
