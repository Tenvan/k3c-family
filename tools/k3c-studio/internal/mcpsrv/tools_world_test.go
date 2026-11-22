package mcpsrv

import (
	"context"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
)

// B-954: Roundtrip der Tools world_* auf einer Kopie von data/ (t.TempDir, echte Engine im Prozess); die Welt-Dateien
// des Checkouts werden nur gelesen.
func worldRepo(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	if err := os.CopyFS(filepath.Join(root, "data"), os.DirFS(filepath.Join(hubRepoRoot, "data"))); err != nil {
		t.Fatal(err)
	}
	return root
}

func TestWorldToolsRoundtrip(t *testing.T) {
	t.Parallel()
	root := worldRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	call := func(tool string, args map[string]any) (string, bool) {
		t.Helper()
		return callText(t, cs, tool, args)
	}
	if text, isErr := call("world_list", map[string]any{}); isErr || !strings.Contains(text, "standard · Standard") {
		t.Fatalf("world_list: %s", text)
	}
	checkWorldCreateDelete(t, call)
	patch := map[string]any{"notes": "Handelsstädte", "combat": map[string]any{"mode": "groups", "groupSize": 5},
		"islands": []any{map[string]any{"id": "island1", "name": "Lübeck", "biomes": []any{"forest", "cave", "mine", "ironhold", "crystal"}}}}
	if text, isErr := call("world_set", map[string]any{"id": "hanse", "patch": patch}); isErr {
		t.Fatalf("world_set: %s", text)
	}
	if text, isErr := call("world_override_set", map[string]any{"id": "hanse", "file": "economy", "path": []any{"purse", "startGold"}, "value": 99}); isErr {
		t.Fatalf("world_override_set: %s", text)
	}
	if text, isErr := call("world_chunks_set", map[string]any{"id": "hanse", "island": "island1", "depth": 0, "chunks": []any{map[string]any{"slot": -7, "kind": "harbor"}}}); isErr {
		t.Fatalf("world_chunks_set: %s", text)
	}
	if text, isErr := call("world_check", map[string]any{"id": "hanse"}); isErr || !strings.Contains(text, "ist gültig") {
		t.Fatalf("world_check: %s", text)
	}
	checkWorldGet(t, call)
	checkWorldRejects(t, call)
}

func checkWorldCreateDelete(t *testing.T, call func(string, map[string]any) (string, bool)) {
	t.Helper()
	if text, isErr := call("world_create", map[string]any{"id": "hanse", "name": "Hanse", "extends": "standard"}); isErr || !strings.Contains(text, "Welt hanse angelegt") || !strings.Contains(text, "Checkout:") {
		t.Fatalf("world_create: %s", text)
	}
	if text, isErr := call("world_create", map[string]any{"id": "hanse", "name": "Nochmal"}); !isErr || !strings.Contains(text, "gibt es schon") {
		t.Fatalf("zweites world_create: %s", text)
	}
	if text, isErr := call("world_delete", map[string]any{"id": "hanse"}); !isErr || !strings.Contains(text, "confirm=true") {
		t.Fatalf("world_delete ohne confirm: %s", text)
	}
	if text, isErr := call("world_delete", map[string]any{"id": "standard", "confirm": true}); !isErr || !strings.Contains(text, "feste Welt") {
		t.Fatalf("world_delete standard: %s", text)
	}
}

func checkWorldRejects(t *testing.T, call func(string, map[string]any) (string, bool)) {
	t.Helper()
	if text, isErr := call("world_set", map[string]any{"id": "hanse", "patch": map[string]any{"farbe": "rot"}}); !isErr || !strings.Contains(text, "unbekanntes Feld") {
		t.Fatalf("ungültiger patch: %s", text)
	}
	if _, err := os.Stat(filepath.Join(hubRepoRoot, "data", "worlds", "hanse.json")); err == nil {
		t.Fatal("Welt-Datei im Checkout geschrieben")
	}
}

func checkWorldGet(t *testing.T, call func(string, map[string]any) (string, bool)) {
	t.Helper()
	text, isErr := call("world_get", map[string]any{"id": "hanse"})
	for _, want := range []string{"hanse · Hanse · extends standard · 1 Inseln", "Lübeck · Tiefen [0 1 2 3 4] → forest, cave, mine, ironhold, crystal", "plannedChunks[0]:", `"startGold": 99`, `"notes": "Handelsstädte"`} {
		if isErr || !strings.Contains(text, want) {
			t.Fatalf("world_get ohne %q:\n%s", want, text)
		}
	}
}

// B-986: Die Liste der erlaubten Dateien in der Beschreibung von world_override_set gleicht der Basis der Engine.
func TestOverrideFilesTextMatchesEngine(t *testing.T) {
	t.Parallel()
	view := worldEditor(worldRepo(t)).Values(context.Background(), "standard")
	var files []string
	for _, v := range view.Entries {
		if !slices.Contains(files, v.File) {
			files = append(files, v.File)
		}
	}
	slices.Sort(files)
	if got := strings.Join(files, ", "); got != overrideFilesText {
		t.Fatalf("overrideFilesText = %q, Engine = %q", overrideFilesText, got)
	}
}
