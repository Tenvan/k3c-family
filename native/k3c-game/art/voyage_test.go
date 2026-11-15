package art

import (
	"slices"
	"testing"

	"k3c/data"
)

// GS2.4: Die Werte der Schiffsfahrt stehen in data/native.json › voyage; Datei und Standard im Code sind gleich, und
// kein Wert der Datei ist unzulässig.
func TestVoyageWieDaten(t *testing.T) {
	raw, err := data.Files.ReadFile("native.json")
	if err != nil {
		t.Fatal(err)
	}
	v, bad, err := ParseVoyage(raw)
	if err != nil || len(bad) != 0 {
		t.Fatalf("native.json: Fehler %v, unzulässig %v", err, bad)
	}
	if v != DefaultVoyage() {
		t.Errorf("native.json › voyage %+v, Standard %+v", v, DefaultVoyage())
	}
	if Voyage() != v {
		t.Errorf("Voyage() %+v, Datei %+v", Voyage(), v)
	}
	if n := testing.AllocsPerRun(20, func() { _ = Voyage() }); n != 0 {
		t.Errorf("Voyage legt %v Objekte an", n)
	}
}

// Fehlende Werte behalten den Standard, unzulässige fallen auf ihn zurück und werden genannt; 0 ist bei Bogen,
// Schaukeln und Welle erlaubt. Kaputtes JSON liefert den Standard mit Fehler.
func TestParseVoyage(t *testing.T) {
	def := DefaultVoyage()
	if v, bad, err := ParseVoyage([]byte(`{"stepBudgetMs": 4}`)); err != nil || len(bad) != 0 || v != def {
		t.Errorf("ohne Block: %+v, %v, %v", v, bad, err)
	}
	v, bad, err := ParseVoyage([]byte(`{"voyage": {"seconds": 5, "arcPx": 0, "bobPx": -1, "flagAt": 2, "scale": 0}}`))
	want := def
	want.Seconds, want.Arc = 5, 0
	if err != nil || v != want || !slices.Equal(bad, []string{"scale", "bobPx", "flagAt"}) {
		t.Errorf("teils unzulässig: %+v (soll %+v), %v, %v", v, want, bad, err)
	}
	if v, _, err := ParseVoyage([]byte(`{"voyage": 3`)); err == nil || v != def {
		t.Errorf("kaputtes JSON: %+v, %v", v, err)
	}
}
