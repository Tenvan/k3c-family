package main

import (
	"bytes"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// teileQuelle: Wand und Gestell teilen sich die Farbe rope; Stufe 2 tauscht rope nur bei der Wand (Schlüssel „wand/rope“).
const teileQuelle = `{"id":"werkstatt","vorlage":{"motiv":"Stand","teile":["Wand","Gestell"]},"kacheln":{"b":2,"h":1},` +
	`"bausteine":[{"art":"wand","rechteck":[0,0,1,1],"farbe":"rope"},{"art":"gestell","rechteck":[1,0,1,1],"farbe":"rope"}],` +
	`"stufen":[{"stufe":2,"aenderung":"Wand aus Stein","farben":{"wand/rope":"dim-gray"}}]}`

func baueWerkstatt(t *testing.T, src string) (string, error) {
	t.Helper()
	dir := t.TempDir()
	if err := os.WriteFile(filepath.Join(dir, "werkstatt.json"), []byte(src), 0o644); err != nil {
		t.Fatal(err)
	}
	_, err := run(dir, "g/")
	return dir, err
}

// TestFarbtauschNachArt: „art/farbe“ tauscht nur Bausteine dieser Art, der engere Schlüssel gilt vor dem weiteren (GEB9.2).
func TestFarbtauschNachArt(t *testing.T) {
	b := building{ID: "x", Kacheln: kacheln{B: 1, H: 1}, Bausteine: []baustein{
		{Art: "wand", Rechteck: [4]float64{0, 0, 0.5, 1}, Farbe: "rope"},
		{Art: "gestell", Rechteck: [4]float64{0.5, 0, 0.5, 1}, Farbe: "rope"},
		{Art: "tresen", Rechteck: [4]float64{0, 0, 0.5, 0.5}, Farbe: "twine"},
	}, Stufen: []stufe{{Stufe: 2, Aenderung: "x", Farben: map[string]string{
		"rope": "heather", "wand/rope": "dim-gray", "twine": "light-steel",
	}}}}
	s := mitStufe(t, b, 2)
	if got := []string{s.Bausteine[0].Farbe, s.Bausteine[1].Farbe, s.Bausteine[2].Farbe}; got[0] != "dim-gray" || got[1] != "heather" || got[2] != "light-steel" {
		t.Errorf("Tausch nach Art: %v", got)
	}
}

// TestStufenNachArtFehler: Art/Farbe-Kombination, die in der Grundform fehlt, bricht mit Meldung ab.
func TestStufenNachArtFehler(t *testing.T) {
	src := strings.Replace(teileQuelle, `"wand/rope"`, `"gestell/twine"`, 1)
	if _, err := baueWerkstatt(t, src); err == nil || !strings.Contains(err.Error(), "Grundform") {
		t.Errorf("fehlende Art/Farbe: %v", err)
	}
}

// TestUebergangGrundform: Hub-Gebäude mit Stufen schreiben `<id>-<d>.png` byte-gleich zu Stufe 1 mit, andere Gebäude nicht.
func TestUebergangGrundform(t *testing.T) {
	dir, err := baueWerkstatt(t, teileQuelle)
	if err != nil {
		t.Fatal(err)
	}
	for _, d := range []string{"16", "32", "64", "128"} {
		a, errA := os.ReadFile(filepath.Join(dir, "werkstatt-"+d+".png"))
		b, errB := os.ReadFile(filepath.Join(dir, "werkstatt-1-"+d+".png"))
		if errA != nil || errB != nil || !bytes.Equal(a, b) {
			t.Errorf("Dichte %s: Grundform fehlt oder weicht von Stufe 1 ab (%v, %v)", d, errA, errB)
		}
	}
	dir, err = baueStufen(t, stufenQuelle) // Mauer steht nicht auf der Übergangsliste
	if err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(filepath.Join(dir, "mauer-32.png")); err == nil {
		t.Error("Gebäude außerhalb der Übergangsliste bekommt keine Grundform-Datei")
	}
}
