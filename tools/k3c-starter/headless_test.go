package main

import (
	"context"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"runtime"
	"testing"
	"time"
)

// -dev: Der Kindprozess läuft in tools/k3c-starter, K3C_ROOT nennt die Wurzel mit dist/ und bin/k3c-server.
func TestFindLayoutMitK3CRoot(t *testing.T) {
	root := t.TempDir()
	name := "k3c-server"
	if runtime.GOOS == "windows" {
		name += ".exe"
	}
	for _, f := range []string{filepath.Join("dist", "index.html"), filepath.Join("bin", name)} {
		if err := os.MkdirAll(filepath.Join(root, filepath.Dir(f)), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(root, f), []byte("x"), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	t.Setenv("K3C_ROOT", root)
	lay, err := findLayout(t.TempDir(), filepath.Join(t.TempDir(), "build", "bin"))
	if err != nil || lay.root != root || filepath.Base(lay.serverExe) != name {
		t.Fatalf("Layout: %+v, %v", lay, err)
	}
}

// Die API läuft über einen echten Listener; ein Signal (ctx) beendet den Server und die API.
func TestServeAPIBeendetServerBeiSignal(t *testing.T) {
	port := freePort(t)
	s := fakeSupervisor(t, port)
	app := &App{sup: s}
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatal(err)
	}
	app.start()
	waitState(t, s, func(st Status) bool { return st.Running })

	ctx, cancel := context.WithCancel(context.Background())
	done := make(chan error, 1)
	go func() { done <- serveAPI(ctx, app, ln) }()

	req, _ := http.NewRequest(http.MethodGet, "http://"+ln.Addr().String()+"/api/starter/state", nil)
	req.Header.Set(starterHeader, starterValue)
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	_ = res.Body.Close()
	if res.StatusCode != http.StatusOK {
		t.Fatalf("API: Status %d", res.StatusCode)
	}

	cancel()
	select {
	case err := <-done:
		if err != nil {
			t.Fatalf("serveAPI: %v", err)
		}
	case <-time.After(30 * time.Second):
		t.Fatal("serveAPI endet nicht")
	}
	if st := s.State(); st.Running || st.Starting {
		t.Fatalf("Server läuft nach dem Signal noch: %+v", st)
	}
}
