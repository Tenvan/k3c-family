package mcpsrv

import (
	"context"
	"errors"
	"strings"
	"sync/atomic"
	"testing"
	"time"

	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/services"
)

// serviceServer baut Server und Controller mit gestelltem Start; healthy steuert die Prüfung, busy den Port.
func serviceServer(t *testing.T, healthy, busy *atomic.Bool) (*Server, *services.Controller) {
	t.Helper()
	store := console.New(0, nil)
	ctl := services.New([]services.Service{
		{Name: "Vite", Command: []string{"x"}, Port: 5173, Health: "http"},
		{Name: "Spielserver", Command: []string{"x"}, Port: 8080, Health: "http"},
	}, services.Options{
		Console: store,
		Start: func(svc services.Service, _ string, out func(string, string)) (services.Process, error) {
			out("stdout", svc.Name+": Zeile 1")
			out("stderr", svc.Name+": Port-Problem")
			return &stubProc{done: make(chan struct{})}, nil
		},
		Check: func(context.Context, services.Service) error {
			if healthy.Load() {
				return nil
			}
			return errors.New("antwortet nicht")
		},
		Listen:    func(context.Context, int) (int, bool) { return 4711, busy.Load() },
		Sample:    func(context.Context, int) (services.Metrics, error) { return services.Metrics{}, nil },
		KillPID:   func(int) error { busy.Store(false); return nil },
		StartPoll: 2 * time.Millisecond, StartTimeout: 60 * time.Millisecond, WatchEvery: time.Hour,
	})
	t.Cleanup(func() { ctl.StopAll(context.Background()) })
	return New(Config{Version: "test", Root: t.TempDir(), Console: store, Services: ctl}), ctl
}

func TestSvcToolsStartStatusStop(t *testing.T) {
	var healthy, busy atomic.Bool
	healthy.Store(true)
	s, _ := serviceServer(t, &healthy, &busy)
	cs := connect(t, s)
	text, isErr := callText(t, cs, "svc_start", map[string]any{"service": "Vite"})
	if isErr || !strings.HasPrefix(text, "Vite · läuft · Port 5173 · PID 42 · < 1 min · ") {
		t.Errorf("svc_start: %q", text)
	}
	if text, isErr = callText(t, cs, "svc_start", map[string]any{"service": "Vite"}); !isErr || !strings.Contains(text, "läuft bereits") {
		t.Errorf("zweiter Start: %q", text)
	}
	text, _ = callText(t, cs, "svc_status", nil)
	if lines := strings.Split(text, "\n"); len(lines) != 2 || !strings.HasPrefix(lines[1], "Spielserver · gestoppt · Port 8080") {
		t.Errorf("svc_status: %q", text)
	}
	if text, _ = callText(t, cs, "workbench_status", nil); !strings.Contains(text, "Dienste: Vite läuft (5173) · Spielserver gestoppt (8080)") {
		t.Errorf("workbench_status: %q", text)
	}
	if text, isErr = callText(t, cs, "svc_restart", map[string]any{"service": "Vite"}); isErr || !strings.HasPrefix(text, "Vite · läuft") {
		t.Errorf("svc_restart: %q", text)
	}
	if text, isErr = callText(t, cs, "svc_stop", map[string]any{"service": "Vite"}); isErr || !strings.HasPrefix(text, "Vite · gestoppt · Port 5173") {
		t.Errorf("svc_stop: %q", text)
	}
	if text, isErr = callText(t, cs, "svc_stop", map[string]any{"service": "Gibtsnicht"}); !isErr || !strings.Contains(text, "gültig: [Vite Spielserver]") {
		t.Errorf("unbekannter Dienst: %q", text)
	}
}

func TestSvcToolsFehlerMitKonsoleUndForce(t *testing.T) {
	var healthy, busy atomic.Bool
	s, ctl := serviceServer(t, &healthy, &busy)
	cs := connect(t, s)
	text, isErr := callText(t, cs, "svc_start", map[string]any{"service": "Vite"})
	if !isErr || !strings.Contains(text, "nicht gesund") || !strings.Contains(text, "letzte Zeilen:\nVite: Zeile 1\nVite: Port-Problem") {
		t.Errorf("Fehler mit Konsole: %q", text)
	}
	healthy.Store(true)
	busy.Store(true)
	ctl.Adopt(context.Background())
	if st := ctl.Statuses()[1]; st.State != services.Adopted {
		t.Fatalf("Spielserver nicht übernommen: %+v", st)
	}
	if text, isErr = callText(t, cs, "svc_stop", map[string]any{"service": "Spielserver"}); !isErr || !strings.Contains(text, "nur mit force") {
		t.Errorf("Stopp ohne force: %q", text)
	}
	if text, isErr = callText(t, cs, "svc_stop", map[string]any{"service": "Spielserver", "force": true}); isErr || !strings.HasPrefix(text, "Spielserver · gestoppt") {
		t.Errorf("Stopp mit force: %q", text)
	}
}

func TestSvcToolsOhneDienste(t *testing.T) {
	s := New(Config{Version: "test", ServicesErr: errors.New("services.json: unknown field \"x\"")})
	if _, err := s.svcStatus(context.Background(), struct{}{}); err == nil || !strings.Contains(err.Error(), "services.json nicht geladen") {
		t.Errorf("kaputte Konfiguration: %v", err)
	}
	if _, err := New(Config{Version: "test"}).svcStart(context.Background(), serviceIn{Service: "Vite"}); err == nil ||
		err.Error() != "keine Dienste konfiguriert" {
		t.Errorf("ohne Controller: %v", err)
	}
}
