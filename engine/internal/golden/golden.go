// Package golden vergleicht Go-Ergebnisse Feld für Feld mit den Golden-Daten aus testdata/golden/ (B-043).
package golden

import (
	"encoding/json"
	"fmt"
	"slices"
	"strconv"
	"testing"
)

// Tree wandelt v über JSON in einen generischen Baum, wie ihn json.Unmarshal liefert (Zahlen als float64).
func Tree(t testing.TB, v any) any {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	var out any
	if err := json.Unmarshal(raw, &out); err != nil {
		t.Fatal(err)
	}
	return out
}

// Diff liefert den Pfad der ersten Abweichung zwischen zwei JSON-Bäumen ("" = gleich). Zahlen als float64.
func Diff(at string, want, got any) string {
	switch w := want.(type) {
	case map[string]any:
		g, ok := got.(map[string]any)
		if !ok {
			return at
		}
		keys := make([]string, 0, len(w)+len(g))
		for k := range w {
			keys = append(keys, k)
		}
		for k := range g {
			keys = append(keys, k)
		}
		slices.Sort(keys)
		for _, k := range slices.Compact(keys) {
			_, inW := w[k]
			_, inG := g[k]
			if inW != inG {
				return fmt.Sprintf("%s.%s (Schlüssel nur auf einer Seite)", at, k)
			}
			if d := Diff(at+"."+k, w[k], g[k]); d != "" {
				return d
			}
		}
	case []any:
		g, ok := got.([]any)
		if !ok || len(g) != len(w) {
			return fmt.Sprintf("%s (Länge)", at)
		}
		for i := range w {
			if d := Diff(at+"["+strconv.Itoa(i)+"]", w[i], g[i]); d != "" {
				return d
			}
		}
	default:
		if want != got {
			return fmt.Sprintf("%s = %v, erwartet %v", at, got, want)
		}
	}
	return ""
}
