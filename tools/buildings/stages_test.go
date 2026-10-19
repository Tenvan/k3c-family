package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// stufenQuelle: 1 × 1 Kachel, Grundform ein Pfosten in rope, Stufe 2 tauscht rope → dim-gray und setzt eine Zinne oben drauf.
const stufenQuelle = `{"id":"mauer","vorlage":{"motiv":"Mauer","teile":["Pfosten"]},"kacheln":{"b":1,"h":1},` +
	`"bausteine":[{"art":"pfosten","rechteck":[0,0.5,1,0.5],"farbe":"rope"}],` +
	`"stufen":[{"stufe":2,"aenderung":"Stein statt Holz, Zinne oben","farben":{"rope":"dim-gray"},` +
	`"zusaetze":[{"art":"zinne","rechteck":[0,0,0.5,0.5],"farbe":"smokey-ash"}]}]}`

func baueStufen(t *testing.T, src string) (string, error) {
	t.Helper()
	dir := t.TempDir()
	if err := os.WriteFile(filepath.Join(dir, "mauer.json"), []byte(src), 0o644); err != nil {
		t.Fatal(err)
	}
	_, err := run(dir, "g/")
	return dir, err
}

// TestStufenRendern: je Stufe eigene PNGs `<id>-<stufe>-<d>.png`, Farbtausch und Zusätze wirken nur ab ihrer Stufe (GEB3 AC-05).
func TestStufenRendern(t *testing.T) {
	dir, err := baueStufen(t, stufenQuelle)
	if err != nil {
		t.Fatal(err)
	}
	for _, f := range []string{"mauer-1-32.png", "mauer-2-32.png", "mauer-1-16.png", "mauer-2-128.png"} {
		if _, err := os.Stat(filepath.Join(dir, f)); err != nil {
			t.Errorf("%s fehlt", f)
		}
	}
	if _, err := os.Stat(filepath.Join(dir, "mauer-32.png")); err == nil {
		t.Error("Gebäude mit Stufen schreibt keine Datei ohne Stufennummer")
	}
	var b building
	if err := json.Unmarshal([]byte(stufenQuelle), &b); err != nil {
		t.Fatal(err)
	}
	_, idx := palette()
	s1, s2 := mustRender(t, b, 32), mustRender(t, mitStufe(t, b, 2), 32)
	if s1.ColorIndexAt(4, 20) != idx["rope"] || s2.ColorIndexAt(4, 20) != idx["dim-gray"] {
		t.Errorf("Farbtausch: Stufe 1 %d, Stufe 2 %d", s1.ColorIndexAt(4, 20), s2.ColorIndexAt(4, 20))
	}
	if s1.ColorIndexAt(4, 4) != 0 || s2.ColorIndexAt(4, 4) != idx["smokey-ash"] {
		t.Error("Zusatz fehlt in Stufe 2 oder steht schon in Stufe 1")
	}
}

func mitStufe(t *testing.T, b building, n int) building {
	t.Helper()
	s, err := stufeBauen(b, n)
	if err != nil {
		t.Fatal(err)
	}
	return s
}

// TestStufenIndex: Index nennt alle Stufen mit Änderung und Dateien; Dateien des Eintrags zeigen Stufe 1.
func TestStufenIndex(t *testing.T) {
	dir, err := baueStufen(t, stufenQuelle)
	if err != nil {
		t.Fatal(err)
	}
	raw, err := os.ReadFile(filepath.Join(dir, "index.json"))
	if err != nil {
		t.Fatal(err)
	}
	var idx []eintrag
	if err := json.Unmarshal(raw, &idx); err != nil {
		t.Fatal(err)
	}
	e := idx[0]
	if len(e.Stufen) != 2 || e.Stufen[1].Stufe != 2 || e.Stufen[1].Aenderung == "" || e.Stufen[1].Dateien["32"] != "g/mauer-2-32.png" {
		t.Fatalf("Stufen im Index: %+v", e.Stufen)
	}
	if e.Dateien["32"] != "g/mauer-1-32.png" {
		t.Errorf("Dateien des Eintrags: %v", e.Dateien)
	}
}

// TestStufenFehler: ungültige Stufen brechen mit Meldung ab, der Ordner bleibt unverändert.
func TestStufenFehler(t *testing.T) {
	cases := map[string]struct{ alt, neu, want string }{
		"ohne Änderung":     {`"aenderung":"Stein statt Holz, Zinne oben"`, `"aenderung":""`, "aenderung"},
		"Lücke":             {`"stufe":2`, `"stufe":3`, "stufe"},
		"Stufe 1":           {`"stufe":2`, `"stufe":1`, "stufe"},
		"fremde Farbe":      {`"rope":"dim-gray"`, `"rope":"lila"`, "lila"},
		"fremder Schlüssel": {`"rope":"dim-gray"`, `"seil":"dim-gray"`, "seil"},
		"Zusatz außerhalb":  {`[0,0,0.5,0.5]`, `[0.75,0,0.5,0.5]`, "außerhalb"},
		"alt fehlt":         {`"rope":"dim-gray"`, `"black":"dim-gray"`, "Grundform"},
		"Zusatz fremd":      {`"farbe":"smokey-ash"`, `"farbe":"lila"`, "lila"},
	}
	for name, c := range cases {
		src := strings.Replace(stufenQuelle, c.alt, c.neu, 1)
		if src == stufenQuelle {
			t.Fatalf("%s: Ersetzung greift nicht", name)
		}
		if _, err := baueStufen(t, src); err == nil || !strings.Contains(err.Error(), c.want) {
			t.Errorf("%s: %v", name, err)
		}
	}
	doppelt := strings.Replace(stufenQuelle, `}]}]}`, `}]},{"stufe":2,"aenderung":"x"}]}`, 1)
	if _, err := baueStufen(t, doppelt); err == nil || !strings.Contains(err.Error(), "stufe") {
		t.Errorf("doppelte Stufe: %v", err)
	}
}

// TestFarbtauschEinmal: Tausch in einem Durchlauf über die Grundform, keine Kette (rope → dim-gray, dim-gray → rope).
func TestFarbtauschEinmal(t *testing.T) {
	b := building{ID: "x", Kacheln: kacheln{B: 1, H: 1}, Bausteine: []baustein{
		{Art: "a", Rechteck: [4]float64{0, 0, 0.5, 1}, Farbe: "rope"},
		{Art: "b", Rechteck: [4]float64{0.5, 0, 0.5, 1}, Farbe: "dim-gray"},
	}, Stufen: []stufe{{Stufe: 2, Aenderung: "getauscht", Farben: map[string]string{"rope": "dim-gray", "dim-gray": "rope"}}}}
	s := mitStufe(t, b, 2)
	if s.Bausteine[0].Farbe != "dim-gray" || s.Bausteine[1].Farbe != "rope" {
		t.Errorf("Tausch: %s, %s", s.Bausteine[0].Farbe, s.Bausteine[1].Farbe)
	}
}
