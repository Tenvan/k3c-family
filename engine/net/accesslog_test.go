package net

import (
	"bytes"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync"
	"testing"
	"time"

	"k3c/engine/store"
)

// lockedBuffer: Handler schreiben aus Server-Goroutinen.
type lockedBuffer struct {
	mu  sync.Mutex
	buf bytes.Buffer
}

func (b *lockedBuffer) Write(p []byte) (int, error) {
	b.mu.Lock()
	defer b.mu.Unlock()
	return b.buf.Write(p)
}

func (b *lockedBuffer) String() string {
	b.mu.Lock()
	defer b.mu.Unlock()
	return b.buf.String()
}

func loggedServer(t *testing.T) (*httptest.Server, *lockedBuffer, *lockedBuffer) {
	t.Helper()
	server, client := &lockedBuffer{}, &lockedBuffer{}
	opts := &slog.HandlerOptions{Level: slog.LevelDebug}
	root := t.TempDir()
	srv := httptest.NewServer(NewHandler(Config{Dist: root, Saves: &store.Saves{Dir: root}, Reports: &store.Reports{Dir: root},
		Log: slog.New(slog.NewTextHandler(server, opts)), ClientLog: slog.New(slog.NewTextHandler(client, opts))}))
	t.Cleanup(srv.Close)
	return srv, server, client
}

func TestZugriffslogMitStatus(t *testing.T) {
	srv, server, _ := loggedServer(t)
	do(t, "GET", srv.URL+"/api/level?seed=x", "")
	do(t, "GET", srv.URL+"/gibt-es-nicht", "")
	out := server.String()
	for _, want := range []string{"HTTP GET /api/level", "status=200", "level=INFO", "status=404", "level=WARN"} {
		if !strings.Contains(out, want) {
			t.Errorf("%q fehlt im Log:\n%s", want, out)
		}
	}
	if strings.Contains(out, "seed=x") {
		t.Errorf("Query darf nicht im Log stehen:\n%s", out)
	}
}

func TestClientLogNimmtMeldungenAn(t *testing.T) {
	srv, _, client := loggedServer(t)
	body := `{"entries":[{"level":"error","msg":"kaputt","url":"/game.html","stack":"at x","device":"abcdef123456"}]}`
	res, _ := do(t, "POST", srv.URL+"/api/clientlog", body)
	if res.StatusCode != http.StatusNoContent {
		t.Fatalf("Status %d", res.StatusCode)
	}
	out := client.String()
	for _, want := range []string{"level=ERROR", "msg=kaputt", "ns=client", "device=abcdef12", "stack=\"at x\""} {
		if !strings.Contains(out, want) {
			t.Errorf("%q fehlt:\n%s", want, out)
		}
	}
	for _, bad := range []string{`{}`, `{"entries":[]}`, `kein json`} {
		if res, _ := do(t, "POST", srv.URL+"/api/clientlog", bad); res.StatusCode != http.StatusBadRequest {
			t.Errorf("%q: Status %d", bad, res.StatusCode)
		}
	}
	if res, _ := do(t, "GET", srv.URL+"/api/clientlog", ""); res.StatusCode != http.StatusMethodNotAllowed {
		t.Errorf("GET: Status %d", res.StatusCode)
	}
}

func TestClientLogBegrenztMenge(t *testing.T) {
	var g clientGate
	now := time.Now()
	if !g.allow(clientRate, now) || g.allow(1, now.Add(time.Second)) {
		t.Fatal("Grenze greift nicht")
	}
	if !g.allow(1, now.Add(clientWindow)) {
		t.Fatal("Fenster läuft nicht ab")
	}
}
