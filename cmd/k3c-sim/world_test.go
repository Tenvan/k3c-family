package main

import (
	"encoding/json"
	"strings"
	"testing"

	"k3c/data"
)

// worldReport liest das Feld report der einzigen result-Zeile.
func worldReport(t *testing.T, out string, dst any) {
	t.Helper()
	var ev struct {
		Event  string
		Report json.RawMessage
	}
	if err := json.Unmarshal([]byte(strings.TrimSpace(out)), &ev); err != nil || ev.Event != "result" {
		t.Fatalf("keine result-Zeile: %v %q", err, out)
	}
	if err := json.Unmarshal(ev.Report, dst); err != nil {
		t.Fatal(err)
	}
}

// B-879: level rechnet die Insel-Stufe der Standard-Welt mit ihrer Planung (Hafen in Tiefe 0).
func TestWorldLevel(t *testing.T) {
	code, out, errOut := call(t, `{"op":"level","world":"standard","island":"island1","stage":0,"seed":"k3c"}`, "world")
	if code != exitOK {
		t.Fatalf("Exit %d: %s", code, errOut)
	}
	var r struct {
		BiomeID  string            `json:"biomeId"`
		Chunks   []json.RawMessage `json:"chunks"`
		Planned  []json.RawMessage `json:"planned"`
		Warnings []string          `json:"warnings"`
	}
	worldReport(t, out, &r)
	if r.BiomeID != "forest" || len(r.Chunks) == 0 || len(r.Planned) == 0 || r.Warnings == nil {
		t.Fatalf("Level = %+v", r)
	}
	for _, bad := range []string{
		`{"op":"level","world":"standard","island":"x","stage":0,"seed":"k3c"}`,
		`{"op":"level","world":"standard","island":"island1","stage":99,"seed":"k3c"}`,
		`{"op":"level","world":"../x","island":"island1","stage":0,"seed":"k3c"}`,
	} {
		if code, _, _ := call(t, bad, "world"); code != exitFailed {
			t.Errorf("%s: Exit %d, erwartet %d", bad, code, exitFailed)
		}
	}
	if code, _, _ := call(t, `{"op":"fly"}`, "world"); code != exitUsage {
		t.Errorf("unbekannte op: Exit %d", code)
	}
}

// values liefert die Basis je Datei (buildings, economy …) und die Overrides der Welt.
func TestWorldValues(t *testing.T) {
	code, out, errOut := call(t, `{"op":"values","world":"standard"}`, "world")
	if code != exitOK {
		t.Fatalf("Exit %d: %s", code, errOut)
	}
	var r worldValuesReport
	worldReport(t, out, &r)
	if len(r.Base["economy"]) == 0 || len(r.Base["buildings"]) == 0 || r.Overrides == nil || r.Children == nil {
		t.Fatalf("Basis-Dateien fehlen: %v", len(r.Base))
	}
}

// check prüft einen Entwurf wie der Server: ein falscher Typ im Override ist ein Fehler, ein passender nicht.
func TestWorldCheck(t *testing.T) {
	raw, err := data.Files.ReadFile("worlds/standard.json")
	if err != nil {
		t.Fatal(err)
	}
	draft := func(ov string) string {
		text := strings.TrimRight(string(raw), "\r\n }") + `, "overrides": ` + ov + "}"
		in, _ := json.Marshal(map[string]any{"op": "check", "world": "standard", "draft": json.RawMessage(text)})
		return string(in)
	}
	if code, out, errOut := call(t, draft(`{"economy":{"purse":{"startGold":20}}}`), "world"); code != exitOK || !strings.Contains(out, `"warnings":[]`) {
		t.Fatalf("gültiger Entwurf: Exit %d %s %s", code, out, errOut)
	}
	if code, out, _ := call(t, draft(`{"economy":{"purse":{"startGold":"viel"}}}`), "world"); code != exitFailed || !strings.Contains(out, "startGold") {
		t.Fatalf("falscher Typ: Exit %d %s", code, out)
	}
}
