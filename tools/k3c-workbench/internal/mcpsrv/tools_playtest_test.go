package mcpsrv

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"

	"k3c/tools/k3c-workbench/internal/playtest"
)

// playtestRepo ist ein Planungs-Repo mit einer Session samt Transkript, Markern und Bild.
func playtestRepo(t *testing.T) string {
	t.Helper()
	root := planRepo(t)
	dir := filepath.Join(root, "playtests", "2026-11-22-1")
	if err := os.MkdirAll(filepath.Join(dir, "frames"), 0o755); err != nil {
		t.Fatal(err)
	}
	files := map[string]string{
		"session.json":  `{"startMs":1002000,"endMs":1100000,"captureMs":1000000,"stand":"aufgenommen"}`,
		"voice.wav":     "x",
		"video.mkv":     "x",
		"markers.jsonl": `{"ts":1005000,"kind":"pad","player":1,"world":"standard","island":2,"stage":3,"phase":"night","day":4,"worst_frame_ms":12.5}` + "\n",
		"survey.json":   `{"spass": 4}`,
		"transcript.json": `{"audioStartMs":1000000,"language":"de","segments":[{"startMs":1004000,"endMs":1006000,"text":"Ich finde den Bauarbeiter nicht"},` +
			`{"startMs":1030000,"endMs":1031000,"text":"Satz zwei"},{"startMs":1040000,"endMs":1041000,"text":"Satz drei"}]}`,
		"frames/marker-1.jpg": "jpg",
	}
	for n, c := range files {
		if err := os.WriteFile(filepath.Join(dir, filepath.FromSlash(n)), []byte(c), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	return root
}

func TestPlaytestToolsLesen(t *testing.T) {
	t.Parallel()
	cs := connect(t, New(Config{Version: "test", Root: playtestRepo(t)}))
	text, isErr := callText(t, cs, "playtest_list", map[string]any{})
	if isErr || !strings.Contains(text, "2026-11-22-1 · transkribiert") || !strings.Contains(text, "1 Marker") {
		t.Errorf("playtest_list: %q", text)
	}
	text, isErr = callText(t, cs, "playtest_get", map[string]any{"id": "2026-11-22-1"})
	for _, want := range []string{"Fragebogen: {\"spass\": 4}", "Sprache: Ich finde den Bauarbeiter nicht", "Marker #1", "Insel 2 Stufe 3",
		"ts=1005000", "00:05", "marker-1.jpg", "Zeitleiste 1–4 von 4"} {
		if isErr || !strings.Contains(text, want) {
			t.Errorf("playtest_get ohne %q: %q", want, text)
		}
	}
	text, _ = callText(t, cs, "playtest_get", map[string]any{"id": "2026-11-22-1", "limit": 2})
	if !strings.Contains(text, "Zeitleiste 1–2 von 4") || !strings.Contains(text, "weiter mit cursor=2") {
		t.Errorf("limit: %q", text)
	}
	text, _ = callText(t, cs, "playtest_get", map[string]any{"id": "2026-11-22-1", "limit": 2, "cursor": 2})
	if !strings.Contains(text, "Zeitleiste 3–4 von 4") || strings.Contains(text, "Fragebogen") || strings.Contains(text, "weiter mit") {
		t.Errorf("cursor: %q", text)
	}
	if text, isErr = callText(t, cs, "playtest_get", map[string]any{"id": "2026-01-01-9"}); !isErr || !strings.Contains(text, "nicht gefunden") {
		t.Errorf("unbekannte ID: %q %v", text, isErr)
	}
	if text, isErr = callText(t, cs, "playtest_get", map[string]any{"id": "../x"}); !isErr {
		t.Errorf("ungültige ID: %q", text)
	}
}

func TestPlaytestFindingsUndTickets(t *testing.T) {
	t.Parallel()
	root := playtestRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	var altID string
	for _, step := range []struct {
		tool string
		args map[string]any
	}{
		{"plan_create", map[string]any{"kind": "projekt", "id": "PXX", "slug": "px", "title": "Projekt X"}},
		{"plan_create", map[string]any{"kind": "sprint", "id": "X1", "slug": "x", "title": "X", "fields": map[string]string{"Domäne": "DEV", "Projekt": "PXX"}}},
		{"plan_create", map[string]any{"kind": "ticket", "slug": "alt", "title": "Alt", "fields": map[string]string{"Domäne": "DEV", "Typ": "Problem", "Prio": "mittel", "Projekt": "PXX", "Sprint": "X1"}}},
	} {
		text, isErr := callText(t, cs, step.tool, step.args)
		if isErr {
			t.Fatalf("%s: %s", step.tool, text)
		}
		if m := regexp.MustCompile(`B-\d+ angelegt`).FindString(text); m != "" {
			altID = strings.TrimSuffix(m, " angelegt")
		}
	}
	text, isErr := callText(t, cs, "playtest_findings_set", map[string]any{"id": "2026-11-22-1", "findings": []map[string]any{
		{"id": "f1", "quote": "Ich finde den Bauarbeiter nicht", "ts": 1005000, "type": "Problem", "frame": "frames/marker-1.jpg", "context": "Insel 2 Stufe 3"},
		{"id": "f2", "quote": "Satz zwei", "ts": 1030000, "type": "Idee", "duplicate_of": altID},
		{"id": "f3", "quote": "nicht angehakt", "ts": 1040000, "type": "Idee"},
	}})
	if isErr || !strings.Contains(text, "3 neu") || !strings.Contains(text, "Checkout:") {
		t.Fatalf("playtest_findings_set: %q", text)
	}
	if text, isErr = callText(t, cs, "playtest_findings_set", map[string]any{"id": "2026-11-22-1", "findings": []map[string]any{{"id": "f9", "quote": "x", "ts": 1, "type": "Bug"}}}); !isErr {
		t.Errorf("ungültiger Typ: %q", text)
	}
	// Haken setzt Ralf auf der Seite
	st := &playtest.Store{Root: filepath.Join(root, "playtests")}
	on := true
	for _, id := range []string{"f1", "f2"} {
		if _, err := st.UpdateFinding("2026-11-22-1", id, playtest.FindingPatch{Selected: &on}); err != nil {
			t.Fatal(err)
		}
	}
	text, isErr = callText(t, cs, "playtest_tickets_create", map[string]any{"id": "2026-11-22-1", "domain": "REG", "project": "PXX", "sprint": "X1"})
	if isErr || !strings.Contains(text, "1 angelegt") || !strings.Contains(text, "1 Hinweise") {
		t.Fatalf("playtest_tickets_create: %q", text)
	}
	got, _ := callText(t, cs, "plan_list", map[string]any{"kind": "ticket"})
	if !strings.Contains(got, "Bauarbeiter") || strings.Contains(got, "nicht angehakt") {
		t.Errorf("Tickets: %s", got)
	}
	text, _ = callText(t, cs, "playtest_get", map[string]any{"id": "2026-11-22-1", "limit": 1})
	if !strings.Contains(text, "f1 [angehakt] → B-") || !strings.Contains(text, "Duplikat von") {
		t.Errorf("Befunde in playtest_get: %q", text)
	}
}

func TestPlaytestFrameOhneFfmpeg(t *testing.T) {
	t.Parallel()
	cs := connect(t, New(Config{Version: "test", Root: playtestRepo(t)}))
	if text, isErr := callText(t, cs, "playtest_frame", map[string]any{"id": "2026-01-01-9", "t": 5}); !isErr || !strings.Contains(text, "nicht gefunden") {
		t.Errorf("unbekannte ID: %q", text)
	}
}
