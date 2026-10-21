package main

import (
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

// call schickt eine Anfrage an den Handler; Gegenstelle, Host und Kopfzeile sind einstellbar.
func call(h http.Handler, method, path, remote, host string, header bool) *httptest.ResponseRecorder {
	req := httptest.NewRequest(method, path, nil)
	req.RemoteAddr, req.Host = remote, host
	if header {
		req.Header.Set(starterHeader, starterValue)
	}
	rec := httptest.NewRecorder()
	h.ServeHTTP(rec, req)
	return rec
}

// Die API liefert denselben Zustand wie die Wails-Bindung, auch ohne gefundenes Layout (Grund als Meldung).
func TestAPIStateOhneLayout(t *testing.T) {
	h := newAPIHandler(newApp(layout{}, errors.New("dist fehlt")))
	rec := call(h, http.MethodGet, "/api/starter/state", "127.0.0.1:50000", "localhost:5194", true)
	if rec.Code != http.StatusOK {
		t.Fatalf("Status %d", rec.Code)
	}
	var st Status
	if err := json.Unmarshal(rec.Body.Bytes(), &st); err != nil {
		t.Fatal(err)
	}
	if st.Message != "dist fehlt" || st.Running || st.Log == nil || st.URLs == nil {
		t.Fatalf("Zustand: %+v", st)
	}
	if !strings.HasPrefix(rec.Header().Get("Content-Type"), "application/json") {
		t.Fatalf("Content-Type %q", rec.Header().Get("Content-Type"))
	}
}

func TestAPIAblehnung(t *testing.T) {
	h := newAPIHandler(newApp(layout{}, errors.New("x")))
	cases := []struct {
		name, remote, host string
		header             bool
	}{
		{"fremde Gegenstelle", "192.168.178.7:50000", "localhost:5194", true},
		{"fremder Host (Rebinding)", "127.0.0.1:50000", "evil.example:5194", true},
		{"LAN-Adresse als Host", "127.0.0.1:50000", "192.168.178.20:8194", true},
		{"Kopfzeile fehlt", "127.0.0.1:50000", "localhost:5194", false},
		{"Gegenstelle ohne Port", "127.0.0.1", "localhost:5194", true},
	}
	for _, c := range cases {
		if rec := call(h, http.MethodGet, "/api/starter/state", c.remote, c.host, c.header); rec.Code != http.StatusForbidden {
			t.Errorf("%s: Status %d, erwartet 403", c.name, rec.Code)
		}
	}
}

// Fremde Seiten im Browser scheitern an Origin und Sec-Fetch-Site, auch bei GET (der Zustand enthält das Server-Log).
func TestAPIFremdeHerkunft(t *testing.T) {
	h := newAPIHandler(newApp(layout{}, errors.New("x")))
	cases := []struct {
		name, method, origin, site string
		want                       int
	}{
		{"fremder Origin GET", http.MethodGet, "http://evil.example", "", http.StatusForbidden},
		{"fremder Origin POST", http.MethodPost, "http://evil.example:5194", "", http.StatusForbidden},
		{"Loopback-Origin anderer Port", http.MethodGet, "http://localhost:9999", "", http.StatusForbidden},
		{"kaputter Origin", http.MethodGet, "://x", "", http.StatusForbidden},
		{"cross-site", http.MethodGet, "", "cross-site", http.StatusForbidden},
		{"same-site", http.MethodPost, "", "same-site", http.StatusForbidden},
		{"gleicher Origin", http.MethodGet, "http://localhost:5194", "same-origin", http.StatusOK},
		{"Adressleiste (none)", http.MethodGet, "", "none", http.StatusOK},
		{"ohne Browser-Kopfzeilen", http.MethodGet, "", "", http.StatusOK},
	}
	for _, c := range cases {
		req := httptest.NewRequest(c.method, "/api/starter/"+map[string]string{http.MethodGet: "state", http.MethodPost: "stop"}[c.method], nil)
		req.RemoteAddr, req.Host = "127.0.0.1:50000", "localhost:5194"
		req.Header.Set(starterHeader, starterValue)
		if c.origin != "" {
			req.Header.Set("Origin", c.origin)
		}
		if c.site != "" {
			req.Header.Set("Sec-Fetch-Site", c.site)
		}
		rec := httptest.NewRecorder()
		h.ServeHTTP(rec, req)
		if rec.Code != c.want {
			t.Errorf("%s: Status %d, erwartet %d", c.name, rec.Code, c.want)
		}
	}
}

func TestAPIErlaubtLoopbackVarianten(t *testing.T) {
	h := newAPIHandler(newApp(layout{}, errors.New("x")))
	for _, c := range [][2]string{{"127.0.0.1:1", "127.0.0.1:8194"}, {"[::1]:1", "[::1]:8194"}, {"127.0.0.1:1", "localhost"}} {
		if rec := call(h, http.MethodGet, "/api/starter/state", c[0], c[1], true); rec.Code != http.StatusOK {
			t.Errorf("%v: Status %d", c, rec.Code)
		}
	}
}

func TestAPIMethoden(t *testing.T) {
	h := newAPIHandler(newApp(layout{}, errors.New("x")))
	if rec := call(h, http.MethodPost, "/api/starter/state", "127.0.0.1:1", "localhost", true); rec.Code != http.StatusMethodNotAllowed {
		t.Errorf("POST state: %d", rec.Code)
	}
	if rec := call(h, http.MethodGet, "/api/starter/start", "127.0.0.1:1", "localhost", true); rec.Code != http.StatusMethodNotAllowed {
		t.Errorf("GET start: %d", rec.Code)
	}
}

// Start und Stop über die API steuern denselben Supervisor wie das Fenster.
func TestAPIStartStop(t *testing.T) {
	s := fakeSupervisor(t, freePort(t))
	h := newAPIHandler(&App{sup: s})
	rec := call(h, http.MethodPost, "/api/starter/start", "127.0.0.1:1", "localhost", true)
	if rec.Code != http.StatusOK {
		t.Fatalf("start: %d", rec.Code)
	}
	waitState(t, s, func(st Status) bool { return st.Running })
	rec = call(h, http.MethodPost, "/api/starter/stop", "127.0.0.1:1", "localhost", true)
	var st Status
	if err := json.Unmarshal(rec.Body.Bytes(), &st); err != nil {
		t.Fatal(err)
	}
	if st.Running || st.Message != "Server ist gestoppt." {
		t.Fatalf("nach stop: %+v", st)
	}
}
