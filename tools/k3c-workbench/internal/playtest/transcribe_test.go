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

func TestAuswertungTranskriptUndBilder(t *testing.T) {
	tr, j, dir := trRig(t, true)
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{}); err != nil {
		t.Fatal(err)
	}
	tr.Wait()
	if st := tr.Status(); st.Running || st.Error != "" || st.Done != 2 || st.Total != 2 {
		t.Fatalf("Stand: %+v", st)
	}
	if len(j.calls) != 3 {
		t.Fatalf("Aufrufe: %d", len(j.calls))
	}
	if !strings.Contains(strings.Join(j.calls[1], " "), "-ss 5.000") || !strings.Contains(strings.Join(j.calls[2], " "), "-ss 20.500") {
		t.Errorf("Bildzeiten: %v %v", j.calls[1], j.calls[2])
	}
	if _, err := os.Stat(filepath.Join(dir, fileRawTmp)); err == nil {
		t.Error("Rohdatei bleibt liegen")
	}
	m := readMeta(dir)
	if m.Stand != StageTranscribed {
		t.Errorf("Stand in session.json: %q", m.Stand)
	}
	ses, _ := tr.Store.Get("2026-11-22-1")
	if ses.Stage != StageTranscribed {
		t.Errorf("Stufe: %q", ses.Stage)
	}
	tl, err := tr.Store.Timeline("2026-11-22-1")
	if err != nil || !tl.HasTranscript || len(tl.Entries) != 4 {
		t.Fatalf("Zeitleiste: %+v %v", tl, err)
	}
	if tl.Stats == nil || tl.Stats.Segments != 2 || tl.Stats.Frames != 2 || tl.Stats.TotalMs < 0 {
		t.Errorf("Statistik der Auswertung: %+v", tl.Stats)
	}
	want := []struct {
		kind string
		vms  int64
	}{{"sprache", 0}, {"marker", 5000}, {"sprache", 9000}, {"marker", 20500}}
	for i, w := range want {
		if e := tl.Entries[i]; e.Kind != w.kind || e.VideoMs != w.vms {
			t.Errorf("Eintrag %d: %+v, erwartet %+v", i, e, w)
		}
	}
	if tl.Entries[1].Frame != "frames/marker-1.jpg" || tl.Entries[3].Frame != "frames/marker-2.jpg" || tl.Entries[1].Nr != 1 {
		t.Errorf("Bilder: %+v", tl.Entries)
	}
	if !strings.Contains(tl.Entries[3].Text, "Ruckler") || !strings.Contains(tl.Entries[1].Text, "Insel 2 Stufe 3") {
		t.Errorf("Text: %q / %q", tl.Entries[1].Text, tl.Entries[3].Text)
	}
	if _, err := tr.Store.FramePath("2026-11-22-1", 2); err != nil {
		t.Error(err)
	}
	if _, err := tr.Store.FramePath("2026-11-22-1", 9); err == nil {
		t.Error("fehlendes Bild muss ablehnen")
	}
}

func TestAuswertungOhneVideo(t *testing.T) {
	tr, j, dir := trRig(t, false)
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{}); err != nil {
		t.Fatal(err)
	}
	tr.Wait()
	st := tr.Status()
	if st.Error != "" || !strings.Contains(st.Message, "video.mkv fehlt") || len(j.calls) != 1 {
		t.Errorf("Stand %+v, Aufrufe %d", st, len(j.calls))
	}
	if _, err := os.Stat(filepath.Join(dir, fileTranscript)); err != nil {
		t.Error("Transkript muss trotzdem entstehen")
	}
}

func TestAuswertungWhisperFehlerLaesstKeinTranskript(t *testing.T) {
	tr, j, dir := trRig(t, true)
	j.fail = "whisper="
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{}); err != nil {
		t.Fatal(err)
	}
	tr.Wait()
	if st := tr.Status(); !strings.Contains(st.Error, "Fehlerzeile") {
		t.Errorf("Fehler ohne Ausgabe: %+v", st)
	}
	if _, err := os.Stat(filepath.Join(dir, fileTranscript)); err == nil {
		t.Error("transcript.json darf nicht entstehen")
	}
}

func TestAuswertungAbbrechen(t *testing.T) {
	tr, j, dir := trRig(t, true)
	j.block = make(chan struct{})
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{}); err != nil {
		t.Fatal(err)
	}
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{}); !errors.Is(err, ErrBusyTrans) {
		t.Errorf("zweiter Start: %v", err)
	}
	time.Sleep(20 * time.Millisecond)
	if err := tr.Cancel(); err != nil {
		t.Fatal(err)
	}
	tr.Wait()
	if st := tr.Status(); st.Running || st.Error != errCanceled.Error() {
		t.Errorf("Stand: %+v", st)
	}
	for _, n := range []string{fileTranscript, fileRawTmp} {
		if _, err := os.Stat(filepath.Join(dir, n)); err == nil {
			t.Errorf("%s bleibt liegen", n)
		}
	}
	if err := tr.Cancel(); !errors.Is(err, ErrNoJob) {
		t.Errorf("Abbruch ohne Auftrag: %v", err)
	}
}

func TestModellUndSpracheWaehlbarNurAusDerListe(t *testing.T) {
	tr, j, dir := trRig(t, true)
	root := filepath.Dir(filepath.Dir(dir)) // Wurzel der Attrappe
	other := filepath.Join(root, "models", "whisper", "anderes.bin")
	if err := os.WriteFile(other, []byte("zz"), 0o644); err != nil {
		t.Fatal(err)
	}
	t.Setenv(ModelEnv, "")
	tr.Models = func() []ModelInfo { return ListModels(root) }
	for _, bad := range []string{"../etc/passwd", other, "C:/Windows/x.bin", "models/../x.bin", "env"} {
		if _, err := tr.Start("2026-11-22-1", TranscribeOptions{Model: bad}); err == nil || !strings.Contains(err.Error(), "unbekanntes Modell") {
			t.Errorf("Pfad %q muss abgelehnt werden: %v", bad, err)
		}
	}
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{Language: "fr"}); err == nil || !strings.Contains(err.Error(), "Sprache") {
		t.Errorf("Sprache: %v", err)
	}
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{Model: "models/whisper/anderes.bin", Language: "en"}); err != nil {
		t.Fatal(err)
	}
	tr.Wait()
	a := strings.Join(j.calls[0], " ")
	if !strings.Contains(a, "anderes.bin") || !strings.Contains(a, "language=en") {
		t.Errorf("Aufruf: %s", a)
	}
	raw, _ := os.ReadFile(filepath.Join(dir, fileTranscript))
	if !strings.Contains(string(raw), `"language": "en"`) || !strings.Contains(string(raw), "anderes.bin") {
		t.Errorf("transcript.json: %s", raw)
	}
}

func TestEnvModellInListe(t *testing.T) {
	root := t.TempDir()
	f := filepath.Join(t.TempDir(), "x.bin")
	if err := os.WriteFile(f, []byte("x"), 0o644); err != nil {
		t.Fatal(err)
	}
	t.Setenv(ModelEnv, f)
	l := ListModels(root)
	if len(l) != 1 || l[0].ID != "env" || !l[0].Default || l[0].Path != f {
		t.Errorf("%+v", l)
	}
}

func TestStartAblehnungen(t *testing.T) {
	tr, _, dir := trRig(t, true)
	if _, err := tr.Start("../x", TranscribeOptions{}); !errors.Is(err, ErrBadID) {
		t.Errorf("falsche Kennung: %v", err)
	}
	if err := os.Remove(filepath.Join(dir, fileVoice)); err != nil {
		t.Fatal(err)
	}
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{}); !errors.Is(err, ErrNoVoice) {
		t.Errorf("ohne voice.wav: %v", err)
	}
}

func TestModelPathSuchreihenfolge(t *testing.T) {
	root := t.TempDir()
	t.Setenv(ModelEnv, "")
	def := filepath.Join(root, "models", ModelFile)
	if ModelPath(root) != def {
		t.Errorf("ohne Datei der erste Kandidat: %s", ModelPath(root))
	}
	sub := filepath.Join(root, "models", "whisper")
	_ = os.MkdirAll(sub, 0o755)
	_ = os.WriteFile(filepath.Join(sub, "b.bin"), []byte("x"), 0o644)
	_ = os.WriteFile(filepath.Join(sub, "a.bin"), []byte("x"), 0o644)
	if ModelPath(root) != filepath.Join(sub, "a.bin") {
		t.Errorf("models/whisper/*.bin: %s", ModelPath(root))
	}
	_ = os.WriteFile(def, []byte("xx"), 0o644)
	if ModelPath(root) != def {
		t.Errorf("models/<Datei> vor whisper/: %s", ModelPath(root))
	}
	t.Setenv(ModelEnv, "D:/m/x.bin")
	if ModelPath(root) != "D:/m/x.bin" {
		t.Errorf("Umgebung zuerst: %s", ModelPath(root))
	}
}

func TestCheckMeldetModellgroesse(t *testing.T) {
	tr, _, _ := trRig(t, true)
	big := filepath.Join(t.TempDir(), "m.bin")
	f, _ := os.Create(big)
	_ = f.Truncate(3 << 20)
	_ = f.Close()
	tr.Model = func() string { return big }
	if c := tr.Check(); !c.Ready || c.ModelMB != 3 {
		t.Errorf("%+v", c)
	}
}

// Ein Marker-Bild lässt sich aus- und wieder einblenden (B-1240): Datei und Marker bleiben, nur die Markierung ändert sich.
func TestSetFrameSkip(t *testing.T) {
	tr, _, dir := trRig(t, true)
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{}); err != nil {
		t.Fatal(err)
	}
	tr.Wait()
	s := tr.Store
	for i := 0; i < 2; i++ { // doppelt ausblenden ändert nichts
		if err := s.SetFrameSkip("2026-11-22-1", 1, true); err != nil {
			t.Fatal(err)
		}
	}
	if _, err := os.Stat(filepath.Join(dir, "frames", "marker-1.jpg")); err != nil {
		t.Fatal("Bilddatei muss bleiben")
	}
	tl, err := s.Timeline("2026-11-22-1")
	if err != nil || !tl.Entries[1].Skip || tl.Entries[1].Frame == "" || tl.Entries[3].Skip {
		t.Fatalf("nur Marker 1 ausgeblendet: %+v %v", tl.Entries, err)
	}
	if got := readMeta(dir).SkipFrames; len(got) != 1 || got[0] != 1 {
		t.Errorf("session.json › skipFrames: %v", got)
	}
	if err := s.SetFrameSkip("2026-11-22-1", 1, false); err != nil {
		t.Fatal(err)
	}
	if tl, _ = s.Timeline("2026-11-22-1"); tl.Entries[1].Skip {
		t.Error("Marker 1 bleibt ausgeblendet")
	}
	if err := s.SetFrameSkip("2026-11-22-1", 0, true); !errors.Is(err, ErrBadID) {
		t.Errorf("Nummer 0: %v", err)
	}
	if err := s.SetFrameSkip("../x", 1, true); !errors.Is(err, ErrBadID) {
		t.Errorf("Pfad in der Kennung: %v", err)
	}
}

// Scheitert nur ein Marker-Bild (Marker nach dem Ende des Videos), bleibt das Transkript und die Auswertung endet mit Hinweis.
func TestAuswertungBildFehltKippsNichtDieAuswertung(t *testing.T) {
	tr, j, dir := trRig(t, true)
	j.fail = "marker-2"
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{}); err != nil {
		t.Fatal(err)
	}
	tr.Wait()
	st := tr.Status()
	if st.Error != "" || !strings.Contains(st.Message, "Marker 2") {
		t.Fatalf("Stand: %+v", st)
	}
	if _, err := os.Stat(filepath.Join(dir, fileTranscript)); err != nil {
		t.Error("Transkript fehlt")
	}
	if _, err := os.Stat(filepath.Join(dir, "frames", "marker-1.jpg")); err != nil {
		t.Error("Bild 1 fehlt")
	}
}
