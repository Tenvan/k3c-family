package mcpsrv

import (
	"context"
	"regexp"
	"strings"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// Workbench-Spec › MCP-Tools: Name <bereich>_<verb>, Beschreibung endet mit „Nutze es bei: … Statt: …“.
func TestToolRegelnDerSpec(t *testing.T) {
	cs := connect(t, New(Config{Version: "test"}))
	res, err := cs.ListTools(context.Background(), &mcp.ListToolsParams{})
	if err != nil {
		t.Fatal(err)
	}
	name := regexp.MustCompile(`^[a-z]+(_[a-z]+)+$`)
	for _, tool := range res.Tools {
		if !name.MatchString(tool.Name) {
			t.Errorf("%s: Name nicht <bereich>_<verb>", tool.Name)
		}
		at := strings.LastIndex(tool.Description, "Nutze es bei: ")
		if at < 0 || !strings.Contains(tool.Description[at:], " Statt: ") {
			t.Errorf("%s: Beschreibung ohne „Nutze es bei: … Statt: …“", tool.Name)
		}
	}
}

func TestNotifyUI(t *testing.T) {
	var got []Notice
	s := New(Config{Version: "test", OnNotify: func(n Notice) { got = append(got, n) }})
	cs := connect(t, s)
	if text, isErr := callText(t, cs, "notify_ui", map[string]any{"level": "warn", "title": "Fertig"}); isErr || text != "angezeigt: Fertig" || len(got) != 1 {
		t.Errorf("notify_ui: %q %v", text, got)
	}
	if text, isErr := callText(t, cs, "notify_ui", map[string]any{"level": "laut", "title": "x"}); !isErr || !strings.Contains(text, "erlaubt info, success, warn, error") {
		t.Errorf("ungültiges level: %q", text)
	}
}
