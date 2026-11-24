package playtest

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// Start-Ablauf (B-1235): Spiel zuerst, Aufnahme erst nach dem ersten Bild, Mikro-Session ohne Spiel.

func TestSpielStartScheitertOhneFFmpeg(t *testing.T) {
	g := newRig(t)
	g.failGame = true
	if _, err := g.rec.Start(Options{Mic: "M"}); err == nil || !strings.Contains(err.Error(), "Spiel nicht gestartet") {
		t.Fatalf("Fehler = %v", err)
	}
	if g.ffp != nil || g.rec.Status().Running {
		t.Errorf("ffmpeg gestartet oder Status laufend: %+v", g.ffp)
	}
	if list, _ := g.rec.Store.List(); len(list) != 0 {
		t.Errorf("Ordner blieb: %v", list)
	}
}

// Die Aufnahme beginnt erst, wenn das Spiel `ready` schreibt; bis dahin meldet Status „startet“ und Stopp beendet das Spiel.
func TestAufnahmeBeginntErstMitDemErstenBild(t *testing.T) {
	g := newRig(t)
	g.rec.Ready = 5 * time.Second
	type res struct {
		st  Status
		err error
	}
	out := make(chan res, 1)
	go func() { st, err := g.rec.Start(Options{Mic: "M"}); out <- res{st, err} }()
	var st Status
	for i := 0; i < 100 && !st.Starting; i++ {
		time.Sleep(10 * time.Millisecond)
		st = g.rec.Status()
	}
	if !st.Starting || !st.Running {
		t.Fatalf("Status beim Start: %+v", st)
	}
	g.mu.Lock()
	n := len(g.procs)
	g.mu.Unlock()
	if n != 1 || g.rec.ActiveID() == "" {
		t.Fatalf("vor dem ersten Bild darf nur das Spiel laufen, Prozesse: %d", n)
	}
	if err := os.WriteFile(filepath.Join(g.rec.Store.Root, g.rec.ActiveID(), fileReady), []byte("1"), 0o644); err != nil {
		t.Fatal(err)
	}
	r := <-out
	if r.err != nil || r.st.Starting || r.st.Encoder != EncoderQSV {
		t.Fatalf("Start: %+v err=%v", r.st, r.err)
	}
	if _, err := os.Stat(filepath.Join(g.rec.Store.Root, r.st.ID, fileReady)); err == nil {
		t.Error("ready-Datei blieb liegen")
	}
	g.gamep.exit <- nil
	g.rec.Wait()
}

func TestStoppWaehrendDasSpielLaedt(t *testing.T) {
	g := newRig(t)
	g.rec.Ready = 5 * time.Second
	errc := make(chan error, 1)
	go func() { _, err := g.rec.Start(Options{Mic: "M"}); errc <- err }()
	for i := 0; i < 100 && g.rec.Status().ID == ""; i++ {
		time.Sleep(10 * time.Millisecond)
	}
	for i := 0; i < 100; i++ { // Stopp ist erst möglich, wenn das Spiel läuft
		if g.rec.Stop() == nil {
			break
		}
		time.Sleep(10 * time.Millisecond)
	}
	if err := <-errc; err == nil {
		t.Error("Start meldet Erfolg trotz Stopp")
	}
	if list, _ := g.rec.Store.List(); len(list) != 0 {
		t.Errorf("Ordner blieb: %v", list)
	}
}

func TestMikroSessionOhneBildUndSpiel(t *testing.T) {
	g := newRig(t)
	g.rec.Game = func() (string, string, error) { return "", "", ErrNoGame } // ohne Spiel-EXE muss sie trotzdem starten
	st, err := g.rec.Start(Options{Mic: "Mikro", Monitor: 99, VoiceOnly: true})
	if err != nil || !st.Running || !st.VoiceOnly || st.Encoder != "" {
		t.Fatalf("st=%+v err=%v", st, err)
	}
	if len(g.procs) != 1 {
		t.Fatalf("nur ffmpeg erwartet, gestartet: %d", len(g.procs))
	}
	a := strings.Join(g.ffp.args, " ")
	if strings.Contains(a, "ddagrab") || strings.Contains(a, "video.mkv") || !strings.Contains(a, "audio=Mikro") || !strings.HasSuffix(a, "voice.wav") {
		t.Errorf("Aufruf: %s", a)
	}
	if s := g.rec.Status(); s.Stats == nil || !s.Stats.NoVideo {
		t.Errorf("Stats: %+v", s.Stats)
	}
	_ = st
}

// Stopp beendet die Mikro-Session sauber: Meta nach Stopp, ffmpeg beendet, kein Absturz.
func TestMikroSessionStopptSauber(t *testing.T) {
	g := newRig(t)
	g.rec.Game = func() (string, string, error) { return "", "", ErrNoGame }
	st, err := g.rec.Start(Options{Mic: "Mikro", VoiceOnly: true})
	if err != nil {
		t.Fatal(err)
	}
	if err := g.rec.Stop(); err != nil {
		t.Fatal(err)
	}
	g.rec.Wait()
	m := g.readMeta(t, st.ID)
	if m.Stand != StageRecorded || m.EndMs == 0 || m.Error != "" || m.Monitor != "" || m.Crash {
		t.Errorf("Meta nach Stop: %+v", m)
	}
	if g.ffp.quits != 1 {
		t.Errorf("ffmpeg nicht sauber beendet: quits=%d", g.ffp.quits)
	}
}
