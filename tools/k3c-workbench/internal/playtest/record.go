package playtest

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"sync"
	"sync/atomic"
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
	//nolint:staticcheck // ST1005: deutsche Meldung für die Oberfläche
	ErrBusy        = errors.New("Aufnahme läuft schon")
	ErrNotRunning  = errors.New("keine Aufnahme aktiv")
	ErrNoFFmpeg    = errors.New("ffmpeg nicht gefunden (ffmpeg installieren)")
	ErrNoGame      = errors.New("k3c-game.exe nicht gefunden (task build)")
	ErrNoMic       = errors.New("kein Mikrofon gewählt")
	ErrBadMonitor  = errors.New("ungültiger Monitor (0 bis 15)")
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
	// VoiceOnly nimmt nur das Mikro auf (diktieren, Mikro testen): kein Bild, kein Spiel; das Ende bestimmt Stop.
	VoiceOnly bool `json:"voiceOnly"`
}

// Status der Aufnahme für Seite und Anzeige.
type Status struct {
	Running   bool   `json:"running"`
	ID        string `json:"id"`
	StartMs   int64  `json:"startMs"`
	Encoder   string `json:"encoder"`
	VoiceOnly bool   `json:"voiceOnly"`       // Mikro-Session ohne Bild und Spiel
	Starting  bool   `json:"starting"`        // das Spiel startet, die Aufnahme beginnt mit seinem ersten Bild
	Stats     *Stats `json:"stats,omitempty"` // Messwerte der laufenden Aufnahme (Diagramme, Warnung)
}

// Recorder steuert höchstens eine Aufnahme.
type Recorder struct {
	Store    *Store
	Spawn    Spawn
	LookPath func(string) (string, error)
	Check    func(ffmpeg string, o Options) error // prüft Mikro und Monitor gegen die echten Geräte; nil: keine Prüfung
	Pick     func(ffmpeg string) string           // wählt den Encoder, der sich wirklich öffnen lässt; nil: QSV, dann libx264
	Ready    time.Duration                        // Frist für das erste Bild des Spiels, danach beginnt die Aufnahme trotzdem (Standard 30 s)
	Interval time.Duration                        // Takt des Wächters (Standard 1 s)
	Game     func() (exe, dir string, err error)
	Now      func() time.Time
	Log      func(level, msg string, kv ...any)
	Settle   time.Duration // so lange darf ffmpeg nach dem Start nicht enden, sonst gilt der Start als gescheitert
	QuitWait time.Duration // Frist für ffmpeg nach `q`, danach Kill

	active   atomic.Pointer[string] // Kennung der laufenden Aufnahme, lock-frei lesbar (Store.Active)
	mu       sync.Mutex
	stats    Stats
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
	scale, codec := encoderSpec(enc)
	// flush_packets und die kurzen Matroska-Blöcke lassen die Dateien jede Sekunde wachsen: Daran misst der Wächter.
	a := []string{ffmpeg, "-hide_banner", "-loglevel", "warning", "-nostats", "-y",
		"-f", "dshow", "-i", "audio=" + mic,
		"-f", "lavfi", "-i", "ddagrab=output_idx=" + strconv.Itoa(monitor) + ":framerate=30",
		"-map", "0:a", "-ac", "1", "-ar", "16000", "-c:a", "pcm_s16le", "-flush_packets", "1", filepath.Join(dir, fileVoice),
		"-map", "1:v", "-vf", "hwdownload,format=bgra," + scale, "-r", "30"}
	a = append(a, codec...)
	return append(a, "-flush_packets", "1", "-cluster_time_limit", "1000", filepath.Join(dir, fileVideo))
}

// VoiceArgs ist der Aufruf der Mikro-Session: nur voice.wav, ohne Bild.
func VoiceArgs(ffmpeg, dir, mic string) []string {
	return []string{ffmpeg, "-hide_banner", "-loglevel", "warning", "-nostats", "-y",
		"-f", "dshow", "-i", "audio=" + mic,
		"-ac", "1", "-ar", "16000", "-c:a", "pcm_s16le", "-flush_packets", "1", filepath.Join(dir, fileVoice)}
}

// stopper ist das „Spiel“ einer Mikro-Session: Es endet mit Kill (Stop) oder wenn ffmpeg endet.
type stopper struct {
	ch   chan struct{}
	dead <-chan struct{}
	once sync.Once
}

func (s *stopper) Wait() error {
	select {
	case <-s.ch:
	case <-s.dead:
	}
	return nil
}
func (s *stopper) Quit() error  { return s.Kill() }
func (s *stopper) Kill() error  { s.once.Do(func() { close(s.ch) }); return nil }
func (s *stopper) Tail() string { return "" }

// GameArgs ist der Spielstart: das Spiel schreibt markers.jsonl und survey.json in den Session-Ordner.
func GameArgs(exe, dir string, extra []string) []string {
	return append([]string{exe, "--playtest", dir}, extra...)
}

// ActiveID ist die Session, die dieser Recorder gerade aufnimmt (leer: keine).
func (r *Recorder) ActiveID() string {
	if p := r.active.Load(); p != nil {
		return *p
	}
	return ""
}

// Status liefert den Stand der Aufnahme.
func (r *Recorder) Status() Status {
	r.mu.Lock()
	defer r.mu.Unlock()
	st := r.st
	if st.Running {
		c := r.stats.clone()
		st.Stats = &c
	}
	return st
}

// busy meldet, ob schon eine Aufnahme läuft (kurz unter r.mu).
func (r *Recorder) busy() (Status, bool) {
	r.mu.Lock()
	defer r.mu.Unlock()
	return r.st, r.st.Running
}

// startFFmpeg probiert zuerst `first` (der Encoder, den die Probe bestätigt hat; leer: QSV), dann libx264; ein
// Prozess, der innerhalb von Settle endet, gilt als gescheitert.
func (r *Recorder) startFFmpeg(ffmpeg, dir string, o Options, first string) (*waited, string, error) {
	if o.VoiceOnly {
		p, err := r.Spawn(VoiceArgs(ffmpeg, dir, o.Mic), dir)
		if err != nil {
			return nil, "", fmt.Errorf("%w: %v", errNoRecording, err)
		}
		w := &waited{Proc: p, dead: make(chan struct{})}
		go func() { w.err = p.Wait(); close(w.dead) }()
		select {
		case <-w.dead:
			return nil, "", fmt.Errorf("%w (Mikro belegt oder Gerät falsch?): %s", errNoRecording, p.Tail())
		case <-time.After(r.Settle):
			return w, "", nil
		}
	}
	var last string
	encs := []string{EncoderQSV, EncoderX264}
	switch first {
	case "":
	case EncoderX264:
		encs = []string{EncoderX264}
	default:
		encs = []string{first, EncoderX264}
	}
	for _, enc := range encs {
		p, err := r.Spawn(FFmpegArgs(ffmpeg, dir, o.Mic, o.Monitor, enc), dir)
		if err != nil {
			return nil, "", fmt.Errorf("%w: %v", errNoRecording, err)
		}
		w := &waited{Proc: p, dead: make(chan struct{})}
		go func() { w.err = p.Wait(); close(w.dead) }()
		select {
		case <-w.dead:
			last = p.Tail()
			r.log("warn", "❌ ffmpeg endet beim Start ("+enc+"): "+last, "encoder", enc)
		case <-time.After(r.Settle):
			return w, enc, nil
		}
	}
	return nil, "", fmt.Errorf("%w (Mikro belegt oder Gerät falsch?): %s", errNoRecording, last)
}

// waited ist ein Proc, dessen Wait schon in einer Goroutine läuft; Wait darf hier mehrfach aufgerufen werden.
type waited struct {
	Proc
	dead chan struct{}
	err  error
}

func (w *waited) Wait() error { <-w.dead; return w.err }

// exited sagt, ob der Prozess schon beendet ist (ohne zu warten).
func (w *waited) exited() bool {
	select {
	case <-w.dead:
		return true
	default:
		return false
	}
}

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
func (r *Recorder) finish(dir string, meta Meta, rec *waited, game Proc, done, stopWatch chan struct{}) {
	werr := game.Wait()
	early := rec.exited() // ffmpeg war vor dem Spielende schon weg: die Aufnahme ist unvollständig
	close(stopWatch)
	tail := rec.Tail()
	r.quit(rec)
	r.mu.Lock()
	meta.EndMs, meta.Stand = r.Now().UnixMilli(), StageRecorded
	if early {
		meta.Error = "ffmpeg vorzeitig beendet, Aufnahme unvollständig: " + tail
	}
	if werr != nil && !r.stopping {
		meta.Crash, meta.GameExit = true, werr.Error()
	}
	id := r.st.ID
	r.st, r.game, r.stats = Status{}, nil, Stats{}
	r.mu.Unlock()
	writeMeta(dir, meta)
	r.active.Store(nil)
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
