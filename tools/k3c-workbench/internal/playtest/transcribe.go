package playtest

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
)

// Auswertung (B-1223): aus voice.wav macht der ffmpeg-Filter `whisper` ein Transkript (transcript.json), aus video.mkv
// kommt je Marker ein Bild (frames/marker-<nr>.jpg). Nur auf Knopfdruck, nie während der Aufnahme. Prozesse entstehen
// über Spawn und Out, damit Tests Attrappen einsetzen (nie ein echtes ffmpeg oder Modell).

// Modell und Ablage.
const (
	ModelFile  = "ggml-large-v3-turbo-q5_0.bin"
	ModelURL   = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/" + ModelFile
	ModelEnv   = "K3C_WHISPER_MODEL" // Pfad der Modelldatei, sonst models/whisper/<ModelFile> im Repo
	dirFrames  = "frames"
	fileRawTmp = "transcript.part"
	minFFmpeg  = 8
	// leadMs: so lange läuft ffmpeg vor session.json › startMs (Settle), wenn captureMs fehlt.
	leadMs = 2000
)

// Fehler der Auswertung; die Oberfläche zeigt den Text.
var (
	ErrNoVoice   = errors.New("voice.wav fehlt, nichts zu transkribieren")
	ErrBusyTrans = errors.New("Auswertung läuft schon")
	ErrNoJob     = errors.New("keine Auswertung aktiv")
)

// ModelPath ist die Modelldatei: K3C_WHISPER_MODEL, sonst models/whisper/ im Repo (gitignored).
func ModelPath(root string) string {
	if p := strings.TrimSpace(os.Getenv(ModelEnv)); p != "" {
		return p
	}
	return filepath.Join(root, "models", "whisper", ModelFile)
}

// Capability sagt, ob die Auswertung möglich ist, und sonst warum nicht (mit Hinweis, was zu tun ist).
type Capability struct {
	Ready   bool   `json:"ready"`
	FFmpeg  string `json:"ffmpeg"` // Version, leer wenn ffmpeg fehlt
	Whisper bool   `json:"whisper"`
	Model   string `json:"model"` // Pfad der Modelldatei
	ModelOK bool   `json:"modelOk"`
	Message string `json:"message"`
}

var versionRe = regexp.MustCompile(`ffmpeg version\s+n?(\d+)\.?(\d*)[.\w-]*`)

// ParseVersion liest die Hauptversion aus `ffmpeg -version` (0, wenn nicht erkennbar; Git-Builds ohne Zahl).
func ParseVersion(out string) (string, int) {
	m := versionRe.FindStringSubmatch(out)
	if m == nil {
		return "", 0
	}
	n, _ := strconv.Atoi(m[1])
	v := strings.Fields(strings.TrimPrefix(m[0], "ffmpeg version"))[0]
	return v, n
}

// HasWhisper prüft die Ausgabe von `ffmpeg -filters` auf den Audio-Filter whisper.
func HasWhisper(filters string) bool {
	for _, l := range strings.Split(filters, "\n") {
		f := strings.Fields(l)
		if len(f) >= 2 && f[1] == "whisper" {
			return true
		}
	}
	return false
}

// Segment ist ein Satz des Transkripts in Unix-ms der Systemuhr.
type Segment struct {
	StartMs int64  `json:"startMs"`
	EndMs   int64  `json:"endMs"`
	Text    string `json:"text"`
}

// Transcript ist transcript.json.
type Transcript struct {
	AudioStartMs int64     `json:"audioStartMs"` // Unix-ms von voice.wav bei 0 s
	Language     string    `json:"language"`
	Model        string    `json:"model"`
	Segments     []Segment `json:"segments"`
}

// WhisperArgs ist der Aufruf im Session-Ordner dir (relative Pfade, damit `C:` im Filter nicht gequotet werden muss);
// das Ergebnis des Filters steht roh in `dest` (Zeilen JSON, Zeiten in ms ab Dateianfang).
func WhisperArgs(ffmpeg, dir, model, dest string) []string {
	return []string{ffmpeg, "-hide_banner", "-loglevel", "warning", "-nostats", "-y", "-i", fileVoice, "-vn",
		"-af", "whisper=model=" + filterPath(dir, model) + ":language=de:format=json:destination=" + dest, "-f", "null", "-"}
}

// filterPath macht die Modelldatei relativ zum Arbeitsordner; auf anderem Laufwerk bleibt der Pfad absolut, dann
// wird `:` für den Filter-Parser maskiert.
func filterPath(dir, model string) string {
	if rel, err := filepath.Rel(dir, model); err == nil {
		return filepath.ToSlash(rel)
	}
	return strings.ReplaceAll(filepath.ToSlash(model), ":", `\\:`)
}

// FrameArgs holt ein Bild bei sek Sekunden Videozeit.
func FrameArgs(ffmpeg string, sek float64, out string) []string {
	return []string{ffmpeg, "-hide_banner", "-loglevel", "error", "-y", "-ss", strconv.FormatFloat(sek, 'f', 3, 64),
		"-i", fileVideo, "-frames:v", "1", "-q:v", "3", out}
}

// FrameName ist der Pfad des Bilds zu Marker nr (1-basiert, Zeilennummer in markers.jsonl), relativ zur Session.
func FrameName(nr int) string { return dirFrames + "/marker-" + strconv.Itoa(nr) + ".jpg" }

// CaptureStart ist Unix-ms bei 0 s von voice.wav und video.mkv: der Start von ffmpeg (captureMs), sonst startMs
// abzüglich der Anlaufzeit.
func CaptureStart(m Meta) int64 {
	if m.CaptureMs > 0 {
		return m.CaptureMs
	}
	if m.StartMs > 0 {
		return m.StartMs - leadMs
	}
	return 0
}

// ParseSegments liest die Rohausgabe des Filters (eine JSON-Zeile je Satz, auch Feld oder {"segments":…}) und rechnet
// die Zeiten ab Dateianfang mit audioStart in Unix-ms um; Leeres entfällt.
func ParseSegments(raw []byte, audioStart int64) ([]Segment, error) {
	type rawSeg struct {
		Start int64  `json:"start"`
		End   int64  `json:"end"`
		Text  string `json:"text"`
	}
	var list []rawSeg
	var wrap struct {
		Segments []rawSeg `json:"segments"`
	}
	text := strings.TrimSpace(string(raw))
	switch {
	case strings.HasPrefix(text, "["):
		if err := json.Unmarshal([]byte(text), &list); err != nil {
			return nil, fmt.Errorf("Transkript nicht lesbar: %w", err)
		}
	case strings.HasPrefix(text, `{"segments"`):
		if err := json.Unmarshal([]byte(text), &wrap); err != nil {
			return nil, fmt.Errorf("Transkript nicht lesbar: %w", err)
		}
		list = wrap.Segments
	default:
		for _, l := range strings.Split(text, "\n") {
			if strings.TrimSpace(l) == "" {
				continue
			}
			var s rawSeg
			if err := json.Unmarshal([]byte(l), &s); err != nil {
				return nil, fmt.Errorf("Transkript nicht lesbar: %w", err)
			}
			list = append(list, s)
		}
	}
	out := []Segment{}
	for _, s := range list {
		if t := strings.TrimSpace(s.Text); t != "" {
			out = append(out, Segment{StartMs: audioStart + s.Start, EndMs: audioStart + s.End, Text: t})
		}
	}
	return out, nil
}
