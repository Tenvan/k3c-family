package sim

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"io/fs"
	"strconv"
	"strings"
	"testing"
	"testing/fstest"

	"k3c/data"
)

// WZ18.2 (B-488/AC-02): Welt-Dateien laden und prüfen (docs/rules/welten.md § 2–4).

// testIsland ist eine gültige Insel; fields ergänzt oder ersetzt Felder (JSON-Fragment mit führendem Komma), ohne
// doppelte Schlüssel.
func testIsland(id, fields string) string {
	v := map[string]json.RawMessage{"id": json.RawMessage(strconv.Quote(id)), "name": json.RawMessage(`"Insel"`),
		"depths": json.RawMessage(`[0,1,2,3,4]`), "depthScaling": json.RawMessage(`{"hp":1,"damage":1,"speed":1}`)}
	if fields != "" {
		if err := json.Unmarshal([]byte("{"+fields[1:]+"}"), &v); err != nil {
			panic(err)
		}
	}
	raw, err := json.Marshal(v)
	if err != nil {
		panic(err)
	}
	return string(raw)
}

// testWorld ist die Welt-Datei „test“ mit den Inseln islands (JSON-Objekte, durch Komma getrennt).
func testWorld(islands string) []byte {
	return []byte(`{"id":"test","name":"Test","islands":[` + islands + `]}`)
}

// parseTestWorld liest eine Test-Welt mit einer Insel „a“ (fields wie bei testIsland).
func parseTestWorld(fields string) (WorldDef, error) {
	return ParseWorld(testWorld(testIsland("a", fields)), "test")
}

func TestWeltAusMapFSLaden(t *testing.T) {
	raw := `{"id":"nordmeer","name":"Nordmeer","notes":"Entwurf","islands":[` + testIsland("island1", `,"plannedChunks":{}`) + `,` +
		testIsland("eisinsel", `,"depths":[0,4],"chunkCounts":{"0":24},"plannedChunks":{"0":[{"slot":-3,"kind":"clearing","seed":"lichtung"}]}`) + `]}`
	w, err := LoadWorld(fstest.MapFS{"nordmeer.json": {Data: []byte(raw)}}, "nordmeer")
	if err != nil {
		t.Fatal(err)
	}
	if w.ID != "nordmeer" || w.Name != "Nordmeer" || w.Notes != "Entwurf" || len(w.Islands) != 2 {
		t.Fatalf("Welt %+v", w)
	}
	if d := w.Islands[1]; d.ID != "eisinsel" || d.countAt(0) != 24 || d.plannedAt(0)[0].Seed != "lichtung" {
		t.Errorf("Insel 2: %+v", d)
	}
	if _, err := LoadWorld(fstest.MapFS{}, "nordmeer"); err == nil || !strings.Contains(err.Error(), "nordmeer") {
		t.Errorf("fehlende Datei: %v", err)
	}
}

// AC-02: Jede eingebettete Welt-Datei (heute nur standard.json) besteht die Prüfung; die Standard-Welt ist Voreinstellung.
func TestEingebetteteWeltenGueltig(t *testing.T) {
	entries, err := fs.ReadDir(data.Files, "worlds")
	if err != nil {
		t.Fatal(err)
	}
	if len(entries) != 1 || entries[0].Name() != "standard.json" {
		t.Errorf("eingebettet: %v, erwartet nur standard.json", entries)
	}
	worlds, _ := fs.Sub(data.Files, "worlds")
	for _, e := range entries {
		id := strings.TrimSuffix(e.Name(), ".json")
		if _, err := LoadWorld(worlds, id); err != nil {
			t.Errorf("%s: %v", e.Name(), err)
		}
	}
	w := StandardWorld()
	if w.ID != StandardWorldID || w.Name != "Standard" || len(w.Islands) == 0 || w.Islands[0].ID != "island1" {
		t.Errorf("Standard-Welt %+v", w)
	}
}

// AC-02: Die Liste islands von standard.json ist byte-gleich zu data/islands.json auf origin/develop vor WZ18.2
// (Hash belegt per `git show 9c54de70:data/islands.json`, Zeilenenden auf LF normiert).
func TestStandardWeltGleichBisherigenInseln(t *testing.T) {
	raw, err := fs.ReadFile(data.Files, "worlds/standard.json")
	if err != nil {
		t.Fatal(err)
	}
	var head struct{ Islands json.RawMessage }
	if err := json.Unmarshal(raw, &head); err != nil {
		t.Fatal(err)
	}
	sum := sha256.Sum256(bytes.ReplaceAll(head.Islands, []byte("\r\n"), []byte("\n")))
	if got := hex.EncodeToString(sum[:]); got != "1c1532aced9af2edf23976472480183f2211fb060e5d82ab6fd224fb6d3812a2" {
		t.Errorf("islands geändert (sha256 %s)", got)
	}
}

// AC-02: Jeder Fehlerfall wird abgelehnt; die Meldung nennt Datei (Welt), Insel, Tiefe und Platz, soweit vorhanden.
func TestWeltFehlerNennenOrt(t *testing.T) {
	const file = "data/worlds/test.json: "
	island := func(fields string) []byte { return testWorld(testIsland("a", fields)) }
	cases := map[string]struct {
		raw  []byte
		want []string
	}{
		"kein JSON":          {[]byte(`{"id":`), []string{file + "kein gültiges JSON"}},
		"ohne Insel":         {testWorld(``), []string{file + "keine Insel"}},
		"doppelte Insel":     {testWorld(testIsland("a", "") + "," + testIsland("a", "")), []string{file + "Insel a: doppelt"}},
		"Insel-ID ungültig":  {testWorld(testIsland("Insel 2", "")), []string{file + `Insel "Insel 2": id ungültig`}},
		"Name leer":          {island(`,"name":""`), []string{file + "Insel a: name"}},
		"ohne Tiefen":        {island(`,"depths":[]`), []string{file + "Insel a: ohne Tiefen"}},
		"Tiefe doppelt":      {island(`,"depths":[0,0,4]`), []string{file + "Insel a: doppelte Tiefe 0"}},
		"unbekannte Tiefe":   {island(`,"depths":[4,9]`), []string{file + "Insel a: unbekannte Tiefe 9"}},
		"Tiefe vor Endboss":  {island(`,"depths":[0,7]`), []string{file + "Insel a: unbekannte Tiefe 7"}},
		"kein Endboss":       {island(`,"depths":[0,1]`), []string{file + "Insel a: kein Endboss in der tiefsten Stufe 1"}},
		"Faktor 0":           {island(`,"depthScaling":{"hp":0,"damage":1,"speed":1}`), []string{file + "Insel a: depthScaling braucht Faktoren > 0"}},
		"Faktor negativ":     {island(`,"depthScaling":{"hp":1,"damage":-1,"speed":1}`), []string{file + "Insel a: depthScaling braucht Faktoren > 0"}},
		"Plan mit Platz":     {island(`,"plannedChunks":{"0":[{"slot":9,"kind":"forest"}]}`), []string{file + "Insel a: plannedChunks Tiefe 0", "Platz +9"}},
		"Plan Tiefe fremd":   {island(`,"plannedChunks":{"7":[]}`), []string{file + `Insel a: plannedChunks: Tiefe "7"`}},
		"Chunk-Anzahl":       {island(`,"chunkCounts":{"0":18}`), []string{file + "Insel a: chunkCounts Tiefe 0", "18", "16, 24, 40, 64, 100, 160"}},
		"Feld der Welt":      {[]byte(`{"id":"test","name":"Test","size":3,"islands":[` + testIsland("a", "") + `]}`), []string{file + `unbekanntes Feld "size"`}},
		"Feld der Insel":     {island(`,"size":3`), []string{file + `Insel a: unbekanntes Feld "size"`}},
		"Feld der Faktoren":  {island(`,"depthScaling":{"hp":1,"damage":1,"speed":1,"armor":1}`), []string{file + `Insel a: depthScaling: unbekanntes Feld "armor"`}},
		"Feld der Planung":   {island(`,"plannedChunks":{"0":[{"slot":1,"kind":"forest","object":[]}]}`), []string{file, "object"}},
		"Pflicht Welt-Name":  {[]byte(`{"id":"test","islands":[` + testIsland("a", "") + `]}`), []string{file + "name fehlt (Pflichtfeld)"}},
		"Welt-Name leer":     {[]byte(`{"id":"test","name":"","islands":[` + testIsland("a", "") + `]}`), []string{file + "name"}},
		"Pflicht Inseln":     {[]byte(`{"id":"test","name":"Test"}`), []string{file + "islands fehlt (Pflichtfeld)"}},
		"Pflicht Faktor":     {testWorld(`{"id":"a","name":"Insel","depths":[4],"depthScaling":{"hp":1,"damage":1}}`), []string{file + "Insel a: depthScaling.speed fehlt (Pflichtfeld)"}},
		"Pflicht Tiefen":     {testWorld(`{"id":"a","name":"Insel","depthScaling":{"hp":1,"damage":1,"speed":1}}`), []string{file + "Insel a: depths fehlt (Pflichtfeld)"}},
		"Pflicht Insel-ID":   {testWorld(`{"name":"Insel","depths":[4],"depthScaling":{"hp":1,"damage":1,"speed":1}}`), []string{file + "Insel Nr. 1: id fehlt (Pflichtfeld)"}},
		"ID passt nicht":     {[]byte(`{"id":"nord","name":"Test","islands":[` + testIsland("a", "") + `]}`), []string{file + `id "nord" passt nicht zum Dateinamen`}},
		"Feld groß":          {island(`,"ID":"b"`), []string{file + `Insel a: unbekanntes Feld "ID"`}},
		"Faktor doppelt":     {testWorld(`{"id":"a","name":"Insel","depths":[4],"depthScaling":{"hp":1,"damage":1,"speed":1,"hp":0}}`), []string{file + `Insel a: depthScaling: doppeltes Feld "hp"`}},
		"Insel-Feld doppelt": {testWorld(`{"id":"a","id":"b","name":"Insel","depths":[4],"depthScaling":{"hp":1,"damage":1,"speed":1}}`), []string{file, `doppeltes Feld "id"`}},
		"Daten nach Ende":    {append(testWorld(testIsland("a", "")), '}'), []string{file + "kein gültiges JSON"}},
		"Pflicht Welt-ID":    {[]byte(`{"name":"Test","islands":[` + testIsland("a", "") + `]}`), []string{file + "id fehlt (Pflichtfeld)"}},
	}
	for name, c := range cases {
		w, err := ParseWorld(c.raw, "test")
		if err == nil || w.ID != "" {
			t.Errorf("%s: kein Fehler (%+v)", name, w)
			continue
		}
		for _, want := range c.want {
			if !strings.Contains(err.Error(), want) {
				t.Errorf("%s: Fehler %q nennt %q nicht", name, err, want)
			}
		}
	}
}

// Die Welt-ID muss zum Muster passen, sonst wird nichts gelesen (kein Pfadzugriff über die ID).
func TestWeltIDMuster(t *testing.T) {
	fsys := fstest.MapFS{"../x.json": {Data: testWorld(testIsland("a", ""))}}
	for _, id := range []string{"", "Nord", "../x", "-a", "a_b", strings.Repeat("a", 33)} {
		if _, err := LoadWorld(fsys, id); err == nil || !strings.Contains(err.Error(), "Welt-ID") {
			t.Errorf("%q: Fehler %v", id, err)
		}
		if _, err := ParseWorld(testWorld(testIsland("a", "")), id); err == nil {
			t.Errorf("ParseWorld %q angenommen", id)
		}
	}
	if _, err := parseTestWorld(""); err != nil {
		t.Errorf("gültige Test-Welt abgelehnt: %v", err)
	}
}
