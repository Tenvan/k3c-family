package world

import (
	"encoding/json"
	"os"
	"slices"
	"strings"
	"testing"
)

// B-954/AC-01: Anlegen, Setzen, Chunks und Prüfen; die Datei bleibt gültig und format-stabil.
func TestCreateSetChunksCheckRoundtrip(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "germania", "Germania", "standard"); err != nil {
		t.Fatal(err)
	}
	if _, err := e.Create(ctx, "germania", "Nochmal", ""); err == nil {
		t.Fatal("vorhandene Welt überschrieben")
	}
	patch := `{"notes":"Römerzeit","combat":{"mode":"groups","groupSize":6},"islands":[
		{"id":"island1","name":"Wald","biomes":["forest","cave","mine","ironhold","crystal"],"depthScaling":{"hp":2}},
		{"id":"island2","name":"Moor"}]}`
	// Insel 1 ist nicht die letzte: ohne Hafen lehnt die Prüfung ab, die Datei bleibt.
	before := readWorld(t, e, "germania")
	if _, err := e.Set(ctx, "germania", json.RawMessage(patch)); err == nil || !strings.Contains(err.Error(), "Hafen") {
		t.Fatalf("Hafen erwartet, Fehler = %v", err)
	}
	if readWorld(t, e, "germania") != before {
		t.Fatal("Datei trotz Fehler geändert")
	}
	patch = strings.Replace(patch, `"depthScaling":{"hp":2}`, `"depthScaling":{"hp":2},"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}`, 1)
	if _, err := e.Set(ctx, "germania", json.RawMessage(patch)); err != nil {
		t.Fatal(err)
	}
	text := readWorld(t, e, "germania")
	for _, want := range []string{`"notes": "Römerzeit"`, `"combat": { "mode": "groups", "groupSize": 6 }`, `"depths": [0, 1, 2, 3, 4]`,
		`"depthScaling": { "hp": 2, "damage": 1.3, "speed": 1.1 }`, `{ "id": "island2", "name": "Moor"`} {
		if !strings.Contains(text, want) {
			t.Fatalf("fehlt %q:\n%s", want, text)
		}
	}
	if strings.Index(text, `"notes"`) > strings.Index(text, `"islands"`) {
		t.Fatalf("notes steht nicht vor islands:\n%s", text)
	}
	// Dieselbe Änderung nochmal: Datei byte-gleich.
	if _, err := e.Set(ctx, "germania", json.RawMessage(patch)); err != nil || readWorld(t, e, "germania") != text {
		t.Fatalf("zweiter Lauf änderte die Datei (%v)", err)
	}
	checkChunks(t, e)
}

func checkChunks(t *testing.T, e Editor) {
	t.Helper()
	three := 16
	if _, err := e.SetChunks(ctx, "germania", "island2", 1, json.RawMessage(`[{"slot":4,"kind":"harbor"}]`), &three); err != nil {
		t.Fatal(err)
	}
	if got := readWorld(t, e, "germania"); !strings.Contains(got, `"plannedChunks": { "1": [ { "slot": 4, "kind": "harbor" } ] }`) || !strings.Contains(got, `"chunkCounts": { "1": 16 }`) {
		t.Fatalf("Chunks fehlen:\n%s", got)
	}
	if _, err := e.SetChunks(ctx, "germania", "island2", 7, json.RawMessage(`[]`), nil); err == nil {
		t.Fatal("fremde Tiefe angenommen")
	}
	zero := 0
	if _, err := e.SetChunks(ctx, "germania", "island2", 1, json.RawMessage(`null`), &zero); err != nil {
		t.Fatal(err)
	}
	if got := readWorld(t, e, "germania"); strings.Contains(got, "chunkCounts") || strings.Contains(got, `"1": [`) {
		t.Fatalf("Chunks nicht entfernt:\n%s", got)
	}
	if w, err := e.Check(ctx, "germania"); err != nil || len(w) != 0 {
		t.Fatalf("Check = %v, %v", w, err)
	}
	if d := e.Load("germania"); d.Error != "" || len(d.Islands) != 2 || d.Islands[0].Biomes[1] != "cave" || d.Extends != "standard" {
		t.Fatalf("Load = %+v", d)
	}
}

func TestSetRejectsInvalid(t *testing.T) {
	e := fixture(t)
	for name, patch := range map[string]string{
		"unbekanntes Feld": `{"farbe":"rot"}`,
		"kein Objekt":      `[1]`,
		"unbekanntes Biom": `{"islands":[{"id":"island1","biomes":["lava"]}]}`,
		"Insel ohne id":    `{"islands":[{"name":"x"}]}`,
		"Kampf":            `{"combat":{"mode":"chaos"}}`,
	} {
		if _, err := e.Set(ctx, "standard", json.RawMessage(patch)); err == nil {
			t.Errorf("%s ohne Fehler", name)
		}
	}
	if _, err := e.Create(ctx, "../x", "X", ""); err == nil {
		t.Error("ungültige ID angelegt")
	}
	if readWorld(t, e, "standard") != standardFixture {
		t.Fatal("Datei geändert")
	}
}

func TestInlineStyle(t *testing.T) {
	for in, want := range map[string]string{
		`{"a":1.50,"b":[1,2],"c":[{"x":-7}],"d":{},"e":[]}`: `{ "a": 1.50, "b": [1, 2], "c": [ { "x": -7 } ], "d": {}, "e": [] }`,
		`"<&>"`: `"<&>"`,
	} {
		got, err := inline([]byte(in))
		if err != nil || got != want {
			t.Errorf("inline(%s) = %q, %v; want %q", in, got, err, want)
		}
		if again, _ := inline([]byte(got)); again != got {
			t.Errorf("nicht stabil: %q", again)
		}
	}
}

// Review: Eine zwischen Lesen und Schreiben geänderte Datei wird nicht überschrieben; Anlegen ist exklusiv.
func TestWriteCheckedRefusesLostUpdate(t *testing.T) {
	e := fixture(t)
	raw := []byte(readWorld(t, e, "standard"))
	if err := os.WriteFile(e.file("standard"), append(slices.Clone(raw), ' '), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := writeChecked(e.file("standard"), raw, []byte("{}")); err == nil || !strings.Contains(err.Error(), "inzwischen geändert") {
		t.Fatalf("Lost Update nicht erkannt: %v", err)
	}
	if err := writeChecked(e.file("standard"), nil, []byte("{}")); err == nil || !strings.Contains(err.Error(), "gibt es schon") {
		t.Fatalf("exklusives Anlegen überschrieb: %v", err)
	}
	if got := readWorld(t, e, "standard"); got != string(raw)+" " {
		t.Fatalf("Datei geändert: %q", got)
	}
}

// B-985/AC-01: Ein Patch setzt den Hafen der bisher letzten Insel und hängt die nächste an; die Welt ist gültig, nie nur halb.
func TestSetAppendIslandWithHarborInOneCall(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	patch := `{"islands":[
		{"id":"island1","plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}},
		{"id":"island2","name":"Visby","depths":[0,4],"plannedChunks":{"0":[{"slot":-5,"kind":"harbor"}]},"chunkCounts":{"0":16}},
		{"id":"island3","name":"Reval"}]}`
	if _, err := e.Set(ctx, "hanse", json.RawMessage(patch)); err != nil {
		t.Fatal(err)
	}
	text := readWorld(t, e, "hanse")
	for _, want := range []string{`"id": "island3"`, `"id": "island2", "name": "Visby"`, `"chunkCounts": { "0": 16 }`, `"kind": "harbor"`} {
		if !strings.Contains(text, want) {
			t.Fatalf("fehlt %q:\n%s", want, text)
		}
	}
	if _, err := e.Check(ctx, "hanse"); err != nil {
		t.Fatalf("Welt ungültig: %v", err)
	}
}

// B-985/AC-02: Ohne Hafen lehnt die Prüfung ab; die Meldung nennt die Insel und das Feld plannedChunks, die Datei bleibt.
func TestSetAppendWithoutHarborNamesIslandAndField(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	before := readWorld(t, e, "hanse")
	_, err := e.Set(ctx, "hanse", json.RawMessage(`{"islands":[{"id":"island2","name":"Visby"}]}`))
	if err == nil {
		t.Fatal("Insel ohne Hafen der Vorgängerin angenommen")
	}
	for _, want := range []string{"island1", "kein Hafen", "plannedChunks", "world_set"} {
		if !strings.Contains(err.Error(), want) {
			t.Fatalf("Meldung ohne %q: %v", want, err)
		}
	}
	if readWorld(t, e, "hanse") != before {
		t.Fatal("Datei trotz Fehler geändert")
	}
	// Ungültiger Chunk-Plan: die Engine nennt den Grund, die Datei bleibt.
	_, err = e.Set(ctx, "hanse", json.RawMessage(`{"islands":[{"id":"island1","plannedChunks":{"0":[{"slot":-7,"kind":"gibtesnicht"}]}},{"id":"island2"}]}`))
	if err == nil || readWorld(t, e, "hanse") != before {
		t.Fatalf("ungültiger Chunk-Plan angenommen (%v)", err)
	}
}
