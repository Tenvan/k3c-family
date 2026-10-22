package mcpsrv

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

const statusJSON = `{"version":"v0.2.0","startedAt":"x","uptimeS":3720,"saves":2,"reports":1,
 "rooms":[
  {"code":"FAMILIE","name":"Familie","depth":0,"taken":2,"free":2,"running":true,"devices":2,"monarchs":["taken","taken","free","free"],"tick":900,"tickMs":{"last":0.4,"p99":1.25}},
  {"code":"TEST1","name":"t","depth":1,"taken":1,"free":3,"running":true,"devices":1,"monarchs":["taken","free","free","free"],"tick":30,"tickMs":{"last":0.2,"p99":0.5}}],
 "failures":[]}`

const roomJSON = `{"code":"FAMILIE","depth":0,"tick":900,"phase":"day","day":2,"gold":[12,7],"troops":{"peasant":3,"archer":1},"enemies":0,"castleHp":100,"wave":1}`

// statusServer antwortet wie engine/net/status.go: Token Pflicht, unbekannter Raum mit JSON-404.
func statusServer(t *testing.T, token string) (*httptest.Server, *string) {
	t.Helper()
	var auth string
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		auth = r.Header.Get("Authorization")
		if token == "" {
			http.NotFound(w, r)
			return
		}
		if auth != "Bearer "+token {
			http.Error(w, `{"error":"Token fehlt oder falsch"}`, http.StatusUnauthorized)
			return
		}
		switch r.URL.Query().Get("room") {
		case "":
			_, _ = w.Write([]byte(statusJSON))
		case "FAMILIE":
			_, _ = w.Write([]byte(roomJSON))
		default:
			http.Error(w, `{"error":"Raum nicht gefunden"}`, http.StatusNotFound)
		}
	}))
	t.Cleanup(ts.Close)
	return ts, &auth
}

func serverTools(t *testing.T) func(string, any) (string, bool) {
	t.Helper()
	cs := connect(t, New(Config{Version: "test"}))
	return func(name string, args any) (string, bool) { return callText(t, cs, name, args) }
}

func TestServerToolsVerdichten(t *testing.T) {
	ts, auth := statusServer(t, "geheim")
	t.Setenv("K3C_SERVER_URL", ts.URL)
	t.Setenv("K3C_STATUS_TOKEN", "geheim")
	call := serverTools(t)

	text, isErr := call("rooms_list", nil)
	lines := strings.Split(text, "\n")
	if isErr || len(lines) != 2 || !strings.HasPrefix(lines[0], "FAMILIE · läuft") || !strings.Contains(lines[0], "p99 1.25 ms") ||
		!strings.Contains(lines[0], "2 Geräte") {
		t.Errorf("rooms_list: %q", text)
	}
	if *auth != "Bearer geheim" {
		t.Errorf("Token nicht als Bearer gesendet: %q", *auth)
	}
	if text, isErr = call("server_status", nil); isErr || !strings.Contains(text, "v0.2.0") || !strings.Contains(text, "1 h 2 min") ||
		!strings.Contains(text, "2 Räume") || !strings.Contains(text, "Abstürze: keine") {
		t.Errorf("server_status: %q", text)
	}
}

func TestRoomSnapshotVerdichtet(t *testing.T) {
	ts, _ := statusServer(t, "geheim")
	t.Setenv("K3C_SERVER_URL", ts.URL)
	t.Setenv("K3C_STATUS_TOKEN", "geheim")
	text, isErr := serverTools(t)("room_snapshot", map[string]any{"room": "FAMILIE"})
	if isErr || !strings.Contains(text, "Tag 2 (day)") || !strings.Contains(text, "12, 7") || !strings.Contains(text, "archer 1, peasant 3") {
		t.Errorf("room_snapshot: %q", text)
	}
}

func TestServerToolsKeineRaeume(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		_, _ = w.Write([]byte(`{"version":"v","uptimeS":1,"rooms":[],"failures":[{"code":"X","name":"n","error":"boom","at":"t"}]}`))
	}))
	defer ts.Close()
	t.Setenv("K3C_SERVER_URL", ts.URL)
	call := serverTools(t)
	if text, _ := call("rooms_list", nil); text != "Keine Räume" {
		t.Errorf("rooms_list: %q", text)
	}
	if text, _ := call("server_status", nil); !strings.Contains(text, "Absturz X (n) t: boom") {
		t.Errorf("server_status: %q", text)
	}
}

func TestServerToolsFehlerMeldungen(t *testing.T) {
	ts, _ := statusServer(t, "geheim")
	t.Setenv("K3C_SERVER_URL", ts.URL)
	call := serverTools(t)

	t.Setenv("K3C_STATUS_TOKEN", "falsch")
	if text, isErr := call("rooms_list", nil); !isErr || !strings.Contains(text, "401, Token prüfen") || strings.Contains(text, "falsch") {
		t.Errorf("401: %q", text)
	}
	t.Setenv("K3C_STATUS_TOKEN", "geheim")
	if text, isErr := call("room_snapshot", map[string]any{"room": "NIX"}); !isErr || !strings.Contains(text, "Raum NIX nicht gefunden") {
		t.Errorf("Raum fehlt: %q", text)
	}

	off, _ := statusServer(t, "")
	t.Setenv("K3C_SERVER_URL", off.URL)
	if text, isErr := call("server_status", nil); !isErr || !strings.Contains(text, "Diagnose am Server aus") {
		t.Errorf("Diagnose aus: %q", text)
	}

	dead := httptest.NewServer(http.NotFoundHandler())
	url := dead.URL
	dead.Close()
	t.Setenv("K3C_SERVER_URL", url)
	if text, isErr := call("server_status", nil); !isErr || !strings.Contains(text, "Server nicht erreichbar unter "+url) {
		t.Errorf("Server aus: %q", text)
	}
}

func TestServerAdresseAusPort(t *testing.T) {
	t.Setenv("K3C_SERVER_URL", "")
	t.Setenv("K3C_HTTP_PORT", "9123")
	t.Setenv("K3C_STATUS_TOKEN", "x")
	call := serverTools(t)
	if text, _ := call("server_status", nil); !strings.Contains(text, "http://127.0.0.1:9123") {
		t.Errorf("Adresse aus K3C_HTTP_PORT: %q", text)
	}
}

func TestEngineToolsLaufenOhneServer(t *testing.T) {
	t.Setenv("K3C_SERVER_URL", "http://127.0.0.1:1") // kein Server: die In-process-Tools brauchen keinen
	call := serverTools(t)
	text, isErr := call("level_generate", map[string]any{"seed": "0", "biome": "forest"})
	if isErr || !strings.HasPrefix(text, "Level forest · Seed \"0\" · Breite 900 Units") {
		t.Errorf("level_generate: %q", text)
	}
	a, isErr := call("sim_run", map[string]any{"seed": "abc", "ticks": 600})
	b, _ := call("sim_run", map[string]any{"seed": "abc", "ticks": 600})
	if isErr || a != b || !strings.Contains(a, "600 Ticks") {
		t.Errorf("sim_run: %q / %q", a, b)
	}
	if text, isErr = call("sim_run", map[string]any{"seed": "abc", "ticks": 100001}); !isErr || !strings.Contains(text, "100000") {
		t.Errorf("Grenze: %q", text)
	}
}
