package main

import (
	"context"
	"encoding/json"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/tools/k3c-workbench/internal/world"
)

// B-879: Der Welten-Editor erreicht die Welt über die Bindings wie die Oberfläche (RPC mit JSON-Argumenten). Die
// Engine-Seite (bin/k3c-sim world) ersetzt hier eine feste Antwort; ihre Rechnung prüft cmd/k3c-sim.
func TestWorldBindingsOverRPC(t *testing.T) {
	a := uiApp(t)
	src := []byte("{\n  \"id\": \"standard\",\n  \"name\": \"Standard\",\n  \"islands\": []\n}\n")
	path := filepath.Join(a.root, "data", "worlds", "standard.json")
	if err := os.MkdirAll(filepath.Dir(path), 0o750); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, src, 0o600); err != nil {
		t.Fatal(err)
	}
	var ops []string
	worldEngine = func(string) world.Engine {
		return func(_ context.Context, input []byte) (json.RawMessage, error) {
			var in struct{ Op string }
			_ = json.Unmarshal(input, &in)
			ops = append(ops, in.Op)
			if in.Op == "values" {
				return json.RawMessage(`{"extends":"","base":{"economy":{"purse":{"startGold":14}}},"overrides":{}}`), nil
			}
			return json.RawMessage(`{"warnings":[]}`), nil
		}
	}
	t.Cleanup(func() { worldEngine = world.SimExe })

	if code, out := uiCall(t, a, http.MethodPost, uiPrefix+"rpc/WorldList", "[]", nil); code != http.StatusOK || len(out["result"].([]any)) != 1 {
		t.Fatalf("WorldList: %d %v", code, out)
	}
	set := `["standard","economy",["purse","startGold"],77]`
	if code, out := uiCall(t, a, http.MethodPost, uiPrefix+"rpc/WorldOverrideSet", set, nil); code != http.StatusOK || out["result"].(map[string]any)["error"] != "" {
		t.Fatalf("WorldOverrideSet: %d %v", code, out)
	}
	if raw, _ := os.ReadFile(path); !strings.Contains(string(raw), `"startGold": 77`) {
		t.Fatalf("Override nicht in der Datei:\n%s", raw)
	}
	if !strings.Contains(strings.Join(ops, ","), "check") {
		t.Fatalf("ohne Prüfung geschrieben: %v", ops)
	}
	reset := `["standard","economy",["purse","startGold"],null]`
	if code, _ := uiCall(t, a, http.MethodPost, uiPrefix+"rpc/WorldOverrideSet", reset, nil); code != http.StatusOK {
		t.Fatalf("Zurücksetzen: %d", code)
	}
	if raw, _ := os.ReadFile(path); string(raw) != string(src) {
		t.Fatalf("nach Zurücksetzen nicht wie vorher:\n%s", raw)
	}
}
