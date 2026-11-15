package sim

import (
	"encoding/json"
	"fmt"

	"k3c/data"
)

// Geschichte je Welt und Insel (B-965, docs/rules/welten.md § 2.2): Kunstsprache mit eingestreuten echten Wörtern, feste
// Zeilen in der Welt-Datei. Alles ist optional; fehlt `story` oder ein Eintrag, gibt es dort keine Zeilen. Ein Eintrag
// ist eine Liste aus 1 bis 2 Zeilen. Prüfung: story_check.go, Wendepunkte (reine Funktionen): story_points.go.

// WorldStory ist `story` der Welt: Wortschatz der Kunstsprache und die Zeilen für Start, Etappen, Ende und Flaschenpost.
type WorldStory struct {
	Lexicon []string            `json:"lexicon"`
	Start   []string            `json:"start"`
	Etappen map[string][]string `json:"etappen"` // Etappen-Nummer (Dezimalzahl) → Zeilen beim Abschluss der Etappe
	End     []string            `json:"end"`
	Bottles [][]string          `json:"bottles"` // nur Format und Prüfung, genutzt erst von der Flaschenpost (B-971)
}

// IslandStory ist `story` der Insel: Ankunft, vor dem Endboss und Abschlusszeile nach dem Sieg über ihn.
type IslandStory struct {
	Arrival []string `json:"arrival"`
	Boss    []string `json:"boss"`
	Done    []string `json:"done"`
}

// StoryLimits sind die Obergrenzen aus data/story.json (vorläufig).
type StoryLimits struct {
	MaxLineChars int `json:"maxLineChars"`
	MaxLexicon   int `json:"maxLexicon"`
}

// storyLimits sind die Obergrenzen aus data/story.json. Fehlt die Datei (älterer data-Ordner neben der EXE) oder sind die
// Werte nicht positiv, startet das Spiel trotzdem; erst eine Welt mit Geschichte wird dann abgelehnt (checkStory).
var storyLimits = loadStoryLimits()

func loadStoryLimits() StoryLimits {
	var v StoryLimits
	_ = loadFrom(data.Files, "story.json", &v)
	return v
}

// usable: beide Obergrenzen sind gesetzt.
func (l StoryLimits) usable() bool { return l.MaxLineChars > 0 && l.MaxLexicon > 0 }

// StoryLimitsOf liefert die Obergrenzen der Geschichte (Zeilenlänge, Wortschatz) für Werkzeuge und Anzeige.
func StoryLimitsOf() StoryLimits { return storyLimits }

// decodeWorldStory liest `story` der Welt streng; fehlt das Feld, ist die Geschichte leer.
func decodeWorldStory(raw json.RawMessage) (WorldStory, error) {
	var s WorldStory
	if raw == nil {
		return s, nil
	}
	if err := strictDecode(raw, &s, "lexicon", "start", "etappen", "end", "bottles"); err != nil {
		return WorldStory{}, fmt.Errorf("story: %w", err)
	}
	return s, nil
}

// decodeIslandStory liest `story` der Insel streng; fehlt das Feld, ist die Geschichte leer.
func decodeIslandStory(raw json.RawMessage) (IslandStory, error) {
	var s IslandStory
	if raw == nil {
		return s, nil
	}
	if err := strictDecode(raw, &s, "arrival", "boss", "done"); err != nil {
		return IslandStory{}, fmt.Errorf("story: %w", err)
	}
	return s, nil
}
