package planning

import "testing"

// B-838/AC-01: Sessions desselben Sprints laufen nie parallel (ein Sprint-Branch), auch ohne Sperre und mit getrennten
// Erlaubten Dateien. plan_waves setzt die zweite in die Folgewelle, plan_next nennt sie nicht unter „Parallel möglich“
// und sperrt den Sprint, solange eine seiner Sessions in Arbeit ist; ein anderer Sprint bleibt parallel möglich.
func sameSprintRepo(t *testing.T, firstStatus string) string {
	t.Helper()
	root := t.TempDir()
	w := waveWriter(t, root)
	w("backlog/README.md", "# Backlog\n")
	w("sprints/geplant/.keep", "")
	w("projekte/CCC-c.md", nextProject("CCC", "aktiv", "1", "C1", "D1"))
	waveSprint(w, "C1", "CCC", "aktiv", "–", []waveSession{
		{"C1.1", "autonom", firstStatus, "–", "a/**", ""},
		{"C1.2", "autonom", "offen", "–", "b/**", ""},
	})
	waveSprint(w, "D1", "CCC", "aktiv", "–", []waveSession{{"D1.1", "autonom", "offen", "–", "d/**", ""}})
	return root
}

func TestSessionsEinesSprintsNacheinander(t *testing.T) {
	t.Parallel()
	root := sameSprintRepo(t, "offen")
	out := must(t)(Waves(root, WavesFilter{}))
	checkText(t, out, []string{
		"── Welle 1, jetzt (2/4 Plätze) ──\n- C1.1 · Session · Titel C1.1 (DEV, Projekt CCC, Sprint C1)\n- D1.1 · Session",
		"── Welle 2 (1/4 Plätze) ──\n- C1.2 · Session · Titel C1.2 (DEV, Projekt CCC, Sprint C1) — kollidiert mit C1.1: Sprint C1",
	}, nil)
	next := must(t)(Next(root, NextFilter{}))
	checkText(t, next, []string{"Nächste Session: C1.1", "── Parallel möglich ──\n- DEV · D1.1"}, []string{"C1.2 ·"})
}

func TestSprintGesperrtSolangeSessionInArbeit(t *testing.T) {
	t.Parallel()
	out := must(t)(Next(sameSprintRepo(t, "in Arbeit"), NextFilter{}))
	checkText(t, out, []string{"Nächste Session: D1.1"}, []string{"Nächste Session: C1.2", "- DEV · C1.2"})
}
