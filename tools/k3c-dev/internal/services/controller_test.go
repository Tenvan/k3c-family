package services

import (
	"context"
	"errors"
	"net"
	"net/http"
	"net/http/httptest"
	"strconv"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"k3c/tools/k3c-dev/internal/console"
)

// fakeProc ist ein gestellter Prozess: exit beendet ihn von selbst, Kill von außen.
type fakeProc struct {
	pid    int
	done   chan struct{}
	once   sync.Once
	killed atomic.Bool
}

func (p *fakeProc) PID() int    { return p.pid }
func (p *fakeProc) Wait() error { <-p.done; return nil }
func (p *fakeProc) exit()       { p.once.Do(func() { close(p.done) }) }
func (p *fakeProc) Kill() error { p.killed.Store(true); p.exit(); return nil }

// fake hält gestellten Start und gestellte Prüfung.
type fake struct {
	mu      sync.Mutex
	procs   []*fakeProc
	healthy atomic.Bool
}

func (f *fake) start(svc Service, _ string, out func(stream, text string)) (Process, error) {
	f.mu.Lock()
	defer f.mu.Unlock()
	if svc.Command[0] == "kaputt" {
		return nil, errors.New("gibt es nicht")
	}
	out("stdout", "hallo von "+svc.Name)
	p := &fakeProc{pid: 1000 + len(f.procs), done: make(chan struct{})}
	f.procs = append(f.procs, p)
	return p, nil
}

func (f *fake) check(context.Context, Service) error {
	if f.healthy.Load() {
		return nil
	}
	return errors.New("antwortet nicht")
}

func (f *fake) last() *fakeProc {
	f.mu.Lock()
	defer f.mu.Unlock()
	return f.procs[len(f.procs)-1]
}

func (f *fake) count() int {
	f.mu.Lock()
	defer f.mu.Unlock()
	return len(f.procs)
}

func testController(f *fake, store *console.Store, list ...Service) *Controller {
	return New(list, Options{Console: store, Start: f.start, Check: f.check, StartPoll: 2 * time.Millisecond,
		StartTimeout: 150 * time.Millisecond, WatchEvery: 5 * time.Millisecond, StopTimeout: time.Second})
}

func svc(name string, autoRestart bool) Service {
	return Service{Name: name, Command: []string{"x"}, Port: 1, Health: "http", AutoRestart: autoRestart}
}

// waitFor wartet, bis cond gilt.
func waitFor(t *testing.T, what string, cond func() bool) {
	t.Helper()
	for range 400 {
		if cond() {
			return
		}
		time.Sleep(5 * time.Millisecond)
	}
	t.Fatalf("wartete vergeblich auf: %s", what)
}

func TestStartLaeuftUndStop(t *testing.T) {
	f := &fake{}
	f.healthy.Store(true)
	store := console.New(0, nil)
	var states []State
	var mu sync.Mutex
	c := testController(f, store, svc("Vite", false))
	c.opts.OnChange = func(s Status) { mu.Lock(); states = append(states, s.State); mu.Unlock() }
	st, err := c.Start(context.Background(), "Vite")
	if err != nil || st.State != Running || st.PID != 1000 {
		t.Fatalf("Start: %+v, %v", st, err)
	}
	if _, err := c.Start(context.Background(), "Vite"); err == nil || !strings.Contains(err.Error(), "läuft bereits") {
		t.Errorf("zweiter Start: %v", err)
	}
	if lines, _ := store.Tail("Vite", 0); len(lines) != 1 || lines[0].Text != "hallo von Vite" {
		t.Errorf("Konsole: %+v", lines)
	}
	st, _ = c.Stop(context.Background(), "Vite")
	mu.Lock()
	got := states
	mu.Unlock()
	if st.State != Stopped || !f.last().killed.Load() || !strings.Contains(stateString(got), "startet läuft stoppt gestoppt") {
		t.Errorf("Stop: %+v, Zustände %v", st, got)
	}
	if _, err := c.Start(context.Background(), "Gibtsnicht"); err == nil || !strings.Contains(err.Error(), "gültig: [Vite]") {
		t.Errorf("unbekannter Dienst: %v", err)
	}
}

func stateString(states []State) string {
	parts := make([]string, len(states))
	for i, s := range states {
		parts[i] = string(s)
	}
	return strings.Join(parts, " ")
}

func TestStartFehlschlaege(t *testing.T) {
	f := &fake{} // nie gesund
	c := testController(f, nil, svc("Vite", true), Service{Name: "Kaputt", Command: []string{"kaputt"}, Port: 2, Health: "tcp"})
	st, err := c.Start(context.Background(), "Vite")
	if err == nil || st.State != Failed || !strings.Contains(st.LastError, "nicht gesund") || !f.last().killed.Load() {
		t.Errorf("Zeitlimit: %+v, %v", st, err)
	}
	st, err = c.Start(context.Background(), "Kaputt")
	if err == nil || st.State != Failed || !strings.Contains(st.LastError, "gibt es nicht") {
		t.Errorf("Startfehler: %+v, %v", st, err)
	}
	f.healthy.Store(false)
	go func() {
		waitFor(t, "Prozess", func() bool { return f.count() == 2 })
		f.last().exit()
	}()
	if st, _ = c.Start(context.Background(), "Vite"); st.State != Failed || !strings.Contains(st.LastError, "endete, bevor") {
		t.Errorf("Prozessende beim Start: %+v", st)
	}
}

func TestAusfallStartetNeuMitGrenze(t *testing.T) {
	f := &fake{}
	f.healthy.Store(true)
	now := time.Date(2026, 9, 30, 12, 0, 0, 0, time.UTC)
	c := testController(f, nil, svc("Vite", true))
	c.opts.Now = func() time.Time { return now }
	if _, err := c.Start(context.Background(), "Vite"); err != nil {
		t.Fatal(err)
	}
	for i := 1; i <= 3; i++ { // drei Abstürze → drei Neustarts
		f.last().exit()
		waitFor(t, "Neustart", func() bool { return f.count() == i+1 && c.Statuses()[0].State == Running })
	}
	f.last().exit() // vierter Absturz in 10 min: kein Neustart mehr
	waitFor(t, "Endzustand", func() bool { return c.Statuses()[0].State == Failed })
	if st := c.Statuses()[0]; st.Restarts != 3 || !strings.Contains(st.LastError, "kein Neustart mehr") || f.count() != 4 {
		t.Errorf("Grenze: %+v, %d Starts", st, f.count())
	}
}

func TestDreiFehlschlaegeImLauf(t *testing.T) {
	f := &fake{}
	f.healthy.Store(true)
	c := testController(f, nil, svc("Heimnetz", false))
	if _, err := c.Start(context.Background(), "Heimnetz"); err != nil {
		t.Fatal(err)
	}
	f.healthy.Store(false)
	waitFor(t, "Ausfall", func() bool { return c.Statuses()[0].State == Failed })
	if st := c.Statuses()[0]; !strings.Contains(st.LastError, "3 Prüfungen in Folge") || !f.last().killed.Load() || f.count() != 1 {
		t.Errorf("ohne Auto-Restart: %+v", st)
	}
}

func TestStartAllUndStopAllRueckwaerts(t *testing.T) {
	f := &fake{}
	f.healthy.Store(true)
	c := testController(f, nil, svc("A", false), Service{Name: "B", Command: []string{"x"}, Port: 2, Health: "http"})
	if err := c.StartAll(context.Background()); err != nil {
		t.Fatal(err)
	}
	var order []string
	c.opts.OnChange = func(s Status) {
		if s.State == Stopping {
			order = append(order, s.Name)
		}
	}
	c.StopAll(context.Background())
	if strings.Join(order, ",") != "B,A" {
		t.Errorf("Reihenfolge: %v", order)
	}
}

func TestHealthCheck(t *testing.T) {
	codes := map[string]int{"/": 200}
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if codes["/"] == 302 {
			http.Redirect(w, r, "http://127.0.0.1:1/woanders", http.StatusFound)
			return
		}
		w.WriteHeader(codes["/"])
	}))
	defer srv.Close()
	port, _ := strconv.Atoi(srv.URL[strings.LastIndex(srv.URL, ":")+1:])
	s := Service{Port: port, Health: "http"}
	for code, ok := range map[int]bool{200: true, 302: true, 500: false} {
		codes["/"] = code
		if err := HealthCheck(context.Background(), s); (err == nil) != ok {
			t.Errorf("http %d: %v", code, err)
		}
	}
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatal(err)
	}
	tcp := Service{Port: ln.Addr().(*net.TCPAddr).Port, Health: "tcp"}
	if err := HealthCheck(context.Background(), tcp); err != nil {
		t.Errorf("tcp offen: %v", err)
	}
	_ = ln.Close()
	if err := HealthCheck(context.Background(), tcp); err == nil {
		t.Error("tcp geschlossen ohne Fehler")
	}
}
