package playtest

import (
	"fmt"
	"os"
	"path/filepath"
	"time"
)

// Stats sind die Messwerte der laufenden Aufnahme für die beiden Diagramme: wie viele Bytes je Sekunde in
// voice.wav und video.mkv ankommen. Steht eine Datei still, steht dort die Aufnahme.
type Stats struct {
	VoiceBytes int64   `json:"voiceBytes"`
	VideoBytes int64   `json:"videoBytes"`
	VoiceRate  []int64 `json:"voiceRate"` // Bytes/s, älteste zuerst, höchstens statSamples
	VideoRate  []int64 `json:"videoRate"`
	Warn       string  `json:"warn"` // leer, solange alles ankommt
}

const (
	statSamples = 90 // so viele Sekunden zeigen die Diagramme
	warmup      = 4  // so viele Takte keine Warnung (Mikro und Encoder starten)
	idleVoice   = 4  // so viele Takte ohne Zuwachs gelten als „Ton steht“
	idleVideo   = 5  // … als „Bild steht“ (Matroska schreibt in Blöcken von höchstens 1 s, siehe FFmpegArgs)
)

// watchState hält, was der Wächter zwischen den Takten braucht.
type watchState struct {
	n                 int
	idleVoice, idleVd int
}

func appendRate(rates []int64, v int64) []int64 {
	rates = append(rates, v)
	if len(rates) > statSamples {
		rates = rates[len(rates)-statSamples:]
	}
	return rates
}

// advance verarbeitet einen Takt: Größen von voice.wav und video.mkv, lebt ffmpeg noch, und was es zuletzt sagte.
func (s *Stats) advance(w *watchState, voice, video int64, alive bool, tail string) {
	w.n++
	dv, dw := voice-s.VoiceBytes, video-s.VideoBytes
	if dv < 0 {
		dv = 0
	}
	if dw < 0 {
		dw = 0
	}
	s.VoiceBytes, s.VideoBytes = voice, video
	s.VoiceRate, s.VideoRate = appendRate(s.VoiceRate, dv), appendRate(s.VideoRate, dw)
	w.idleVoice = idleCount(w.idleVoice, dv)
	w.idleVd = idleCount(w.idleVd, dw)
	switch {
	case !alive:
		s.Warn = "ffmpeg ist beendet – es wird nichts mehr aufgezeichnet"
		if tail != "" {
			s.Warn += " (" + tail + ")"
		}
	case w.n <= warmup:
		s.Warn = ""
	case w.idleVoice >= idleVoice && w.idleVd >= idleVideo:
		s.Warn = fmt.Sprintf("Ton und Bild kommen nicht mehr an (seit %d s)", w.idleVd)
	case w.idleVoice >= idleVoice:
		s.Warn = fmt.Sprintf("Ton kommt nicht mehr an (seit %d s) – Mikro prüfen", w.idleVoice)
	case w.idleVd >= idleVideo:
		s.Warn = fmt.Sprintf("Bild kommt nicht mehr an (seit %d s)", w.idleVd)
	default:
		s.Warn = ""
	}
}

func idleCount(n int, delta int64) int {
	if delta > 0 {
		return 0
	}
	return n + 1
}

func (s Stats) clone() Stats {
	s.VoiceRate = append([]int64(nil), s.VoiceRate...)
	s.VideoRate = append([]int64(nil), s.VideoRate...)
	return s
}

func fileSize(p string) int64 {
	st, err := os.Stat(p)
	if err != nil {
		return 0
	}
	return st.Size()
}

// watch misst jede Sekunde, was in voice.wav und video.mkv ankommt, und merkt, wenn ffmpeg stirbt; die Warnung
// steht im Status und einmal im Log. Endet mit der Aufnahme (done).
func (r *Recorder) watch(dir string, rec *waited, done <-chan struct{}) {
	interval := r.Interval
	if interval <= 0 {
		interval = time.Second
	}
	tick := time.NewTicker(interval)
	defer tick.Stop()
	var w watchState
	warned := ""
	for {
		select {
		case <-done:
			return
		case <-tick.C:
		}
		voice, video := fileSize(filepath.Join(dir, fileVoice)), fileSize(filepath.Join(dir, fileVideo))
		r.mu.Lock()
		r.stats.advance(&w, voice, video, !rec.exited(), rec.Tail())
		warn := r.stats.Warn
		r.mu.Unlock()
		if warn != "" && warn != warned {
			r.log("warn", "⏳ Aufnahme: "+warn)
		}
		warned = warn
	}
}
