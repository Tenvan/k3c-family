package world

import (
	"encoding/json"
	"os"
	"strings"
	"testing"
)

// storyFixture ist eine gültige Geschichte der Welt mit einer Insel island1 (Muster aus docs/rules/welten.md § 2.2).
func storyFixture() StoryPatch {
	return StoryPatch{
		Story: Story{
			Lexicon: []string{"oru", "kamae", "no", "sen", "shiro", "nu"},
			Start:   []string{"Oru kamae KRONE no sen. Shiro nu BRENNT."},
			Etappen: map[string][]string{"1": {"Sen no MEER, oru KRONE kamae."}},
			End:     []string{"Shiro no FRIEDE, kamae KRONE sen."},
			Bottles: [][]string{{"Nu sen FLASCHE, oru MEER no."}},
		},
		Islands: map[string]IslandStory{"island1": {
			Arrival: []string{"Oru no REIS, sen SCHIFF nu."},
			Boss:    []string{"Nu kamae MASKE, shiro TIEF."},
			Done:    []string{"MASKE nu, KRONE oru sen."},
		}},
	}
}

const storyBlock = `  "story": {
    "lexicon": ["oru", "kamae", "no", "sen", "shiro", "nu"],
    "start": ["Oru kamae KRONE no sen. Shiro nu BRENNT."],
    "etappen": { "1": ["Sen no MEER, oru KRONE kamae."] },
    "end": ["Shiro no FRIEDE, kamae KRONE sen."],
    "bottles": [ ["Nu sen FLASCHE, oru MEER no."] ]
  },
`

const islandStoryText = `, "story": { "arrival": ["Oru no REIS, sen SCHIFF nu."], "boss": ["Nu kamae MASKE, shiro TIEF."], "done": ["MASKE nu, KRONE oru sen."] }`

// B-975/AC-01: SetStory schreibt Welt- und Insel-Geschichte, die übrigen Bytes bleiben; ein zweiter Lauf ändert nichts,
// Entfernen stellt die Datei byte-gleich wieder her.
func TestSetStoryWritesOnlyStory(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "saga", "Saga", ""); err != nil {
		t.Fatal(err)
	}
	before := readWorld(t, e, "saga")
	res, err := e.SetStory(ctx, "saga", storyFixture())
	if err != nil {
		t.Fatal(err)
	}
	after := readWorld(t, e, "saga")
	stripped := strings.Replace(strings.Replace(after, storyBlock, "", 1), islandStoryText, "", 1)
	if stripped != before {
		t.Fatalf("übrige Felder nicht byte-gleich:\n%s", after)
	}
	if strings.Index(after, `"story"`) > strings.Index(after, `"islands"`) {
		t.Fatalf("story steht nicht vor islands:\n%s", after)
	}
	want := "lexicon (6 Wörter), start, etappen.1, end, bottles (1 Einträge), island1.arrival, island1.boss, island1.done"
	if got := strings.Join(res.Set, ", "); got != want || len(res.Removed)+len(res.Missing) != 0 {
		t.Fatalf("Antwort = %+v", res)
	}
	if _, err := e.SetStory(ctx, "saga", storyFixture()); err != nil || readWorld(t, e, "saga") != after {
		t.Fatalf("zweiter Lauf änderte die Datei (%v)", err)
	}
	d := e.Load("saga")
	if d.Story == nil || len(d.Story.Lexicon) != 6 || d.Islands[0].Story == nil || d.Islands[0].Story.Done[0] != "MASKE nu, KRONE oru sen." {
		t.Fatalf("Load = %+v", d)
	}
	checkStoryCleared(t, e, before)
}

// checkStoryCleared: leere Einträge entfernen die Geschichte, die Datei ist wieder byte-gleich before.
func checkStoryCleared(t *testing.T, e Editor, before string) {
	t.Helper()
	empty := []string{}
	none := StoryPatch{
		Story:   Story{Lexicon: empty, Start: empty, Etappen: map[string][]string{"1": empty}, End: empty, Bottles: [][]string{}},
		Islands: map[string]IslandStory{"island1": {Arrival: empty, Boss: empty, Done: empty}},
	}
	res, err := e.SetStory(ctx, "saga", none)
	if err != nil || readWorld(t, e, "saga") != before {
		t.Fatalf("Entfernen stellt die Datei nicht wieder her (%v):\n%s", err, readWorld(t, e, "saga"))
	}
	if len(res.Removed) != 8 || strings.Join(res.Missing, "; ") != "story; Insel island1: story" {
		t.Fatalf("Antwort = %+v", res)
	}
}

// B-975/AC-01 und Fehlerfälle: Eine ungültige Zeile, ein verkleinerter Wortschatz, eine unbekannte Welt oder Insel
// werden abgelehnt, die Datei bleibt.
func TestSetStoryRejects(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "saga", "Saga", ""); err != nil {
		t.Fatal(err)
	}
	if _, err := e.SetStory(ctx, "saga", storyFixture()); err != nil {
		t.Fatal(err)
	}
	before := readWorld(t, e, "saga")
	cases := []struct {
		name, world, want string
		patch             StoryPatch
	}{
		{"Wort fehlt im Wortschatz", "saga", `story.start Zeile 1: Wort "kame" fehlt im lexicon`, StoryPatch{Story: Story{Start: []string{"Oru kame KRONE no MEER."}}}},
		{"zu wenig echte Wörter", "saga", "Insel island1: story.boss Zeile 2", StoryPatch{Islands: map[string]IslandStory{"island1": {Boss: []string{"Nu kamae MASKE, shiro TIEF.", "Oru KRONE sen."}}}}},
		{"Wortschatz verkleinert", "saga", "fehlt im lexicon", StoryPatch{Story: Story{Lexicon: []string{"oru", "kamae"}}}},
		{"Etappe ohne Insel", "saga", "Etappe 2 hat keine Insel", StoryPatch{Story: Story{Etappen: map[string][]string{"2": {"Sen no MEER, oru KRONE kamae."}}}}},
		{"unbekannte Insel", "saga", `die Insel "island9" gibt es nicht (bekannt: island1)`, StoryPatch{Islands: map[string]IslandStory{"island9": {Done: []string{"MASKE nu, KRONE oru sen."}}}}},
		{"unbekannte Welt", "nirgendwo", `die Welt "nirgendwo" gibt es nicht (bekannt: `, storyFixture()},
		{"leerer Aufruf", "saga", "kein Eintrag genannt", StoryPatch{}},
	}
	for _, c := range cases {
		if _, err := e.SetStory(ctx, c.world, c.patch); err == nil || !strings.Contains(err.Error(), c.want) {
			t.Errorf("%s: Fehler = %v, erwartet %q", c.name, err, c.want)
		}
		if readWorld(t, e, "saga") != before {
			t.Fatalf("%s: Datei trotz Fehler geändert", c.name)
		}
	}
}

// Review B-975: Die übrigen Werkzeuge (world_set, world_chunks_set, Etappe setzen) lassen `story` stehen; world_set
// nimmt `story` nicht an und verweist auf world_story_set.
func TestOtherEditsKeepStory(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "saga", "Saga", ""); err != nil {
		t.Fatal(err)
	}
	if _, err := e.SetStory(ctx, "saga", storyFixture()); err != nil {
		t.Fatal(err)
	}
	if _, err := e.Set(ctx, "saga", json.RawMessage(`{"notes":"Sagen","islands":[{"id":"island1","name":"Fjord","depthScaling":{"hp":2}}]}`)); err != nil {
		t.Fatal(err)
	}
	count := 24
	if _, err := e.SetChunks(ctx, "saga", "island1", 0, json.RawMessage(`[{"slot":-7,"kind":"harbor"}]`), &count); err != nil {
		t.Fatal(err)
	}
	one := 1
	if v := e.SetEtappeView(ctx, "saga", "island1", &one); v.Error != "" {
		t.Fatal(v.Error)
	}
	text := readWorld(t, e, "saga")
	for _, want := range []string{storyBlock, islandStoryText, `"notes": "Sagen"`, `"name": "Fjord", "etappe": 1`, `"chunkCounts": { "0": 24 }`} {
		if !strings.Contains(text, want) {
			t.Fatalf("fehlt %q:\n%s", want, text)
		}
	}
	for _, patch := range []string{`{"story":{"lexicon":["oru"]}}`, `{"islands":[{"id":"island1","story":{"done":["MASKE nu, KRONE oru sen."]}}]}`} {
		if _, err := e.Set(ctx, "saga", json.RawMessage(patch)); err == nil || !strings.Contains(err.Error(), "world_story_set") {
			t.Fatalf("world_set mit story: Fehler = %v", err)
		}
	}
	if readWorld(t, e, "saga") != text {
		t.Fatal("Datei trotz Fehler geändert")
	}
}

// B-975/AC-03: Anlegen mit Geschichte schreibt sie mit; eine ungültige Geschichte legt keine Welt an. Zeilenenden der
// Datei bleiben (CRLF).
func TestCreateWithStoryAndCRLF(t *testing.T) {
	e := fixture(t)
	bad := storyFixture()
	bad.Start = []string{"Oru kamae KRONE."}
	if _, err := e.CreateWithStory(ctx, "saga", "Saga", "", &bad); err == nil || !strings.Contains(err.Error(), "story.start Zeile 1") {
		t.Fatalf("Fehler = %v", err)
	}
	if d := e.Load("saga"); d.Error == "" {
		t.Fatal("Welt trotz ungültiger Geschichte angelegt")
	}
	p := storyFixture()
	if _, err := e.CreateWithStory(ctx, "saga", "Saga", "", &p); err != nil {
		t.Fatal(err)
	}
	text := readWorld(t, e, "saga")
	if !strings.Contains(text, storyBlock) || !strings.Contains(text, islandStoryText) || len(e.Load("saga").StoryGaps()) != 0 {
		t.Fatalf("Geschichte fehlt:\n%s", text)
	}
	if _, err := e.Create(ctx, "crlf", "CRLF", ""); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(e.file("crlf"), []byte(strings.ReplaceAll(readWorld(t, e, "crlf"), "\n", "\r\n")), 0o644); err != nil {
		t.Fatal(err)
	}
	if _, err := e.SetStory(ctx, "crlf", storyFixture()); err != nil {
		t.Fatal(err)
	}
	if got := readWorld(t, e, "crlf"); strings.Contains(strings.ReplaceAll(got, "\r\n", ""), "\n") || !strings.Contains(got, "\"story\": {\r\n") {
		t.Fatalf("Zeilenenden gemischt:\n%q", got)
	}
}

// B-975/R3: fehlende Einträge je Welt und Insel.
func TestStoryGaps(t *testing.T) {
	d := Detail{Story: &Story{Lexicon: []string{"oru"}, Start: []string{"x"}}, Islands: []Island{
		{ID: "a", Story: &IslandStory{Arrival: []string{"x"}, Done: []string{"x"}}}, {ID: "b"},
	}}
	want := "story.end; story.bottles; Insel a: story.boss; Insel b: story"
	if got := strings.Join(d.StoryGaps(), "; "); got != want {
		t.Fatalf("StoryGaps = %q", got)
	}
}

// Review B-975: Nennt der Aufruf nur Inseln, bleibt ein von Hand geschriebenes `story` der Welt byte-gleich; ein Eintrag
// der Welt schreibt es danach im Stil der Datei neu.
func TestSetStoryIslandOnlyKeepsWorldStoryBytes(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "saga", "Saga", ""); err != nil {
		t.Fatal(err)
	}
	hand := `"story": {"lexicon":["oru","kamae","no","sen","shiro","nu"]},` + "\n  "
	text := strings.Replace(readWorld(t, e, "saga"), `"islands"`, hand+`"islands"`, 1)
	if err := os.WriteFile(e.file("saga"), []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
	p := StoryPatch{Islands: storyFixture().Islands}
	if _, err := e.SetStory(ctx, "saga", p); err != nil {
		t.Fatal(err)
	}
	if got := readWorld(t, e, "saga"); got != strings.Replace(text, ` }`+"\n  ]", islandStoryText+` }`+"\n  ]", 1) {
		t.Fatalf("Welt-story nicht byte-gleich:\n%s", got)
	}
	if _, err := e.SetStory(ctx, "saga", StoryPatch{Story: Story{End: []string{"Shiro no FRIEDE, kamae KRONE sen."}}}); err != nil {
		t.Fatal(err)
	}
	if got := readWorld(t, e, "saga"); !strings.Contains(got, "\"story\": {\n    \"lexicon\": [\"oru\", \"kamae\", \"no\", \"sen\", \"shiro\", \"nu\"],\n    \"end\"") {
		t.Fatalf("Welt-story nicht im Stil der Datei:\n%s", got)
	}
}
