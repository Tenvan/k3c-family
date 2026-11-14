package mcpsrv

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// B-831/AC-01: dev:test und task:check:dev laufen über Task (Frontend-Build vorher), mit einem Limit für den vollen
// Lauf; ohne Frontend-Abhängigkeiten bricht check_run mit dem Hinweis task install ab, ohne einen Prozess zu starten.
func TestCheckRunWorkbenchZiele(t *testing.T) {
	t.Parallel()
	cases := []struct{ target, pattern, cmd string }{
		{"dev:test", "", "task k3c-workbench:gotest"},
		{"dev:test", "TestX", "task k3c-workbench:gotest GO_TEST_FLAGS=-run='TestX'"},
		{"task:check:dev", "", "task check:dev"},
	}
	for _, c := range cases {
		var got runSpec
		s, started := fakeServer(t, func(spec runSpec) runResult { got = spec; return runResult{} })
		if _, err := s.checkRun(context.Background(), checkIn{Target: c.target, Pattern: c.pattern}); err == nil ||
			!strings.Contains(err.Error(), "task install") || started.Load() != 0 {
			t.Fatalf("%s ohne node_modules: %v (gestartet %d)", c.target, err, started.Load())
		}
		if err := os.MkdirAll(filepath.Join(s.cfg.Root, filepath.FromSlash(devFrontendDeps)), 0o755); err != nil {
			t.Fatal(err)
		}
		if _, err := s.checkRun(context.Background(), checkIn{Target: c.target, Pattern: c.pattern}); err != nil {
			t.Fatalf("%s: %v", c.target, err)
		}
		if strings.Join(got.args, " ") != c.cmd || got.timeout < 20*time.Minute || got.dir != filepath.Clean(s.cfg.Root) {
			t.Errorf("%s: Befehl %v, Limit %v, Ordner %q", c.target, got.args, got.timeout, got.dir)
		}
	}
	if !strings.Contains(checkRunDescription, "task:check:dev") {
		t.Error("Beschreibung von check_run nennt task:check:dev nicht")
	}
}
