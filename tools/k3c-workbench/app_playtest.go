package main

import (
	"errors"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
	"sync"

	"k3c/tools/k3c-workbench/internal/playtest"
	"k3c/tools/workkit/uikit"
)

// Seite Spieltest (B-1218): Sessions in `playtests/<JJJJ-MM-TT>-<nr>/` im Repo (gitignored), Aufräumen per Knopf.
// Der Ordner-Vertrag steht in internal/playtest; die Bindings reichen nur durch und melden ins Log.

// playtestMediaPrefix ist die Adresse der Player (nur HTTP-Betrieb; Ablage-Datei nur voice.wav und video.mkv).
const playtestMediaPrefix = uikit.Prefix + "playtest/media/"

func (a *App) playtestStore() *playtest.Store {
	return &playtest.Store{Root: filepath.Join(a.root, "playtests"), Active: func() string { return a.recorder().ActiveID() }}
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

// windowMediaHandler liefert dem Fenster (Wails-Asset-Server, kein HTTP-Port) die Player- und Marker-Dateien unter denselben
// Adressen wie im Client/Server-Betrieb. Nur diese beiden Lesewege, alles andere ist 404; die Dateien kommen über
// Store.MediaPath und FramePath (feste Namen, geprüfte Session-ID), nie über einen freien Pfad.
func (a *App) windowMediaHandler() http.Handler {
	mux := http.NewServeMux()
	mux.Handle("GET "+playtestMediaPrefix+"{id}/{file}", a.playtestMedia())
	mux.Handle("GET "+playtestFramePrefix+"{id}/{nr}", a.playtestFrame())
	return mux
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

// PlaytestDevices nennt die Mikrofone (ffmpeg -list_devices), die aufnehmbaren Monitore (Probebild) und ob ffmpeg gefunden wird.
func (a *App) PlaytestDevices() playtest.Devices { return playtest.ListDevices(playtest.FindFFmpeg) }

// PlaytestStart nimmt Mikro und Bild auf und startet das Spiel mit --playtest.
func (a *App) PlaytestStart(o playtest.Options) (playtest.Status, error) {
	return a.recorder().Start(o)
}

// PlaytestStop beendet das Spiel; die Aufnahme endet danach wie bei Spielende.
func (a *App) PlaytestStop() error { return a.recorder().Stop() }

// PlaytestStatus ist der Stand der Aufnahme.
func (a *App) PlaytestStatus() playtest.Status { return a.recorder().Status() }

// playtestFramePrefix ist die Adresse der Marker-Bilder.
const playtestFramePrefix = uikit.Prefix + "playtest/frame/"

// transcribers hält je App eine Auswertung (wie recorders).
var transcribers sync.Map

func (a *App) transcriber() *playtest.Transcriber {
	if r, ok := transcribers.Load(a); ok {
		return r.(*playtest.Transcriber)
	}
	r := playtest.NewTranscriber(a.root, a.playtestStore(), func(level, msg string, kv ...any) {
		attrs := append([]any{"ns", "main"}, kv...)
		if level == "warn" {
			a.log.Warn(msg, attrs...)
			return
		}
		a.log.Info(msg, attrs...)
	})
	act, _ := transcribers.LoadOrStore(a, r)
	return act.(*playtest.Transcriber)
}

// PlaytestCheck sagt, ob die Auswertung möglich ist (ffmpeg ab 8 mit whisper-Filter, Modelldatei), sonst warum nicht.
func (a *App) PlaytestCheck() playtest.Capability { return a.transcriber().Check() }

// PlaytestTranscribe wertet eine Session im Hintergrund aus: Transkript aus voice.wav, ein Bild je Marker.
func (a *App) PlaytestTranscribe(id string, opt playtest.TranscribeOptions) (playtest.JobStatus, error) {
	return a.transcriber().Start(id, opt)
}

// PlaytestModels sind die wählbaren Modelldateien (K3C_WHISPER_MODEL, models/, models/whisper/) mit Größe.
func (a *App) PlaytestModels() []playtest.ModelInfo { return playtest.ListModels(a.root) }

// PlaytestTranscribeStatus ist der Stand der Auswertung.
func (a *App) PlaytestTranscribeStatus() playtest.JobStatus { return a.transcriber().Status() }

// PlaytestTranscribeCancel bricht die Auswertung ab.
func (a *App) PlaytestTranscribeCancel() error { return a.transcriber().Cancel() }

// PlaytestTimeline ist die Zeitleiste einer Session: Sätze, Marker und deren Bilder.
func (a *App) PlaytestTimeline(id string) (playtest.Timeline, error) {
	return a.playtestStore().Timeline(id)
}

// PlaytestFindings sind die Befunde einer Session (findings.json).
func (a *App) PlaytestFindings(id string) ([]playtest.Finding, error) {
	return a.playtestStore().Findings(id)
}

// PlaytestFindingUpdate setzt Haken und Vorschlag (Domäne, Typ, Prio, Projekt, Sprint) eines Befunds.
func (a *App) PlaytestFindingUpdate(id, fid string, p playtest.FindingPatch) (playtest.Finding, error) {
	return a.playtestStore().UpdateFinding(id, fid, p)
}

// PlaytestFrameSkip blendet das Bild eines Marker aus oder ein (die Datei bleibt).
func (a *App) PlaytestFrameSkip(id string, nr int, skip bool) error {
	return a.playtestStore().SetFrameSkip(id, nr, skip)
}

// playtestFrame liefert ein Marker-Bild (frames/marker-<nr>.jpg).
func (a *App) playtestFrame() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		nr, _ := strconv.Atoi(r.PathValue("nr"))
		p, err := a.playtestStore().FramePath(r.PathValue("id"), nr)
		if err != nil {
			uikit.Fail(w, http.StatusNotFound, err.Error())
			return
		}
		w.Header().Set("Cache-Control", "no-store")
		http.ServeFile(w, r, p)
	})
}
