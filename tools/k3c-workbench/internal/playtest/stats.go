package playtest

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// EvalStats ist die Statistik der letzten Auswertung einer Session (in transcript.json › stats): wie lange sie gedauert hat,
// im Verhältnis zur Länge der Aufnahme.
type EvalStats struct {
	TotalMs      int64   `json:"totalMs"`      // Transkript und Bilder
	TranscribeMs int64   `json:"transcribeMs"` // nur whisper
	FramesMs     int64   `json:"framesMs"`     // Marker-Bilder
	AudioMs      int64   `json:"audioMs"`      // Länge der Aufnahme
	Segments     int     `json:"segments"`
	Words        int     `json:"words"`
	Frames       int     `json:"frames"`
	SecPerMin    float64 `json:"secPerMin"` // Auswertungs-Sekunden je Aufnahme-Minute
	Factor       float64 `json:"factor"`    // Auswertung geteilt durch Aufnahme (< 1: schneller als Echtzeit)
}

// MakeStats rechnet die Kennzahlen; ohne Aufnahmelänge bleiben Verhältnis und Faktor 0.
func MakeStats(transcribe, frames time.Duration, audioMs int64, tr Transcript, nFrames int) EvalStats {
	st := EvalStats{TotalMs: (transcribe + frames).Milliseconds(), TranscribeMs: transcribe.Milliseconds(), FramesMs: frames.Milliseconds(),
		AudioMs: audioMs, Segments: len(tr.Segments), Frames: nFrames}
	for _, seg := range tr.Segments {
		st.Words += len(strings.Fields(seg.Text))
	}
	if audioMs > 0 {
		st.Factor = float64(st.TotalMs) / float64(audioMs)
		st.SecPerMin = st.Factor * 60
	}
	return st
}

// saveStats trägt die Statistik in transcript.json ein (t0 Start, t1 Ende des Transkripts, t2 Ende der Bilder).
func saveStats(dir string, t0, t1, t2 time.Time, audioMs int64) error {
	p := filepath.Join(dir, fileTranscript)
	raw, err := os.ReadFile(p)
	if err != nil {
		return err
	}
	var tr Transcript
	if err := json.Unmarshal(raw, &tr); err != nil {
		return err
	}
	n := 0
	if ents, err := os.ReadDir(filepath.Join(dir, dirFrames)); err == nil {
		n = len(ents)
	}
	st := MakeStats(t1.Sub(t0), t2.Sub(t1), audioMs, tr, n)
	tr.Stats = &st
	out, _ := json.MarshalIndent(tr, "", "  ")
	tmp := p + ".tmp"
	if err := os.WriteFile(tmp, out, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, p)
}
