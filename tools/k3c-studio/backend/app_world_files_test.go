package backend

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// Welten anlegen (leer, als Kopie), wählen und entfernen (WZ18.6, B-488/AC-05); nur in Kopien (t.TempDir()).

func stand(t *testing.T, a *App) string {
	t.Helper()
	raw, err := a.readIslands()
	if err != nil {
		t.Fatal(err)
	}
	return planStand(raw)
}

// validWorld prüft die Datei der Welt id mit der Prüfung des Ladens.
func validWorld(t *testing.T, a *App, id string) string {
	t.Helper()
	raw := readFile(t, a.worldFile(id))
	if _, err := sim.ParseWorld([]byte(raw), id); err != nil {
		t.Fatal(err)
	}
	return raw
}

func TestWeltLeerAnlegenWaehlenEntfernen(t *testing.T) {
	a, standardPath := worldApp(t)
	if v := a.PlanWorlds(); v.Active != "standard" || len(v.Worlds) != 1 {
		t.Fatalf("zu Beginn: %+v", v)
	}
	v := a.PlanWorldAdd("Süd Küste")
	if v.Error != "" || v.Active != "sued-kueste" || len(v.Worlds) != 2 || v.Worlds[1].Name != "Süd Küste" {
		t.Fatalf("nach Anlegen: %+v", v)
	}
	raw := validWorld(t, a, "sued-kueste")
	if !strings.Contains(raw, `"id": "sued-kueste"`) || strings.Contains(raw, "clearing") {
		t.Errorf("neue Welt: %s", raw)
	}
	// Der Entwurf gilt für die gewählte Welt: eine Insel mehr ändert nur ihre Datei.
	d, at := draftOf(t, a)
	d.Islands = append(d.Islands, IslandDraft{ID: "island2", Name: "Insel 2", Depths: []int{0, 4}, Scaling: d.Islands[0].Scaling})
	saveDraft(t, a, d, at)
	if got := len(a.PlanLoad().Islands); got != 2 {
		t.Errorf("neue Welt hat %d Inseln, erwartet 2", got)
	}
	validWorld(t, a, "sued-kueste")
	if readFile(t, standardPath) != worldFixture {
		t.Error("die Standard-Welt wurde mitgeändert")
	}
	if v := a.PlanWorldSelect("standard"); v.Active != "standard" || len(a.PlanLoad().Islands) != 2 {
		t.Errorf("Wechsel zur Standard-Welt: %+v", v)
	}
	removeAndCheck(t, a)
}

func removeAndCheck(t *testing.T, a *App) {
	t.Helper()
	if v := a.PlanWorldRemove("sued-kueste"); v.Error != "" || len(v.Worlds) != 1 || v.Active != "standard" {
		t.Errorf("nach Entfernen: %+v", v)
	}
	if _, err := os.Stat(a.worldFile("sued-kueste")); err == nil {
		t.Error("Datei blieb liegen")
	}
}

func TestWeltAlsKopieAnlegen(t *testing.T) {
	a, standardPath := worldApp(t)
	v := a.PlanWorldCopy("Nordmeer")
	if v.Error != "" || v.Active != "nordmeer" || len(v.Worlds) != 2 {
		t.Fatalf("nach Kopie: %+v", v)
	}
	raw := validWorld(t, a, "nordmeer")
	want := strings.Replace(strings.Replace(worldFixture, `"id": "standard"`, `"id": "nordmeer"`, 1), `"name": "Standard"`, `"name": "Nordmeer"`, 1)
	if raw != want {
		t.Errorf("Kopie\n%s\nerwartet\n%s", raw, want)
	}
	if readFile(t, standardPath) != worldFixture {
		t.Error("die Standard-Welt wurde geändert")
	}
	if v := a.PlanWorldCopy("Nordmeer"); v.Error == "" || v.Active != "nordmeer" {
		t.Errorf("doppelte Welt angenommen: %+v", v)
	}
}

// Planung und Chunk-Anzahl einer Stufe lassen sich in jeder Welt bearbeiten (Editor aus WZ8/WZ17).
func TestPlanungUndAnzahlInDerKopie(t *testing.T) {
	a, _ := worldApp(t)
	a.PlanWorldCopy("Nordmeer")
	chunks := []level.PlannedChunk{{Slot: 4, Kind: "clearing"}}
	if res := a.PlanSave("island2", 0, chunks, 40, stand(t, a)); res.Error != "" || res.Hint != planRegHint {
		t.Fatalf("speichern: %+v", res)
	}
	v := a.PlanLoad()
	if isl := v.Islands[1]; len(isl.Planned["0"]) != 1 || isl.ChunkCounts["0"] != 40 {
		t.Errorf("Insel 2: %+v", isl)
	}
	raw := validWorld(t, a, "nordmeer")
	if !strings.Contains(raw, `"speed": 1.15 }, "plannedChunks": {"0": [{"slot":4,"kind":"clearing"}]}, "chunkCounts": {"0": 40} }`) {
		t.Errorf("Datei: %s", raw)
	}
}

func TestWeltenAblehnungen(t *testing.T) {
	a, _ := worldApp(t)
	a.PlanWorldAdd("Süd Küste")
	if v := a.PlanWorldAdd("Süd Küste"); v.Error == "" {
		t.Error("doppelte Welt angenommen")
	}
	for _, bad := range []string{"", "   ", "!!!", "Standard"} {
		if v := a.PlanWorldAdd(bad); v.Error == "" {
			t.Errorf("Name %q angenommen", bad)
		}
		if v := a.PlanWorldCopy(bad); v.Error == "" {
			t.Errorf("Kopie %q angenommen", bad)
		}
	}
	if v := a.PlanWorldSelect("../x"); v.Error == "" {
		t.Error("ungültige ID gewählt")
	}
	if v := a.PlanWorldRemove("standard"); v.Error == "" {
		t.Error("Standard-Welt entfernt")
	}
	if entries, _ := os.ReadDir(filepath.Join(a.root, worldsDir)); len(entries) != 2 {
		t.Errorf("Dateien %v", entries)
	}
}

// Eine ungültige Welt wird nicht kopiert (Speichern nur Gültiges).
func TestUngueltigeWeltWirdNichtKopiert(t *testing.T) {
	a, _ := planApp(t, islandsFixture)
	if v := a.PlanWorldCopy("Kaputt"); v.Error == "" || v.Active != "standard" {
		t.Errorf("Kopie einer ungültigen Welt: %+v", v)
	}
	if _, err := os.Stat(a.worldFile("kaputt")); err == nil {
		t.Error("Datei angelegt")
	}
}
