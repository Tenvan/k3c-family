package main

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

const vorlageJSON = `"vorlage":{"motiv":"Probe","teile":["Pfosten"]},`

// schreibe legt `<id>.json` in einem Temp-Ordner an und liefert den Ordner.
func schreibe(t *testing.T, id, klasse, rest string) string {
	t.Helper()
	dir := t.TempDir()
	body := `{"id":"` + id + `",` + klasse + vorlageJSON + rest
	if err := os.WriteFile(filepath.Join(dir, id+".json"), []byte(body), 0o644); err != nil {
		t.Fatal(err)
	}
	return dir
}

// TestWiederholungKopien: `anzahl` Kopien im Abstand `schritt`, die erste ist das Original (GEB7 AC-01).
func TestWiederholungKopien(t *testing.T) {
	list, err := aufloesen("probe", []baustein{
		{Art: "stamm", Rechteck: [4]float64{1, 0, 0.5, 1}, Farbe: "rope", Wiederholung: &wiederholung{Schritt: 2.5, Anzahl: 3}},
		{Art: "boden", Rechteck: [4]float64{0, 1, 8, 1}, Farbe: "dell"},
	}, master)
	if err != nil {
		t.Fatal(err)
	}
	if len(list) != 4 {
		t.Fatalf("Bausteine: %d, erwartet 4", len(list))
	}
	for i, x := range []float64{1, 3.5, 6} {
		if list[i].Rechteck[0] != x || list[i].Wiederholung != nil {
			t.Errorf("Kopie %d: x=%g wiederholung=%v, erwartet x=%g ohne wiederholung", i, list[i].Rechteck[0], list[i].Wiederholung, x)
		}
	}
	if list[3].Art != "boden" {
		t.Errorf("Reihenfolge: letzter Baustein %q", list[3].Art)
	}
}

// TestWiederholungFehler: Schritt 0, negativ, nicht im 1/32-Raster, Anzahl unter 2 und Kopien außerhalb brechen ab und nennen Feld und Baustein.
func TestWiederholungFehler(t *testing.T) {
	cases := map[string]struct{ w, want string }{
		"schritt 0":         {`{"schritt":0,"anzahl":3}`, "schritt"},
		"schritt negativ":   {`{"schritt":-1,"anzahl":3}`, "schritt"},
		"schritt im Raster": {`{"schritt":0.01,"anzahl":3}`, "schritt"},
		"anzahl 1":          {`{"schritt":1,"anzahl":1}`, "anzahl"},
		"überlauf":          {`{"schritt":3,"anzahl":3}`, "außerhalb"},
	}
	for name, c := range cases {
		dir := schreibe(t, "probe", "", `"kacheln":{"b":6,"h":1},"bausteine":[{"art":"stamm","rechteck":[0,0,1,1],"farbe":"rope","wiederholung":`+c.w+`}]}`)
		_, err := run(dir, "")
		if err == nil || !strings.Contains(err.Error(), c.want) || !strings.Contains(err.Error(), "stamm") {
			t.Errorf("%s: %v", name, err)
		}
	}
}

// TestWiederholungImBild: die Kopien erscheinen an den Pixeln der Dichte 32.
func TestWiederholungImBild(t *testing.T) {
	dir := schreibe(t, "probe", "", `"kacheln":{"b":4,"h":1},"bausteine":[{"art":"stamm","rechteck":[0,0,0.5,1],"farbe":"rope","wiederholung":{"schritt":1.5,"anzahl":3}}]}`)
	if _, err := run(dir, ""); err != nil {
		t.Fatal(err)
	}
	img := decodePNG(t, filepath.Join(dir, "probe-32.png"))
	for x, want := range map[int]bool{0: true, 15: true, 16: false, 47: false, 48: true, 63: true, 64: false, 95: false, 96: true, 111: true, 112: false} {
		if got := img.ColorIndexAt(x, 8) != 0; got != want {
			t.Errorf("x=%d: deckend=%v, erwartet %v", x, got, want)
		}
	}
}

// TestHintergrundNurSechzehnUndZweiunddreissig: Klasse `hintergrund` schreibt kein 64/128 und keinen Eintrag dafür (GEB7 AC-01).
func TestHintergrundNurSechzehnUndZweiunddreissig(t *testing.T) {
	rest := `"kacheln":{"b":2,"h":1},"bausteine":[{"art":"boden","rechteck":[0,0.5,2,0.5],"farbe":"dell"}]}`
	dir := schreibe(t, "hg-probe", `"klasse":"hintergrund",`, rest)
	if _, err := run(dir, ""); err != nil {
		t.Fatal(err)
	}
	for _, d := range []string{"16", "32"} {
		if _, err := os.Stat(filepath.Join(dir, "hg-probe-"+d+".png")); err != nil {
			t.Errorf("Dichte %s fehlt: %v", d, err)
		}
	}
	for _, d := range []string{"64", "128"} {
		if _, err := os.Stat(filepath.Join(dir, "hg-probe-"+d+".png")); err == nil {
			t.Errorf("Dichte %s darf es für hintergrund nicht geben", d)
		}
	}
	idx, _ := os.ReadFile(filepath.Join(dir, "index.json"))
	if strings.Contains(string(idx), `"64"`) || strings.Contains(string(idx), `"128"`) {
		t.Error("index.json nennt 64 oder 128 für hintergrund")
	}
	other := schreibe(t, "probe", "", rest)
	if _, err := run(other, ""); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(filepath.Join(other, "probe-128.png")); err != nil {
		t.Errorf("andere Klassen behalten 128: %v", err)
	}
}
