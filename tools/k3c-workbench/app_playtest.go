package main

import (
	"errors"
	"net/http"
	"os"
	"path/filepath"

	"k3c/tools/k3c-workbench/internal/playtest"
	"k3c/tools/workkit/uikit"
)

// Seite Spieltest (B-1218): Sessions in `playtests/<JJJJ-MM-TT>-<nr>/` im Repo (gitignored), Aufräumen per Knopf.
// Der Ordner-Vertrag steht in internal/playtest; die Bindings reichen nur durch und melden ins Log.

// playtestMediaPrefix ist die Adresse der Player (nur HTTP-Betrieb; Ablage-Datei nur voice.wav und video.mkv).
const playtestMediaPrefix = uikit.Prefix + "playtest/media/"

func (a *App) playtestStore() *playtest.Store {
	return &playtest.Store{Root: filepath.Join(a.root, "playtests")}
}

// PlaytestList listet die Sessions, neueste zuerst.
func (a *App) PlaytestList() ([]playtest.Session, error) {
	list, err := a.playtestStore().List()
	if err != nil {
		a.log.Warn("❌ Spieltests nicht gelesen: "+err.Error(), "ns", "main")
		return nil, err
	}
	a.log.Info("📂 Spieltests geladen", "ns", "main", "anzahl", len(list))
	return list, nil
}

// PlaytestPlan nennt, was die Aktion (video, session, ausgewertet) löschen würde; es wird nichts gelöscht.
func (a *App) PlaytestPlan(action, id string) (playtest.Plan, error) {
	return a.playtestStore().Plan(action, id)
}

// PlaytestDelete führt die Aktion aus, aber nur mit confirm=true (die Oberfläche fragt vorher mit PlaytestPlan).
func (a *App) PlaytestDelete(action, id string, confirm bool) (playtest.Plan, error) {
	p, err := a.playtestStore().Delete(action, id, confirm)
	if err != nil {
		if !errors.Is(err, playtest.ErrNotConfirmed) {
			a.log.Warn("❌ Aufräumen der Spieltests fehlgeschlagen: "+err.Error(), "ns", "main", "aktion", action, "id", id)
		}
		return p, err
	}
	a.log.Info("🧹 Spieltests aufgeräumt", "ns", "main", "aktion", action, "id", id, "dateien", len(p.Files), "bytes", p.Total)
	return p, nil
}

// playtestMedia liefert voice.wav und video.mkv einer Session für die Player (mit Bereichsanfragen zum Springen).
func (a *App) playtestMedia() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		p, err := a.playtestStore().MediaPath(r.PathValue("id"), r.PathValue("file"))
		if err != nil {
			uikit.Fail(w, http.StatusNotFound, err.Error())
			return
		}
		f, err := os.Open(p)
		if err != nil {
			uikit.Fail(w, http.StatusNotFound, err.Error())
			return
		}
		defer func() { _ = f.Close() }()
		st, err := f.Stat()
		if err != nil {
			uikit.Fail(w, http.StatusInternalServerError, err.Error())
			return
		}
		w.Header().Set("Cache-Control", "no-store")
		http.ServeContent(w, r, filepath.Base(p), st.ModTime(), f)
	})
}
