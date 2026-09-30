package services

import (
	"context"
	"fmt"
	"net"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
	"time"
)

func TestUebernahmeUndAblehnungen(t *testing.T) {
	f := &fake{listenPID: 4711}
	f.healthy.Store(true)
	c := testController(f, nil, svc("Heimnetz", true))
	c.Adopt(context.Background())
	st := c.Statuses()[0]
	if st.State != Adopted || st.PID != 4711 || st.StartedAt.Hour() != 9 {
		t.Fatalf("Übernahme: %+v", st)
	}
	if _, err := c.Start(context.Background(), "Heimnetz"); err == nil || !strings.Contains(err.Error(), "läuft bereits") {
		t.Errorf("Start eines übernommenen: %v", err)
	}
	if _, err := c.Restart(context.Background(), "Heimnetz"); err == nil {
		t.Error("Neustart eines übernommenen ohne Fehler")
	}
}

func TestUebernommenStoppNurMitForce(t *testing.T) {
	f := &fake{listenPID: 4711}
	f.healthy.Store(true)
	c := testController(f, nil, svc("Heimnetz", true))
	c.Adopt(context.Background())
	c.StopAll(context.Background()) // übernommene bleiben
	if _, err := c.Stop(context.Background(), "Heimnetz", false); err == nil || !strings.Contains(err.Error(), "nur mit force") ||
		c.Statuses()[0].State != Adopted || len(f.killed) != 0 {
		t.Errorf("Stopp ohne force: %v, %+v, getötet %v", err, c.Statuses()[0], f.killed)
	}
	st, err := c.Stop(context.Background(), "Heimnetz", true)
	if err != nil || st.State != Stopped || len(f.killed) != 1 || f.killed[0] != 4711 || f.count() != 0 {
		t.Errorf("Stopp mit force: %+v, %v, getötet %v", st, err, f.killed)
	}
}

func TestUebernommenerDienstVerschwindet(t *testing.T) {
	f := &fake{listenPID: 4711}
	f.healthy.Store(true)
	c := testController(f, nil, svc("Heimnetz", true))
	c.Adopt(context.Background())
	f.healthy.Store(false)
	waitFor(t, "gestoppt", func() bool { return c.Statuses()[0].State == Stopped })
	if st := c.Statuses()[0]; st.LastError != lostAdopted || st.PID != 0 || f.count() != 0 {
		t.Errorf("verschwunden: %+v, %d Starts (kein Auto-Restart erwartet)", st, f.count())
	}
}

func TestPortBelegtStartetNicht(t *testing.T) {
	for pid, want := range map[int]string{8812: "Port 1 bereits belegt (PID 8812)", 0: "Port 1 bereits belegt (PID unbekannt)"} {
		f := &fake{listenPID: pid}
		f.busy.Store(true)
		c := testController(f, nil, svc("Vite", true))
		st, err := c.Start(context.Background(), "Vite")
		if err == nil || st.State != Failed || st.LastError != want || f.count() != 0 {
			t.Errorf("PID %d: %+v, %v, %d Starts", pid, st, err, f.count())
		}
	}
}

func TestMetrikenFuerLaufendeDienste(t *testing.T) {
	f := &fake{}
	f.healthy.Store(true)
	c := testController(f, nil, svc("Vite", false), Service{Name: "Aus", Command: []string{"x"}, Port: 2, Health: "tcp"})
	if _, err := c.Start(context.Background(), "Vite"); err != nil {
		t.Fatal(err)
	}
	c.sampleAll(context.Background())
	vite, aus := c.Statuses()[0], c.Statuses()[1]
	if vite.CPU != 3.1 || vite.Memory != 480<<20 || aus.Memory != 0 {
		t.Errorf("Metriken: %+v / %+v", vite, aus)
	}
	if st, _ := c.Stop(context.Background(), "Vite", false); st.CPU != 0 || st.Memory != 0 {
		t.Errorf("nach Stopp: %+v", st)
	}
}

func TestLogLevelsLetzteStunde(t *testing.T) {
	root := t.TempDir()
	now := time.Now()
	line := func(ago time.Duration, level string) string {
		return fmt.Sprintf(`{"time":%q,"level":%q,"msg":"m"}`+"\n", now.Add(-ago).Format(time.RFC3339), level)
	}
	data := line(2*time.Hour, "ERROR") + line(50*time.Minute, "WARN") + line(10*time.Minute, "ERROR") + line(time.Minute, "INFO")
	if err := os.MkdirAll(filepath.Join(root, "logs"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, "logs", "k3c-server.jsonl"), []byte(data), 0o644); err != nil {
		t.Fatal(err)
	}
	c := New([]Service{{Name: "Server", Command: []string{"x"}, Port: 1, Health: "tcp", Log: "k3c-server"}, svc("Vite", false)},
		Options{Root: root})
	counts, ok, err := c.LogLevels("Server")
	if err != nil || !ok || counts.Total != 3 || counts.Counts["ERROR"] != 1 || counts.Counts["WARN"] != 1 || counts.Counts["INFO"] != 1 {
		t.Errorf("Zähler: %+v, %v, %v", counts, ok, err)
	}
	if _, ok, err := c.LogLevels("Vite"); ok || err != nil {
		t.Errorf("ohne log: %v, %v", ok, err)
	}
}

// TestHelperProcess ist kein Test: als "listen" öffnet das Test-Binary einen Port, schreibt ihn in K3C_PORTFILE und wartet.
func TestHelperProcess(t *testing.T) {
	if os.Getenv("K3C_HELPER") != "listen" {
		return
	}
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		os.Exit(2)
	}
	_ = os.WriteFile(os.Getenv("K3C_PORTFILE"), []byte(strconv.Itoa(ln.Addr().(*net.TCPAddr).Port)), 0o644)
	time.Sleep(time.Minute)
}

func TestGopsutilGegenEchtenProzess(t *testing.T) {
	portFile := filepath.Join(t.TempDir(), "port")
	cmd := exec.Command(os.Args[0], "-test.run=^TestHelperProcess$")
	cmd.Env = append(os.Environ(), "K3C_HELPER=listen", "K3C_PORTFILE="+portFile)
	if err := cmd.Start(); err != nil {
		t.Fatal(err)
	}
	defer func() { _ = cmd.Process.Kill(); _ = cmd.Wait() }()
	var port int
	waitFor(t, "Port", func() bool {
		b, err := os.ReadFile(portFile)
		port, _ = strconv.Atoi(string(b))
		return err == nil && port > 0
	})
	pid, listening := PortListener(context.Background(), port)
	if !listening || pid != cmd.Process.Pid {
		t.Fatalf("PortListener: pid %d (erwartet %d), lauscht %v", pid, cmd.Process.Pid, listening)
	}
	sample := NewSampler()
	m, err := sample(context.Background(), pid)
	if err != nil || m.Memory == 0 || m.Started.IsZero() || m.Started.After(time.Now()) {
		t.Errorf("Messung: %+v, %v", m, err)
	}
	if _, listening := PortListener(context.Background(), 1); listening {
		t.Error("Port 1 als belegt gemeldet")
	}
}
