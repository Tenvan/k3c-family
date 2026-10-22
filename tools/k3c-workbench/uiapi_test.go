package main

import (
	"bufio"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"reflect"
	"regexp"
	"slices"
	"strings"
	"testing"
	"time"
)

// uiApp ist eine App ohne startup: genug für Info, Planung und den Bus.
func uiApp(t *testing.T) *App {
	t.Helper()
	return &App{root: t.TempDir(), bus: newEventBus()}
}

// uiCall schickt eine Anfrage direkt an den Handler (ohne Netz) und liefert Status und Antwort.
func uiCall(t *testing.T, a *App, method, path, body string, edit func(*http.Request)) (int, map[string]any) {
	t.Helper()
	r := httptest.NewRequest(method, path, strings.NewReader(body))
	r.RemoteAddr = "127.0.0.1:50000"
	r.Host = "127.0.0.1:8191"
	r.Header.Set(uiHeader, "1")
	if edit != nil {
		edit(r)
	}
	w := httptest.NewRecorder()
	a.uiHandler().ServeHTTP(w, r)
	var out map[string]any
	_ = json.Unmarshal(w.Body.Bytes(), &out)
	return w.Code, out
}

// k3c-workbench startet Prozesse: nur Loopback, nur localhost/Loopback-IP als Host, POST nur mit Kopfzeile.
func TestUIGuard(t *testing.T) {
	a := uiApp(t)
	cases := map[string]func(*http.Request){
		"LAN-Adresse":    func(r *http.Request) { r.RemoteAddr = "192.168.1.20:50000" },
		"fremder Host":   func(r *http.Request) { r.Host = "evil.example:8191" },
		"LAN-Host":       func(r *http.Request) { r.Host = "192.168.1.20:8191" },
		"ohne Kopfzeile": func(r *http.Request) { r.Header.Del(uiHeader) },
		"falscher Wert":  func(r *http.Request) { r.Header.Set(uiHeader, "ja") },
		"kodierter Pfad": func(r *http.Request) { r.URL.RawPath = "/api/dev/rpc/%49nfo" },
		"fremde Origin":  func(r *http.Request) { r.Header.Set("Origin", "https://evil.example") },
		"lokale Seite":   func(r *http.Request) { r.Header.Set("Origin", "http://127.0.0.1:5183") },
		"Origin null":    func(r *http.Request) { r.Header.Set("Origin", "null") },
		"cross-site":     func(r *http.Request) { r.Header.Set("Sec-Fetch-Site", "cross-site") },
		"same-site":      func(r *http.Request) { r.Header.Set("Sec-Fetch-Site", "same-site") },
	}
	for name, edit := range cases {
		if code, _ := uiCall(t, a, http.MethodPost, uiPrefix+"rpc/Info", "[]", edit); code != http.StatusForbidden {
			t.Errorf("%s: Status %d, erwartet 403", name, code)
		}
	}
	for _, host := range []string{"localhost:5191", "127.0.0.1:5191", "[::1]:5191"} {
		code, _ := uiCall(t, a, http.MethodPost, uiPrefix+"rpc/Info", "[]", func(r *http.Request) { r.Host = host })
		if code != http.StatusOK {
			t.Errorf("Host %s: Status %d, erwartet 200", host, code)
		}
	}
	// Die Oberfläche hinter Vite: Origin gleich Host, Sec-Fetch-Site same-origin; auch für den Ereignisstrom.
	vite := func(r *http.Request) {
		r.Host = "localhost:5191"
		r.Header.Set("Origin", "http://localhost:5191")
		r.Header.Set("Sec-Fetch-Site", "same-origin")
	}
	if code, _ := uiCall(t, a, http.MethodPost, uiPrefix+"rpc/Info", "[]", vite); code != http.StatusOK {
		t.Errorf("Oberfläche hinter Vite: Status %d, erwartet 200", code)
	}
	foreign := func(r *http.Request) { r.Header.Set("Origin", "http://localhost:5183") }
	if code, _ := uiCall(t, a, http.MethodGet, uiPrefix+"events", "", foreign); code != http.StatusForbidden {
		t.Errorf("Ereignisstrom für fremde Seite: Status %d, erwartet 403", code)
	}
}

func TestUIRPC(t *testing.T) {
	a := uiApp(t)
	code, out := uiCall(t, a, http.MethodPost, uiPrefix+"rpc/Info", "[]", nil)
	if res, _ := out["result"].(map[string]any); code != http.StatusOK || res["version"] != version {
		t.Errorf("Info: %d %v", code, out)
	}
	// Argumente kommen als JSON-Array; ein Fehler aus Go kommt als {error} mit 422.
	if code, out := uiCall(t, a, http.MethodPost, uiPrefix+"rpc/PlanningDoc", `["gibtsnicht"]`, nil); code != http.StatusUnprocessableEntity || out["error"] == nil {
		t.Errorf("PlanningDoc: %d %v", code, out)
	}
	bad := map[string]string{"zu wenige": `[]`, "falscher Typ": `[5]`, "kein Array": `{}`}
	for name, body := range bad {
		if code, _ := uiCall(t, a, http.MethodPost, uiPrefix+"rpc/PlanningDoc", body, nil); code != http.StatusBadRequest {
			t.Errorf("%s: Status %d, erwartet 400", name, code)
		}
	}
}

// Nur die Methoden der Oberfläche: exportiert, aber MCP-intern (TaskList) oder unbekannt heißt 404.
func TestUIRPCUnbekannt(t *testing.T) {
	a := uiApp(t)
	for _, m := range []string{"TaskList", "TaskState", "Gibtsnicht", "startup"} {
		if code, out := uiCall(t, a, http.MethodPost, uiPrefix+"rpc/"+m, "[]", nil); code != http.StatusNotFound || out["error"] == nil {
			t.Errorf("%s: %d %v", m, code, out)
		}
	}
}

// Die Freigabeliste ist genau GoApp aus frontend/src/api/wails.ts, und jede Methode gibt es an App.
func TestUIMethodsWieWailsTS(t *testing.T) {
	src, err := os.ReadFile("frontend/src/api/wails.ts")
	if err != nil {
		t.Fatal(err)
	}
	body := regexp.MustCompile(`(?s)interface GoApp \{(.*?)\n\}`).FindSubmatch(src)
	if body == nil {
		t.Fatal("interface GoApp nicht gefunden")
	}
	var ts []string
	for _, m := range regexp.MustCompile(`(?m)^\s+(\w+)\(`).FindAllSubmatch(body[1], -1) {
		ts = append(ts, string(m[1]))
	}
	var goNames []string
	typ := reflect.TypeFor[*App]()
	for name := range uiMethods {
		goNames = append(goNames, name)
		if _, ok := typ.MethodByName(name); !ok {
			t.Errorf("App hat keine Methode %s", name)
		}
	}
	slices.Sort(ts)
	slices.Sort(goNames)
	if !slices.Equal(ts, goNames) {
		t.Errorf("GoApp = %v\nuiMethods = %v", ts, goNames)
	}
}

// emit erreicht offene Ströme als `event: <name>` mit dem ersten Argument als JSON, ohne Argument null.
func TestUIEvents(t *testing.T) {
	a := uiApp(t)
	srv := httptest.NewServer(a.uiHandler())
	defer srv.Close()
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	req, _ := http.NewRequestWithContext(ctx, http.MethodGet, srv.URL+uiPrefix+"events", nil)
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = res.Body.Close() }()
	if ct := res.Header.Get("Content-Type"); ct != "text/event-stream" {
		t.Fatalf("Content-Type = %q", ct)
	}
	rd := bufio.NewReader(res.Body)
	if line, _ := rd.ReadString('\n'); !strings.HasPrefix(line, ": offen") {
		t.Fatalf("erste Zeile = %q", line)
	}
	a.emitAll(ctx, evMCPState, MCPState{Addr: "127.0.0.1:5190", Listening: true})
	a.emitAll(ctx, evPlanning, nil)
	a.emitAll(ctx, evPlanning)
	want := []string{
		"event: mcp:state", `data: {"addr":"127.0.0.1:5190","listening":true,"error":""}`,
		"event: planning:changed", "data: null", "event: planning:changed", "data: null",
	}
	var got []string
	for len(got) < len(want) {
		line, err := rd.ReadString('\n')
		if err != nil {
			t.Fatalf("Strom abgebrochen nach %v: %v", got, err)
		}
		if line = strings.TrimSpace(line); line != "" {
			got = append(got, line)
		}
	}
	if !slices.Equal(got, want) {
		t.Errorf("Strom = %q", got)
	}
}
