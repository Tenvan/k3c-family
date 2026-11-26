package mcpsrv

import (
	"strings"
	"testing"
)

// B-1266: world_set trägt endBoss und skins bis world_get durch. Welt-Events folgen mit B-1260 (k3c-data kennt welt-events nicht).
func TestWorldToolsEndBossSkinsRoundtrip(t *testing.T) {
	t.Parallel()
	cs := connect(t, New(Config{Version: "test", Root: worldRepo(t)}))
	call := func(tool string, args map[string]any) string {
		t.Helper()
		text, isErr := callText(t, cs, tool, args)
		if isErr {
			t.Fatalf("%s: %s", tool, text)
		}
		return text
	}
	call("world_create", map[string]any{"id": "hanse", "name": "Hanse", "extends": "standard"})
	call("world_set", map[string]any{"id": "hanse", "patch": map[string]any{
		"skins":   map[string]any{"gegner/wolf": "gegner/ninja"},
		"islands": []any{map[string]any{"id": "island1", "endBoss": "forestChieftain"}},
	}})
	got := call("world_get", map[string]any{"id": "hanse"})
	for _, want := range []string{`"endBoss": "forestChieftain"`, `"gegner/ninja"`} {
		if !strings.Contains(got, want) {
			t.Errorf("world_get ohne %s:\n%s", want, got)
		}
	}
}
