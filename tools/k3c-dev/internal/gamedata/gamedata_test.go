package gamedata

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// fixtureRoot baut einen Repo-Aufbau im Temp-Ordner aus den flachen Beispielen in testdata/. Flach, weil die
// .gitignore im Repo reports/ und saves/ überall ignoriert.
func fixtureRoot(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	copies := map[string]string{
		"report-neu.json":     "reports/gamepad-2026-09-30T19-14-02-123Z.json",
		"report-alt.json":     "reports/gamepad-2026-09-29T08-00-05-000Z.json",
		"report-kaputt.json":  "reports/kaputt.json",
		"save-autosave.json":  "saves/autosave.json",
		"save-sicherung.json": "saves/autosave-2026-09-29T10-00-00-000Z.json",
		"save-kaputt.json":    "saves/kaputt.json",
		"forest.json":         "data/biomes/forest.json",
	}
	for from, to := range copies {
		data, err := os.ReadFile(filepath.Join("testdata", from))
		if err != nil {
			t.Fatal(err)
		}
		dst := filepath.Join(root, filepath.FromSlash(to))
		if err := os.MkdirAll(filepath.Dir(dst), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(dst, data, 0o644); err != nil {
			t.Fatal(err)
		}
	}
	return root
}

func local(iso string) string {
	t, _ := time.Parse(time.RFC3339, iso)
	return t.Local().Format("2006-01-02 15:04")
}

func TestReportsListNeuesteZuerst(t *testing.T) {
	text, err := ReportsList(fixtureRoot(t))
	lines := strings.Split(text, "\n")
	want := []string{
		"gamepad-2026-09-30T19-14-02-123Z.json · " + local("2026-09-30T19:14:02.123Z") +
			" · Windows NT 10.0; Win64; x64; Xbox; Xbox One · 2 Controller · 58 FPS bei 4000 Sprites",
		"gamepad-2026-09-29T08-00-05-000Z.json · " + local("2026-09-29T08:00:05.000Z") + " · Linux; Android 14 · 0 Controller",
		"kaputt.json · nicht lesbar",
	}
	if err != nil || strings.Join(lines, "\n") != strings.Join(want, "\n") {
		t.Errorf("Liste:\n%s\nerwartet:\n%s (%v)", text, strings.Join(want, "\n"), err)
	}
	if text, err := ReportsList(t.TempDir()); err != nil || text != "keine Berichte" {
		t.Errorf("ohne Ordner: %q, %v", text, err)
	}
}

func TestReportRead(t *testing.T) {
	root := fixtureRoot(t)
	text, err := ReportRead(root, "gamepad-2026-09-30T19-14-02-123Z.json")
	for _, part := range []string{
		"Controller 0: Xbox Wireless Controller (standard) · Tasten A X Y View Menu Xbox",
		"Controller 1: Xbox Wireless Controller (standard) · Tasten keine",
		"FPS min/Ø: 100 Sprites 59/60 · 4000 Sprites 41/58",
		"Vollbild: button → ok · auto → Fehler: Permissions check failed",
		"Gleichzeitig max. 2 Controller · Zurück-Navigationen 1 · Tasten-Ereignisse 3",
	} {
		if err != nil || !strings.Contains(text, part) {
			t.Errorf("fehlt %q in:\n%s (%v)", part, text, err)
		}
	}
	if _, err := ReportRead(root, "kaputt.json"); err == nil || !strings.Contains(err.Error(), "nicht lesbar") {
		t.Errorf("kaputt: %v", err)
	}
}

func TestReportReadLehntPfadeAb(t *testing.T) {
	root := fixtureRoot(t)
	if err := os.WriteFile(filepath.Join(root, "geheim.json"), []byte("{}"), 0o644); err != nil {
		t.Fatal(err)
	}
	for _, name := range []string{"../geheim.json", `..\geheim.json`, "sub/x.json", `C:\x.json`, "/etc/x.json",
		".versteckt.json", "gamepad.txt", "a..json", ""} {
		if _, err := ReportRead(root, name); err == nil {
			t.Errorf("%q nicht abgelehnt", name)
		}
	}
}

func TestSavesList(t *testing.T) {
	root := fixtureRoot(t)
	text, err := SavesList(root, SavesDir(root, ""))
	want := []string{
		"autosave-2026-09-29T10-00-00-000Z · Stufe 0/0 · Tag 1 · 1 Spieler · " + local("2026-09-29T10:00:00.000Z"),
		// time 4000 s, Zyklus (10 + 1 + 5) min = 960 s → Tag 5
		"autosave · Stufe 2/3 · Tag 5 · 2 Spieler · " + local("2026-09-30T18:02:00.000Z"),
		"kaputt · nicht lesbar",
	}
	if err != nil || text != strings.Join(want, "\n") {
		t.Errorf("Liste:\n%s\nerwartet:\n%s (%v)", text, strings.Join(want, "\n"), err)
	}
	if err := os.Remove(filepath.Join(root, "data", "biomes", "forest.json")); err != nil {
		t.Fatal(err)
	}
	text, _ = SavesList(root, SavesDir(root, ""))
	if !strings.HasPrefix(text, "Tag unbekannt: ") || !strings.Contains(text, "autosave · Stufe 2/3 · Tag ? ·") {
		t.Errorf("ohne forest.json: %q", text)
	}
}

func TestSavesDirUndLeererOrdner(t *testing.T) {
	root := t.TempDir()
	abs := filepath.Join(t.TempDir(), "woanders")
	cases := map[string]string{"": filepath.Join(root, "saves"), "spiel/stand": filepath.Join(root, "spiel", "stand"), abs: abs}
	for env, want := range cases {
		if got := SavesDir(root, env); got != want {
			t.Errorf("SavesDir(%q) = %q, erwartet %q", env, got, want)
		}
	}
	if text, err := SavesList(root, SavesDir(root, "")); err != nil || text != "keine Spielstände" {
		t.Errorf("ohne Ordner: %q, %v", text, err)
	}
}
