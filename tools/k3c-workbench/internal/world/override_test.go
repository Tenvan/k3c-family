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
	e.SetOverride(ctx, "standard", "economy", []string{"purse", "maxGold"}, json.RawMessage("40.50"))
	if text := readWorld(t, e, "standard"); !strings.Contains(text, `"startGold": 99`) || !strings.Contains(text, `"maxGold": 40.50`) {
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
