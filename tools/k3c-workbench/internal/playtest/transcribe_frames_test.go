package playtest

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

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
