package planning

import (
	"fmt"
	"os"
	"regexp"
	"sort"
	"strings"
	"unicode"
)

// Ähnlichkeits-Hinweis beim Anlegen eines Tickets (B-696 Teil A): Wortüberlappung des Titels mit den Titeln im Index
// (offen und Archiv), ohne KI und deterministisch. Der Hinweis blockiert nie.

const (
	simMinLen    = 4   // kürzere Wörter zählen nicht
	simStem      = 6   // Wörter werden auf ihre ersten Zeichen gekürzt (Seite/Seiten, Variante/Varianten)
	simThreshold = 0.4 // Jaccard-Maß ab dem ein Titel als ähnlich gilt
	simMax       = 5   // höchstens so viele Treffer
)

// indexFiles sind die Index-Dateien des Backlogs: offen und eingeplant, erledigt und verworfen (B-696 Teil B).
var indexFiles = []string{"backlog/README.md", archiveIndex}

var (
	reIndexRow = regexp.MustCompile(`^\| \[(B-\d{3})\]\([^)]*\) \|[^|]*\|[^|]*\|[^|]*\| (\S+) \|[^|]*\| (.*) \|$`)
	stopWords  = map[string]bool{}
)

func init() {
	for _, w := range strings.Fields("aber alle allen auch beide beiden beim bleibt bleiben dann dass denn diese dieser " +
		"dieses durch eine einem einen einer eines gibt haben hier immer jede jedem jeden jeder jedes kann kein keine " +
		"keinen mehr mehrere muss nach nicht noch nur ohne schon sein seine sich sind soll statt steht über unter " +
		"vom wenn werden wieder wird wurde zeigt zwei drei damit ihre ihren können läuft laufen") {
		stopWords[w] = true
	}
}

// titleWords sind die gekürzten Wörter eines Titels ohne Stoppwörter und kurze Wörter.
func titleWords(title string) map[string]bool {
	out := map[string]bool{}
	for _, w := range strings.FieldsFunc(strings.ToLower(title), func(r rune) bool { return !unicode.IsLetter(r) && !unicode.IsDigit(r) }) {
		rs := []rune(w)
		if len(rs) < simMinLen || stopWords[w] {
			continue
		}
		out[string(rs[:min(simStem, len(rs))])] = true
	}
	return out
}

// jaccard ist |A∩B| / |A∪B|.
func jaccard(a, b map[string]bool) float64 {
	if len(a) == 0 || len(b) == 0 {
		return 0
	}
	both := 0
	for w := range a {
		if b[w] {
			both++
		}
	}
	return float64(both) / float64(len(a)+len(b)-both)
}

type simHit struct {
	nr, where, title string
	score            float64
}

// similarNotes liefert den Hinweis auf ähnliche Tickets; ist ein Index unlesbar, kommt nur ein Hinweis darauf.
func similarNotes(c *changeSet, title string) (notes []string) {
	defer func() {
		if r := recover(); r != nil {
			notes = []string{fmt.Sprintf("Hinweis: Ähnlichkeitsprüfung abgebrochen (%v)", r)}
		}
	}()
	want := titleWords(title)
	seen := map[string]bool{}
	var hits []simHit
	for _, rel := range indexFiles {
		text, err := c.read(rel)
		if err != nil {
			if rel == archiveIndex && !exists(c.root, rel) {
				continue
			}
			return []string{"Hinweis: Ähnlichkeitsprüfung übersprungen (docs/" + rel + " nicht lesbar)"}
		}
		for _, l := range splitLines(text) {
			m := reIndexRow.FindStringSubmatch(l)
			if m == nil || seen[m[1]] {
				continue
			}
			seen[m[1]] = true
			if s := jaccard(want, titleWords(m[3])); s >= simThreshold {
				hits = append(hits, simHit{nr: m[1], where: archiveLabel(m[2]), title: m[3], score: s})
			}
		}
	}
	return simLines(hits)
}

// exists: die Datei unter docs/ ist vorhanden (auch unlesbar).
func exists(root, rel string) bool {
	_, err := os.Stat(docsPath(root, rel))
	return err == nil
}

func archiveLabel(status string) string {
	if status == "erledigt" || status == "verworfen" {
		return "archiv, " + status
	}
	return status
}

// simLines ordnet nach Ähnlichkeit, dann Nummer, und kürzt auf simMax.
func simLines(hits []simHit) []string {
	if len(hits) == 0 {
		return nil
	}
	sort.Slice(hits, func(i, j int) bool {
		if hits[i].score != hits[j].score {
			return hits[i].score > hits[j].score
		}
		return hits[i].nr < hits[j].nr
	})
	out := []string{"Hinweis: ähnliche Tickets (Titel, nur Hinweis):"}
	for _, h := range hits[:min(simMax, len(hits))] {
		out = append(out, fmt.Sprintf("  %s (%s) · %s", h.nr, h.where, h.title))
	}
	return out
}
