package assets

import "testing"

// B-677: Lizenz-Marke aus dem Lizenztext (frei, sa, pruefen, gesperrt).
func TestLicenseMark(t *testing.T) {
	for license, want := range map[string]string{
		"CC0 1.0":          MarkFree,
		"CC-BY 4.0":        MarkFree,
		"CC BY 3.0":        MarkFree,
		"OGA-BY 3.0":       MarkFree,
		selfMade:           MarkFree,
		"CC BY-SA 3.0":     MarkSA,
		"CC-BY-NC 4.0":     MarkBlocked,
		"CC BY-ND 4.0":     MarkBlocked,
		"CC-BY-NC-SA 4.0":  MarkBlocked,
		"Sampling+ 1.0":    MarkBlocked,
		"":                 MarkCheck,
		"unbekannt":        MarkCheck,
		"Eigene Bedingung": MarkCheck,
	} {
		if got := licenseMark(license); got != want {
			t.Errorf("%q: %s, erwartet %s", license, got, want)
		}
	}
}
