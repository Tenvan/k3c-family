package mcpsrv

import (
	"strings"
	"testing"
)

// B-696/AC-11: plan_digest ist über MCP erreichbar, dry ist Standard, die Instruktionen nennen es.
func TestPlanDigestUeberMCP(t *testing.T) {
	t.Parallel()
	cs := connect(t, New(Config{Version: "test", Root: planRepo(t)}))
	if text, isErr := callText(t, cs, "plan_digest", map[string]any{}); isErr || !strings.HasPrefix(text, "Zusammenfassungen:") ||
		!strings.Contains(text, "Checkout:") {
		t.Errorf("plan_digest: %q (Fehler %v)", text, isErr)
	}
	if !strings.Contains(instructions, "`plan_digest") {
		t.Error("Instruktionen nennen plan_digest nicht")
	}
}
