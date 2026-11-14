package mcpsrv

import (
	"strings"
	"testing"
)

// B-864: plan_index ist über MCP erreichbar, dry ist Standard, die Instruktionen nennen es.
func TestPlanIndexUeberMCP(t *testing.T) {
	t.Parallel()
	cs := connect(t, New(Config{Version: "test", Root: planRepo(t)}))
	text, isErr := callText(t, cs, "plan_index", map[string]any{})
	if isErr || !strings.Contains(text, "Checkout:") {
		t.Errorf("plan_index: %q (Fehler %v)", text, isErr)
	}
	if !strings.Contains(instructions, "`plan_index") {
		t.Error("Instruktionen nennen plan_index nicht")
	}
}
