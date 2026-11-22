package mcpsrv

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// B-975: world_story_set, world_get, world_check und world_create mit der Geschichte, auf einer Kopie von data/ mit der
// echten Engine im Prozess (Prüfung sim.ParseWorld).

type worldCall func(tool string, args map[string]any) (string, bool)

func storyArgs() map[string]any {
	return map[string]any{
		"lexicon": []any{"oru", "kamae", "no", "sen", "shiro", "nu"},
		"start":   []any{"Oru kamae KRONE no sen. Shiro nu BRENNT."},
		"etappen": map[string]any{"1": []any{"Sen no MEER, oru KRONE kamae."}},
		"end":     []any{"Shiro no FRIEDE, kamae KRONE sen."},
		"islands": map[string]any{"island1": map[string]any{
			"arrival": []any{"Oru no REIS, sen SCHIFF nu."},
			"done":    []any{"MASKE nu, KRONE oru sen."},
		}},
	}
}

func withID(id string, args map[string]any) map[string]any {
	out := map[string]any{"id": id}
	for k, v := range args {
		out[k] = v
	}
	return out
}

func TestWorldStoryTools(t *testing.T) {
	t.Parallel()
	root := worldRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	call := func(tool string, args map[string]any) (string, bool) {
		t.Helper()
		return callText(t, cs, tool, args)
	}
	file := filepath.Join(root, "data", "worlds", "saga.json")
	read := func() string {
		t.Helper()
		raw, err := os.ReadFile(file)
		if err != nil {
			t.Fatal(err)
		}
		return string(raw)
	}
	// AC-03: ohne Geschichte endet die Antwort mit dem Hinweis (danach folgt nur die Checkout-Zeile des Servers).
	text, isErr := call("world_create", map[string]any{"id": "saga", "name": "Saga"})
	if isErr || !strings.Contains(text, "Welt saga angelegt") || !strings.Contains(text, "\n"+worldStoryMissing) {
		t.Fatalf("world_create ohne Geschichte: %s", text)
	}
	before := read()
	checkStoryHints(t, call, "Hinweis: Geschichte fehlt (story)", "Hinweis: Geschichte fehlt (Insel island1: story)")
	checkStorySet(t, call, read, before)
	checkStoryGetAndCheck(t, call)
	checkStoryRejects(t, call, read, file)
	checkStoryCreate(t, call)
}

// AC-01: schreibt Welt- und Insel-Geschichte, die übrigen Felder bleiben byte-gleich.
func checkStorySet(t *testing.T, call worldCall, read func() string, before string) {
	t.Helper()
	text, isErr := call("world_story_set", withID("saga", storyArgs()))
	for _, want := range []string{"Geschichte der Welt saga gespeichert", "Gesetzt: lexicon (6 Wörter), start, etappen.1, end, island1.arrival, island1.done",
		"Fehlt noch: story.bottles, Insel island1: story.boss", "Checkout:"} {
		if isErr || !strings.Contains(text, want) {
			t.Fatalf("world_story_set ohne %q:\n%s", want, text)
		}
	}
	after := read()
	world := "  \"story\": {\n    \"lexicon\": [\"oru\", \"kamae\", \"no\", \"sen\", \"shiro\", \"nu\"],\n    \"start\": [\"Oru kamae KRONE no sen. Shiro nu BRENNT.\"],\n" +
		"    \"etappen\": { \"1\": [\"Sen no MEER, oru KRONE kamae.\"] },\n    \"end\": [\"Shiro no FRIEDE, kamae KRONE sen.\"]\n  },\n"
	island := `, "story": { "arrival": ["Oru no REIS, sen SCHIFF nu."], "done": ["MASKE nu, KRONE oru sen."] }`
	if got := strings.Replace(strings.Replace(after, world, "", 1), island, "", 1); got != before {
		t.Fatalf("übrige Felder nicht byte-gleich:\n%s", after)
	}
	// Je Teil: ein Eintrag dazu, einer weg ([] entfernt), der Rest bleibt.
	text, isErr = call("world_story_set", map[string]any{"id": "saga", "bottles": []any{[]any{"Nu sen FLASCHE, oru MEER no."}},
		"islands": map[string]any{"island1": map[string]any{"boss": []any{"Nu kamae MASKE, shiro TIEF."}, "arrival": []any{}}}})
	if isErr || !strings.Contains(text, "Entfernt: island1.arrival") || !strings.Contains(text, "Fehlt noch: Insel island1: story.arrival") {
		t.Fatalf("world_story_set je Teil: %s", text)
	}
	if got := read(); strings.Contains(got, `"arrival"`) || !strings.Contains(got, `"boss": ["Nu kamae MASKE, shiro TIEF."], "done"`) || !strings.Contains(got, `"start": ["Oru kamae`) {
		t.Fatalf("Teil-Änderung falsch:\n%s", got)
	}
}

// AC-02: world_get zeigt Wortschatz und Zeilen, fehlende als „fehlt“; world_check nennt fehlende Einträge als Hinweis.
func checkStoryGetAndCheck(t *testing.T, call worldCall) {
	t.Helper()
	text, isErr := call("world_get", map[string]any{"id": "saga"})
	for _, want := range []string{"Geschichte:\n  lexicon (6): oru, kamae, no, sen, shiro, nu\n", "  start: Oru kamae KRONE no sen. Shiro nu BRENNT.\n",
		"  etappen.1: Sen no MEER, oru KRONE kamae.\n", "  bottles[1]: Nu sen FLASCHE, oru MEER no.\n", "  island1.arrival: fehlt\n", "  island1.boss: Nu kamae MASKE, shiro TIEF.\n"} {
		if isErr || !strings.Contains(text, want) {
			t.Fatalf("world_get ohne %q:\n%s", want, text)
		}
	}
	checkStoryHints(t, call, "Hinweis: Geschichte fehlt (Insel island1: story.arrival)")
}

func checkStoryHints(t *testing.T, call worldCall, hints ...string) {
	t.Helper()
	text, isErr := call("world_check", map[string]any{"id": "saga"})
	if isErr || !strings.Contains(text, "Welt saga ist gültig") || strings.Count(text, "Hinweis:") != len(hints) {
		t.Fatalf("world_check: %s", text)
	}
	for _, want := range hints {
		if !strings.Contains(text, want) {
			t.Fatalf("world_check ohne %q:\n%s", want, text)
		}
	}
}

// AC-01/AC-02: Verstöße sind Fehler mit Feld und Zeile, geschrieben wird nichts; world_check meldet sie als Fehler.
func checkStoryRejects(t *testing.T, call worldCall, read func() string, file string) {
	t.Helper()
	before := read()
	cases := []struct {
		want string
		args map[string]any
	}{
		{`story.start Zeile 1: Wort "kame" fehlt im lexicon`, map[string]any{"start": []any{"Oru kame KRONE no MEER."}}},
		{"Insel island1: story.done Zeile 1:", map[string]any{"islands": map[string]any{"island1": map[string]any{"done": []any{"MASKE nu oru sen."}}}}},
		{"fehlt im lexicon", map[string]any{"lexicon": []any{"oru"}}},
		{`die Insel "island7" gibt es nicht (bekannt: island1)`, map[string]any{"islands": map[string]any{"island7": map[string]any{"done": []any{"MASKE nu, KRONE oru sen."}}}}},
	}
	for _, c := range cases {
		if text, isErr := call("world_story_set", withID("saga", c.args)); !isErr || !strings.Contains(text, c.want) {
			t.Fatalf("erwartet %q: %s", c.want, text)
		}
		if read() != before {
			t.Fatalf("Datei trotz Fehler geändert (%s)", c.want)
		}
	}
	if text, isErr := call("world_story_set", withID("nirgendwo", storyArgs())); !isErr || !strings.Contains(text, `die Welt "nirgendwo" gibt es nicht (bekannt: `) || !strings.Contains(text, "saga") {
		t.Fatalf("unbekannte Welt: %s", text)
	}
	if text, isErr := call("world_set", map[string]any{"id": "saga", "patch": map[string]any{"story": map[string]any{}}}); !isErr || !strings.Contains(text, "world_story_set") {
		t.Fatalf("world_set mit story: %s", text)
	}
	// Von Hand verdorbene Datei: world_check meldet den Verstoß als Fehler.
	write := func(text string) {
		t.Helper()
		if err := os.WriteFile(file, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	write(strings.Replace(before, "Oru kamae KRONE no sen.", "Oru kamae krone no sen.", 1))
	if text, isErr := call("world_check", map[string]any{"id": "saga"}); !isErr || !strings.Contains(text, `story.start Zeile 1: Wort "krone" fehlt im lexicon`) {
		t.Fatalf("world_check mit Verstoß: %s", text)
	}
	write(before)
}

// AC-03: world_create mit Geschichte schreibt sie mit, die Antwort nennt den Hinweis dann nicht.
func checkStoryCreate(t *testing.T, call worldCall) {
	t.Helper()
	text, isErr := call("world_create", map[string]any{"id": "edda", "name": "Edda", "story": storyArgs()})
	if isErr || !strings.Contains(text, "Welt edda angelegt") || strings.Contains(text, worldStoryMissing) {
		t.Fatalf("world_create mit Geschichte: %s", text)
	}
	text, isErr = call("world_get", map[string]any{"id": "edda"})
	if isErr || !strings.Contains(text, "  island1.done: MASKE nu, KRONE oru sen.\n") || !strings.Contains(text, `"lexicon": ["oru", "kamae"`) {
		t.Fatalf("world_get nach world_create: %s", text)
	}
	bad := storyArgs()
	bad["end"] = []any{"Shiro no kamae sen."}
	if text, isErr := call("world_create", map[string]any{"id": "kaputt", "name": "Kaputt", "story": bad}); !isErr || !strings.Contains(text, "story.end Zeile 1") {
		t.Fatalf("world_create mit ungültiger Geschichte: %s", text)
	}
	if text, isErr := call("world_get", map[string]any{"id": "kaputt"}); !isErr {
		t.Fatalf("Welt trotz ungültiger Geschichte angelegt: %s", text)
	}
}
