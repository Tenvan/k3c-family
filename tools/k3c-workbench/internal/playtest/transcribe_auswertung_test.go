package playtest

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

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
}

// Zeitleiste der Auswertung: Reihenfolge, Bilder, Texte, Statistik.
func TestAuswertungZeitleiste(t *testing.T) {
	tr, _, _ := trRig(t, true)
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{}); err != nil {
		t.Fatal(err)
	}
	tr.Wait()
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
}

// Marker-Bilder und Texte der Zeitleiste, Bildzugriff über die Store-Funktion.
func TestAuswertungBilderUndText(t *testing.T) {
	tr, _, _ := trRig(t, true)
	if _, err := tr.Start("2026-11-22-1", TranscribeOptions{}); err != nil {
		t.Fatal(err)
	}
	tr.Wait()
	tl, err := tr.Store.Timeline("2026-11-22-1")
	if err != nil || len(tl.Entries) != 4 {
		t.Fatalf("Zeitleiste: %v", err)
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
