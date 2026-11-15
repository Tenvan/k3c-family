package sim

import (
	"encoding/json"
	"slices"
	"strings"
	"testing"
)

// B-965 (GS1.2) AC-02 bis AC-05: Feld `story` laden und prüfen, Wendepunkte als reine Funktionen.

const storyLex = `"lexicon":["oru","kamae","no","sen","shiro","nu"]`

// storyWorldJSON: Welt „test“ mit Inseln a (Etappe 1), b und c (Etappe 2); worldStory ist der Inhalt von `story` der Welt
// (ohne Klammern, leer = kein Feld), islandStory je Insel a, b, c der Inhalt von `story` (leer = kein Feld).
func storyWorldJSON(worldStory string, islandStory ...string) []byte {
	var islands []string
	for i, id := range []string{"a", "b", "c"} {
		f := harborPlan
		if id != "a" {
			f += `,"etappe":2`
		}
		if i < len(islandStory) && islandStory[i] != "" {
			f += `,"story":{` + islandStory[i] + `}`
		}
		islands = append(islands, testIsland(id, f))
	}
	raw := testWorld(strings.Join(islands, ","))
	if worldStory == "" {
		return raw
	}
	return []byte(strings.Replace(string(raw), `"islands"`, `"story":{`+worldStory+`},"islands"`, 1))
}

// storyExample: die Welt aus B-965 › Beispiele mit eindeutig erkennbaren Zeilen je Wendepunkt.
func storyExample(t *testing.T) WorldDef {
	t.Helper()
	ws := storyLex + `,"start":["Oru kamae KRONE no sen. Shiro nu BRENNT."],"etappen":{"1":["Sen no MEER, oru KRONE kamae."]},` +
		`"end":["Shiro no FRIEDE, kamae KRONE sen."],"bottles":[["Nu sen FLASCHE, oru MEER."]]`
	w, err := ParseWorld(storyWorldJSON(ws,
		`"arrival":["Oru no REIS, sen SCHIFF nu."],"boss":["Nu kamae MASKE, shiro TIEF."],"done":["MASKE nu, KRONE oru sen."]`,
		`"arrival":["Sen oru HAFEN, nu SEE."],"done":["Nu sen RUHE, oru BERG."]`,
		`"arrival":["Kamae no WIND, sen SEE."],"boss":["Shiro nu DRACHE, sen NACHT."]`), "test")
	if err != nil {
		t.Fatal(err)
	}
	return w
}

func TestStoryLaedtMitUndOhne(t *testing.T) {
	w := storyExample(t)
	if len(w.Story.Lexicon) != 6 || w.Story.Etappen["1"][0] == "" || len(w.Story.Bottles) != 1 || w.Islands[0].Story.Boss == nil {
		t.Errorf("Geschichte nicht gelesen: %+v / %+v", w.Story, w.Islands[0].Story)
	}
	plain, err := ParseWorld(storyWorldJSON(""), "test")
	if err != nil {
		t.Fatal(err)
	}
	if len(plain.Story.Lexicon) != 0 || plain.Story.Start != nil || plain.Islands[0].Story.Arrival != nil {
		t.Errorf("Welt ohne story: %+v", plain.Story)
	}
	if got := plain.StoryStart(); len(got) != 0 {
		t.Errorf("Zeilen ohne story: %v", got)
	}
	std := StandardWorld()
	if len(std.Story.Start) != 0 || len(std.StoryStart()) != 0 {
		t.Errorf("Standard-Welt hat Zeilen: %+v", std.Story)
	}
}

func TestStoryLimitsAusDaten(t *testing.T) {
	if l := StoryLimitsOf(); l.MaxLineChars != 64 || l.MaxLexicon != 24 {
		t.Errorf("data/story.json: %+v", l)
	}
}

// longLine ist 65 Zeichen lang, aber nur Kunst- und echte Wörter im Wortschatz.
var longLine = "Oru KRONE " + strings.Repeat("no ", 17) + "SENN"

func TestStoryPruefungen(t *testing.T) {
	big := make([]string, 25)
	for i := range big {
		big[i] = "w" + strings.Repeat("a", i)
	}
	bigJSON, _ := json.Marshal(big)
	cases := []struct {
		name, want string
		world      string
		islands    []string
	}{
		{"unbekanntes Feld Welt", `story: unbekanntes Feld "intro"`, storyLex + `,"intro":["x"]`, nil},
		{"unbekanntes Feld Insel", `Insel a: story: unbekanntes Feld "ende"`, storyLex, []string{`"ende":["x"]`}},
		{"Wortschatz zu groß", `story.lexicon: 25 Wörter (höchstens 24)`, `"lexicon":` + string(bigJSON), nil},
		{"Wortschatz Wort groß", `story.lexicon Wort 2: "Kamae" ist nicht klein geschrieben`, `"lexicon":["oru","Kamae"]`, nil},
		{"Wortschatz doppelt", `story.lexicon Wort 2: "oru" doppelt`, `"lexicon":["oru","oru"]`, nil},
		{"drei Zeilen", `story.start: 3 Zeilen (1 bis 2)`, storyLex + `,"start":["a","b","c"]`, nil},
		{"null Zeilen", `story.end: 0 Zeilen (1 bis 2)`, storyLex + `,"end":[]`, nil},
		{"leere Zeile", `story.end Zeile 2: leer`, storyLex + `,"end":["Oru KRONE no MEER.","  "]`, nil},
		{"Zeile zu lang", `Insel b: story.arrival Zeile 1: 65 Zeichen (höchstens 64)`, storyLex, []string{``, `"arrival":["` + longLine + `"]`}},
		{"Kunstwort fehlt", `story.start Zeile 1: Wort "kame" fehlt im lexicon`, storyLex + `,"start":["Kame KRONE no MEER."]`, nil},
		{"Kunstwort groß/klein egal, aber Tippfehler", `Insel c: story.done Zeile 1: Wort "sem" fehlt im lexicon`, storyLex, []string{``, ``, `"done":["Sem KRONE no MEER."]`}},
		{"ein echtes Wort", `Insel a: story.boss Zeile 1: 1 echtes Wort (2 bis 3)`, storyLex, []string{`"boss":["Oru KRONE no sen."]`}},
		{"vier echte Wörter", `story.etappen.1 Zeile 1: 4 echte Wörter (2 bis 3)`, storyLex + `,"etappen":{"1":["Oru KRONE MEER SCHIFF WIND."]}`, nil},
		{"Flaschenpost", `story.bottles Eintrag 2 Zeile 1: 1 echtes Wort (2 bis 3)`, storyLex + `,"bottles":[["Oru KRONE MEER."],["Oru KRONE."]]`, nil},
		{"Etappe ohne Insel", `story.etappen: Etappe 3 hat keine Insel`, storyLex + `,"etappen":{"3":["Oru KRONE MEER."]}`, nil},
		{"Etappen-Schlüssel", `story.etappen: Schlüssel "01" ist keine Etappen-Nummer`, storyLex + `,"etappen":{"01":["Oru KRONE MEER."]}`, nil},
	}
	for _, c := range cases {
		_, err := ParseWorld(storyWorldJSON(c.world, c.islands...), "test")
		if err == nil || !strings.Contains(err.Error(), "data/worlds/test.json: ") || !strings.Contains(err.Error(), c.want) {
			t.Errorf("%s: erwartet %q, Fehler %v", c.name, c.want, err)
		}
	}
}

// Satzzeichen am Rand, Gedankenstrich, Apostroph und Bindestrich im Kunstwort, Umlaute und Sonderzeichen sind erlaubt.
func TestStoryWoerterMitSonderzeichen(t *testing.T) {
	ws := `"lexicon":["akh'et","sen-hotep","hjørn","ma'u"],"start":["Akh'et ra KRONE – sen-hotep, hjørn FLUT!"]`
	if _, err := ParseWorld(storyWorldJSON(ws), "test"); err == nil || !strings.Contains(err.Error(), `Wort "ra" fehlt`) {
		t.Fatalf("ra fehlt im Wortschatz: %v", err)
	}
	ws = strings.Replace(ws, `"ma'u"`, `"ma'u","ra"`, 1)
	w, err := ParseWorld(storyWorldJSON(ws+`,"end":["Ma'u ÄTHER KÜSTE."]`), "test")
	if err != nil {
		t.Fatal(err)
	}
	if len(w.Story.Start) != 1 || len(w.Story.End) != 1 {
		t.Errorf("Zeilen %+v", w.Story)
	}
}

// extends vererbt die Geschichte nicht: die Datei der Kind-Welt trägt ihre eigene story (Feld bleibt bei der Datei).
func TestStoryNichtVererbt(t *testing.T) {
	raw := strings.Replace(string(storyWorldJSON(storyLex)), `"islands"`, `"extends":"standard","islands"`, 1)
	w, err := ParseWorld([]byte(raw), "test")
	if err != nil {
		t.Fatal(err)
	}
	if len(w.Story.Start) != 0 || w.Islands[0].Story.Arrival != nil {
		t.Errorf("geerbt: %+v", w.Story)
	}
}

func TestStoryWendepunkteBeispiel(t *testing.T) {
	w := storyExample(t)
	check := func(name string, got, want []string) {
		t.Helper()
		if !slices.Equal(got, want) {
			t.Errorf("%s: %q, erwartet %q", name, got, want)
		}
	}
	s, a, b, c := w.Story, w.Islands[0].Story, w.Islands[1].Story, w.Islands[2].Story
	check("Start", w.StoryStart(), []string{s.Start[0], a.Arrival[0]})
	check("Ankunft c", w.StoryArrival("c"), c.Arrival)
	check("Ankunft unbekannt", w.StoryArrival("zz"), nil)
	check("Boss a", w.StoryBoss("a"), a.Boss)
	check("Boss b ohne Eintrag", w.StoryBoss("b"), nil)
	check("Sieg a: Etappe 1 abgeschlossen", w.StoryDefeated("a", nil), []string{a.Done[0], s.Etappen["1"][0]})
	check("Sieg c, b offen: nur done", w.StoryDefeated("c", []string{"a"}), nil) // c ohne done
	check("Sieg b nach c: letzte Insel", w.StoryDefeated("b", []string{"a", "c"}), []string{b.Done[0], s.End[0]})
	check("Sieg c nach b: letzte Insel", w.StoryDefeated("c", []string{"a", "b"}), []string{s.End[0]})
	check("Sieg b, c offen: nur done", w.StoryDefeated("b", []string{"a"}), b.Done)
	check("Variante auf früherer Insel", w.StoryVictory("a", nil), nil)
	check("Variante auf letzter Insel", w.StoryVictory("b", []string{"a", "c"}), []string{b.Done[0], s.End[0]})
	check("Insel unbekannt", w.StoryDefeated("zz", nil), nil)
}

// Ohne `etappe` ist jede Insel eine eigene Etappe: auf jede Abschlusszeile folgt die Etappenzeile der Welt.
func TestStoryEtappeOhneAngabe(t *testing.T) {
	std := StandardWorld()
	if len(std.Islands) < 2 {
		t.Skip("Standard-Welt hat nur eine Insel")
	}
	std.Story = WorldStory{Etappen: map[string][]string{"1": {"x"}}, End: []string{"e"}}
	std.Islands = slices.Clone(std.Islands)
	std.Islands[0].Story.Done = []string{"d"}
	if got := std.StoryDefeated(std.Islands[0].ID, nil); !slices.Equal(got, []string{"d", "x"}) {
		t.Errorf("Zeilen %q", got)
	}
}

// AC-05: Zwei Aufrufe liefern dasselbe, die Funktionen ändern weder Welt noch Zustand, der Aufrufer darf das Ergebnis ändern.
func TestStoryDeterministischUndOhneZustand(t *testing.T) {
	w := storyExample(t)
	before, _ := json.Marshal([]any{w.Story, w.Islands})
	first := w.StoryDefeated("a", nil)
	first[0] = "verändert"
	second := w.StoryDefeated("a", nil)
	third := w.StoryDefeated("a", nil)
	if !slices.Equal(second, third) || second[0] == "verändert" {
		t.Errorf("nicht deterministisch oder Alias: %q %q", second, third)
	}
	_ = w.StoryStart()
	_ = w.StoryBoss("a")
	_ = w.StoryVictory("b", []string{"a", "c"})
	if after, _ := json.Marshal([]any{w.Story, w.Islands}); string(after) != string(before) {
		t.Error("Welt verändert")
	}
}

// AC-05: Spielstand einer Insel mit Geschichte ist byte-gleich zu dem ohne; die Funktionen ändern Insel und Stufe nicht.
func TestStorySpielstandByteGleich(t *testing.T) {
	plain, _ := testIslands()
	withStory := plain
	withStory.Story = WorldStory{Lexicon: []string{"oru"}, Start: []string{"Oru KRONE MEER."}, End: []string{"Oru KRONE MEER."}}
	withStory.Islands = slices.Clone(plain.Islands)
	withStory.Islands[0].Story = IslandStory{Arrival: []string{"Oru KRONE MEER."}, Done: []string{"Oru KRONE MEER."}}
	save := func(w WorldDef) []byte {
		isl, err := CreateIslandIn(w, "story", []int{0, 1}, islandSpeed, CreateOptions{})
		if err != nil {
			t.Fatal(err)
		}
		AddIslandPlayer(isl, 0)
		AddIslandPlayer(isl, 0)
		stepSeconds(isl, 1)
		if len(w.Story.Start) > 0 {
			before, _ := json.Marshal(isl.ToSave("t"))
			_ = isl.StoryArrival()
			_ = isl.StoryBoss()
			_ = isl.StoryDefeated()
			_ = isl.StoryVictory()
			if after, _ := json.Marshal(isl.ToSave("t")); string(after) != string(before) {
				t.Error("Wendepunkt-Funktionen ändern den Zustand")
			}
		}
		raw, err := json.Marshal(isl.ToSave("t"))
		if err != nil {
			t.Fatal(err)
		}
		return raw
	}
	if a, b := save(plain), save(withStory); string(a) != string(b) {
		t.Error("Spielstand mit Geschichte weicht ab")
	}
}
