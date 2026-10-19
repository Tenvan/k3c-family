package table

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

const root = "../../.."

func read(t *testing.T, rel string) string {
	t.Helper()
	b, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(rel)))
	if err != nil {
		t.Fatal(err)
	}
	return string(b)
}

// WZ15.2/AC-01: Der Block in § 3 ist genau der aus data/hub.json erzeugte.
func TestBlockDerRegelnIstGleichDerJSON(t *testing.T) {
	gleich, ist, soll, err := Gegenprobe(read(t, "docs/rules/materialien-gebaeude.md"), []byte(read(t, "data/hub.json")))
	if err != nil {
		t.Fatal(err)
	}
	if !gleich {
		t.Fatalf("Block weicht ab (task rules:update):\n--- ist\n%s\n--- soll\n%s", ist, soll)
	}
}

func TestBlockInhaltFuerDieHeutigenFuenfStufen(t *testing.T) {
	block, err := Block([]byte(read(t, "data/hub.json")))
	if err != nil {
		t.Fatal(err)
	}
	for _, want := range []string{
		"| 1 | Holz | Werkstatt −16; Farm −52, +52 | Linie 1: Mauer ±44, Turm ±36 (fest) | Kaufen +16, Verkaufen +20 (nur Tiefe 0) |",
		"| 2 | Stein | Treppe hoch +16 (ab Tiefe 1); Treppe runter +8 (nur mit tieferer Stufe); Taverne −32; Kaserne +24; Lager −26 (je Insel) | Linie 2: Mauer ±64, Turm ±56, Tor ±68 (0…+4 je Seed); Tor Linie 1 ±48 | – |",
		"| 5 | Kristall | – | Linie 5: Mauer ±124, Turm ±116, Tor ±128 (0…+4 je Seed) | – |",
	} {
		if !strings.Contains(block, want) {
			t.Errorf("Zeile fehlt: %s\n%s", want, block)
		}
	}
	if strings.Count(block, "\n| ") != 5+1 { // Kopf und fünf Stufen
		t.Errorf("Zeilen: %d\n%s", strings.Count(block, "\n| "), block)
	}
}

// WZ15.2/AC-03: Mehr Hub-Stufen ohne Umbau, keine feste 5.
func TestSiebenStufenFixture(t *testing.T) {
	block, err := Block([]byte(read(t, "testdata/hub/hub7.json")))
	if err != nil {
		t.Fatal(err)
	}
	for _, want := range []string{"| 6 | Stein | Schmiede +28 | Linie 6: Mauer ±112, Turm ±108, Tor ±116 (0…+0 je Seed) | – |", "| 7 | Eisen | Rüstkammer +40 | Linie 7: Mauer ±124"} {
		if !strings.Contains(block, want) {
			t.Errorf("Zeile fehlt: %s\n%s", want, block)
		}
	}
	if got := strings.Count(block, "\n| "); got != 7+1 {
		t.Errorf("Zeilen: %d", got)
	}
}

func TestErsetzenLaesstDenRestByteGleich(t *testing.T) {
	doc := "Kopf\n" + Start + "\nalt\n" + End + "\nFuß\n"
	got, err := Ersetzen(doc, Start+"\nneu\n"+End)
	if err != nil {
		t.Fatal(err)
	}
	if got != "Kopf\n"+Start+"\nneu\n"+End+"\nFuß\n" {
		t.Fatalf("Ergebnis: %q", got)
	}
	crlf, err := Ersetzen(strings.ReplaceAll(doc, "\n", "\r\n"), Start+"\nneu\n"+End)
	if err != nil || crlf != strings.ReplaceAll("Kopf\n"+Start+"\nneu\n"+End+"\nFuß\n", "\n", "\r\n") {
		t.Fatalf("CRLF: %q %v", crlf, err)
	}
}

func TestErsetzenFehlerfaelle(t *testing.T) {
	for name, doc := range map[string]string{
		"ohne Marken":    "nichts",
		"nur Start":      Start + "\n",
		"Start doppelt":  Start + End + Start,
		"Ende vor Start": End + "\n" + Start,
		"Ende doppelt":   Start + End + End,
	} {
		if _, err := Ersetzen(doc, "x"); err == nil {
			t.Errorf("%s: Fehler erwartet", name)
		}
	}
}

func TestBlockFehler(t *testing.T) {
	if _, err := Block([]byte("{")); err == nil {
		t.Error("kaputtes JSON")
	}
	if _, err := Block([]byte(`{"levels":[]}`)); err == nil {
		t.Error("ohne levels")
	}
}

func TestBlockKommazahlenUndStufenGrenzen(t *testing.T) {
	doc := func(site string, levels string) []byte {
		return []byte(`{"levels":` + levels + `,"sites":[` + site + `],"wallLines":{"wallUnits":[44.5],"fixedLines":1,"jitterOutwardUnits":4,"towerInsetUnits":8,"gateOutsetUnits":4,"gateHubLevel":1},"merchant":{"buyOffsetUnits":16,"sellOffsetUnits":20,"onlyDepth":0}}`)
	}
	one := `[{"cost":{}}]`
	block, err := Block(doc(`{"kind":"workshop","offsetUnits":-16.5}`, one))
	if err != nil || !strings.Contains(block, "Werkstatt −16.5") || !strings.Contains(block, "Mauer ±44.5, Turm ±36.5, Tor ±48.5") {
		t.Fatalf("Kommazahlen: %v\n%s", err, block)
	}
	// ein fehlender hubLevel heißt Stufe 1 wie in der Engine
	if block, err = Block(doc(`{"kind":"farm","offsetUnits":52}`, one)); err != nil || !strings.Contains(block, "Farm +52") {
		t.Fatalf("hubLevel fehlt: %v\n%s", err, block)
	}
	if _, err = Block(doc(`{"kind":"farm","offsetUnits":52,"hubLevel":3}`, one)); err == nil {
		t.Error("hubLevel außerhalb von levels muss ein Fehler sein")
	}
	if _, err = Block(doc(`{"kind":"farm","offsetUnits":52,"hubLevel":1}`, `[{"cost":{}},{"cost":{"stone":1,"iron":2}}]`)); err == nil {
		t.Error("mehrere Materialien müssen ein Fehler sein")
	}
	if _, err = Block(doc(`{"kind":"farm","offsetUnits":52,"hubLevel":1}`, `[{"cost":{}},{"cost":{"mithril":1}}]`)); err == nil {
		t.Error("unbekanntes Material muss ein Fehler sein")
	}
}

func TestErsetzenNormalisiertDenBlock(t *testing.T) {
	doc := Start + "\r\nalt\r\n" + End + "\r\n"
	got, err := Ersetzen(doc, Start+"\r\nneu\r\n"+End)
	if err != nil || got != Start+"\r\nneu\r\n"+End+"\r\n" {
		t.Fatalf("%q %v", got, err)
	}
}
