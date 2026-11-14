package mcpsrv

import (
	"strings"
	"testing"
)

// B-783/AC-04: plan_waves und plan_queue sind über MCP erreichbar und stehen in den Instruktionen.
func TestPlanWavesUndQueueUeberMCP(t *testing.T) {
	t.Parallel()
	cs := connect(t, New(Config{Version: "test", Root: planRepo(t)}))
	for tool, want := range map[string]string{
		"plan_waves": "Arbeitswellen: 0 in Arbeit, 0 eingeplant in 1 Wellen, 0 in keiner Welle · 4 Plätze",
		"plan_queue": "Ralf-Queue: 0 Freigaben · 0 Entscheidungen · 0 Abnahmen an 0 Geräten",
	} {
		if text, isErr := callText(t, cs, tool, map[string]any{}); isErr || !strings.HasPrefix(text, want) {
			t.Errorf("%s: %q (Fehler %v)", tool, text, isErr)
		}
	}
	if text, isErr := callText(t, cs, "plan_waves", map[string]any{"slots": 9}); !isErr || !strings.Contains(text, "1 bis 4") {
		t.Errorf("slots 9: %q", text)
	}
	if text, isErr := callText(t, cs, "plan_check", map[string]any{"id": "B-404"}); !isErr || !strings.Contains(text, "nicht gefunden") {
		t.Errorf("plan_check B-404: %q", text)
	}
	for _, s := range []string{"`plan_waves", "`plan_queue", "`plan_check"} {
		if !strings.Contains(instructions, s) {
			t.Errorf("Instruktionen nennen %s nicht", s)
		}
	}
}
