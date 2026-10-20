package backend

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync/atomic"
	"testing"
)

// B-513: „Im Spiel öffnen“ wahlweise auf einem Spielserver (Loopback) statt in der Engine des Studios; der Server wird
// über GET /api/health erkannt, „voll ausgebaut“ über POST /api/dev/fullsave angelegt (nur im Dev-Mode des Servers).

// gameServer ist ein Spielserver auf Loopback (httptest): /api/health antwortet 200, /api/dev meldet dev, fullsave
// antwortet mit status und body und zählt die Aufrufe samt letztem Seed.
func gameServer(t *testing.T, dev bool, status int, body string) (*httptest.Server, *atomic.Int32, *atomic.Value) {
	t.Helper()
	var calls atomic.Int32
	var lastSeed atomic.Value
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch {
		case r.URL.Path == "/api/health":
			_, _ = w.Write([]byte(`{"ok":true,"version":"v9"}`))
		case r.URL.Path == "/api/dev" && r.Method == http.MethodGet:
			_ = json.NewEncoder(w).Encode(map[string]any{"dev": dev, "rooms": []any{}})
		case r.URL.Path == "/api/dev/fullsave" && r.Method == http.MethodPost:
			calls.Add(1)
			var in struct {
				Seed string `json:"seed"`
			}
			_ = json.NewDecoder(r.Body).Decode(&in)
			lastSeed.Store(in.Seed)
			w.WriteHeader(status)
			_, _ = w.Write([]byte(body))
		default:
			http.NotFound(w, r)
		}
	}))
	t.Cleanup(srv.Close)
	return srv, &calls, &lastSeed
}

// closedURL ist eine Loopback-Adresse, an der nichts mehr lauscht.
func closedURL() string {
	srv := httptest.NewServer(http.NotFoundHandler())
	u := srv.URL
	srv.Close()
	return u
}

func TestSpielserverErkannt(t *testing.T) {
	srv, _, _ := gameServer(t, true, 200, `{}`)
	a := &App{serverBase: srv.URL}
	v := a.WorldGameServers()
	if len(v.Servers) != 1 {
		t.Fatalf("Server: %+v", v)
	}
	s := v.Servers[0]
	if s.Base != srv.URL || !s.Reachable || !s.Dev || s.Version != "v9" {
		t.Errorf("erkannt: %+v", s)
	}
}

func TestSpielserverNichtErreichbarOderOhneDevMode(t *testing.T) {
	a := &App{serverBase: closedURL()}
	if s := a.WorldGameServers().Servers[0]; s.Reachable || s.Dev {
		t.Errorf("nicht erreichbar: %+v", s)
	}
	srv, _, _ := gameServer(t, false, 200, `{}`)
	a = &App{serverBase: srv.URL}
	if s := a.WorldGameServers().Servers[0]; !s.Reachable || s.Dev {
		t.Errorf("ohne Dev-Mode: %+v", s)
	}
}

func TestStandardAdresseDesSpielservers(t *testing.T) {
	t.Setenv("K3C_HTTP_PORT", "")
	if got := (&App{}).gameServerBase(); got != "http://127.0.0.1:8080" {
		t.Errorf("Standard: %s", got)
	}
	t.Setenv("K3C_HTTP_PORT", "8090")
	if got := (&App{}).gameServerBase(); got != "http://127.0.0.1:8090" {
		t.Errorf("K3C_HTTP_PORT: %s", got)
	}
	t.Setenv("K3C_HTTP_PORT", "evil.com:80/")
	if got := (&App{}).gameServerBase(); got != "http://127.0.0.1:8080" {
		t.Errorf("ungültiger Port: %s", got)
	}
}

func TestServerNeuOeffnetDieLandingpageDesServers(t *testing.T) {
	srv, calls, _ := gameServer(t, true, 200, `{}`)
	a := &App{}
	res := a.WorldOpenGameServer(srv.URL, "probe", "forest", "new")
	if want := srv.URL + "/#game.html%3Fautostart%3D1%26save%3Dprobe"; res.Error != "" || res.URL != want || res.Save != "probe" {
		t.Errorf("Ergebnis %+v, erwartet %s", res, want)
	}
	if calls.Load() != 0 {
		t.Error("ein neues Spiel braucht keine Dev-API")
	}
}

func TestServerVollLegtDenSpielstandDortAn(t *testing.T) {
	srv, calls, seed := gameServer(t, true, 200, `{"save":"voll-abc"}`)
	a, root := engineApp(t)
	res := a.WorldOpenGameServer(srv.URL, "abc", "cave", "full")
	if res.Error != "" || res.Save != "voll-abc" || res.URL != srv.URL+"/#game.html%3Fautostart%3D1%26save%3Dvoll-abc" {
		t.Fatalf("Ergebnis %+v", res)
	}
	if got, _ := seed.Load().(string); calls.Load() != 1 || got != "abc" {
		t.Errorf("Dev-API %d-mal mit Seed %q", calls.Load(), got)
	}
	if v := a.WorldTestSaves(); len(v.Saves) != 0 {
		t.Errorf("Spielstand landete in der Studio-Engine: %+v", v)
	}
	noServerSaves(t, root)
}

func TestServerVollHinweise(t *testing.T) {
	cases := map[string]struct {
		status int
		body   string
		hint   string
	}{
		"kein Dev-Mode":  {403, `{"error":"verboten"}`, "Dev-Mode"},
		"Raum belegt":    {409, `{"error":"Spielstand ist gerade in einem Raum mit verbundenem Gerät offen"}`, "verbundenem Gerät"},
		"Server kaputt":  {500, `{}`, "Fehler 500"},
		"keine Antwort":  {200, `{}`, "keinen Spielstand"},
		"falscher Name":  {200, `{"save":"voll-A B"}`, "keinen Spielstand"},
		"Seed abgelehnt": {400, `{"error":"Seed ungültig"}`, "Seed ungültig"},
	}
	for name, c := range cases {
		srv, _, _ := gameServer(t, true, c.status, c.body)
		if res := (&App{}).WorldOpenGameServer(srv.URL, "abc", "forest", "full"); !strings.Contains(res.Error, c.hint) || res.URL != "" {
			t.Errorf("%s: %+v", name, res)
		}
	}
}

func TestServerAusOderKeinSpielserver(t *testing.T) {
	a := &App{}
	for _, mode := range []string{"new", "full"} {
		if res := a.WorldOpenGameServer(closedURL(), "abc", "forest", mode); !strings.Contains(res.Error, "Spielserver läuft nicht") || res.URL != "" {
			t.Errorf("%s: %+v", mode, res)
		}
	}
	other := httptest.NewServer(http.NotFoundHandler())
	defer other.Close()
	if res := a.WorldOpenGameServer(other.URL, "abc", "forest", "new"); !strings.Contains(res.Error, "Spielserver läuft nicht") {
		t.Errorf("kein Spielserver: %+v", res)
	}
}

// Nur Loopback: kein Aufruf an eine fremde Adresse, auch nicht über eine Weiterleitung.
func TestServerNurLoopbackOhneWeiterleitung(t *testing.T) {
	a := &App{}
	for _, base := range []string{"http://example.com", "http://192.168.1.20:8080", "http://localhost.evil.com", "http://user:pw@127.0.0.1:8080", ""} {
		if res := a.WorldOpenGameServer(base, "abc", "forest", "full"); res.Error == "" || res.URL != "" {
			t.Errorf("%q angenommen: %+v", base, res)
		}
	}
	var foreign atomic.Int32
	target := httptest.NewServer(http.HandlerFunc(func(http.ResponseWriter, *http.Request) { foreign.Add(1) }))
	defer target.Close()
	redirect := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, target.URL+r.URL.Path, http.StatusTemporaryRedirect)
	}))
	defer redirect.Close()
	if res := a.WorldOpenGameServer(redirect.URL, "abc", "forest", "full"); res.Error == "" || foreign.Load() != 0 {
		t.Errorf("Weiterleitung gefolgt: %+v, fremde Aufrufe %d", res, foreign.Load())
	}
}

// Eine andere als die Standard-Welt startet der Server nur im Dev-Mode (WZ18.5).
func TestServerAndereWeltNurImDevMode(t *testing.T) {
	srv, _, _ := gameServer(t, true, 200, `{}`)
	a := &App{world: "probe"}
	want := srv.URL + "/#game.html%3Fautostart%3D1%26fresh%3D1%26save%3Dprobe%26world%3Dprobe"
	if res := a.WorldOpenGameServer(srv.URL, "probe", "forest", "new"); res.Error != "" || res.URL != want {
		t.Errorf("Ergebnis %+v, erwartet %s", res, want)
	}
	plain, _, _ := gameServer(t, false, 200, `{}`)
	if res := a.WorldOpenGameServer(plain.URL, "probe", "forest", "new"); !strings.Contains(res.Error, "Dev-Mode") || res.URL != "" {
		t.Errorf("ohne Dev-Mode: %+v", res)
	}
}
