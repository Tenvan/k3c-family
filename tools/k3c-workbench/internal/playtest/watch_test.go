package playtest

import (
	"encoding/json"
	"strings"
	"testing"
	"time"
)

func TestCloneLiefertNieNilListen(t *testing.T) {
	b, err := json.Marshal(Stats{}.clone())
	if err != nil {
		t.Fatal(err)
	}
	if strings.Contains(string(b), "null") {
		t.Errorf("leere Stats enthalten null: %s", b)
	}
}

func TestAdvanceWarntErstNachDemAnlauf(t *testing.T) {
	var s Stats
	var w watchState
	for i := 0; i < warmup; i++ {
		s.advance(&w, 0, 0, true, "", 0)
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
		s.advance(&w, voice, video, true, "", 0)
	}
	if s.Warn != "" || s.VoiceRate[len(s.VoiceRate)-1] != 32000 || s.VideoRate[len(s.VideoRate)-1] != 90000 {
		t.Fatalf("laufend: %+v", s)
	}
	for i := 0; i < idleVoice; i++ { // Ton steht, Bild läuft
		video += 90000
		s.advance(&w, voice, video, true, "", 0)
	}
	if !strings.Contains(s.Warn, "Ton kommt nicht mehr an") {
		t.Fatalf("Ton-Warnung erwartet: %q", s.Warn)
	}
	for i := 0; i < idleVideo; i++ { // jetzt steht auch das Bild
		s.advance(&w, voice, video, true, "", 0)
	}
	if !strings.Contains(s.Warn, "Ton und Bild") {
		t.Fatalf("beide erwartet: %q", s.Warn)
	}
	voice += 100 // Ton kommt wieder
	video += 100
	s.advance(&w, voice, video, true, "", 0)
	if s.Warn != "" {
		t.Fatalf("Warnung bleibt: %q", s.Warn)
	}
}

func TestAdvanceFfmpegTot(t *testing.T) {
	var s Stats
	var w watchState
	s.advance(&w, 100, 100, false, "Error while opening encoder", 0)
	if !strings.Contains(s.Warn, "ffmpeg ist beendet") || !strings.Contains(s.Warn, "Error while opening encoder") {
		t.Fatalf("%q", s.Warn)
	}
}

func TestAdvanceHaeltHoechstensStatSamples(t *testing.T) {
	var s Stats
	var w watchState
	for i := 0; i < statSamples+30; i++ {
		s.advance(&w, int64(i+1)*10, int64(i+1)*10, true, "", 0)
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
	g.ffp.exit <- nil // ffmpeg stirbt mitten in der Aufnahme
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

func TestAdvanceMikroSessionWarntNieWegenBild(t *testing.T) {
	s := Stats{NoVideo: true}
	var w watchState
	voice := int64(0)
	for i := 0; i < 12; i++ {
		voice += 32000
		s.advance(&w, voice, 0, true, "", 0)
	}
	if s.Warn != "" {
		t.Errorf("Warnung ohne Bild: %q", s.Warn)
	}
	for i := 0; i < idleVoice; i++ {
		s.advance(&w, voice, 0, true, "", 0)
	}
	if !strings.Contains(s.Warn, "Ton kommt nicht mehr an") {
		t.Errorf("Ton steht: %q", s.Warn)
	}
}
