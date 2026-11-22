package uikit

import (
	"bufio"
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"reflect"
	"slices"
	"strings"
	"testing"
	"time"
)

// target ist eine App mit Bindings im Kleinen.
type target struct{}

func (target) Info() map[string]string { return map[string]string{"version": "test"} }
func (target) Doc(name string) (string, error) {
	if name == "gibtsnicht" {
		return "", errors.New("unbekanntes Dokument")
	}
	return "# " + name, nil
}
func (target) Intern() string { return "nur MCP" }

func testAPI() *API {
	return &API{App: "k3c-test", Target: target{}, Methods: map[string]bool{"Info": true, "Doc": true}, Bus: NewBus()}
}

// call schickt eine Anfrage direkt an den Handler (ohne Netz) und liefert Status und Antwort.
func call(t *testing.T, h http.Handler, method, path, body string, edit func(*http.Request)) (int, map[string]any) {
	t.Helper()
	r := httptest.NewRequest(method, path, strings.NewReader(body))
	r.RemoteAddr = "127.0.0.1:50000"
	r.Host = "127.0.0.1:8191"
	r.Header.Set(Header, "1")
	if edit != nil {
		edit(r)
	}
	w := httptest.NewRecorder()
	h.ServeHTTP(w, r)
	var out map[string]any
	_ = json.Unmarshal(w.Body.Bytes(), &out)
	return w.Code, out
}

// Die Apps starten Prozesse: nur Loopback, nur localhost/Loopback-IP als Host, POST nur mit Kopfzeile.
func TestGuard(t *testing.T) {
	h := testAPI().Handler()
	cases := map[string]func(*http.Request){
		"LAN-Adresse":    func(r *http.Request) { r.RemoteAddr = "192.168.1.20:50000" },
		"fremder Host":   func(r *http.Request) { r.Host = "evil.example:8191" },
		"LAN-Host":       func(r *http.Request) { r.Host = "192.168.1.20:8191" },
		"ohne Kopfzeile": func(r *http.Request) { r.Header.Del(Header) },
		"falscher Wert":  func(r *http.Request) { r.Header.Set(Header, "ja") },
		"kodierter Pfad": func(r *http.Request) { r.URL.RawPath = "/api/dev/rpc/%49nfo" },
		"fremde Origin":  func(r *http.Request) { r.Header.Set("Origin", "https://evil.example") },
		"lokale Seite":   func(r *http.Request) { r.Header.Set("Origin", "http://127.0.0.1:5183") },
		"Origin null":    func(r *http.Request) { r.Header.Set("Origin", "null") },
		"cross-site":     func(r *http.Request) { r.Header.Set("Sec-Fetch-Site", "cross-site") },
		"same-site":      func(r *http.Request) { r.Header.Set("Sec-Fetch-Site", "same-site") },
	}
	for name, edit := range cases {
		if code, _ := call(t, h, http.MethodPost, Prefix+"rpc/Info", "[]", edit); code != http.StatusForbidden {
			t.Errorf("%s: Status %d, erwartet 403", name, code)
		}
	}
	for _, host := range []string{"localhost:5191", "127.0.0.1:5191", "[::1]:5191"} {
		code, _ := call(t, h, http.MethodPost, Prefix+"rpc/Info", "[]", func(r *http.Request) { r.Host = host })
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
	if code, _ := call(t, h, http.MethodPost, Prefix+"rpc/Info", "[]", vite); code != http.StatusOK {
		t.Errorf("Oberfläche hinter Vite: Status %d, erwartet 200", code)
	}
	foreign := func(r *http.Request) { r.Header.Set("Origin", "http://localhost:5183") }
	if code, _ := call(t, h, http.MethodGet, Prefix+"events", "", foreign); code != http.StatusForbidden {
		t.Errorf("Ereignisstrom für fremde Seite: Status %d, erwartet 403", code)
	}
}

func TestRPC(t *testing.T) {
	h := testAPI().Handler()
	code, out := call(t, h, http.MethodPost, Prefix+"rpc/Info", "[]", nil)
	if res, _ := out["result"].(map[string]any); code != http.StatusOK || res["version"] != "test" {
		t.Errorf("Info: %d %v", code, out)
	}
	// Argumente kommen als JSON-Array; ein Fehler aus Go kommt als {error} mit 422.
	if code, out := call(t, h, http.MethodPost, Prefix+"rpc/Doc", `["gibtsnicht"]`, nil); code != http.StatusUnprocessableEntity || out["error"] == nil {
		t.Errorf("Doc: %d %v", code, out)
	}
	bad := map[string]string{"zu wenige": `[]`, "falscher Typ": `[5]`, "kein Array": `{}`}
	for name, body := range bad {
		if code, _ := call(t, h, http.MethodPost, Prefix+"rpc/Doc", body, nil); code != http.StatusBadRequest {
			t.Errorf("%s: Status %d, erwartet 400", name, code)
		}
	}
}

// Nur die freigegebenen Methoden: exportiert, aber nicht freigegeben (Intern) oder unbekannt heißt 404.
func TestRPCUnbekannt(t *testing.T) {
	h := testAPI().Handler()
	for _, m := range []string{"Intern", "Gibtsnicht", "info"} {
		if code, out := call(t, h, http.MethodPost, Prefix+"rpc/"+m, "[]", nil); code != http.StatusNotFound || out["error"] == nil {
			t.Errorf("%s: %d %v", m, code, out)
		}
	}
}

// Track misst jeden Aufruf unter "ui:<Methode>".
func TestRPCTrack(t *testing.T) {
	api := testAPI()
	var got []string
	api.Track = func(name string) func() { got = append(got, name); return func() {} }
	call(t, api.Handler(), http.MethodPost, Prefix+"rpc/Info", "[]", nil)
	if !slices.Equal(got, []string{"ui:Info"}) {
		t.Errorf("Track = %v", got)
	}
}

func TestCheckMethods(t *testing.T) {
	ts := []byte("interface GoApp {\n  Info(): Promise<Info>;\n  Doc(name: string): Promise<string>;\n}\n")
	typ := reflect.TypeFor[target]()
	if err := CheckMethods(ts, map[string]bool{"Info": true, "Doc": true}, typ); err != nil {
		t.Error(err)
	}
	if err := CheckMethods(ts, map[string]bool{"Info": true}, typ); err == nil {
		t.Error("fehlende Methode in der Liste nicht bemerkt")
	}
	if err := CheckMethods(ts, map[string]bool{"Info": true, "Doc": true, "Fehlt": true}, typ); err == nil {
		t.Error("Methode ohne Gegenstück an der App nicht bemerkt")
	}
	if _, err := TSMethods([]byte("type X = {}")); err == nil {
		t.Error("fehlendes interface GoApp nicht bemerkt")
	}
}

// Publish erreicht offene Ströme als `event: <name>` mit dem ersten Argument als JSON, ohne Argument null.
func TestEvents(t *testing.T) {
	api := testAPI()
	srv := httptest.NewServer(api.Handler())
	defer srv.Close()
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	req, _ := http.NewRequestWithContext(ctx, http.MethodGet, srv.URL+Prefix+"events", nil)
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
	api.Bus.Publish("mcp:state", []any{map[string]any{"addr": "127.0.0.1:5190", "listening": true}})
	api.Bus.Publish("planning:changed", []any{nil})
	api.Bus.Publish("planning:changed", nil)
	want := []string{
		"event: mcp:state", `data: {"addr":"127.0.0.1:5190","listening":true}`,
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

func TestResolvePort(t *testing.T) {
	if p, err := ResolvePort("X", " ", 8191); err != nil || p != 8191 {
		t.Errorf("leer: %d %v", p, err)
	}
	if p, err := ResolvePort("X", "8193", 8191); err != nil || p != 8193 {
		t.Errorf("8193: %d %v", p, err)
	}
	for _, raw := range []string{"abc", "0", "70000"} {
		if _, err := ResolvePort("X", raw, 8191); err == nil {
			t.Errorf("%q angenommen", raw)
		}
	}
}
