package balance

import (
	"fmt"
	"slices"
	"strings"
)

// Stellschrauben nach Wirkung: Derselbe Wert in data/ taucht bei mehreren verletzten Zielen als Hebel auf. Die
// Zusammenfassung nennt ihn einmal und listet, welche Verletzungen er (laut Richtwerten) verbessern soll.

// LeverGroup ist ein Wert in data/ mit allen verletzten Zielen, die er verbessern soll. Conflict: Ein anderes verletztes
// Ziel braucht denselben Wert in die Gegenrichtung; dann hilft der Hebel dem einen und schadet vermutlich dem anderen.
type LeverGroup struct {
	Path      string    `json:"path"`
	Direction string    `json:"direction"`
	Plan      LeverPlan `json:"plan"`
	Targets   []string  `json:"targets"`
	Conflict  string    `json:"conflict"`
	Text      string    `json:"text"`
}

// leverGroups fasst die Hebel der verletzten Ziele nach Pfad und Richtung zusammen; die meisten Verletzungen zuerst.
func leverGroups(in Insights) []LeverGroup {
	out := []LeverGroup{}
	index := map[string]int{}
	dirs := map[string][]string{} // Pfad → Richtungen, die vorkommen
	for _, t := range worst(in) {
		for i, l := range t.Levers {
			if i >= len(t.Plans) {
				break
			}
			key := l.Path + "|" + l.Direction
			j, seen := index[key]
			if !seen {
				j = len(out)
				index[key] = j
				out = append(out, LeverGroup{Path: l.Path, Direction: l.Direction, Plan: t.Plans[i]})
				dirs[l.Path] = append(dirs[l.Path], l.Direction)
			}
			out[j].Targets = append(out[j].Targets, t.label())
		}
	}
	for i := range out {
		g := &out[i]
		if len(dirs[g.Path]) > 1 {
			g.Conflict = "Achtung: ein anderes verletztes Ziel braucht diesen Wert in die Gegenrichtung."
		}
		g.Text = g.text()
	}
	slices.SortStableFunc(out, func(a, b LeverGroup) int { return len(b.Targets) - len(a.Targets) })
	return out
}

func (g LeverGroup) text() string {
	way := "erhöhen"
	if g.Direction == "-" {
		way = "senken"
	}
	s := fmt.Sprintf("%s %s – soll %d Verletzung(en) verbessern: %s. %s", g.Plan.Label, "`"+g.Path+"`", len(g.Targets),
		strings.Join(g.Targets, "; "), g.Plan.Short)
	if !strings.Contains(g.Plan.Label, way) {
		s = "(" + way + ") " + s
	}
	if g.Conflict != "" {
		s += " " + g.Conflict
	}
	return s
}

// recommendations ist ein Kopierblock mit dem Startvorschlag jeder Stellschraube als Wert-Variante (`datei.json:a.b.c=wert`),
// je Zeile mit Kommentar (Ziele, Bereich). Ein Wert in Gegenrichtung zu einem Vorschlag mit mehr Verletzungen entfällt,
// ebenso ein Hebel ohne seriösen Startwert (der steht im Kommentar-Block darunter).
func recommendations(groups []LeverGroup) (Command, bool) {
	var lines, skipped []string
	taken := map[string]bool{}
	for _, g := range groups {
		switch {
		case taken[g.Path]:
			skipped = append(skipped, fmt.Sprintf("# %s: Gegenrichtung zu einem Vorschlag mit mehr Verletzungen, nicht mitgenommen.", g.Path))
		case !g.Plan.Known:
			skipped = append(skipped, fmt.Sprintf("# %s: Startwert nicht seriös abschätzbar, Wert von Hand wählen.", g.Path))
		default:
			taken[g.Path] = true
			lines = append(lines, fmt.Sprintf("%s=%s  # %s; aktuell %s, Bereich %s … %s; verbessert: %s", g.Path, g.Plan.Start,
				g.Plan.Label, g.Plan.Current, g.Plan.Min, g.Plan.Max, strings.Join(g.Targets, "; ")))
		}
	}
	if len(lines) == 0 {
		return Command{}, false
	}
	text := "# Empfehlungen aller Stellschrauben (Startvorschläge) als Wert-Varianten, ein Lauf mit allen zugleich:\n" +
		strings.Join(append(lines, skipped...), "\n")
	return Command{Label: "Alle Empfehlungen als ein Block", Where: WhereChat, Text: text}, true
}
