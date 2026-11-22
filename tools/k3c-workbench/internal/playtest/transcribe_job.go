package playtest

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"sync"
	"time"
)

// Auftrag der Auswertung: Prüfung von ffmpeg und Modell, Hintergrundlauf mit Fortschritt und Abbruch.

// JobStatus ist der Stand der Auswertung für die Seite.
type JobStatus struct {
	Running bool   `json:"running"`
	ID      string `json:"id"`
	Step    string `json:"step"` // Transkript, Bilder
	Done    int    `json:"done"`
	Total   int    `json:"total"`
	Message string `json:"message"` // Hinweis nach dem Ende (z. B. Bilder entfallen)
	Error   string `json:"error"`
}

// Transcriber steuert höchstens eine Auswertung.
type Transcriber struct {
	Store    *Store
	Spawn    Spawn
	Out      func(args []string) (string, error) // kurzer Aufruf mit Ausgabe (Prüfung von ffmpeg)
	LookPath func(string) (string, error)
	Model    func() string      // Standardmodell
	Models   func() []ModelInfo // wählbare Modelle (nur daraus darf der Aufruf wählen)
	Now      func() time.Time
	Log      func(level, msg string, kv ...any)

	mu     sync.Mutex
	st     JobStatus
	cur    Proc
	cancel bool
	done   chan struct{}
}

func (t *Transcriber) log(level, msg string, kv ...any) {
	if t.Log != nil {
		t.Log(level, msg, kv...)
	}
}

// Check prüft ffmpeg (Version ≥ 8, Filter whisper) und die Modelldatei; Message nennt, was zu tun ist.
func (t *Transcriber) Check() Capability { return t.check(t.Model()) }

func (t *Transcriber) check(model string) Capability {
	c := Capability{Model: model}
	if st, err := os.Stat(c.Model); err == nil && !st.IsDir() {
		c.ModelOK, c.ModelMB = true, int(st.Size()/(1<<20))
	}
	ff, err := t.LookPath("ffmpeg")
	if err != nil {
		c.Message = ErrNoFFmpeg.Error()
		return c
	}
	out, _ := t.Out([]string{ff, "-hide_banner", "-version"})
	ver, major := ParseVersion(out)
	c.FFmpeg = ver
	if major < minFFmpeg {
		c.Message = fmt.Sprintf("ffmpeg %s ist zu alt, nötig ist Version %d oder neuer mit whisper-Filter (gyan full build)", orUnknown(ver), minFFmpeg)
		return c
	}
	filters, _ := t.Out([]string{ff, "-hide_banner", "-filters"})
	if c.Whisper = HasWhisper(filters); !c.Whisper {
		c.Message = "dieses ffmpeg hat keinen whisper-Filter (ffmpeg " + ver + "); die Version „full build“ von gyan.dev installieren"
		return c
	}
	if !c.ModelOK {
		c.Message = "Modell fehlt: " + c.Model + ". Datei " + ModelFile + " (574041195 Byte, SHA-256 " + ModelSHA + ") von " + ModelURL +
			" von Hand laden und nach models/ legen (oder models/whisper/*.bin bzw. Pfad in " + ModelEnv + "); die Workbench lädt nichts selbst"
		return c
	}
	c.Ready = true
	return c
}

func orUnknown(v string) string {
	if v == "" {
		return "(unbekannt)"
	}
	return v
}

// Status liefert den Stand der Auswertung.
func (t *Transcriber) Status() JobStatus {
	t.mu.Lock()
	defer t.mu.Unlock()
	return t.st
}

// TranscribeOptions wählt Modell (ID aus ListModels, leer: Standard) und Sprache (de, en, auto; leer: de).
type TranscribeOptions struct {
	Model    string `json:"model"`
	Language string `json:"language"`
}

// resolveModel nimmt nur Modelle aus der Liste (models/ oder K3C_WHISPER_MODEL), nie einen Pfad vom Client.
func (t *Transcriber) resolveModel(id string) (string, error) {
	if id == "" {
		return t.Model(), nil
	}
	if t.Models != nil {
		for _, m := range t.Models() {
			if m.ID == id {
				return m.Path, nil
			}
		}
	}
	return "", fmt.Errorf("unbekanntes Modell %q: nur Dateien aus models/ oder K3C_WHISPER_MODEL (Liste der Seite)", id)
}

// Start prüft alles Nötige und wertet die Session im Hintergrund aus (Transkript, dann Bilder).
func (t *Transcriber) Start(id string, opt TranscribeOptions) (JobStatus, error) {
	ses, err := t.Store.Get(id)
	if err != nil {
		return JobStatus{}, err
	}
	if ses.Running {
		return JobStatus{}, ErrRunning
	}
	if !ses.HasVoice {
		return JobStatus{}, ErrNoVoice
	}
	model, err := t.resolveModel(opt.Model)
	if err != nil {
		return JobStatus{}, err
	}
	lang, err := ParseLanguage(opt.Language)
	if err != nil {
		return JobStatus{}, err
	}
	if c := t.check(model); !c.Ready {
		return JobStatus{}, errors.New(c.Message)
	}
	ff, _ := t.LookPath("ffmpeg")
	t.mu.Lock()
	defer t.mu.Unlock()
	if t.st.Running {
		return t.st, ErrBusyTrans
	}
	t.st = JobStatus{Running: true, ID: id, Step: "Transkript"}
	t.cancel, t.done = false, make(chan struct{})
	t.log("info", "🚀 Auswertung gestartet", "id", id)
	go t.run(ff, id, ses.HasVideo, model, lang, t.done)
	return t.st, nil
}

// Cancel bricht die Auswertung ab; ein halbes Transkript bleibt nicht liegen.
func (t *Transcriber) Cancel() error {
	t.mu.Lock()
	defer t.mu.Unlock()
	if !t.st.Running {
		return ErrNoJob
	}
	t.cancel = true
	if t.cur != nil {
		_ = t.cur.Kill()
	}
	return nil
}

// Wait wartet in Tests aufs Ende der Auswertung.
func (t *Transcriber) Wait() {
	t.mu.Lock()
	d := t.done
	t.mu.Unlock()
	if d != nil {
		<-d
	}
}

func (t *Transcriber) set(f func(s *JobStatus)) {
	t.mu.Lock()
	f(&t.st)
	t.mu.Unlock()
}

// exec führt einen Aufruf aus und wartet; Abbruch und Fehler kommen als error zurück.
func (t *Transcriber) exec(args []string, dir string) error {
	t.mu.Lock()
	if t.cancel {
		t.mu.Unlock()
		return errCanceled
	}
	p, err := t.Spawn(args, dir)
	if err != nil {
		t.mu.Unlock()
		return err
	}
	t.cur = p
	t.mu.Unlock()
	werr := p.Wait()
	t.mu.Lock()
	t.cur = nil
	canceled := t.cancel
	t.mu.Unlock()
	if canceled {
		return errCanceled
	}
	if werr != nil {
		return fmt.Errorf("ffmpeg: %v: %s", werr, p.Tail())
	}
	return nil
}

var errCanceled = errors.New("abgebrochen")

// run ist der Hintergrundauftrag.
func (t *Transcriber) run(ffmpeg, id string, hasVideo bool, model, lang string, done chan struct{}) {
	defer close(done)
	dir := filepath.Join(t.Store.Root, id)
	err := t.transcribe(ffmpeg, dir, model, lang)
	note := ""
	if err == nil {
		note, err = t.frames(ffmpeg, dir, hasVideo)
	}
	t.set(func(s *JobStatus) {
		s.Running, s.Message = false, note
		if err != nil {
			s.Error = err.Error()
		}
	})
	switch {
	case errors.Is(err, errCanceled):
		t.log("info", "🛑 Auswertung abgebrochen", "id", id)
	case err != nil:
		t.log("warn", "❌ Auswertung fehlgeschlagen: "+err.Error(), "id", id)
	default:
		t.log("info", "✅ Auswertung fertig", "id", id)
	}
}

// transcribe lässt whisper laufen und schreibt transcript.json erst, wenn alles gelesen ist.
func (t *Transcriber) transcribe(ffmpeg, dir, model, lang string) error {
	part := filepath.Join(dir, fileRawTmp)
	defer func() { _ = os.Remove(part) }()
	_ = os.Remove(part)
	if err := t.exec(WhisperArgs(ffmpeg, dir, model, lang, fileRawTmp), dir); err != nil {
		return err
	}
	raw, err := os.ReadFile(part)
	if err != nil {
		return fmt.Errorf("whisper hat kein Ergebnis geschrieben: %w", err)
	}
	meta := readMeta(dir)
	segs, err := ParseSegments(raw, CaptureStart(meta))
	if err != nil {
		return err
	}
	tr := Transcript{AudioStartMs: CaptureStart(meta), Language: lang, Model: filepath.Base(model), Segments: segs}
	out, _ := json.MarshalIndent(tr, "", "  ")
	tmp := filepath.Join(dir, fileTranscript+".tmp")
	if err := os.WriteFile(tmp, out, 0o644); err != nil {
		return err
	}
	if err := os.Rename(tmp, filepath.Join(dir, fileTranscript)); err != nil {
		return err
	}
	meta.Stand = StageTranscribed
	writeMeta(dir, meta)
	return nil
}

// frames zieht je Marker ein Bild; ohne Video entfallen sie mit Hinweis.
func (t *Transcriber) frames(ffmpeg, dir string, hasVideo bool) (string, error) {
	marks, err := ReadMarkers(dir)
	if err != nil || len(marks) == 0 {
		return "keine Marker, keine Bilder", err
	}
	if !hasVideo {
		return "video.mkv fehlt, Bilder entfallen", nil
	}
	if err := os.MkdirAll(filepath.Join(dir, dirFrames), 0o755); err != nil {
		return "", err
	}
	t.set(func(s *JobStatus) { s.Step, s.Done, s.Total = "Bilder", 0, len(marks) })
	start := CaptureStart(readMeta(dir))
	for i, m := range marks {
		sek := float64(max(m.Ts-start, 0)) / 1000
		if err := t.exec(FrameArgs(ffmpeg, sek, filepath.FromSlash(FrameName(i+1))), dir); err != nil {
			return "", err
		}
		t.set(func(s *JobStatus) { s.Done = i + 1 })
	}
	return "", nil
}

// readMeta liest session.json; fehlt es, ist Meta leer.
func readMeta(dir string) Meta {
	var m Meta
	if raw, err := os.ReadFile(filepath.Join(dir, fileMeta)); err == nil {
		_ = json.Unmarshal(raw, &m)
	}
	return m
}
