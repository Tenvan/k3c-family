package playtest

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// put legt eine Datei samt Ordnern an.
func put(t *testing.T, root, rel, content string) {
	t.Helper()
	p := filepath.Join(root, filepath.FromSlash(rel))
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(p, []byte(content), 0o644); err != nil {
		t.Fatal(err)
	}
}

func exists(root, rel string) bool {
	_, err := os.Stat(filepath.Join(root, filepath.FromSlash(rel)))
	return err == nil
}

const meta = `{"startMs":1000,"endMs":61000,"mic":"Mikro","monitor":"1","ffmpeg":"ffmpeg -i x","gameArgs":["--a"],"stand":"aufgenommen"}`

const markers = "{\"t\":1}\n{\"t\":2}\n\n"

// fixture: aufgenommen, ausgewertet, kaputt und laufend; dazu ein fremder Ordner.
func fixture(t *testing.T) *Store {
	root := t.TempDir()
	put(t, root, "2026-11-20-1/session.json", meta)
	put(t, root, "2026-11-20-1/video.mkv", "vvvv")
	put(t, root, "2026-11-20-1/voice.wav", "aa")
	put(t, root, "2026-11-20-1/markers.jsonl", markers)
	put(t, root, "2026-11-21-1/session.json", meta)
	put(t, root, "2026-11-21-1/video.mkv", "vvvvvv")
	put(t, root, "2026-11-21-1/transcript.json", "{}")
	put(t, root, "2026-11-21-1/findings.json", "[]")
	put(t, root, "2026-11-21-1/frames/f1.jpg", "xxx")
	put(t, root, "2026-11-22-1/session.json", "{kaputt")
	put(t, root, "2026-11-22-2/session.json", `{"startMs":3000,"endMs":0,"stand":"laeuft"}`)
	put(t, root, "2026-11-22-2/video.mkv", "v")
	put(t, root, "kein-session-name/x", "x")
	return &Store{Root: root}
}

func TestListLiestSessionsNeuesteZuerst(t *testing.T) {
	got, err := fixture(t).List()
	if err != nil {
		t.Fatal(err)
	}
	var ids []string
	by := map[string]Session{}
	for _, s := range got {
		ids = append(ids, s.ID)
		by[s.ID] = s
	}
	if strings.Join(ids, ",") != "2026-11-22-2,2026-11-21-1,2026-11-20-1,2026-11-22-1" {
		t.Fatalf("Reihenfolge/Auswahl: %v", ids)
	}
	a := by["2026-11-20-1"]
	wantSize := int64(4 + 2 + len(markers) + len(meta))
	if a.Stage != StageRecorded || a.DurationMs != 60000 || a.Markers != 2 || a.Size != wantSize || a.Date != "2026-11-20" {
		t.Errorf("aufgenommen: %+v", a)
	}
	if b := by["2026-11-21-1"]; b.Stage != StageEvaluated || !b.HasVideo || b.Size == 0 {
		t.Errorf("ausgewertet: %+v", b)
	}
	if by["2026-11-22-1"].Stage != StageIncomplete {
		t.Errorf("kaputt: %+v", by["2026-11-22-1"])
	}
	if c := by["2026-11-22-2"]; !c.Running || c.Stage != StageRunning {
		t.Errorf("laufend: %+v", c)
	}
}

// Die Reihenfolge folgt der Startzeit: Kennung „-10“ ist neuer als „-9“, obwohl sie als Text kleiner ist.
func TestListSortiertNachStartzeitAbwaerts(t *testing.T) {
	root := t.TempDir()
	put(t, root, "2026-11-22-9/session.json", `{"startMs":2000,"endMs":3000,"stand":"aufgenommen"}`)
	put(t, root, "2026-11-22-10/session.json", `{"startMs":9000,"endMs":9500,"stand":"aufgenommen"}`)
	put(t, root, "2026-11-22-3/session.json", `{"startMs":1000,"endMs":1500,"stand":"aufgenommen"}`)
	got, err := (&Store{Root: root}).List()
	if err != nil {
		t.Fatal(err)
	}
	var ids []string
	for _, s := range got {
		ids = append(ids, s.ID)
	}
	if strings.Join(ids, ",") != "2026-11-22-10,2026-11-22-9,2026-11-22-3" {
		t.Errorf("Reihenfolge: %v", ids)
	}
}

func TestTranskribiert(t *testing.T) {
	s := fixture(t)
	put(t, s.Root, "2026-11-20-1/transcript.json", "{}")
	got, _ := s.Get("2026-11-20-1")
	if got.Stage != StageTranscribed {
		t.Errorf("Stand = %q", got.Stage)
	}
}

func TestListOhneOrdnerIstLeer(t *testing.T) {
	got, err := (&Store{Root: filepath.Join(t.TempDir(), "fehlt")}).List()
	if err != nil || len(got) != 0 {
		t.Fatalf("got %v, err %v", got, err)
	}
}

func TestPlanNenntDateienUndLoeschtNichts(t *testing.T) {
	s := fixture(t)
	p, err := s.Plan(ActionVideo, "2026-11-20-1")
	if err != nil {
		t.Fatal(err)
	}
	if len(p.Files) != 1 || p.Files[0].Name != "video.mkv" || p.Files[0].Size != 4 || p.Total != 4 {
		t.Errorf("plan = %+v", p)
	}
	if !exists(s.Root, "2026-11-20-1/video.mkv") {
		t.Error("Plan hat gelöscht")
	}
}

func TestVideoLoeschenNurMitBestaetigungUndNurVideo(t *testing.T) {
	s := fixture(t)
	if _, err := s.Delete(ActionVideo, "2026-11-21-1", false); !errors.Is(err, ErrNotConfirmed) {
		t.Fatalf("ohne Bestätigung: %v", err)
	}
	if !exists(s.Root, "2026-11-21-1/video.mkv") {
		t.Fatal("ohne Bestätigung gelöscht")
	}
	p, err := s.Delete(ActionVideo, "2026-11-21-1", true)
	if err != nil || p.Total != 6 {
		t.Fatalf("p=%+v err=%v", p, err)
	}
	for rel, want := range map[string]bool{"2026-11-21-1/video.mkv": false, "2026-11-21-1/transcript.json": true,
		"2026-11-21-1/findings.json": true, "2026-11-21-1/session.json": true, "2026-11-21-1/frames/f1.jpg": true} {
		if exists(s.Root, rel) != want {
			t.Errorf("%s vorhanden = %v, erwartet %v", rel, !want, want)
		}
	}
}

func TestSessionLoeschen(t *testing.T) {
	s := fixture(t)
	if _, err := s.Delete(ActionSession, "2026-11-22-1", false); !errors.Is(err, ErrNotConfirmed) {
		t.Fatalf("ohne Bestätigung: %v", err)
	}
	if !exists(s.Root, "2026-11-22-1/session.json") {
		t.Fatal("ohne Bestätigung gelöscht")
	}
	if _, err := s.Delete(ActionSession, "2026-11-22-1", true); err != nil {
		t.Fatal(err) // kaputte Session bleibt löschbar
	}
	if exists(s.Root, "2026-11-22-1") || !exists(s.Root, "2026-11-20-1/video.mkv") {
		t.Error("falsche Auswahl gelöscht")
	}
}

func TestLaufendeAufnahmeIstGesperrt(t *testing.T) {
	s := fixture(t)
	for _, a := range []string{ActionVideo, ActionSession} {
		if _, err := s.Plan(a, "2026-11-22-2"); !errors.Is(err, ErrRunning) {
			t.Errorf("Plan %s: %v", a, err)
		}
		if _, err := s.Delete(a, "2026-11-22-2", true); !errors.Is(err, ErrRunning) {
			t.Errorf("Delete %s: %v", a, err)
		}
	}
	if !exists(s.Root, "2026-11-22-2/video.mkv") {
		t.Error("laufende Aufnahme angefasst")
	}
}

func TestVideosAusgewerteterSessions(t *testing.T) {
	s := fixture(t)
	put(t, s.Root, "2026-11-23-1/session.json", meta)
	put(t, s.Root, "2026-11-23-1/findings.json", "[]") // ausgewertet, aber ohne Video
	p, err := s.Plan(ActionEvaluated, "")
	if err != nil {
		t.Fatal(err)
	}
	if len(p.Files) != 1 || p.Files[0].Name != "2026-11-21-1/video.mkv" || p.Total != 6 {
		t.Fatalf("plan = %+v", p)
	}
	if _, err := s.Delete(ActionEvaluated, "", false); !errors.Is(err, ErrNotConfirmed) {
		t.Fatalf("ohne Bestätigung: %v", err)
	}
	if !exists(s.Root, "2026-11-21-1/video.mkv") {
		t.Fatal("ohne Bestätigung gelöscht")
	}
	if _, err := s.Delete(ActionEvaluated, "", true); err != nil {
		t.Fatal(err)
	}
	if exists(s.Root, "2026-11-21-1/video.mkv") || !exists(s.Root, "2026-11-20-1/video.mkv") || !exists(s.Root, "2026-11-22-2/video.mkv") {
		t.Error("falsche Videos gelöscht")
	}
}

func TestUngueltigeIdUndAktion(t *testing.T) {
	s := fixture(t)
	for _, id := range []string{"", "..", "../x", "2026-11-20-1/..", `2026-11-20-1\x`, "kein-session-name"} {
		if _, err := s.Delete(ActionSession, id, true); !errors.Is(err, ErrBadID) {
			t.Errorf("id %q: %v", id, err)
		}
	}
	if _, err := s.Plan("alles", "2026-11-20-1"); !errors.Is(err, ErrBadAction) {
		t.Errorf("Aktion: %v", err)
	}
	if _, err := s.Plan(ActionSession, "2030-01-01-1"); !errors.Is(err, ErrNotFound) {
		t.Errorf("unbekannt: %v", err)
	}
}

func TestMediaPfad(t *testing.T) {
	s := fixture(t)
	if p, err := s.MediaPath("2026-11-20-1", "voice.wav"); err != nil || !strings.HasSuffix(filepath.ToSlash(p), "2026-11-20-1/voice.wav") {
		t.Errorf("p=%q err=%v", p, err)
	}
	if _, err := s.MediaPath("2026-11-20-1", "video.mkv"); err != nil {
		t.Errorf("video: %v", err)
	}
	for _, f := range []string{"session.json", "../x", "video.mkv/.."} {
		if _, err := s.MediaPath("2026-11-20-1", f); !errors.Is(err, ErrNotFound) {
			t.Errorf("Datei %q erlaubt: %v", f, err)
		}
	}
	if _, err := s.MediaPath("2026-11-22-2", "voice.wav"); !errors.Is(err, ErrNotFound) {
		t.Errorf("fehlende Datei erlaubt: %v", err)
	}
}

func TestUeberbleibselEinesAbsturzsIstLoeschbar(t *testing.T) {
	s := fixture(t)
	s.Active = func() string { return "" } // keine Aufnahme in diesem Prozess
	got, _ := s.Get("2026-11-22-2")
	if got.Running || got.Stage != StageIncomplete {
		t.Fatalf("laeuft ohne aktive Aufnahme muss unvollständig sein: %+v", got)
	}
	if _, err := s.Delete(ActionSession, "2026-11-22-2", true); err != nil {
		t.Fatalf("Löschen sollte möglich sein: %v", err)
	}
	s = fixture(t)
	s.Active = func() string { return "2026-11-22-2" }
	if got, _ := s.Get("2026-11-22-2"); !got.Running {
		t.Fatalf("die aktive Aufnahme bleibt gesperrt: %+v", got)
	}
}

func TestUngueltigerMonitor(t *testing.T) {
	g := newRig(t)
	if _, err := g.rec.Start(Options{Mic: "Mikro", Monitor: -1}); !errors.Is(err, ErrBadMonitor) {
		t.Fatalf("Monitor -1 muss abgelehnt werden: %v", err)
	}
}
