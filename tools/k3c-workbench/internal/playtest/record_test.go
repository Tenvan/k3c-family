package playtest

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"
)

// Attrappen statt ffmpeg und Spiel: geprüft werden Argumente und Ablauf, nie eine echte Aufnahme.

type fakeProc struct {
	args   []string
	exit   chan error
	quits  int
	kills  int
	tail   string
	ignore bool // ignoriert Quit (hängt), nur Kill beendet
	mu     sync.Mutex
}

func newFake(args []string) *fakeProc { return &fakeProc{args: args, exit: make(chan error, 1)} }

func (f *fakeProc) Wait() error  { return <-f.exit }
func (f *fakeProc) Tail() string { return f.tail }
func (f *fakeProc) Quit() error {
	f.mu.Lock()
	defer f.mu.Unlock()
	f.quits++
	if !f.ignore {
		f.exit <- nil
	}
	return nil
}
func (f *fakeProc) Kill() error {
	f.mu.Lock()
	defer f.mu.Unlock()
	f.kills++
	f.exit <- errors.New("beendet")
	return nil
}

type rig struct {
	mu         sync.Mutex // schützt procs, ffp, gamep (Start läuft in Tests in einer Goroutine)
	rec        *Recorder
	procs      []*fakeProc
	failQS     bool // QSV-ffmpeg endet sofort
	failAll    bool
	failGame   bool
	ffp, gamep *fakeProc // die ffmpeg- und die Spiel-Attrappe der letzten Aufnahme
}

func newRig(t *testing.T) *rig {
	g := &rig{}
	root := t.TempDir()
	g.rec = &Recorder{Store: &Store{Root: filepath.Join(root, "playtests")},
		LookPath: func(string) (string, error) { return "C:/ff/ffmpeg.exe", nil },
		Game:     func() (string, string, error) { return "C:/g/k3c-game.exe", "C:/g", nil },
		Now:      func() time.Time { return time.Date(2026, 11, 22, 18, 0, 0, 0, time.UTC) },
		Settle:   30 * time.Millisecond, QuitWait: 100 * time.Millisecond, Ready: 40 * time.Millisecond}
	g.rec.Spawn = func(args []string, dir string) (Proc, error) {
		g.mu.Lock()
		defer g.mu.Unlock()
		p := newFake(args)
		isFF := strings.HasSuffix(args[0], "ffmpeg.exe")
		if !isFF && g.failGame {
			return nil, errors.New("kaputt")
		}
		if isFF && (g.failAll || (g.failQS && strings.Contains(strings.Join(args, " "), "h264_qsv"))) {
			p.tail = "Error: device busy"
			p.exit <- errors.New("exit 1")
		}
		g.procs = append(g.procs, p)
		if isFF {
			g.ffp = p
		} else {
			g.gamep = p
		}
		return p, nil
	}
	return g
}

func (g *rig) readMeta(t *testing.T, id string) Meta {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(g.rec.Store.Root, id, "session.json"))
	if err != nil {
		t.Fatal(err)
	}
	var m Meta
	if err := json.Unmarshal(raw, &m); err != nil {
		t.Fatal(err)
	}
	return m
}

func TestFFmpegArgsQSV(t *testing.T) {
	a := strings.Join(FFmpegArgs("ffmpeg", "D", "Mikro (USB)", 1, EncoderQSV), " ")
	for _, want := range []string{"-f dshow -i audio=Mikro (USB)", "ddagrab=output_idx=1:framerate=30", "-ac 1 -ar 16000 -c:a pcm_s16le -flush_packets 1 " + filepath.Join("D", "voice.wav"),
		"scale=1920:1080", "-c:v h264_qsv", "-r 30", filepath.Join("D", "video.mkv")} {
		if !strings.Contains(a, want) {
			t.Errorf("fehlt %q in %s", want, a)
		}
	}
	if strings.Contains(a, "libx264") {
		t.Error("QSV-Aufruf nennt libx264")
	}
}

func TestFFmpegArgsFallback(t *testing.T) {
	a := strings.Join(FFmpegArgs("ffmpeg", "D", "M", 0, EncoderX264), " ")
	for _, want := range []string{"libx264 -preset ultrafast", "scale=1280:720", "output_idx=0"} {
		if !strings.Contains(a, want) {
			t.Errorf("fehlt %q in %s", want, a)
		}
	}
	if strings.Contains(a, "qsv") {
		t.Error("Fallback nennt qsv")
	}
}

func TestStartStoppMitSpielende(t *testing.T) {
	g := newRig(t)
	st, err := g.rec.Start(Options{Mic: "Mikro", Monitor: 1})
	if err != nil || !st.Running || st.ID != "2026-11-22-1" || st.Encoder != EncoderQSV {
		t.Fatalf("st=%+v err=%v", st, err)
	}
	if len(g.procs) != 2 || !strings.Contains(strings.Join(g.gamep.args, " "), "--playtest "+filepath.Join(g.rec.Store.Root, "2026-11-22-1")) {
		t.Fatalf("Spielstart: %+v", g.procs)
	}
	if _, err := g.rec.Start(Options{Mic: "Mikro"}); !errors.Is(err, ErrBusy) {
		t.Errorf("zweiter Start: %v", err)
	}
	if m := g.readMeta(t, st.ID); m.Stand != standRunning || m.StartMs == 0 || m.EndMs != 0 || m.Encoder != EncoderQSV || m.Mic != "Mikro" {
		t.Errorf("Meta laufend: %+v", m)
	}
	if got, _ := g.rec.Store.Get(st.ID); !got.Running {
		t.Error("Liste zeigt keine laufende Aufnahme")
	}
}

// Spielende beendet die Aufnahme sauber; danach ist eine neue Aufnahme möglich.
func TestSpielendeBeendetAufnahme(t *testing.T) {
	g := newRig(t)
	st, _ := g.rec.Start(Options{Mic: "Mikro", Monitor: 1})
	g.gamep.exit <- nil // Spielende
	g.rec.Wait()
	if g.ffp.quits != 1 || g.ffp.kills != 0 {
		t.Errorf("ffmpeg: quits=%d kills=%d", g.ffp.quits, g.ffp.kills)
	}
	m := g.readMeta(t, st.ID)
	if m.Stand != StageRecorded || m.EndMs == 0 || m.Crash {
		t.Errorf("Meta Ende: %+v", m)
	}
	if got, _ := g.rec.Store.Get(st.ID); got.Stage != StageRecorded || got.Running {
		t.Errorf("Stand = %+v", got)
	}
	if g.rec.Status().Running {
		t.Error("Status noch laufend")
	}
	if _, err := g.rec.Start(Options{Mic: "Mikro"}); err != nil { // nächste Aufnahme möglich, Nummer zählt hoch
		t.Errorf("neuer Start: %v", err)
	}
}

func TestAbsturzDesSpiels(t *testing.T) {
	g := newRig(t)
	st, _ := g.rec.Start(Options{Mic: "M"})
	g.gamep.exit <- errors.New("exit status 0xc0000005")
	g.rec.Wait()
	m := g.readMeta(t, st.ID)
	if m.Stand != StageRecorded || !m.Crash || !strings.Contains(m.GameExit, "c0000005") || m.EndMs == 0 || g.ffp.quits != 1 {
		t.Errorf("Meta: %+v quits=%d", m, g.ffp.quits)
	}
}

func TestStoppBeendetSpielOhneAbsturz(t *testing.T) {
	g := newRig(t)
	st, _ := g.rec.Start(Options{Mic: "M"})
	if err := g.rec.Stop(); err != nil {
		t.Fatal(err)
	}
	g.rec.Wait()
	if m := g.readMeta(t, st.ID); m.Crash || m.Stand != StageRecorded || g.gamep.kills != 1 {
		t.Errorf("Meta: %+v kills=%d", m, g.gamep.kills)
	}
	if err := g.rec.Stop(); !errors.Is(err, ErrNotRunning) {
		t.Errorf("Stop ohne Aufnahme: %v", err)
	}
}

func TestHaengendesFFmpegWirdNachFristBeendet(t *testing.T) {
	g := newRig(t)
	g.rec.Spawn = func(args []string, dir string) (Proc, error) {
		g.mu.Lock()
		defer g.mu.Unlock()
		p := newFake(args)
		isFF := strings.HasSuffix(args[0], "ffmpeg.exe")
		p.ignore = isFF
		g.procs = append(g.procs, p)
		if isFF {
			g.ffp = p
		} else {
			g.gamep = p
		}
		return p, nil
	}
	if _, err := g.rec.Start(Options{Mic: "M"}); err != nil {
		t.Fatal(err)
	}
	g.gamep.exit <- nil
	g.rec.Wait()
	if g.ffp.quits != 1 || g.ffp.kills != 1 {
		t.Errorf("quits=%d kills=%d", g.ffp.quits, g.ffp.kills)
	}
}

func TestQSVFaelltAufX264Zurueck(t *testing.T) {
	g := newRig(t)
	g.failQS = true
	st, err := g.rec.Start(Options{Mic: "M"})
	if err != nil || st.Encoder != EncoderX264 {
		t.Fatalf("st=%+v err=%v", st, err)
	}
	if m := g.readMeta(t, st.ID); m.Encoder != EncoderX264 || !strings.Contains(m.FFmpeg, "libx264") {
		t.Errorf("Meta: %+v", m)
	}
}

func TestFFmpegScheitertBeendetDasSpiel(t *testing.T) {
	g := newRig(t)
	g.failAll = true
	if _, err := g.rec.Start(Options{Mic: "M"}); err == nil || !strings.Contains(err.Error(), "device busy") {
		t.Fatalf("Fehler = %v", err)
	}
	if g.gamep == nil || g.gamep.kills != 1 {
		t.Errorf("Spiel nicht beendet: %+v", g.gamep)
	}
	if list, _ := g.rec.Store.List(); len(list) != 0 {
		t.Errorf("Ordner blieb: %v", list)
	}
	if g.rec.Status().Running {
		t.Error("Status laufend")
	}
	g.rec.Wait() // darf nicht hängen
}

func TestVoraussetzungen(t *testing.T) {
	g := newRig(t)
	g.rec.LookPath = func(string) (string, error) { return "", errors.New("nix") }
	if _, err := g.rec.Start(Options{Mic: "M"}); !errors.Is(err, ErrNoFFmpeg) {
		t.Errorf("ffmpeg fehlt: %v", err)
	}
	g = newRig(t)
	if _, err := g.rec.Start(Options{}); !errors.Is(err, ErrNoMic) {
		t.Errorf("Mikro: %v", err)
	}
	g.rec.Game = func() (string, string, error) { return "", "", ErrNoGame }
	if _, err := g.rec.Start(Options{Mic: "M"}); !errors.Is(err, ErrNoGame) {
		t.Errorf("Spiel: %v", err)
	}
	if len(g.procs) != 0 {
		t.Error("Prozess gestartet")
	}
}

func TestParseMics(t *testing.T) {
	neu := `[dshow @ 000] "Mikrofon (Realtek)" (audio)
[dshow @ 000]   Alternative name "@device_cm_{33D9}\wave_{1}"
[dshow @ 000] "Integrated Camera" (video)
[dshow @ 000] "Raummikro (USB)" (audio)`
	if got := ParseMics(neu); strings.Join(got, "|") != "Mikrofon (Realtek)|Raummikro (USB)" {
		t.Errorf("neu: %v", got)
	}
	alt := `[dshow @ 000] DirectShow video devices
[dshow @ 000]  "Camera"
[dshow @ 000] DirectShow audio devices
[dshow @ 000]  "Mikro alt"
[dshow @ 000]     Alternative name "@device"`
	if got := ParseMics(alt); strings.Join(got, "|") != "Mikro alt" {
		t.Errorf("alt: %v", got)
	}
}

func TestListDevicesOhneFFmpeg(t *testing.T) {
	d := ListDevices(func(string) (string, error) { return "", errors.New("nix") })
	if d.FFmpeg || len(d.Mics) != 0 || !strings.Contains(d.Message, "ffmpeg") {
		t.Errorf("d = %+v", d)
	}
}

func TestStartPruefGeraeteVorDemStart(t *testing.T) {
	g := newRig(t)
	boom := errors.New("Mikrofon „M“ lässt sich nicht öffnen")
	g.rec.Check = func(string, Options) error { return boom }
	if _, err := g.rec.Start(Options{Mic: "M"}); !errors.Is(err, boom) {
		t.Fatalf("Gerätefehler erwartet, bekam %v", err)
	}
	if g.rec.Status().Running {
		t.Error("ohne nutzbares Gerät darf nichts laufen")
	}
	if entries, _ := os.ReadDir(g.rec.Store.Root); len(entries) != 0 {
		t.Errorf("kein Session-Ordner erwartet: %v", entries)
	}
}

func TestStatusAntwortetWaehrendDerGeraetepruefung(t *testing.T) {
	g := newRig(t)
	release := make(chan struct{})
	entered := make(chan struct{})
	g.rec.Check = func(string, Options) error {
		close(entered)
		<-release
		return nil
	}
	done := make(chan struct{})
	go func() { _, _ = g.rec.Start(Options{Mic: "M"}); close(done) }()
	<-entered
	got := make(chan struct{})
	go func() { _ = g.rec.Status(); close(got) }()
	select {
	case <-got:
	case <-time.After(2 * time.Second):
		t.Fatal("Status() blockiert während der Geräteprüfung")
	}
	close(release)
	<-done
}
