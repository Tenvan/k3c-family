package mcpsrv

import (
	"context"
	"strings"
	"testing"

	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/services"
)

// stubProc ist ein Dienst-Prozess, der läuft, bis er beendet wird.
type stubProc struct{ done chan struct{} }

func (p *stubProc) PID() int    { return 42 }
func (p *stubProc) Wait() error { <-p.done; return nil }
func (p *stubProc) Kill() error { close(p.done); return nil }

func TestDienstAusgabeInLogsSources(t *testing.T) {
	store := console.New(0, nil)
	s := New(Config{Version: "test", Root: t.TempDir(), Console: store})
	ctl := services.New([]services.Service{{Name: "Vite", Command: []string{"x"}, Port: 1, Health: "tcp"}}, services.Options{
		Console: store,
		Start: func(_ services.Service, _ string, out func(string, string)) (services.Process, error) {
			out("stdout", "VITE ready in 312 ms")
			return &stubProc{done: make(chan struct{})}, nil
		},
		Check:  func(context.Context, services.Service) error { return nil },
		Listen: func(context.Context, int) (int, bool) { return 0, false },
	})
	if _, err := ctl.Start(context.Background(), "Vite"); err != nil {
		t.Fatal(err)
	}
	defer ctl.StopAll(context.Background())
	text, _ := s.logsSources(context.Background(), struct{}{})
	if !strings.Contains(text, "Konsole Vite · 1 Zeilen") {
		t.Errorf("Quellen: %q", text)
	}
	tail, _ := s.consoleTail(context.Background(), tailIn{Source: "Vite"})
	if !strings.Contains(tail, "VITE ready in 312 ms") {
		t.Errorf("console_tail: %q", tail)
	}
}
