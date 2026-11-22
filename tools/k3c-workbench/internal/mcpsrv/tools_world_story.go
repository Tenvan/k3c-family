package mcpsrv

import (
	"context"
	"fmt"
	"slices"
	"strconv"
	"strings"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/world"
)

// Geschichte einer Welt als MCP-Tool (B-975, docs/rules/welten.md § 2.2): world_story_set schreibt `story` der Welt und
// je Insel; world_get zeigt sie, world_check nennt fehlende Einträge als Hinweis, world_create nimmt sie mit auf. Die
// Regeln prüft die Engine (sim.ParseWorld), nicht die Workbench.

// worldStoryMissing ist der Schluss der Antwort von world_create ohne Geschichte (B-975/R4).
const worldStoryMissing = "Geschichte fehlt: world_story_set"

type worldIslandStoryIn struct {
	Arrival []string `json:"arrival,omitempty" jsonschema:"Zeilen bei der Ankunft auf der Insel (1 bis 2); [] entfernt, fehlt = unverändert"`
	Boss    []string `json:"boss,omitempty" jsonschema:"Zeilen, wenn der Endboss ausgelöst wird (1 bis 2); [] entfernt, fehlt = unverändert"`
	Done    []string `json:"done,omitempty" jsonschema:"Abschlusszeile nach dem Sieg über den Endboss (1 bis 2 Zeilen); [] entfernt, fehlt = unverändert"`
}

// worldStoryArgs ist die Geschichte als Argument: jedes Feld fehlt = unverändert, [] = entfernen.
type worldStoryArgs struct {
	Lexicon []string                      `json:"lexicon,omitempty" jsonschema:"Wortschatz der Kunstsprache: Kunstwörter klein, Apostroph und Bindestrich im Wort erlaubt; ersetzt die ganze Liste, [] entfernt"`
	Start   []string                      `json:"start,omitempty" jsonschema:"Zeilen beim Start einer neuen Kampagne (1 bis 2; je Zeile 2 bis 3 echte Wörter in GROSSBUCHSTABEN, jedes andere Wort aus lexicon); [] entfernt"`
	Etappen map[string][]string           `json:"etappen,omitempty" jsonschema:"Etappen-Nummer (z. B. 1) → Zeilen beim Abschluss der Etappe; nur genannte Etappen ändern sich, [] entfernt die Etappe"`
	End     []string                      `json:"end,omitempty" jsonschema:"Zeilen beim Kampagnensieg (1 bis 2); [] entfernt"`
	Bottles [][]string                    `json:"bottles,omitempty" jsonschema:"Einträge der Flaschenpost, je Eintrag 1 bis 2 Zeilen; ersetzt die ganze Liste, [] entfernt"`
	Islands map[string]worldIslandStoryIn `json:"islands,omitempty" jsonschema:"Insel-ID → {arrival, boss, done}; nur genannte Inseln und Einträge ändern sich"`
}

type worldStoryIn struct {
	ID string `json:"id" jsonschema:"Welt-ID"`
	worldStoryArgs
}

// patch übersetzt das Argument in die Änderung des Editors (nil bleibt nil: unverändert).
func (a worldStoryArgs) patch() world.StoryPatch {
	p := world.StoryPatch{Story: world.Story{Lexicon: a.Lexicon, Start: a.Start, Etappen: a.Etappen, End: a.End, Bottles: a.Bottles}}
	for id, isl := range a.Islands {
		if p.Islands == nil {
			p.Islands = map[string]world.IslandStory{}
		}
		p.Islands[id] = world.IslandStory{Arrival: isl.Arrival, Boss: isl.Boss, Done: isl.Done}
	}
	return p
}

func registerWorldStory(s *Server) {
	add(s, &mcp.Tool{
		Name: "world_story_set",
		Description: "Setzt die Geschichte einer Welt (story: lexicon, start, etappen, end, bottles) und ihrer Inseln (islands: arrival, boss, done) in einem Aufruf oder je Teil; " +
			"fehlt ein Feld, bleibt es, [] entfernt den Eintrag. Geschrieben wird nur, wenn die Prüfung wie beim Laden im Spiel besteht (docs/rules/welten.md § 2.2), " +
			"sonst nennt der Fehler Feld und Zeile; alle übrigen Felder der Datei bleiben byte-gleich. Die Antwort nennt gesetzte, entfernte und noch fehlende Einträge. " +
			"Nutze es bei: Wortschatz und Zeilen der Geschichte einer Welt schreiben oder ändern. Statt: story in der Welt-Datei von Hand bearbeiten.",
		Annotations: worldWrite(),
	}, func(ctx context.Context, in worldStoryIn) (string, error) {
		res, err := worldEditor(s.ws(ctx).Root).SetStory(ctx, in.ID, in.patch())
		if err != nil {
			return "", err
		}
		return worldDone(storyDone(in.ID, res), res.Warnings, nil)
	})
}

// worldCreate legt die Welt an (world_create), mit der Geschichte aus dem Argument; ohne Geschichte endet die Antwort
// mit dem Hinweis auf world_story_set.
func worldCreate(ctx context.Context, e world.Editor, in worldCreateIn) (string, error) {
	var story *world.StoryPatch
	if in.Story != nil {
		p := in.Story.patch()
		story = &p
	}
	warnings, err := e.CreateWithStory(ctx, in.ID, in.Name, in.Extends, story)
	text, err := worldDone(fmt.Sprintf("Welt %s angelegt (data/worlds/%s.json)", in.ID, in.ID), warnings, err)
	if err == nil && e.Load(in.ID).Story == nil {
		text += "\n" + worldStoryMissing
	}
	return text, err
}

// storyDone ist der Erfolgssatz von world_story_set: gesetzte, entfernte und noch fehlende Einträge.
func storyDone(id string, res world.StoryResult) string {
	var b strings.Builder
	fmt.Fprintf(&b, "Geschichte der Welt %s gespeichert", id)
	if len(res.Set) > 0 {
		b.WriteString("\nGesetzt: " + strings.Join(res.Set, ", "))
	}
	if len(res.Removed) > 0 {
		b.WriteString("\nEntfernt: " + strings.Join(res.Removed, ", "))
	}
	if len(res.Missing) > 0 {
		b.WriteString("\nFehlt noch: " + strings.Join(res.Missing, ", "))
	}
	return b.String()
}

// storyHints sind die Hinweise von world_check: je fehlendem Eintrag eine Zeile (keine Fehler, B-975/R3).
func storyHints(d world.Detail) string {
	var b strings.Builder
	for _, gap := range d.StoryGaps() {
		b.WriteString("\nHinweis: Geschichte fehlt (" + gap + ")")
	}
	return b.String()
}

// storyText ist der Abschnitt „Geschichte“ von world_get: Wortschatz und je Wendepunkt die Zeilen, fehlende als „fehlt“.
func storyText(d world.Detail) string {
	var b strings.Builder
	s := world.Story{}
	if d.Story != nil {
		s = *d.Story
	}
	entry := func(label string, lines []string) {
		fmt.Fprintf(&b, "  %s: %s\n", label, orMissing(strings.Join(lines, " | ")))
	}
	b.WriteString("Geschichte:\n")
	fmt.Fprintf(&b, "  lexicon (%d): %s\n", len(s.Lexicon), orMissing(strings.Join(s.Lexicon, ", ")))
	entry("start", s.Start)
	for _, k := range slices.Sorted(mapKeys(s.Etappen)) {
		entry("etappen."+strconv.Itoa(k), s.Etappen[strconv.Itoa(k)])
	}
	if len(s.Etappen) == 0 {
		entry("etappen", nil)
	}
	entry("end", s.End)
	for i, lines := range s.Bottles {
		entry("bottles["+strconv.Itoa(i+1)+"]", lines)
	}
	if len(s.Bottles) == 0 {
		entry("bottles", nil)
	}
	for _, isl := range d.Islands {
		is := world.IslandStory{}
		if isl.Story != nil {
			is = *isl.Story
		}
		entry(isl.ID+".arrival", is.Arrival)
		entry(isl.ID+".boss", is.Boss)
		entry(isl.ID+".done", is.Done)
	}
	return b.String()
}

func orMissing(s string) string {
	if s == "" {
		return "fehlt"
	}
	return s
}
