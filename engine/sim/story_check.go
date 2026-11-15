package sim

import (
	"fmt"
	"maps"
	"slices"
	"strconv"
	"strings"
	"unicode"
	"unicode/utf8"
)

// Prüfung der Geschichte beim Laden (docs/rules/welten.md § 3, B-965/R4), nach den Inseln, in der Reihenfolge lexicon,
// start, etappen (aufsteigend), end, bottles, dann je Insel arrival, boss, done; je Zeile: leer, Länge, Kunstwörter,
// Anzahl echter Wörter. Meldung: `story.<Feld> Zeile <n>: <Grund>`, bei Inseln mit `Insel <id>: ` davor.

const (
	storyMaxLines     = 2 // Zeilen je Eintrag
	storyMinRealWords = 2 // echte Wörter je Zeile
	storyMaxRealWords = 3
)

// checkStory prüft die Geschichte der Welt und ihrer Inseln; Inselfehler tragen die Insel-ID.
func checkStory(w WorldDef) error {
	lex, err := checkLexicon(w.Story.Lexicon)
	if err != nil {
		return err
	}
	if err := checkWorldEntries(w, lex); err != nil {
		return err
	}
	for _, d := range w.Islands {
		entries := []struct {
			field string
			lines []string
		}{{"arrival", d.Story.Arrival}, {"boss", d.Story.Boss}, {"done", d.Story.Done}}
		for _, e := range entries {
			if e.lines == nil {
				continue
			}
			if err := checkLines("story."+e.field, e.lines, lex); err != nil {
				return islandError{d.ID, err}
			}
		}
	}
	return nil
}

// checkWorldEntries prüft start, etappen, end und bottles der Welt.
func checkWorldEntries(w WorldDef, lex map[string]bool) error {
	s := w.Story
	if s.Start != nil {
		if err := checkLines("story.start", s.Start, lex); err != nil {
			return err
		}
	}
	if err := checkEtappen(w, lex); err != nil {
		return err
	}
	if s.End != nil {
		if err := checkLines("story.end", s.End, lex); err != nil {
			return err
		}
	}
	for i, b := range s.Bottles {
		if err := checkLines("story.bottles Eintrag "+strconv.Itoa(i+1), b, lex); err != nil {
			return err
		}
	}
	return nil
}

// checkEtappen prüft `etappen` aufsteigend: Schlüssel ist eine Etappen-Nummer, die Etappe hat eine Insel, dann die Zeilen.
func checkEtappen(w WorldDef, lex map[string]bool) error {
	keys := slices.Sorted(maps.Keys(w.Story.Etappen))
	slices.SortStableFunc(keys, func(a, b string) int {
		if len(a) != len(b) {
			return len(a) - len(b)
		}
		return strings.Compare(a, b)
	})
	for _, k := range keys {
		n, err := strconv.Atoi(k)
		if err != nil || n < 1 || strconv.Itoa(n) != k {
			return fmt.Errorf("story.etappen: Schlüssel %q ist keine Etappen-Nummer (Dezimalzahl ab 1, ohne führende Null)", k)
		}
		if !slices.ContainsFunc(w.Islands, func(d IslandDef) bool { return d.Etappe == n }) {
			return fmt.Errorf("story.etappen: Etappe %d hat keine Insel", n)
		}
		if err := checkLines("story.etappen."+k, w.Story.Etappen[k], lex); err != nil {
			return err
		}
	}
	return nil
}

// checkLexicon prüft den Wortschatz (Größe, Wörter klein, ohne Leerraum, einmalig) und liefert ihn als Menge.
func checkLexicon(words []string) (map[string]bool, error) {
	if len(words) > storyLimits.MaxLexicon {
		return nil, fmt.Errorf("story.lexicon: %d Wörter (höchstens %d)", len(words), storyLimits.MaxLexicon)
	}
	set := make(map[string]bool, len(words))
	for i, word := range words {
		switch {
		case word == "" || strings.ContainsFunc(word, unicode.IsSpace):
			return nil, fmt.Errorf("story.lexicon Wort %d: %q ist leer oder enthält Leerraum", i+1, word)
		case word != strings.ToLower(word):
			return nil, fmt.Errorf("story.lexicon Wort %d: %q ist nicht klein geschrieben", i+1, word)
		case set[word]:
			return nil, fmt.Errorf("story.lexicon Wort %d: %q doppelt", i+1, word)
		}
		set[word] = true
	}
	return set, nil
}

// checkLines prüft die Zeilen eines vorhandenen Eintrags: 1 bis 2 Zeilen, dann je Zeile checkLine.
func checkLines(label string, lines []string, lex map[string]bool) error {
	if len(lines) < 1 || len(lines) > storyMaxLines {
		return fmt.Errorf("%s: %d Zeilen (1 bis %d)", label, len(lines), storyMaxLines)
	}
	for i, line := range lines {
		if reason := checkLine(line, lex); reason != "" {
			return fmt.Errorf("%s Zeile %d: %s", label, i+1, reason)
		}
	}
	return nil
}

// checkLine nennt den ersten Verstoß der Zeile (leer = in Ordnung): leer, Länge, Kunstwort fehlt, Anzahl echter Wörter.
func checkLine(line string, lex map[string]bool) string {
	if strings.TrimSpace(line) == "" {
		return "leer"
	}
	if n := utf8.RuneCountInString(line); n > storyLimits.MaxLineChars {
		return fmt.Sprintf("%d Zeichen (höchstens %d)", n, storyLimits.MaxLineChars)
	}
	real := 0
	for _, word := range storyWords(line) {
		if isRealWord(word) {
			real++
		} else if !lex[strings.ToLower(word)] {
			return fmt.Sprintf("Wort %q fehlt im lexicon", strings.ToLower(word))
		}
	}
	if real < storyMinRealWords || real > storyMaxRealWords {
		noun := "echte Wörter"
		if real == 1 {
			noun = "echtes Wort"
		}
		return fmt.Sprintf("%d %s (%d bis %d)", real, noun, storyMinRealWords, storyMaxRealWords)
	}
	return ""
}

// storyWords trennt die Zeile am Leerraum und schneidet Satzzeichen am Rand ab; Stücke ohne Buchstaben (Gedankenstrich)
// sind keine Wörter.
func storyWords(line string) []string {
	var out []string
	for _, f := range strings.Fields(line) {
		if w := strings.TrimFunc(f, func(r rune) bool { return !unicode.IsLetter(r) }); w != "" {
			out = append(out, w)
		}
	}
	return out
}

// isRealWord: mindestens zwei Buchstaben, alle Großbuchstaben (`KRONE`); jedes andere Wort ist ein Kunstwort.
func isRealWord(word string) bool {
	letters := 0
	for _, r := range word {
		if !unicode.IsLetter(r) {
			continue
		}
		if !unicode.IsUpper(r) {
			return false
		}
		letters++
	}
	return letters >= storyMinRealWords
}
