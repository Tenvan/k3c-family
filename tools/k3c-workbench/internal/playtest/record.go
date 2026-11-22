package playtest

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"sync"
	"time"
)

// Aufnahme (B-1219): ein ffmpeg-Prozess nimmt das Raummikro (voice.wav) und das Spielbild (video.mkv) auf, danach startet
// das Spiel mit `--playtest <ordner>`; endet das Spiel (oder Stopp), endet ffmpeg mit `q` auf stdin. Prozesse entstehen
// nur über Spawn, damit Tests Attrappen einsetzen (nie ein echtes ffmpeg).

// Encoder der Bildaufnahme.
const (
	EncoderQSV  = "h264_qsv 1080p30"
	EncoderX264 = "libx264 ultrafast 720p30"
)

// Fehler der Aufnahme; die Oberfläche zeigt den Text.
var (
	ErrBusy        = errors.New("Aufnahme läuft schon")
	ErrNotRunning  = errors.New("keine Aufnahme aktiv")
	ErrNoFFmpeg    = errors.New("ffmpeg nicht im PATH gefunden (ffmpeg installieren)")
	ErrNoGame      = errors.New("k3c-game.exe nicht gefunden (task build)")
	ErrNoMic       = errors.New("kein Mikrofon gewählt")
	errNoRecording = errors.New("ffmpeg startet nicht")
)

// Proc ist ein laufender Prozess; Wait darf nur einer aufrufen.
type Proc interface {
	Wait() error
	Quit() error // sauber beenden (ffmpeg: `q` auf stdin)
	Kill() error
	Tail() string // letzte Zeile von stderr
}

// Spawn startet args (args[0] = Programm) in dir; stdin nur für ffmpeg nötig.
type Spawn func(args []string, dir string) (Proc, error)

// Options wählt Gerät und Bildquelle und gibt dem Spiel Zusatzargumente.
type Options struct {
	Mic      string   `json:"mic"`      // Name des DirectShow-Geräts
	Monitor  int      `json:"monitor"`  // ddagrab output_idx, 0-basiert
	GameArgs []string `json:"gameArgs"` // zusätzlich zu --playtest <ordner>
}

// Status der Aufnahme für Seite und Anzeige.
type Status struct {
	Running bool   `json:"running"`
	ID      string `json:"id"`
	StartMs int64  `json:"startMs"`
	Encoder string `json:"encoder"`
}

// Recorder steuert höchstens eine Aufnahme.
type Recorder struct {
	Store    *Store
	Spawn    Spawn
	LookPath func(string) (string, error)
	Game     func() (exe, dir string, err error)
	Now      func() time.Time
	Log      func(level, msg string, kv ...any)
	Settle   time.Duration // so lange darf ffmpeg nach dem Start nicht enden, sonst gilt der Start als gescheitert
	QuitWait time.Duration // Frist für ffmpeg nach `q`, danach Kill

	mu       sync.Mutex
	st       Status
	game     Proc
	stopping bool
	done     chan struct{}
}

func (r *Recorder) log(level, msg string, kv ...any) {
	if r.Log != nil {
		r.Log(level, msg, kv...)
	}
}

// FFmpegArgs ist der Aufruf: Mikro mono 16 kHz PCM, Bild per ddagrab (Desktop Duplication, GPU) vom Monitor.
func FFmpegArgs(ffmpeg, dir, mic string, monitor int, enc string) []string {
	chain, codec := "hwdownload,format=bgra,scale=1920:1080:flags=fast_bilinear,format=nv12",
		[]string{"-c:v", "h264_qsv", "-global_quality", "26", "-look_ahead", "0"}
	if enc == EncoderX264 {
		chain = "hwdownload,format=bgra,scale=1280:720:flags=fast_bilinear,format=yuv420p"
		codec = []string{"-c:v", "libx264", "-preset", "ultrafast", "-crf", "28"}
	}
	a := []string{ffmpeg, "-hide_banner", "-loglevel", "warning", "-nostats", "-y",
		"-f", "dshow", "-i", "audio=" + mic,
		"-f", "lavfi", "-i", "ddagrab=output_idx=" + strconv.Itoa(monitor) + ":framerate=30",
		"-map", "0:a", "-ac", "1", "-ar", "16000", "-c:a", "pcm_s16le", filepath.Join(dir, fileVoice),
		"-map", "1:v", "-vf", chain, "-r", "30"}
	a = append(a, codec...)
	return append(a, filepath.Join(dir, fileVideo))
}

// GameArgs ist der Spielstart: das Spiel schreibt markers.jsonl und survey.json in den Session-Ordner.
func GameArgs(exe, dir string, extra []string) []string {
	return append([]string{exe, "--playtest", dir}, extra...)
}

// Status liefert den Stand der Aufnahme.
func (r *Recorder) Status() Status {
	r.mu.Lock()
	defer r.mu.Unlock()
	return r.st
}

// Start prüft ffmpeg, Spiel und Mikro, nimmt auf (QSV, sonst libx264) und startet das Spiel. Scheitert etwas davor,
// startet kein Spiel und der Ordner verschwindet wieder.
func (r *Recorder) Start(o Options) (Status, error) {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.st.Running {
		return r.st, ErrBusy
	}
	ffmpeg, err := r.LookPath("ffmpeg")
	if err != nil {
		return r.st, ErrNoFFmpeg
	}
	exe, gdir, err := r.Game()
	if err != nil {
		return r.st, err
	}
	if strings.TrimSpace(o.Mic) == "" {
		return r.st, ErrNoMic
	}
	id, dir, err := r.newDir()
	if err != nil {
		return r.st, err
	}
	rec, enc, err := r.startFFmpeg(ffmpeg, dir, o)
	if err != nil {
		_ = os.RemoveAll(dir)
		r.log("warn", "❌ Aufnahme nicht gestartet: "+err.Error(), "id", id)
		return r.st, err
	}
	meta := Meta{StartMs: r.Now().UnixMilli(), Mic: o.Mic, Monitor: strconv.Itoa(o.Monitor), Encoder: enc, Stand: standRunning,
		FFmpeg: strings.Join(FFmpegArgs("ffmpeg", dir, o.Mic, o.Monitor, enc), " "), GameArgs: GameArgs("k3c-game", dir, o.GameArgs)[1:]}
	writeMeta(dir, meta)
	game, err := r.Spawn(GameArgs(exe, dir, o.GameArgs), gdir)
	if err != nil {
		r.quit(rec)
		meta.EndMs, meta.Stand, meta.Error = r.Now().UnixMilli(), StageRecorded, "Spiel nicht gestartet: "+err.Error()
		writeMeta(dir, meta)
		r.log("warn", "❌ Spiel nicht gestartet: "+err.Error(), "id", id)
		return r.st, fmt.Errorf("Spiel nicht gestartet: %w", err)
	}
	r.st = Status{Running: true, ID: id, StartMs: meta.StartMs, Encoder: enc}
	r.game, r.stopping, r.done = game, false, make(chan struct{})
	r.log("info", "🚀 Spieltest gestartet", "id", id, "encoder", enc)
	go r.finish(dir, meta, rec, game, r.done)
	return r.st, nil
}

// startFFmpeg probiert QSV, dann libx264; ein Prozess, der innerhalb von Settle endet, gilt als gescheitert.
func (r *Recorder) startFFmpeg(ffmpeg, dir string, o Options) (Proc, string, error) {
	var last string
	for _, enc := range []string{EncoderQSV, EncoderX264} {
		p, err := r.Spawn(FFmpegArgs(ffmpeg, dir, o.Mic, o.Monitor, enc), dir)
		if err != nil {
			return nil, "", fmt.Errorf("%w: %v", errNoRecording, err)
		}
		ended := make(chan error, 1)
		go func() { ended <- p.Wait() }()
		select {
		case <-ended:
			last = p.Tail()
			r.log("warn", "❌ ffmpeg endet beim Start ("+enc+"): "+last, "encoder", enc)
		case <-time.After(r.Settle):
			return &waited{Proc: p, ended: ended}, enc, nil
		}
	}
	return nil, "", fmt.Errorf("%w (Mikro belegt oder Gerät falsch?): %s", errNoRecording, last)
}

// waited ist ein Proc, dessen Wait schon in einer Goroutine läuft.
type waited struct {
	Proc
	ended chan error
}

func (w *waited) Wait() error { return <-w.ended }

// quit beendet ffmpeg sauber (`q`), nach QuitWait hart.
func (r *Recorder) quit(p Proc) {
	_ = p.Quit()
	ended := make(chan struct{})
	go func() { _ = p.Wait(); close(ended) }()
	select {
	case <-ended:
	case <-time.After(r.QuitWait):
		_ = p.Kill()
		<-ended
	}
}

// finish wartet aufs Spielende, beendet ffmpeg und schreibt session.json (Stand „aufgenommen“, Absturz vermerkt).
func (r *Recorder) finish(dir string, meta Meta, rec, game Proc, done chan struct{}) {
	werr := game.Wait()
	r.quit(rec)
	r.mu.Lock()
	meta.EndMs, meta.Stand = r.Now().UnixMilli(), StageRecorded
	if werr != nil && !r.stopping {
		meta.Crash, meta.GameExit = true, werr.Error()
	}
	id := r.st.ID
	r.st, r.game = Status{}, nil
	r.mu.Unlock()
	writeMeta(dir, meta)
	if meta.Crash {
		r.log("warn", "💥 Spiel abgestürzt, Aufnahme beendet: "+meta.GameExit, "id", id)
	} else {
		r.log("info", "🛑 Spieltest beendet", "id", id, "dauerMs", meta.EndMs-meta.StartMs)
	}
	close(done)
}

// Stop beendet das Spiel; die Aufnahme endet danach wie bei Spielende. Wartet nicht auf das Ende.
func (r *Recorder) Stop() error {
	r.mu.Lock()
	defer r.mu.Unlock()
	if !r.st.Running || r.game == nil {
		return ErrNotRunning
	}
	r.stopping = true
	return r.game.Kill()
}

// Wait wartet in Tests aufs Ende der laufenden Aufnahme.
func (r *Recorder) Wait() {
	r.mu.Lock()
	d := r.done
	r.mu.Unlock()
	if d != nil {
		<-d
	}
}

// newDir legt playtests/<JJJJ-MM-TT>-<nr>/ an (nr zählt je Tag ab 1).
func (r *Recorder) newDir() (id, dir string, err error) {
	day := r.Now().Format("2006-01-02")
	list, err := r.Store.List()
	if err != nil {
		return "", "", err
	}
	n := 1
	for _, s := range list {
		var k int
		if s.Date == day {
			if k, _ = strconv.Atoi(s.ID[11:]); k >= n {
				n = k + 1
			}
		}
	}
	id = fmt.Sprintf("%s-%d", day, n)
	dir = filepath.Join(r.Store.Root, id)
	return id, dir, os.MkdirAll(dir, 0o755)
}

func writeMeta(dir string, m Meta) {
	raw, _ := json.MarshalIndent(m, "", "  ")
	_ = os.WriteFile(filepath.Join(dir, fileMeta), raw, 0o644)
}
