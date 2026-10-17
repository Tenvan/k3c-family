package main

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync/atomic"
	"testing"
)

// WZ7.4 (B-460/AC-06): „Im Spiel öffnen“ neu und voll ausgebaut über die Landingpage, nur Loopback.

// gameServer ist ein Spielserver auf Loopback (httptest): /api/health antwortet 200, fullsave antwortet mit status und body
// und zählt die Aufrufe.
func gameServer(t *testing.T, status int, body string) (*httptest.Server, *atomic.Int32, *atomic.Value) {
	t.Helper()
	var calls atomic.Int32
	var lastSeed atomic.Value
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/health" {
			w.WriteHeader(http.StatusOK)
			return
		}
		calls.Add(1)
		var in struct {
			Seed string `json:"seed"`
		}
		_ = json.NewDecoder(r.Body).Decode(&in)
		lastSeed.Store(in.Seed)
		if r.Method != http.MethodPost || r.URL.Path != "/api/dev/fullsave" {
			http.NotFound(w, r)
			return
		}
		w.WriteHeader(status)
		_, _ = w.Write([]byte(body))
	}))
	t.Cleanup(srv.Close)
	return srv, &calls, &lastSeed
}

func opener() (*App, *[]string) {
	var opened []string
	return &App{open: func(u string) { opened = append(opened, u) }}, &opened
}

func TestOpenGameNeuOeffnetDieLandingpageMitDemSeed(t *testing.T) {
	srv, calls, _ := gameServer(t, 200, `{}`)
	a, opened := opener()
	res := a.WorldOpenGame(srv.URL, "probe", "forest", "new")
	if res.Error != "" || res.Save != "probe" {
		t.Fatalf("Ergebnis %+v", res)
	}
	want := srv.URL + "/#game.html%3Fautostart%3D1%26save%3Dprobe"
	if res.URL != want || len(*opened) != 1 || (*opened)[0] != want {
		t.Errorf("geöffnet %v, erwartet %s", *opened, want)
	}
	if calls.Load() != 0 {
		t.Error("ein neues Spiel braucht keine Dev-API")
	}
}

func TestOpenGameNeuNurWaldUndNamensformat(t *testing.T) {
	srv, _, _ := gameServer(t, 200, `{}`)
	a, opened := opener()
	for name, c := range map[string][2]string{
		"Höhle": {"probe", "cave"}, "Mine": {"probe", "mine"}, "Seed groß": {"Probe", "forest"}, "Seed leer": {"", "forest"},
		"Seed zu lang": {strings.Repeat("k", 33), "forest"}, "Seed mit Leerzeichen": {"mit leerzeichen", "forest"},
	} {
		if res := a.WorldOpenGame(srv.URL, c[0], c[1], "new"); res.Error == "" || res.URL != "" {
			t.Errorf("%s: %+v", name, res)
		}
	}
	if len(*opened) != 0 {
		t.Errorf("trotz Hinweis geöffnet: %v", *opened)
	}
	if res := a.WorldOpenGame(srv.URL, strings.Repeat("k", 32), "forest", "new"); res.Error != "" {
		t.Errorf("32 Zeichen sind erlaubt: %s", res.Error)
	}
}

func TestOpenGameVollLegtDenSpielstandAnUndLaedtIhn(t *testing.T) {
	srv, calls, seed := gameServer(t, 200, `{"save":"voll-abc"}`)
	a, opened := opener()
	res := a.WorldOpenGame(srv.URL, "abc", "cave", "full") // voll ausgebaut ist nicht auf den Wald beschränkt
	if res.Error != "" || res.Save != "voll-abc" {
		t.Fatalf("Ergebnis %+v", res)
	}
	if got, _ := seed.Load().(string); calls.Load() != 1 || got != "abc" {
		t.Errorf("Dev-API %d-mal mit Seed %q aufgerufen", calls.Load(), got)
	}
	if want := srv.URL + "/#game.html%3Fautostart%3D1%26save%3Dvoll-abc"; len(*opened) != 1 || (*opened)[0] != want {
		t.Errorf("geöffnet %v, erwartet %s", *opened, want)
	}
}

func TestOpenGameVollHinweiseDesServers(t *testing.T) {
	cases := map[string]struct {
		status int
		body   string
		hint   string
	}{
		"kein Dev-Mode":  {403, `{"error":"verboten"}`, "Dev-Mode"},
		"Raum belegt":    {409, `{"error":"Spielstand ist gerade in einem Raum mit verbundenem Gerät offen"}`, "Raum mit verbundenem Gerät"},
		"Server kaputt":  {500, `{}`, "Fehler 500"},
		"keine Antwort":  {200, `{}`, "keinen Spielstand"},
		"falscher Name":  {200, `{"save":"abc"}`, "keinen Spielstand"},
		"Seed abgelehnt": {400, `{"error":"Seed ungültig"}`, "Seed ungültig"},
	}
	for name, c := range cases {
		srv, _, _ := gameServer(t, c.status, c.body)
		a, opened := opener()
		res := a.WorldOpenGame(srv.URL, "abc", "forest", "full")
		if !strings.Contains(res.Error, c.hint) || res.URL != "" || len(*opened) != 0 {
			t.Errorf("%s: %+v, geöffnet %v", name, res, *opened)
		}
	}
}

func TestOpenGameVollSeedFormatUndServerNichtErreichbar(t *testing.T) {
	srv, calls, _ := gameServer(t, 200, `{"save":"voll-abc"}`)
	a, _ := opener()
	for _, seed := range []string{"", "Abc", strings.Repeat("k", 28), "a b"} {
		if res := a.WorldOpenGame(srv.URL, seed, "forest", "full"); res.Error == "" {
			t.Errorf("Seed %q wurde nicht abgelehnt", seed)
		}
	}
	if calls.Load() != 0 {
		t.Error("bei ungültigem Seed darf die Dev-API nicht aufgerufen werden")
	}
	closed := httptest.NewServer(http.NotFoundHandler())
	url := closed.URL
	closed.Close()
	if res := a.WorldOpenGame(url, "abc", "forest", "full"); !strings.Contains(res.Error, "Spielserver läuft nicht") {
		t.Errorf("Server aus: %+v", res)
	}
}

func TestOpenGameNurLoopback(t *testing.T) {
	srv, calls, _ := gameServer(t, 200, `{"save":"voll-abc"}`)
	a, opened := opener()
	for _, base := range []string{
		"http://example.com", "https://192.168.1.20:8080", "http://10.0.0.5", "http://localhost.evil.com", "http://127.0.0.1.evil.com",
		"http://user:pw@127.0.0.1:8080", "ftp://127.0.0.1", "javascript:alert(1)", "", "127.0.0.1:8080", "http://",
	} {
		if res := a.WorldOpenGame(base, "abc", "forest", "full"); res.Error == "" || res.URL != "" {
			t.Errorf("%q wurde nicht abgelehnt: %+v", base, res)
		}
	}
	if len(*opened) != 0 || calls.Load() != 0 {
		t.Errorf("fremde Adresse angesprochen: geöffnet %v, Aufrufe %d", *opened, calls.Load())
	}
	for _, base := range []string{srv.URL, "http://localhost:8080", "http://[::1]:8080", "https://127.0.0.2"} {
		if _, err := loopbackBase(base); err != nil {
			t.Errorf("%q gilt als Loopback: %v", base, err)
		}
	}
	if base, _ := loopbackBase("http://127.0.0.1:8080/pfad?x=1"); base != "http://127.0.0.1:8080" {
		t.Errorf("Pfad und Abfrage bleiben draußen: %q", base)
	}
}

func TestOpenGameUnbekannteArt(t *testing.T) {
	a, opened := opener()
	if res := a.WorldOpenGame("http://127.0.0.1:8080", "abc", "forest", "dev"); res.Error == "" || len(*opened) != 0 {
		t.Errorf("%+v", res)
	}
}

func TestOpenGameServerAusKeinBrowser(t *testing.T) {
	closed := httptest.NewServer(http.NotFoundHandler())
	url := closed.URL
	closed.Close()
	a, opened := opener()
	for _, mode := range []string{"new", "full"} {
		if res := a.WorldOpenGame(url, "abc", "forest", mode); !strings.Contains(res.Error, "Spielserver läuft nicht") || res.URL != "" {
			t.Errorf("%s: %+v", mode, res)
		}
	}
	notHealthy := httptest.NewServer(http.NotFoundHandler()) // antwortet, ist aber kein Spielserver (/api/health → 404)
	defer notHealthy.Close()
	if res := a.WorldOpenGame(notHealthy.URL, "abc", "forest", "new"); !strings.Contains(res.Error, "Spielserver läuft nicht") {
		t.Errorf("kein Spielserver: %+v", res)
	}
	if len(*opened) != 0 {
		t.Errorf("trotz Hinweis geöffnet: %v", *opened)
	}
}

// Ein Dienst auf dem Loopback-Port, der weiterleitet, bringt das Studio nicht dazu, eine fremde Adresse anzusprechen.
func TestOpenGameFolgtKeinerWeiterleitung(t *testing.T) {
	var foreign atomic.Int32
	other := httptest.NewServer(http.HandlerFunc(func(http.ResponseWriter, *http.Request) { foreign.Add(1) }))
	defer other.Close()
	redirect := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/health" {
			w.WriteHeader(http.StatusOK)
			return
		}
		http.Redirect(w, r, other.URL+"/api/dev/fullsave", http.StatusTemporaryRedirect)
	}))
	defer redirect.Close()
	a, opened := opener()
	res := a.WorldOpenGame(redirect.URL, "abc", "forest", "full")
	if res.Error == "" || foreign.Load() != 0 || len(*opened) != 0 {
		t.Errorf("Weiterleitung gefolgt: %+v, fremde Aufrufe %d, geöffnet %v", res, foreign.Load(), *opened)
	}
	redirectHealth := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, other.URL+"/api/health", http.StatusFound)
	}))
	defer redirectHealth.Close()
	if res := a.WorldOpenGame(redirectHealth.URL, "abc", "forest", "new"); res.Error == "" || foreign.Load() != 0 {
		t.Errorf("Weiterleitung der Gesundheitsprüfung gefolgt: %+v, fremde Aufrufe %d", res, foreign.Load())
	}
}

func TestOpenGameAntwortMitUngueltigemSpielstandName(t *testing.T) {
	for _, name := range []string{"voll-A", "voll-" + strings.Repeat("k", 40), "voll-a b"} {
		srv, _, _ := gameServer(t, 200, `{"save":"`+name+`"}`)
		a, opened := opener()
		if res := a.WorldOpenGame(srv.URL, "abc", "forest", "full"); res.Error == "" || len(*opened) != 0 {
			t.Errorf("Name %q wurde angenommen: %+v", name, res)
		}
	}
}
