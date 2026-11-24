package world

import (
	"encoding/json"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
)

// SI7.1 (B-1003): Wetter, Reittier und Einsiedler je Welt über world_set; die Zuordnung steht in den Katalog-Dateien,
// die Reihenfolge und der Rest der Datei bleiben, „null“ entfernt den Eintrag, eine unbekannte ID lehnt ab.
func TestPicksSetzenUndEntfernen(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	if _, err := e.Set(ctx, "hanse", json.RawMessage(`{"weather":"taifun","mount":"camel","hermit":"priestess"}`)); err != nil {
		t.Fatal(err)
	}
	got, err := e.Picks("hanse")
	if err != nil || got["weather"] != "taifun" || got["mount"] != "camel" || got["hermit"] != "priestess" {
		t.Fatalf("Picks: %v %v", got, err)
	}
	weather, _ := os.ReadFile(filepath.Join(e.Root, "data", "weather.json"))
	if !strings.Contains(string(weather), `"sengoku": "taifun"`) || !strings.Contains(string(weather), `"hanse": "taifun"`) {
		t.Errorf("weather.json: Reihenfolge oder Eintrag falsch:\n%s", weather)
	}
	if _, err := e.Set(ctx, "hanse", json.RawMessage(`{"mount":null}`)); err != nil {
		t.Fatal(err)
	}
	mounts, _ := os.ReadFile(filepath.Join(e.Root, "data", "mounts.json"))
	if strings.Contains(string(mounts), `"hanse"`) || !strings.Contains(string(mounts), `"sengoku": "sengokuHorse"`) {
		t.Errorf("mounts.json nach keins: %s", mounts)
	}
}

// Eine unbekannte ID lehnt ab, die Katalog-Datei bleibt unverändert.
func TestPickUnbekannteIDLehntAb(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	before, _ := os.ReadFile(filepath.Join(e.Root, "data", "hermits.json"))
	if _, err := e.Set(ctx, "hanse", json.RawMessage(`{"hermit":"gibtEsNicht"}`)); err == nil || !strings.Contains(err.Error(), "gibtEsNicht") {
		t.Fatalf("unbekannte ID muss ablehnen: %v", err)
	}
	after, _ := os.ReadFile(filepath.Join(e.Root, "data", "hermits.json"))
	if string(before) != string(after) {
		t.Error("hermits.json darf sich bei Ablehnung nicht ändern")
	}
}

// SI7.1: Die Sicht für die Oberfläche liefert Werte und die gültigen IDs je Art; eine ungültige Auswahl kommt als Fehler.
func TestPicksViewLiefertOptionenUndLehntAb(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	view := e.PicksView("sengoku")
	if view.Error != "" || view.Values["weather"] != "taifun" || !slices.Contains(view.Options["mount"], "camel") {
		t.Fatalf("Sicht: %+v", view)
	}
	if got := e.SetPickView("hanse", "mount", "gibtEsNicht"); got.Error == "" {
		t.Error("unbekannte ID muss in der Antwort als error stehen")
	}
	if got := e.SetPickView("hanse", "mount", "camel"); got.Error != "" {
		t.Errorf("gültige ID: %s", got.Error)
	}
}

// Ein Patch mit einer gültigen und einer ungültigen Art schreibt nichts: weder weather.json noch mounts.json ändern sich.
func TestPickPatchMitUngueltigerArtSchreibtNichts(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	weatherBefore, _ := os.ReadFile(filepath.Join(e.Root, "data", "weather.json"))
	mountsBefore, _ := os.ReadFile(filepath.Join(e.Root, "data", "mounts.json"))
	if _, err := e.Set(ctx, "hanse", json.RawMessage(`{"weather":"taifun","hermit":"gibtEsNicht"}`)); err == nil {
		t.Fatal("ungültige ID im Patch muss ablehnen")
	}
	weatherAfter, _ := os.ReadFile(filepath.Join(e.Root, "data", "weather.json"))
	mountsAfter, _ := os.ReadFile(filepath.Join(e.Root, "data", "mounts.json"))
	if string(weatherBefore) != string(weatherAfter) || string(mountsBefore) != string(mountsAfter) {
		t.Error("ein abgelehnter Patch darf keine Katalog-Datei ändern")
	}
}
