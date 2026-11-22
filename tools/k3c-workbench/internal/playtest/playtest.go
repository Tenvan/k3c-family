// Package playtest liest die Ablage der Spieltests (`playtests/<JJJJ-MM-TT>-<nr>/`, B-1218) und räumt sie auf.
// Ordner-Vertrag (gilt für WZ64–WZ66): session.json, voice.wav, video.mkv, markers.jsonl, survey.json,
// transcript.json, frames/, findings.json; Zeitstempel überall Unix-Millisekunden der Systemuhr. Gelöscht wird
// nur nach Bestätigung (confirm im Aufruf), eine laufende Aufnahme ist gesperrt.
package playtest

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// Dateien des Ordner-Vertrags, die das Paket kennt.
const (
	fileMeta       = "session.json"
	fileVideo      = "video.mkv"
	fileVoice      = "voice.wav"
	fileMarkers    = "markers.jsonl"
	fileTranscript = "transcript.json"
	fileFindings   = "findings.json"
)

// Stand einer Session.
const (
	StageRunning     = "läuft"
	StageIncomplete  = "unvollständig"
	StageRecorded    = "aufgenommen"
	StageTranscribed = "transkribiert"
	StageEvaluated   = "ausgewertet"
)

// Aufräum-Aktionen.
const (
	ActionVideo     = "video"       // video.mkv einer Session
	ActionSession   = "session"     // ganze Session
	ActionEvaluated = "ausgewertet" // video.mkv aller ausgewerteten Sessions
)

// standRunning ist der Wert von session.json › stand während der Aufnahme.
const standRunning = "laeuft"

// Fehler; die Oberfläche zeigt den Text.
var (
	ErrBadID        = errors.New("ungültige Session-Kennung")
	ErrBadAction    = errors.New("unbekannte Aktion (video, session oder ausgewertet)")
	ErrNotFound     = errors.New("Session oder Datei nicht gefunden")
	ErrRunning      = errors.New("Aufnahme läuft, Löschen gesperrt")
	ErrNotConfirmed = errors.New("Löschen nicht bestätigt")
)

var idRe = regexp.MustCompile(`^\d{4}-\d{2}-\d{2}-\d+$`)

// Meta ist session.json (Schreiber: Aufnahme, WZ64.2).
type Meta struct {
	StartMs   int64    `json:"startMs"`
	CaptureMs int64    `json:"captureMs,omitempty"` // Start von ffmpeg: Unix-ms bei 0 s von voice.wav und video.mkv
	EndMs     int64    `json:"endMs"`
	Mic       string   `json:"mic"`
	Monitor   string   `json:"monitor"`
	FFmpeg    string   `json:"ffmpeg"`
	GameArgs  []string `json:"gameArgs"`
	Stand     string   `json:"stand"`
	Encoder   string   `json:"encoder,omitempty"`  // Bildaufnahme (QSV oder libx264-Fallback)
	Crash     bool     `json:"crash,omitempty"`    // Spiel endete mit Fehler, ohne dass Stopp gedrückt wurde
	GameExit  string   `json:"gameExit,omitempty"` // Fehlertext des Spielendes bei Absturz
	Error     string   `json:"error,omitempty"`    // Grund, wenn das Spiel nicht startete
}

// File ist eine Datei oder ein Ordner einer Session mit Größe in Bytes (Ordner: Summe).
type File struct {
	Name string `json:"name"`
	Size int64  `json:"size"`
}

// Session ist eine Zeile der Liste.
type Session struct {
	ID         string `json:"id"`
	Date       string `json:"date"`
	StartMs    int64  `json:"startMs"`
	DurationMs int64  `json:"durationMs"`
	Markers    int    `json:"markers"`
	Size       int64  `json:"size"`
	Stage      string `json:"stage"`
	Running    bool   `json:"running"`
	HasVideo   bool   `json:"hasVideo"`
	HasVoice   bool   `json:"hasVoice"`
	Files      []File `json:"files"`
}

// Plan sagt vor der Bestätigung, was eine Aktion löschen würde.
type Plan struct {
	Action string `json:"action"`
	ID     string `json:"id"`
	Files  []File `json:"files"`
	Total  int64  `json:"total"`
}

// Store ist die Ablage `playtests/` im Repo.
// Active nennt die Session der laufenden Aufnahme dieses Prozesses (leer: keine). Ist es gesetzt, gilt ein
// `laeuft` in session.json ohne diese Kennung als Überbleibsel eines Absturzes: unvollständig und löschbar.
type Store struct {
	Root   string
	Active func() string
}

// List liest alle Sessions, neueste zuerst; fehlt der Ordner, ist die Liste leer. Namen außerhalb des Schemas
// werden übergangen.
func (s *Store) List() ([]Session, error) {
	ents, err := os.ReadDir(s.Root)
	if errors.Is(err, os.ErrNotExist) {
		return []Session{}, nil
	}
	if err != nil {
		return nil, err
	}
	out := []Session{}
	for _, e := range ents {
		if e.IsDir() && idRe.MatchString(e.Name()) {
			out = append(out, s.read(e.Name()))
		}
	}
	sort.Slice(out, func(i, j int) bool { return out[i].ID > out[j].ID })
	return out, nil
}

// Get liest eine Session.
func (s *Store) Get(id string) (Session, error) {
	if !idRe.MatchString(id) {
		return Session{}, ErrBadID
	}
	if st, err := os.Stat(filepath.Join(s.Root, id)); err != nil || !st.IsDir() {
		return Session{}, ErrNotFound
	}
	return s.read(id), nil
}

// read baut die Zeile einer vorhandenen Session; kaputtes oder fehlendes session.json ergibt „unvollständig“.
func (s *Store) read(id string) Session {
	dir := filepath.Join(s.Root, id)
	ses := Session{ID: id, Date: id[:10], Files: []File{}}
	ents, _ := os.ReadDir(dir)
	has := map[string]bool{}
	for _, e := range ents {
		f := File{Name: e.Name(), Size: sizeOf(filepath.Join(dir, e.Name()))}
		ses.Files = append(ses.Files, f)
		ses.Size += f.Size
		has[e.Name()] = true
	}
	ses.HasVideo, ses.HasVoice = has[fileVideo], has[fileVoice]
	ses.Markers = countLines(filepath.Join(dir, fileMarkers))
	var m Meta
	raw, err := os.ReadFile(filepath.Join(dir, fileMeta))
	if err != nil || json.Unmarshal(raw, &m) != nil {
		ses.Stage = StageIncomplete
		if s.Active != nil && s.Active() == id { // Ordner angelegt, session.json folgt gleich
			ses.Stage, ses.Running = StageRunning, true
		}
		return ses
	}
	ses.StartMs = m.StartMs
	stale := s.Active != nil && s.Active() != id
	if m.EndMs > m.StartMs {
		ses.DurationMs = m.EndMs - m.StartMs
	}
	switch {
	case m.Stand == standRunning && m.EndMs == 0 && stale:
		ses.Stage = StageIncomplete
	case m.Stand == standRunning && m.EndMs == 0:
		ses.Stage, ses.Running = StageRunning, true
	case has[fileFindings]:
		ses.Stage = StageEvaluated
	case has[fileTranscript]:
		ses.Stage = StageTranscribed
	default:
		ses.Stage = StageRecorded
	}
	return ses
}

// sizeOf ist die Größe einer Datei oder die Summe eines Ordners.
func sizeOf(p string) int64 {
	var n int64
	_ = filepath.WalkDir(p, func(_ string, d os.DirEntry, err error) error {
		if err == nil && !d.IsDir() {
			if info, e := d.Info(); e == nil {
				n += info.Size()
			}
		}
		return nil
	})
	return n
}

// countLines zählt die nichtleeren Zeilen einer Datei.
func countLines(p string) int {
	raw, err := os.ReadFile(p)
	if err != nil {
		return 0
	}
	n := 0
	for _, l := range strings.Split(string(raw), "\n") {
		if strings.TrimSpace(l) != "" {
			n++
		}
	}
	return n
}

// Plan nennt die Dateien, die die Aktion löschen würde, ohne etwas anzufassen. id bleibt bei „ausgewertet“ leer.
func (s *Store) Plan(action, id string) (Plan, error) {
	p := Plan{Action: action, ID: id, Files: []File{}}
	switch action {
	case ActionVideo, ActionSession:
		ses, err := s.Get(id)
		if err != nil {
			return p, err
		}
		if ses.Running {
			return p, ErrRunning
		}
		if action == ActionSession {
			p.Files = append(p.Files, File{Name: id + "/", Size: ses.Size})
			p.Total = ses.Size
			return p, nil
		}
		p.add(ses, fileVideo)
	case ActionEvaluated:
		list, err := s.List()
		if err != nil {
			return p, err
		}
		p.ID = ""
		for _, ses := range list {
			if ses.Stage == StageEvaluated {
				p.add(ses, fileVideo)
			}
		}
	default:
		return p, ErrBadAction
	}
	return p, nil
}

// add nimmt die Datei in den Plan, wenn die Session sie hat; bei der Sammelaktion mit Session-Ordner im Namen.
func (p *Plan) add(ses Session, name string) {
	for _, f := range ses.Files {
		if f.Name == name {
			if p.Action == ActionEvaluated {
				f.Name = ses.ID + "/" + f.Name
			}
			p.Files = append(p.Files, f)
			p.Total += f.Size
		}
	}
}

// Delete führt die Aktion aus, aber nur mit confirm; ohne bleibt alles stehen (ErrNotConfirmed). Das Ergebnis ist
// der ausgeführte Plan.
func (s *Store) Delete(action, id string, confirm bool) (Plan, error) {
	p, err := s.Plan(action, id)
	if err != nil {
		return p, err
	}
	if !confirm {
		return p, ErrNotConfirmed
	}
	if action == ActionSession {
		return p, os.RemoveAll(filepath.Join(s.Root, id))
	}
	for _, f := range p.Files {
		name := f.Name
		if action == ActionVideo {
			name = id + "/" + name
		}
		if err := os.Remove(filepath.Join(s.Root, filepath.FromSlash(name))); err != nil && !errors.Is(err, os.ErrNotExist) {
			return p, err
		}
	}
	return p, nil
}

// MediaPath ist der Pfad einer abspielbaren Datei; nur voice.wav und video.mkv sind erlaubt.
func (s *Store) MediaPath(id, name string) (string, error) {
	if !idRe.MatchString(id) {
		return "", ErrBadID
	}
	if name != fileVoice && name != fileVideo {
		return "", ErrNotFound
	}
	p := filepath.Join(s.Root, id, name)
	if st, err := os.Stat(p); err != nil || st.IsDir() {
		return "", ErrNotFound
	}
	return p, nil
}
