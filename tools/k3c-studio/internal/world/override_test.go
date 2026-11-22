package world

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"strings"
	"testing"
)

func readWorld(t *testing.T, e Editor, id string) string {
	t.Helper()
	raw, err := os.ReadFile(e.file(id))
	if err != nil {
		t.Fatal(err)
	}
	return string(raw)
}

var ctx = context.Background()

// B-879/AC-02: Ein übersteuerter Wert landet in `overrides` der Welt-Datei, die übrigen Bytes bleiben.
func TestSetOverrideWritesWorldFile(t *testing.T) {
	e := fixture(t)
	v := e.SetOverride(ctx, "standard", "economy", []string{"purse", "startGold"}, json.RawMessage("99"))
	if v.Error != "" {
		t.Fatalf("Error = %q", v.Error)
	}
	if got := entry(t, v, "economy", "purse", "startGold").Override; string(got) != "99" {
		t.Fatalf("Override = %s", got)
	}
	text := readWorld(t, e, "standard")
	if !strings.HasPrefix(text, strings.TrimSuffix(standardFixture, "\n}\n")) {
		t.Fatalf("Kopf und Inseln nicht byte-gleich:\n%s", text)
	}
	var f struct {
		Overrides map[string]map[string]map[string]float64 `json:"overrides"`
	}
	if err := json.Unmarshal([]byte(text), &f); err != nil || f.Overrides["economy"]["purse"]["startGold"] != 99 {
		t.Fatalf("overrides = %+v (%v)", f.Overrides, err)
	}
	// Ein zweites Feld kommt dazu, das erste bleibt; Kommazahlen bleiben, wie sie eingegeben wurden.
	e.SetOverride(ctx, "standard", "economy", []string{"payIntervalSeconds"}, json.RawMessage("0.50"))
	if text := readWorld(t, e, "standard"); !strings.Contains(text, `"startGold": 99`) || !strings.Contains(text, `"payIntervalSeconds": 0.50`) {
		t.Fatalf("zwei Felder erwartet:\n%s", text)
	}
}

// Zurücksetzen (null) und ein Wert gleich dem Basiswert entfernen den Override; ohne Overrides entfällt der Schlüssel.
func TestSetOverrideResetRestoresFile(t *testing.T) {
	e := fixture(t)
	path := []string{"purse", "startGold"}
	e.SetOverride(ctx, "standard", "economy", path, json.RawMessage("99"))
	if v := e.SetOverride(ctx, "standard", "economy", path, json.RawMessage("14")); v.Error != "" {
		t.Fatal(v.Error)
	}
	if got := readWorld(t, e, "standard"); got != standardFixture {
		t.Fatalf("Datei nicht wie vorher:\n%s", got)
	}
	e.SetOverride(ctx, "standard", "economy", path, json.RawMessage("5"))
	if v := e.SetOverride(ctx, "standard", "economy", path, nil); v.Error != "" {
		t.Fatal(v.Error)
	}
	if got := readWorld(t, e, "standard"); got != standardFixture {
		t.Fatalf("Datei nach Zurücksetzen nicht wie vorher:\n%s", got)
	}
}

// Ein abgelehnter Wert, ein unbekanntes Feld, eine ungültige ID und eine fehlende Engine lassen die Datei unverändert.
func TestSetOverrideRejectsInvalid(t *testing.T) {
	e := fixture(t)
	path := []string{"purse", "startGold"}
	cases := map[string]ValuesView{
		"falscher Typ":   e.SetOverride(ctx, "standard", "economy", path, json.RawMessage(`"viel"`)),
		"kein JSON":      e.SetOverride(ctx, "standard", "economy", path, json.RawMessage(`{`)),
		"Feld unbekannt": e.SetOverride(ctx, "standard", "economy", []string{"gibt", "esnicht"}, json.RawMessage("1")),
		"ID":             e.SetOverride(ctx, "../standard", "economy", path, json.RawMessage("1")),
	}
	broken := e
	broken.Engine = func(context.Context, []byte) (json.RawMessage, error) { return nil, errors.New("bin/k3c-sim fehlt") }
	cases["ohne Engine"] = broken.SetOverride(ctx, "standard", "economy", path, json.RawMessage("1"))
	for name, v := range cases {
		if v.Error == "" {
			t.Errorf("%s ohne Fehler", name)
		}
	}
	if got := readWorld(t, e, "standard"); got != standardFixture {
		t.Fatalf("Datei geändert:\n%s", got)
	}
}

func TestPatchOverridesKeepsCRLF(t *testing.T) {
	raw := []byte("{\r\n  \"id\": \"x\"\r\n}\r\n")
	out, err := patchOverrides(raw, map[string]json.RawMessage{"economy": json.RawMessage(`{"a":1}`)})
	if err != nil {
		t.Fatal(err)
	}
	if strings.Contains(strings.ReplaceAll(string(out), "\r\n", ""), "\n") {
		t.Fatalf("LF in CRLF-Datei: %q", out)
	}
	back, err := patchOverrides(out, map[string]json.RawMessage{})
	if err != nil || string(back) != string(raw) {
		t.Fatalf("Entfernen = %q (%v)", back, err)
	}
}

// B-986/AC-01: Unbekannte Datei → die Meldung nennt alle erlaubten Dateien, gelesen aus der Engine.
func TestSetOverrideUnknownFileListsAllowedFiles(t *testing.T) {
	e := fixture(t)
	v := e.SetOverride(ctx, "standard", "paper", []string{"x"}, json.RawMessage("1"))
	if !strings.Contains(v.Error, `unbekannte Datei "paper"`) {
		t.Fatalf("Error = %q", v.Error)
	}
	// Die Engine nennt dieselbe Liste: Wächst sie dort, folgt die Meldung ohne Änderung des Studios.
	var raw struct {
		Warnings []string `json:"warnings"`
	}
	draft := strings.Replace(standardFixture, "\n}\n", ",\n  \"overrides\": {\"paper\": {}}\n}\n", 1)
	err := e.call(ctx, map[string]any{"op": "check", "world": "standard", "draft": json.RawMessage(draft)}, &raw)
	if err == nil {
		t.Fatal("Engine nahm paper an")
	}
	list := err.Error()[strings.Index(err.Error(), "(erlaubt: ")+len("(erlaubt: "):]
	list = strings.TrimSuffix(strings.TrimSpace(list), ")")
	if !strings.Contains(v.Error, "(erlaubt: "+list+")") {
		t.Fatalf("Liste der Engine %q fehlt in %q", list, v.Error)
	}
	if got := readWorld(t, e, "standard"); got != standardFixture {
		t.Fatal("Datei geändert")
	}
}

// B-986/AC-02: Gesperrtes Feld → Grund und Verweis; unbekanntes Feld → Felder der Ebene (höchstens 12, danach …).
func TestSetOverrideLockedAndUnknownFieldMessages(t *testing.T) {
	e := fixture(t)
	for _, c := range []struct {
		file string
		path []string
	}{{"hub", []string{"sitePlacement"}}, {"economy", []string{"payRangeUnits"}}} {
		v := e.SetOverride(ctx, "standard", c.file, c.path, json.RawMessage("1"))
		if !strings.Contains(v.Error, "nicht übersteuerbar") || !strings.Contains(v.Error, "welten.md § 1.3") {
			t.Errorf("%s: Error = %q", c.file, v.Error)
		}
	}
	v := e.SetOverride(ctx, "standard", "economy", []string{"purse", "gibtesnicht"}, json.RawMessage("1"))
	if !strings.Contains(v.Error, `unbekanntes Feld "gibtesnicht" in economy › purse (Felder: startGold`) {
		t.Errorf("Error = %q", v.Error)
	}
	v = e.SetOverride(ctx, "standard", "buildings", []string{"gibtesnicht"}, json.RawMessage("1"))
	if !strings.Contains(v.Error, "unbekanntes Feld") || !strings.Contains(v.Error, ", …)") {
		t.Errorf("mehr als 12 Felder ohne …: %q", v.Error)
	}
	v = e.SetOverride(ctx, "standard", "economy", []string{"purse"}, json.RawMessage("1"))
	if !strings.Contains(v.Error, "Objekt oder eine Liste") || !strings.Contains(v.Error, "startGold") {
		t.Errorf("Objekt: Error = %q", v.Error)
	}
	if got := readWorld(t, e, "standard"); got != standardFixture {
		t.Fatal("Datei geändert")
	}
}

// Review: Auch ein tieferer Pfad unter einem gesperrten Feld nennt den Grund.
func TestSetOverrideDeepPathUnderLockedField(t *testing.T) {
	e := fixture(t)
	v := e.SetOverride(ctx, "standard", "hub", []string{"sitePlacement", "x", "y"}, json.RawMessage("1"))
	if !strings.Contains(v.Error, "welten.md § 1.3") {
		t.Fatalf("Error = %q", v.Error)
	}
}
