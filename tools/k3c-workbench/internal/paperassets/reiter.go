package paperassets

// Abhängigkeit Reiter → Reittier (B-706): Eine Spec, deren Animationen ein Reittier nutzen, hängt beim Prüfen und
// Rendern von dessen Spec ab (Regel reittier-unbekannt, tools/paper/rig/reiten.ts).

import (
	"os"
	"slices"

	"k3c/tools/k3c-workbench/internal/review"
)

// ReiterVon sind die IDs der Assets, die id als Reittier nutzen (sortiert); ihr Render ist nach einer Änderung am
// Reittier veraltet.
func ReiterVon(root, id string) []string {
	alle, err := Assets(root)
	if err != nil {
		return nil
	}
	var out []string
	for _, a := range alle {
		b, err := os.ReadFile(a.Abs(root))
		if err == nil && slices.Contains(review.Reittiere(b), id) {
			out = append(out, a.ID)
		}
	}
	slices.Sort(out)
	return out
}

// reittierStand ist der Datei-Stand der Reittier-Specs von pfad (leer ohne Reittier); ein geändertes oder
// gelöschtes Reittier macht den Gültigkeits-Eintrag des Reiters ungültig.
func reittierStand(root, pfad string, nachID map[string]Asset) string {
	b, err := os.ReadFile(pfad)
	if err != nil {
		return ""
	}
	stand := ""
	for _, id := range review.Reittiere(b) {
		r, ok := nachID[id]
		if !ok {
			stand += "|" + id + ":–"
			continue
		}
		stand += "|" + id + ":" + dateiStand(r.Abs(root))
	}
	return stand
}
