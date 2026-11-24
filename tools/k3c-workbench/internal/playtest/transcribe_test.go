package playtest

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"
)

// Attrappe: schreibt, was ein echtes ffmpeg schriebe, und merkt sich die Aufrufe.
type trJob struct {
	mu    sync.Mutex
	calls [][]string
	fail  string // Teil eines Aufrufs, bei dem der Prozess scheitert
	block chan struct{}
}

type trProc struct {
	j    *trJob
	args []string
	dir  string
	kill chan struct{}
}

func (p *trProc) Quit() error { return nil }
func (p *trProc) Kill() error {
	select {
	case <-p.kill:
	default:
		close(p.kill)
	}
	return nil
}
func (p *trProc) Tail() string { return "Fehlerzeile" }
func (p *trProc) Wait() error {
	if p.j.block != nil {
		select {
		case <-p.j.block:
		case <-p.kill:
			return errors.New("killed")
		}
	}
	joined := strings.Join(p.args, " ")
	if p.j.fail != "" && strings.Contains(joined, p.j.fail) {
		return errors.New("exit 1")
	}
	if strings.Contains(joined, "whisper=") {
		raw := `{"start":0,"end":2500,"text":" Hallo Welt."}` + "\n" + `{"start":4000,"end":6000,"text":"  "}` + "\n" +
			`{"start":9000,"end":11000,"text":"Der Turm fehlt."}` + "\n"
		return os.WriteFile(filepath.Join(p.dir, fileRawTmp), []byte(raw), 0o644)
	}
	return os.WriteFile(filepath.Join(p.dir, filepath.FromSlash(p.args[len(p.args)-1])), []byte("jpg"), 0o644)
}

func (j *trJob) spawn(args []string, dir string) (Proc, error) {
	j.mu.Lock()
	j.calls = append(j.calls, args)
	j.mu.Unlock()
	return &trProc{j: j, args: args, dir: dir, kill: make(chan struct{})}, nil
}

const goodVersion = "ffmpeg version 9.0.2-full_build-www.gyan.dev Copyright (c) 2000-2025\n"
const goodFilters = " ... whisper           A->A       Transcribe audio using Whisper.\n ... volume  A->A  Change volume.\n"

// rig baut Ablage, Modell und Transcriber mit Attrappen; die Session hat Startzeit 1_000_000 ms und zwei Marker.
func trRig(t *testing.T, video bool) (*Transcriber, *trJob, string) {
	t.Helper()
	root := t.TempDir()
	dir := filepath.Join(root, "playtests", "2026-11-22-1")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	writeMeta(dir, Meta{CaptureMs: 1_000_000, StartMs: 1_002_000, EndMs: 1_100_000, Stand: StageRecorded})
	files := map[string]string{
		fileVoice: "wav",
		fileMarkers: `{"ts":1005000,"kind":"pad","player":1,"island":2,"stage":3}` + "\n" +
			`{"ts":1020500,"kind":"ruckler","player":0,"frame_ms":48.5}` + "\n",
	}
	if video {
		files[fileVideo] = "mkv"
	}
	for n, c := range files {
		if err := os.WriteFile(filepath.Join(dir, n), []byte(c), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	model := filepath.Join(root, "models", "whisper", ModelFile)
	_ = os.MkdirAll(filepath.Dir(model), 0o755)
	_ = os.WriteFile(model, []byte("m"), 0o644)
	j := &trJob{}
	tr := &Transcriber{Store: &Store{Root: filepath.Join(root, "playtests")}, Spawn: j.spawn,
		Out: func(a []string) (string, error) {
			if a[len(a)-1] == "-filters" {
				return goodFilters, nil
			}
			return goodVersion, nil
		},
		LookPath: func(string) (string, error) { return "ffmpeg", nil }, Model: func() string { return model }, Now: time.Now}
	return tr, j, dir
}

func TestWhisperArgsRelativ(t *testing.T) {
	dir := filepath.FromSlash("C:/repo/playtests/x")
	model := filepath.FromSlash("C:/repo/models/whisper/") + ModelFile
	a := strings.Join(WhisperArgs("ffmpeg", dir, model, "de", "transcript.part"), " ")
	for _, want := range []string{"-i voice.wav", "-af whisper=model=../../models/whisper/" + ModelFile, "language=de", "queue=30", "format=json", "destination=transcript.part", "-f null -"} {
		if !strings.Contains(a, want) {
			t.Errorf("Aufruf ohne %q: %s", want, a)
		}
	}
	if strings.Contains(a, `C:`) {
		t.Errorf("Laufwerk im Filter: %s", a)
	}
}

func TestFrameArgs(t *testing.T) {
	a := strings.Join(FrameArgs("ffmpeg", 5.5, "frames/marker-1.jpg"), " ")
	for _, want := range []string{"-ss 5.500", "-i video.mkv", "-frames:v 1", "frames/marker-1.jpg"} {
		if !strings.Contains(a, want) {
			t.Errorf("Aufruf ohne %q: %s", want, a)
		}
	}
	if strings.Index(a, "-ss") > strings.Index(a, "-i video.mkv") {
		t.Error("-ss muss vor -i stehen (schnelles Suchen)")
	}
}

func TestParseSegmentsZeitUmrechnung(t *testing.T) {
	raw := []byte(`{"start":0,"end":2500,"text":" Hallo."}` + "\n" + `{"start":100,"end":200,"text":""}` + "\n" + `{"start":9000,"end":11000,"text":"Zwei"}`)
	segs, err := ParseSegments(raw, 1_000_000)
	if err != nil || len(segs) != 2 {
		t.Fatalf("segs=%v err=%v", segs, err)
	}
	if segs[0].StartMs != 1_000_000 || segs[0].EndMs != 1_002_500 || segs[0].Text != "Hallo." || segs[1].StartMs != 1_009_000 {
		t.Errorf("Umrechnung falsch: %+v", segs)
	}
	if _, err := ParseSegments([]byte("kein json"), 0); err == nil {
		t.Error("kaputte Ausgabe muss Fehler sein")
	}
	arr, err := ParseSegments([]byte(`[{"start":5,"end":6,"text":"a"}]`), 10)
	if err != nil || len(arr) != 1 || arr[0].StartMs != 15 {
		t.Errorf("Feldform: %v %v", arr, err)
	}
}

func TestCaptureStart(t *testing.T) {
	if CaptureStart(Meta{CaptureMs: 5, StartMs: 9000}) != 5 || CaptureStart(Meta{StartMs: 9000}) != 7000 || CaptureStart(Meta{}) != 0 {
		t.Error("CaptureStart")
	}
}

func TestCheckFehlerfaelle(t *testing.T) {
	tr, _, _ := trRig(t, true)
	if c := tr.Check(); !c.Ready || c.FFmpeg != "9.0.2-full_build-www.gyan.dev" {
		t.Fatalf("bereit erwartet: %+v", c)
	}
	tr.LookPath = func(string) (string, error) { return "", errors.New("nein") }
	if c := tr.Check(); c.Ready || !strings.Contains(c.Message, "ffmpeg nicht gefunden") {
		t.Errorf("ohne ffmpeg: %+v", c)
	}
	tr.LookPath = func(string) (string, error) { return "ffmpeg", nil }
	tr.Out = func(a []string) (string, error) { return "ffmpeg version 6.1.1 x\n", nil }
	if c := tr.Check(); c.Ready || !strings.Contains(c.Message, "zu alt") {
		t.Errorf("alte Version: %+v", c)
	}
	tr.Out = func(a []string) (string, error) {
		if a[len(a)-1] == "-filters" {
			return " ... volume A->A x\n", nil
		}
		return goodVersion, nil
	}
	if c := tr.Check(); c.Ready || c.Whisper || !strings.Contains(c.Message, "whisper-Filter") {
		t.Errorf("ohne whisper: %+v", c)
	}
}

// Fehlendes Modell: Meldung mit Download-Hinweis, Start lehnt ab.
func TestCheckModellFehlt(t *testing.T) {
	tr2, _, _ := trRig(t, true)
	tr2.Model = func() string { return filepath.Join(t.TempDir(), "fehlt.bin") }
	c := tr2.Check()
	if c.Ready || c.ModelOK || !strings.Contains(c.Message, ModelURL) || !strings.Contains(c.Message, ModelSHA) || !strings.Contains(c.Message, ModelEnv) {
		t.Errorf("ohne Modell: %+v", c)
	}
	if _, err := tr2.Start("2026-11-22-1", TranscribeOptions{}); err == nil || !strings.Contains(err.Error(), "Modell fehlt") {
		t.Errorf("Start ohne Modell muss ablehnen: %v", err)
	}
}
