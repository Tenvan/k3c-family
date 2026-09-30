package main

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"k3c/tools/k3c-dev/internal/services"
)

type stubProc struct{ done chan struct{} }

func (p *stubProc) PID() int    { return 42 }
func (p *stubProc) Wait() error { <-p.done; return nil }
func (p *stubProc) Kill() error { close(p.done); return nil }

// serviceApp baut eine App mit gestelltem Controller: Vite (ohne Log) startet, Heimnetz (Log server) läuft
// schon vor dem Start und wird übernommen. events sammelt die Namen der gemeldeten Zustände.
func serviceApp(t *testing.T) (*App, func() []string) {
	t.Helper()
	root := t.TempDir()
	var adopted, started atomic.Bool
	adopted.Store(true)
	var mu sync.Mutex
	var events []string
	app := &App{root: root, svcCtx: context.Background()}
	app.emit = func(_ context.Context, name string, data ...any) {
		if st, ok := data[0].(services.Status); ok && name == evServiceState {
			mu.Lock()
			events = append(events, st.Name+" "+string(st.State))
			mu.Unlock()
		}
	}
	app.ctl = services.New([]services.Service{
		{Name: "Vite", Command: []string{"x"}, Port: 5173, Health: "http"},
		{Name: "Heimnetz", Command: []string{"x"}, Port: 8080, Health: "http", Log: "server"},
	}, services.Options{
		Root: root,
		Start: func(services.Service, string, func(string, string)) (services.Process, error) {
			started.Store(true)
			return &stubProc{done: make(chan struct{})}, nil
		},
		Check: func(_ context.Context, svc services.Service) error {
			if (svc.Name == "Heimnetz" && !adopted.Load()) || (svc.Name == "Vite" && !started.Load()) {
				return errors.New("antwortet nicht")
			}
			return nil
		},
		Listen:    func(_ context.Context, port int) (int, bool) { return 4711, port == 8080 && adopted.Load() },
		Sample:    func(context.Context, int) (services.Metrics, error) { return services.Metrics{}, nil },
		KillPID:   func(int) error { adopted.Store(false); return nil },
		OnChange:  func(st services.Status) { app.emit(context.Background(), evServiceState, st) },
		StartPoll: 2 * time.Millisecond, StartTimeout: 60 * time.Millisecond, WatchEvery: time.Hour,
		StopTimeout: 50 * time.Millisecond,
	})
	app.ctl.Adopt(context.Background())
	t.Cleanup(func() { app.ctl.StopAll(context.Background()) })
	return app, func() []string {
		mu.Lock()
		defer mu.Unlock()
		return append([]string(nil), events...)
	}
}

func TestServicesBindings(t *testing.T) {
	app, _ := serviceApp(t)
	view := app.Services()
	if view.Error != "" || len(view.Services) != 2 || view.Services[1].Log != "server" || view.Services[0].Log != "" {
		t.Fatalf("Services() = %+v", view)
	}
	if st := view.Services[1].State; st != services.Adopted {
		t.Fatalf("Heimnetz = %s, erwartet übernommen", st)
	}
	if st, err := app.ServiceStart("Vite"); err != nil || st.State != services.Running {
		t.Errorf("ServiceStart = %+v, %v", st, err)
	}
	if err := app.ServicesStartAll(); err != nil {
		t.Errorf("ServicesStartAll mit laufenden = %v", err)
	}
}

// Ein übernommener Dienst stoppt nur mit force (B-068/AC-02); jede Änderung kommt als service:state.
func TestServicesStopUndEreignisse(t *testing.T) {
	app, events := serviceApp(t)
	if _, err := app.ServiceStart("Vite"); err != nil {
		t.Fatal(err)
	}
	if _, err := app.ServiceStop("Heimnetz", false); err == nil {
		t.Error("übernommener Dienst ohne force gestoppt")
	}
	if st, err := app.ServiceStop("Heimnetz", true); err != nil || st.State != services.Stopped {
		t.Errorf("Stopp mit force = %+v, %v", st, err)
	}
	if err := app.ServicesStopAll(); err != nil || app.Services().Services[0].State != services.Stopped {
		t.Errorf("ServicesStopAll: %v, %+v", err, app.Services().Services[0])
	}
	got := strings.Join(events(), ",")
	for _, want := range []string{"Heimnetz übernommen", "Vite startet", "Vite läuft", "Heimnetz gestoppt", "Vite stoppt"} {
		if !strings.Contains(got, want) {
			t.Errorf("Ereignis %q fehlt in %s", want, got)
		}
	}
}

func TestServiceLogLevels(t *testing.T) {
	app, _ := serviceApp(t)
	if _, err := app.ServiceLogLevels("Vite"); err == nil || !strings.Contains(err.Error(), "kein Log") {
		t.Errorf("ohne Log: %v", err)
	}
	now := time.Now().UTC().Format(time.RFC3339Nano)
	line := func(level string) string { return `{"time":"` + now + `","level":"` + level + `","msg":"x","ns":"http"}` + "\n" }
	dir := filepath.Join(app.root, "logs")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, "server.jsonl"), []byte(line("INFO")+line("WARN")+line("ERROR")+line("INFO")), 0o644); err != nil {
		t.Fatal(err)
	}
	c, err := app.ServiceLogLevels("Heimnetz")
	if err != nil || c.Total != 4 || c.Counts["INFO"] != 2 || c.Counts["ERROR"] != 1 {
		t.Errorf("Heimnetz: %+v, %v", c, err)
	}
}

func TestServicesOhneKonfiguration(t *testing.T) {
	app := &App{svcCtx: context.Background(), svcErr: errors.New(`C:\k3c\tools\k3c-dev\services.json: unbekanntes Feld "prot"`)}
	if v := app.Services(); len(v.Services) != 0 || !strings.Contains(v.Error, "services.json") {
		t.Errorf("Services() = %+v", v)
	}
	if _, err := app.ServiceStart("Vite"); err == nil || !strings.Contains(err.Error(), "unbekanntes Feld") {
		t.Errorf("ServiceStart ohne Konfiguration: %v", err)
	}
}
