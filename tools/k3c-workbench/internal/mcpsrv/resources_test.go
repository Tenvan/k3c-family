package mcpsrv

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"k3c/tools/k3c-workbench/internal/resmon"
)

func TestWorkbenchResourcesZeigtPufferUndVorgaenge(t *testing.T) {
	s := New(Config{Version: "test", Root: t.TempDir(), Resources: resmon.New(resmon.Config{})})
	s.console.Add("check:task:test", "stdout", "zeile")
	defer s.cfg.Resources.Track("tool:x")()
	text, err := s.workbenchResources(context.Background(), struct{}{})
	if err != nil {
		t.Fatal(err)
	}
	for _, want := range []string{"heap live", "Konsole-Zeilen 1", "Prüfläufe 0", "Testläufe 0", "MCP-Sessions 0"} {
		if !strings.Contains(text, want) {
			t.Errorf("Bericht ohne %q:\n%s", want, text)
		}
	}
}

func TestWorkbenchResourcesOhneMessgeraetMeldetEsKlar(t *testing.T) {
	s := New(Config{Version: "test", Root: t.TempDir()})
	if _, err := s.workbenchResources(context.Background(), struct{}{}); err == nil {
		t.Fatal("ohne Messgerät muss das Tool einen Fehler melden")
	}
}

func TestPprofNurFuerLoopbackHosts(t *testing.T) {
	mux := http.NewServeMux()
	mountPprof(mux)
	for host, want := range map[string]int{"127.0.0.1:5180": http.StatusOK, "localhost:5180": http.StatusOK, "evil.example:5180": http.StatusForbidden} {
		req := httptest.NewRequest(http.MethodGet, "/debug/pprof/cmdline", nil)
		req.Host = host
		rec := httptest.NewRecorder()
		mux.ServeHTTP(rec, req)
		if rec.Code != want {
			t.Errorf("Host %s: %d, will %d", host, rec.Code, want)
		}
	}
}
