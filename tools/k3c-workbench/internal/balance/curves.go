package balance

import (
	"cmp"
	"encoding/json"
	"fmt"
	"slices"
	"strings"

	"k3c/data"
)

// Grade (B-158, BAL3.3) für die Prüfung der Ziele; die Grad-Kurven selbst rechnet engine/balance (k3c-sim balance --curves).

// Grades liefert die Grade aus data/difficulty.json, nach Wellengröße sortiert (dev, easy, normal, hard, ultra).
func Grades() ([]string, error) {
	var d struct {
		Grades map[string]struct{ WaveSize float64 }
	}
	raw, err := data.Files.ReadFile("difficulty.json")
	if err == nil {
		err = json.Unmarshal(raw, &d)
	}
	if err != nil {
		return nil, fmt.Errorf("data/difficulty.json: %w", err)
	}
	names := make([]string, 0, len(d.Grades))
	for n := range d.Grades {
		names = append(names, n)
	}
	slices.SortFunc(names, func(a, b string) int {
		return cmp.Or(cmp.Compare(d.Grades[a].WaveSize, d.Grades[b].WaveSize), strings.Compare(a, b))
	})
	return names, nil
}

func medianOrNil(vals []float64) *float64 {
	if len(vals) == 0 {
		return nil
	}
	m := median(vals)
	return &m
}
