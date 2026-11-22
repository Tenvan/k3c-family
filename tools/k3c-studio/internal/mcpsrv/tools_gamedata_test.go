package mcpsrv

import (
	"strings"
	"testing"
)

// Spieldaten-Tools im Studio (WZ63.5, aus usage_test.go der Workbench): ohne Ordner keine Berichte, `..` abgelehnt.
func TestSpieldatenTools(t *testing.T) {
	t.Parallel()
	cs := connect(t, New(Config{Version: "test", Root: t.TempDir()}))
	if text, _ := callText(t, cs, "reports_list", nil); text != "keine Berichte" {
		t.Errorf("reports_list ohne Ordner: %q", text)
	}
	if text, _ := callText(t, cs, "report_read", map[string]any{"name": "../x.json"}); !strings.Contains(text, "abgelehnt") {
		t.Errorf("report_read mit ..: %q", text)
	}
}
