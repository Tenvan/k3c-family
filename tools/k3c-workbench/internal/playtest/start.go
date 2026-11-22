package playtest

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

// fileReady schreibt das Spiel (`--playtest`), sobald sein erstes richtiges Bild steht (Unix-ms als Text).
const fileReady = "ready"

// readyWait ist die Frist, nach der die Aufnahme auch ohne `ready` beginnt (älteres Spiel ohne die Meldung).
const readyWait = 30 * time.Second

// Start prüft ffmpeg, Spiel und Mikro, startet das Spiel, wartet auf sein erstes Bild und beginnt dann die Aufnahme
// (Ton und Bild starten mit dem Spiel, nicht beim Laden). Scheitert etwas, endet das Spiel wieder und der Ordner
// verschwindet. Während des Starts meldet Status `Starting`; Status und Stopp antworten ohne zu warten.
func (r *Recorder) Start(o Options) (Status, error) {
	if st, busy := r.busy(); busy {
		return st, ErrBusy
	}
	ffmpeg, err := r.LookPath("ffmpeg")
	if err != nil {
		return r.Status(), noFFmpeg(err)
	}
	var exe, gdir string
	if !o.VoiceOnly {
		if exe, gdir, err = r.Game(); err != nil {
			return r.Status(), err
		}
	}
	if strings.TrimSpace(o.Mic) == "" {
		return r.Status(), ErrNoMic
	}
	if !o.VoiceOnly && (o.Monitor < 0 || o.Monitor > 15) {
		return r.Status(), ErrBadMonitor
	}
	// Die Geräteprobe dauert bis zu einigen Sekunden und läuft ohne r.mu, damit Status() in dieser Zeit antwortet.
	if r.Check != nil {
		if err := r.Check(ffmpeg, o); err != nil {
			r.log("warn", "❌ Gerät nicht nutzbar: "+err.Error())
			return r.Status(), err
		}
	}
	first := ""
	if r.Pick != nil && !o.VoiceOnly {
		first = r.Pick(ffmpeg)
	}
	id, dir, err := r.reserve(o)
	if err != nil {
		return r.Status(), err
	}
	if o.VoiceOnly {
		return r.begin(ffmpeg, dir, id, o, first, nil)
	}
	game, err := r.Spawn(GameArgs(exe, dir, o.GameArgs), gdir)
	if err != nil {
		return r.fail(dir, id, fmt.Errorf("Spiel nicht gestartet: %w", err))
	}
	gw := &waited{Proc: game, dead: make(chan struct{})}
	go func() { gw.err = game.Wait(); close(gw.dead) }()
	r.mu.Lock()
	r.game = gw // ab jetzt beendet Stopp das Spiel, auch während es noch lädt
	r.mu.Unlock()
	if err := r.waitReady(dir, gw); err != nil {
		return r.fail(dir, id, err)
	}
	return r.begin(ffmpeg, dir, id, o, first, gw)
}

// reserve legt Ordner und Kennung an und setzt den Stand „startet“; danach lehnt jeder weitere Start mit ErrBusy ab.
func (r *Recorder) reserve(o Options) (id, dir string, err error) {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.st.Running {
		return "", "", ErrBusy
	}
	if id, dir, err = r.newDir(); err != nil {
		return "", "", err
	}
	r.active.Store(&id)
	r.st = Status{Running: true, Starting: true, ID: id, StartMs: r.Now().UnixMilli(), VoiceOnly: o.VoiceOnly}
	r.stats, r.stopping, r.done = Stats{NoVideo: o.VoiceOnly}, false, make(chan struct{})
	return id, dir, nil
}

// fail räumt einen gescheiterten Start auf: Stand zurück, Ordner weg, Wait() frei.
func (r *Recorder) fail(dir, id string, err error) (Status, error) {
	_ = os.RemoveAll(dir)
	r.mu.Lock()
	r.st, r.game, r.stats = Status{}, nil, Stats{}
	done := r.done
	r.mu.Unlock()
	r.active.Store(nil)
	r.log("warn", "❌ Aufnahme nicht gestartet: "+err.Error(), "id", id)
	close(done)
	return Status{}, err
}

// waitReady wartet auf `ready` im Session-Ordner. Endet das Spiel vorher, scheitert der Start; kommt nichts innerhalb
// der Frist, beginnt die Aufnahme trotzdem.
func (r *Recorder) waitReady(dir string, game *waited) error {
	limit := r.Ready
	if limit <= 0 {
		limit = readyWait
	}
	path := filepath.Join(dir, fileReady)
	deadline := time.After(limit)
	tick := time.NewTicker(50 * time.Millisecond)
	defer tick.Stop()
	for {
		if _, err := os.Stat(path); err == nil {
			_ = os.Remove(path)
			return nil
		}
		select {
		case <-game.dead:
			return fmt.Errorf("Spiel vor dem ersten Bild beendet: %s", game.Tail())
		case <-deadline:
			r.log("warn", "⏳ Spiel meldet kein erstes Bild, Aufnahme beginnt trotzdem")
			return nil
		case <-tick.C:
		}
	}
}

// begin startet ffmpeg (mit Spiel erst nach dessen erstem Bild), schreibt session.json und startet Wächter und Ende.
func (r *Recorder) begin(ffmpeg, dir, id string, o Options, first string, game *waited) (Status, error) {
	capture := r.Now().UnixMilli()
	rec, enc, err := r.startFFmpeg(ffmpeg, dir, o, first)
	if err != nil {
		if game != nil {
			_ = game.Kill()
			<-game.dead
		}
		return r.fail(dir, id, err)
	}
	meta := Meta{CaptureMs: capture, StartMs: r.Now().UnixMilli(), Mic: o.Mic, Monitor: strconv.Itoa(o.Monitor), Encoder: enc, Stand: standRunning,
		FFmpeg: strings.Join(FFmpegArgs("ffmpeg", dir, o.Mic, o.Monitor, enc), " "), GameArgs: GameArgs("k3c-game", dir, o.GameArgs)[1:]}
	var proc Proc = game
	if o.VoiceOnly {
		meta.Monitor, meta.FFmpeg, meta.GameArgs = "", strings.Join(VoiceArgs("ffmpeg", dir, o.Mic), " "), nil
		proc = &stopper{ch: make(chan struct{}), dead: rec.dead}
	}
	writeMeta(dir, meta)
	r.mu.Lock()
	r.game = proc
	r.st = Status{Running: true, ID: id, StartMs: meta.StartMs, Encoder: enc, VoiceOnly: o.VoiceOnly}
	st, done := r.st, r.done
	r.mu.Unlock()
	r.log("info", "🚀 Spieltest gestartet", "id", id, "encoder", enc, "nurMikro", o.VoiceOnly)
	stopWatch := make(chan struct{}) // endet mit dem Spiel, vor ffmpeg: sonst warnt der Wächter beim normalen Stopp
	go r.watch(dir, rec, stopWatch)
	go r.finish(dir, meta, rec, proc, done, stopWatch)
	return st, nil
}
