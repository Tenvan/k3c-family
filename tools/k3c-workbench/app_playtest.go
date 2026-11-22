package main

import (
	"errors"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"sync"

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

// recorders hält je App einen Recorder (App-Struktur bleibt unberührt, WZ64.2).
var recorders sync.Map

func (a *App) recorder() *playtest.Recorder {
	if r, ok := recorders.Load(a); ok {
		return r.(*playtest.Recorder)
	}
	r := playtest.NewRecorder(a.root, a.playtestStore(), func(level, msg string, kv ...any) {
		attrs := append([]any{"ns", "main"}, kv...)
		if level == "warn" {
			a.log.Warn(msg, attrs...)
			return
		}
		a.log.Info(msg, attrs...)
	})
	act, _ := recorders.LoadOrStore(a, r)
	return act.(*playtest.Recorder)
}

// PlaytestDevices nennt die Mikrofone (ffmpeg -list_devices) und ob ffmpeg im PATH liegt.
func (a *App) PlaytestDevices() playtest.Devices { return playtest.ListDevices(exec.LookPath) }

// PlaytestStart nimmt Mikro und Bild auf und startet das Spiel mit --playtest.
func (a *App) PlaytestStart(o playtest.Options) (playtest.Status, error) {
	return a.recorder().Start(o)
}

// PlaytestStop beendet das Spiel; die Aufnahme endet danach wie bei Spielende.
func (a *App) PlaytestStop() error { return a.recorder().Stop() }

// PlaytestStatus ist der Stand der Aufnahme.
func (a *App) PlaytestStatus() playtest.Status { return a.recorder().Status() }
