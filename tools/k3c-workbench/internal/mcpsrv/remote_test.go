package mcpsrv

import (
	"context"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"
)

// fakePi ist ein Spielserver mit den Dev-Lese-Endpunkten (B-770): zwei Logs, Cursor und Kopfzeilen wie im Original.
// gotQuery hält die letzte Anfrage an /api/dev/log.
type fakePi struct {
	*httptest.Server
	gotQuery string
	// Abweichungen des Servers für die Fehlerfälle
	evilList, noSize, big bool
}

func newFakePi(t *testing.T, status int, body string) *fakePi {
	t.Helper()
	p := &fakePi{}
	now := time.Now().UTC().Format(time.RFC3339)
	server := fmt.Sprintf(`{"time":%q,"level":"ERROR","msg":"💥 Absturz im Raum","ns":"room"}`+"\n"+`{"time":%q,"level":"INFO","msg":"🚀 Start","ns":"main"}`, now, now)
	client := fmt.Sprintf(`{"time":%q,"level":"WARN","msg":"⚠ WebGL Context lost","ns":"client"}`, now)
	p.Server = httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if status != 0 {
			if status >= 300 && status < 400 {
				w.Header().Set("Location", "http://example.invalid/")
			}
			w.WriteHeader(status)
			_, _ = w.Write([]byte(body))
			return
		}
		switch r.URL.Path {
		case "/api/dev/logs":
			_, _ = fmt.Fprintf(w, "server 2048 %s\nclient 512 %s\n", now, now)
			if p.evilList {
				_, _ = fmt.Fprintf(w, "a/../../../x 1 %s\n..\\..\\y 1 %s\n", now, now)
			}
		case "/api/dev/log":
			p.gotQuery = r.URL.RawQuery
			if !p.noSize {
				w.Header().Set("X-Log-Size", "777")
			}
			if p.big {
				_, _ = w.Write([]byte(strings.Repeat("x", 3<<20)))
				return
			}
			if r.URL.Query().Get("file") == "client" {
				_, _ = w.Write([]byte(client))
				return
			}
			_, _ = w.Write([]byte(server))
		case "/api/dev/status":
			_, _ = w.Write([]byte(`{"version":"v0.23.2","uptimeS":3700,"saves":8,"reports":2,"rooms":[],"failures":[],"log":{"on":true,"files":["server","client"]}}`))
		case "/api/dev/reports":
			if name := r.URL.Query().Get("name"); name != "" {
				_, _ = fmt.Fprintf(w, `{"kind":"gl-diagnose","name":%q,"maxTex":4096}`, name)
				return
			}
			_, _ = fmt.Fprintf(w, "session-1.json 1643 %s\n", now)
		default:
			http.NotFound(w, r)
		}
	}))
	t.Cleanup(p.Close)
	return p
}

func (p *fakePi) addr() string { return strings.TrimPrefix(p.URL, "http://") }

// B-771/AC-01: logs_errors, logs_query und logs_stats lesen mit server den Dev-Endpunkt; die Antwort nennt die Quelle.
func TestRemote_errorsQueryStats(t *testing.T) {
	t.Parallel()
	pi := newFakePi(t, 0, "")
	s := logServer(t)
	ctx := context.Background()
	text, err := s.logsErrors(ctx, errorsIn{Server: pi.addr()})
	if err != nil || !strings.HasPrefix(text, "Quelle: Server ") || !strings.Contains(text, "Absturz im Raum") || !strings.Contains(text, "WebGL Context lost") ||
		strings.Contains(text, "Start") {
		t.Errorf("errors: %v\n%s", err, text)
	}
	text, err = s.logsQuery(ctx, queryIn{Server: pi.addr(), Source: "k3c-client", Pattern: "WebGL"})
	if err != nil || !strings.Contains(text, "WebGL Context lost") || strings.Contains(text, "Absturz") {
		t.Errorf("query client: %v\n%s", err, text)
	}
	text, err = s.logsStats(ctx, statsIn{Server: pi.addr(), Since: "1h"})
	if err != nil || !strings.Contains(text, "ERROR 1") || !strings.Contains(text, "WARN 1") {
		t.Errorf("stats: %v\n%s", err, text)
	}
	if text, _ := s.logsErrors(ctx, errorsIn{}); strings.Contains(text, "Quelle: Server") {
		t.Errorf("ohne server muss lokal gelesen werden: %q", text)
	}
}

// B-771/AC-01: logs_services mit server listet die Dateien des Servers unter den Namen, die service erwartet.
func TestRemote_services(t *testing.T) {
	t.Parallel()
	pi := newFakePi(t, 0, "")
	text, err := logServer(t).logsSources(context.Background(), sourcesIn{Server: pi.addr()})
	if err != nil || !strings.Contains(text, "Log k3c-server · 2,0 KB") || !strings.Contains(text, "Log k3c-client · 512 B") {
		t.Errorf("services: %v\n%s", err, text)
	}
}

// B-771/AC-03: server_status mit server zeigt Version, Laufzeit und Log-Status des entfernten Servers; Berichte lassen sich listen und lesen.
func TestRemote_statusUndBerichte(t *testing.T) {
	t.Parallel()
	pi := newFakePi(t, 0, "")
	s := logServer(t)
	ctx := context.Background()
	text, err := s.serverStatus(ctx, statusIn{Server: pi.addr()})
	if err != nil || !strings.Contains(text, "Server v0.23.2") || !strings.Contains(text, "1 h 1 min") || !strings.Contains(text, "Log: an · server, client") ||
		!strings.HasPrefix(text, "Quelle: Server ") {
		t.Errorf("status: %v\n%s", err, text)
	}
	text, err = s.reportsList(ctx, reportsIn{Server: pi.addr()})
	if err != nil || !strings.Contains(text, "session-1.json 1643") {
		t.Errorf("reports_list: %v\n%s", err, text)
	}
	text, err = s.reportRead(ctx, reportIn{Server: pi.addr(), Name: "session-1.json"})
	if err != nil || !strings.Contains(text, `"kind":"gl-diagnose"`) || !strings.Contains(text, `"maxTex":4096`) {
		t.Errorf("report_read: %v\n%s", err, text)
	}
	if _, err := s.serverStatus(ctx, statusIn{Server: "evil.example.com:80"}); err == nil {
		t.Error("fremde Adresse im Status ohne Fehler")
	}
}

// B-771/AC-01: logs_since mit server reicht den Cursor an den Server, der nächste Cursor ist dessen X-Log-Size.
func TestRemote_since(t *testing.T) {
	t.Parallel()
	pi := newFakePi(t, 0, "")
	text, err := logServer(t).logsSince(context.Background(), sinceIn{Server: pi.addr(), Source: "k3c-server", Cursor: 500})
	if err != nil || !strings.Contains(text, "Absturz im Raum") || !strings.Contains(text, "cursor=777") {
		t.Errorf("since: %v\n%s", err, text)
	}
	if !strings.Contains(pi.gotQuery, "since=500") || !strings.Contains(pi.gotQuery, "file=server") {
		t.Errorf("Anfrage an den Server: %s", pi.gotQuery)
	}
}

// B-771/AC-02: Fehler nennen Adresse und Grund: 403 (Dev-Mode), 404 „Log aus“ (K3C_LOG_DIR), Weiterleitung, nicht erreichbar.
func TestRemote_fehler(t *testing.T) {
	t.Parallel()
	ctx := context.Background()
	s := logServer(t)
	for _, c := range []struct {
		status int
		body   string
		want   string
	}{
		{http.StatusForbidden, `{"error":"Nur im Dev-Mode (K3C_DEV)"}`, "K3C_DEV"},
		{http.StatusNotFound, `{"error":"Log aus (K3C_LOG_DIR nicht gesetzt)"}`, "K3C_LOG_DIR"},
		{http.StatusFound, "", "Weiterleitungen"},
	} {
		pi := newFakePi(t, c.status, c.body)
		if _, err := s.logsErrors(ctx, errorsIn{Server: pi.addr()}); err == nil || !strings.Contains(err.Error(), c.want) || !strings.Contains(err.Error(), pi.addr()) {
			t.Errorf("Status %d: %v, erwartet Hinweis %q und die Adresse", c.status, err, c.want)
		}
	}
	gone := newFakePi(t, 0, "")
	addr := gone.addr()
	gone.Close()
	if _, err := s.logsErrors(ctx, errorsIn{Server: addr}); err == nil || !strings.Contains(err.Error(), "nicht erreichbar") {
		t.Errorf("nicht erreichbar: %v", err)
	}
}

// B-771/AC-02: Unbekannte Namen, fremde Adressen und lokale Dienste mit server werden abgelehnt, mit den gültigen Werten.
func TestRemote_abgelehnt(t *testing.T) {
	t.Parallel()
	ctx := context.Background()
	s := logServer(t)
	for _, bad := range []string{"evil.example.com:80", "8.8.8.8:80", "http://pi-gaming:8080", "unbekannt", "pi-gaming:0", "pi-gaming:99999"} {
		if _, err := s.logsErrors(ctx, errorsIn{Server: bad}); err == nil || !strings.Contains(err.Error(), "gültig: pi") {
			t.Errorf("server %q: %v, erwartet Ablehnung mit gültigen Werten", bad, err)
		}
	}
	pi := newFakePi(t, 0, "")
	if _, err := s.logsQuery(ctx, queryIn{Server: pi.addr(), Source: "k3c-workbench"}); err == nil || !strings.Contains(err.Error(), "k3c-server, k3c-client") {
		t.Errorf("lokaler Dienst mit server: %v", err)
	}
	if _, err := s.logsSince(ctx, sinceIn{Server: pi.addr(), Source: "k3c-server", Cursor: -1}); err == nil {
		t.Error("negativer Cursor ohne Fehler")
	}
}

// B-771: Bekannte Namen: pi ist voreingestellt, weitere kommen aus K3C_REMOTE_SERVERS; Heimnetz-Adressen gehen, öffentliche nicht.
func TestResolveRemote(t *testing.T) {
	t.Setenv(remoteEnv, "lab=10.0.0.5:8080, x=")
	for spec, want := range map[string]string{
		"pi": "http://pi-gaming:8080", "lab": "http://10.0.0.5:8080", "pi-gaming:8080": "http://pi-gaming:8080",
		"192.168.2.10:8080": "http://192.168.2.10:8080", "localhost:8080": "http://localhost:8080",
	} {
		if got, err := resolveRemote(spec); err != nil || got.base != want {
			t.Errorf("%q: %+v %v, erwartet %s", spec, got, err, want)
		}
	}
	for _, bad := range []string{"", "x", "1.2.3.4:80", "pi.fritz.box:80", "pi-gaming"} {
		if got, err := resolveRemote(bad); err == nil {
			t.Errorf("%q ist erlaubt: %+v", bad, got)
		}
	}
}

// B-771 Review: Dateinamen vom Server werden nur aus der festen Liste übernommen; ein Name mit Pfad landet nie im Dateisystem.
func TestRemote_boeserDateiname(t *testing.T) {
	t.Parallel()
	pi := newFakePi(t, 0, "")
	pi.evilList = true
	s := logServer(t)
	text, err := s.logsSources(context.Background(), sourcesIn{Server: pi.addr()})
	if err != nil || strings.Contains(text, "..") || strings.Contains(text, "a/") {
		t.Errorf("services: %v\n%s", err, text)
	}
	if _, err := s.logsErrors(context.Background(), errorsIn{Server: pi.addr()}); err != nil {
		t.Errorf("errors mit bösem Namen in der Liste: %v", err)
	}
	if _, err := os.Stat(filepath.Join(s.cfg.Root, "x.jsonl")); err == nil {
		t.Error("Datei außerhalb des Ordners angelegt")
	}
}

// B-771 Review: Ziffern-Hosts (Dezimal, Hex) sind keine Namen; der Dialer prüft die verbundene Adresse.
func TestRemote_ziffernHostUndDialer(t *testing.T) {
	t.Parallel()
	for _, bad := range []string{"2130706433:8080", "134744072:80", "0x7f000001:80", "[::ffff:8.8.8.8]:80", "[2001:4860:4860::8888]:80"} {
		if got, err := resolveRemote(bad); err == nil {
			t.Errorf("%q ist erlaubt: %+v", bad, got)
		}
	}
	for addr, ok := range map[string]bool{"127.0.0.1:80": true, "192.168.2.10:8080": true, "[fe80::1]:80": true, "8.8.8.8:80": false, "[2001:4860:4860::8888]:80": false, "kaputt": false} {
		if err := homeControl("tcp", addr, nil); (err == nil) != ok {
			t.Errorf("homeControl(%q) = %v, erwartet ok=%v", addr, err, ok)
		}
	}
}

// B-771 Review: Ein Name aus K3C_REMOTE_SERVERS, der ein Ordnername wäre („..“, mit Punkt), wird übersprungen.
func TestRemote_ungueltigerKurzname(t *testing.T) {
	t.Setenv(remoteEnv, "..=localhost:1,a.b=localhost:2,gut=localhost:3")
	known := remoteServers()
	if _, ok := known[".."]; ok {
		t.Error("Name .. wurde übernommen")
	}
	if _, ok := known["a.b"]; ok {
		t.Error("Name mit Punkt wurde übernommen")
	}
	if known["gut"] != "http://localhost:3" {
		t.Errorf("gültiger Name fehlt: %v", known)
	}
}

// B-771 Review: logs_since ohne X-Log-Size ist ein Fehler (sonst fängt der Aufrufer bei 0 neu an), eine zu große Antwort auch.
func TestRemote_cursorPflichtUndGroesse(t *testing.T) {
	t.Parallel()
	s := logServer(t)
	ctx := context.Background()
	pi := newFakePi(t, 0, "")
	pi.noSize = true
	if _, err := s.logsSince(ctx, sinceIn{Server: pi.addr(), Source: "k3c-server"}); err == nil || !strings.Contains(err.Error(), "X-Log-Size") {
		t.Errorf("ohne Cursor: %v", err)
	}
	big := newFakePi(t, 0, "")
	big.big = true
	if _, err := s.logsSince(ctx, sinceIn{Server: big.addr(), Source: "k3c-server"}); err == nil || !strings.Contains(err.Error(), "größer als") {
		t.Errorf("zu große Antwort: %v", err)
	}
}

// B-771 Review: Parallele Aufrufe auf denselben Server stören sich nicht (jeder Aufruf hat seinen eigenen Ordner) und räumen auf.
func TestRemote_parallel(t *testing.T) {
	t.Parallel()
	pi := newFakePi(t, 0, "")
	s := logServer(t)
	var wg sync.WaitGroup
	errs := make(chan string, 16)
	for i := 0; i < 8; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			text, err := s.logsErrors(context.Background(), errorsIn{Server: pi.addr()})
			if err != nil || !strings.Contains(text, "Absturz im Raum") {
				errs <- fmt.Sprintf("%v %q", err, text)
			}
		}()
	}
	wg.Wait()
	close(errs)
	for e := range errs {
		t.Error(e)
	}
	left, _ := filepath.Glob(filepath.Join(s.cfg.Root, ".work", "remote-logs", "*"))
	if len(left) != 0 {
		t.Errorf("Ordner nicht aufgeräumt: %v", left)
	}
}
