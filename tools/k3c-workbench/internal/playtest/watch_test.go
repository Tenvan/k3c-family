package playtest

import (
	"strings"
	"testing"
	"time"
)

func TestAdvanceWarntErstNachDemAnlauf(t *testing.T) {
	var s Stats
	var w watchState
	for i := 0; i < warmup; i++ {
		s.advance(&w, 0, 0, true, "")
		if s.Warn != "" {
			t.Fatalf("Warnung im Anlauf (Takt %d): %q", i+1, s.Warn)
		}
	}
}

func TestAdvanceMisstRateUndWarntBeiStillstand(t *testing.T) {
	var s Stats
	var w watchState
	voice, video := int64(0), int64(0)
	for i := 0; i < 8; i++ { // alles kommt an
		voice += 32000
		video += 90000
		s.advance(&w, voice, video, true, "")
	}
	if s.Warn != "" || s.VoiceRate[len(s.VoiceRate)-1] != 32000 || s.VideoRate[len(s.VideoRate)-1] != 90000 {
		t.Fatalf("laufend: %+v", s)
	}
	for i := 0; i < idleVoice; i++ { // Ton steht, Bild läuft
		video += 90000
		s.advance(&w, voice, video, true, "")
	}
	if !strings.Contains(s.Warn, "Ton kommt nicht mehr an") {
		t.Fatalf("Ton-Warnung erwartet: %q", s.Warn)
	}
	for i := 0; i < idleVideo; i++ { // jetzt steht auch das Bild
		s.advance(&w, voice, video, true, "")
	}
	if !strings.Contains(s.Warn, "Ton und Bild") {
		t.Fatalf("beide erwartet: %q", s.Warn)
	}
	voice += 100 // Ton kommt wieder
	video += 100
	s.advance(&w, voice, video, true, "")
	if s.Warn != "" {
		t.Fatalf("Warnung bleibt: %q", s.Warn)
	}
}

func TestAdvanceFfmpegTot(t *testing.T) {
	var s Stats
	var w watchState
	s.advance(&w, 100, 100, false, "Error while opening encoder")
	if !strings.Contains(s.Warn, "ffmpeg ist beendet") || !strings.Contains(s.Warn, "Error while opening encoder") {
		t.Fatalf("%q", s.Warn)
	}
}

func TestAdvanceHaeltHoechstensStatSamples(t *testing.T) {
	var s Stats
	var w watchState
	for i := 0; i < statSamples+30; i++ {
		s.advance(&w, int64(i+1)*10, int64(i+1)*10, true, "")
	}
	if len(s.VoiceRate) != statSamples || len(s.VideoRate) != statSamples {
		t.Fatalf("%d/%d", len(s.VoiceRate), len(s.VideoRate))
	}
}

func TestWaechterMeldetToteAufnahmeImStatus(t *testing.T) {
	g := newRig(t)
	g.rec.Interval = 5 * time.Millisecond
	if _, err := g.rec.Start(Options{Mic: "M"}); err != nil {
		t.Fatal(err)
	}
	ff := g.procs[len(g.procs)-2] // ffmpeg, danach das Spiel
	if !strings.HasSuffix(ff.args[0], "ffmpeg.exe") {
		ff = g.procs[0]
	}
	ff.exit <- nil // ffmpeg stirbt mitten in der Aufnahme
	deadline := time.Now().Add(2 * time.Second)
	for {
		st := g.rec.Status()
		if st.Stats != nil && strings.Contains(st.Stats.Warn, "ffmpeg ist beendet") {
			break
		}
		if time.Now().After(deadline) {
			t.Fatalf("keine Warnung im Status: %+v", st.Stats)
		}
		time.Sleep(5 * time.Millisecond)
	}
	_ = g.rec.Stop()
	g.rec.Wait()
}
