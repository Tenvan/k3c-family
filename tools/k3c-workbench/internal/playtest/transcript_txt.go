package playtest

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// fileTranscriptText ist die lesbare Fassung des Transkripts neben transcript.json (Aufnahmezeit mm:ss, ein Satz je Zeile).
const fileTranscriptText = "transcript.txt"

// TranscriptText macht aus dem Transkript Zeilen „[mm:ss] Satz“; die Zeit zählt ab Beginn von voice.wav.
func TranscriptText(tr Transcript) string {
	var b strings.Builder
	for _, s := range tr.Segments {
		sec := (s.StartMs - tr.AudioStartMs) / 1000
		if sec < 0 {
			sec = 0
		}
		text := strings.TrimSpace(s.Text)
		if text == "" {
			continue
		}
		fmt.Fprintf(&b, "[%02d:%02d] %s\n", sec/60, sec%60, text)
	}
	return b.String()
}

// writeTranscriptText speichert transcript.txt atomar (erst .tmp, dann umbenennen).
func writeTranscriptText(dir string, tr Transcript) error {
	tmp := filepath.Join(dir, fileTranscriptText+".tmp")
	if err := os.WriteFile(tmp, []byte(TranscriptText(tr)), 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, filepath.Join(dir, fileTranscriptText))
}
