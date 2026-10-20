package main

import (
	"context"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"k3c/tools/k3c-studio/backend"
)

// fixture legt eine Mini-Repo-Wurzel mit public/ und dist/ an.
func fixture(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	files := map[string]string{
		"public/bild.png":        "aus public",
		"public/beide.txt":       "public gewinnt",
		"dist/beide.txt":         "dist verliert",
		"dist/nur-dist.txt":      "aus dist",
		"dist/studio/index.html": "<html>alt</html>", // alter Build des Spielservers, gilt nicht mehr (B-507)
		"tools/k3c-studio/frontend/dist/index.html": "<html>studio</html>",
	}
	for name, body := range files {
		path := filepath.Join(root, filepath.FromSlash(name))
		if err := os.MkdirAll(filepath.Dir(path), 0o750); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(path, []byte(body), 0o600); err != nil {
			t.Fatal(err)
		}
	}
	return root
}

func get(t *testing.T, h http.Handler, method, path string) *httptest.ResponseRecorder {
	t.Helper()
	rec := httptest.NewRecorder()
	h.ServeHTTP(rec, httptest.NewRequest(method, path, nil))
	return rec
}

func TestSiteHandlerPublicBeforeDist(t *testing.T) {
	h := siteHandler(fixture(t))
	cases := map[string]string{"/bild.png": "aus public", "/beide.txt": "public gewinnt", "/nur-dist.txt": "aus dist", "/studio/": "studio"}
	for path, want := range cases {
		rec := get(t, h, http.MethodGet, path)
		if rec.Code != http.StatusOK || !strings.Contains(rec.Body.String(), want) {
			t.Errorf("%s: %d %q, erwartet %q", path, rec.Code, rec.Body.String(), want)
		}
	}
}

func TestSiteHandlerRootAndMethods(t *testing.T) {
	h := siteHandler(fixture(t))
	if rec := get(t, h, http.MethodGet, "/"); rec.Code != http.StatusFound || rec.Header().Get("Location") != studioPage {
		t.Errorf("Wurzel: %d → %q", rec.Code, rec.Header().Get("Location"))
	}
	if rec := get(t, h, http.MethodPost, "/beide.txt"); rec.Code != http.StatusMethodNotAllowed {
		t.Errorf("POST: %d, erwartet 405", rec.Code)
	}
	if rec := get(t, h, http.MethodGet, "/gibt-es-nicht"); rec.Code != http.StatusNotFound {
		t.Errorf("fehlende Datei: %d, erwartet 404", rec.Code)
	}
}

func TestHandlerServesStudioAPI(t *testing.T) {
	root := fixture(t)
	h, _ := newHandler(t.Context(), root, nil)
	srv := httptest.NewServer(h)
	defer srv.Close()
	// Studio-API und Spiel-API der eingebundenen Engine (B-507/AC-02): kein Spielserver nötig.
	for _, path := range []string{backend.Prefix + "ping", "/api/health", "/api/saves"} {
		res, err := http.Get(srv.URL + path)
		if err != nil {
			t.Fatal(err)
		}
		_ = res.Body.Close()
		if res.StatusCode != http.StatusOK {
			t.Errorf("%s: %d", path, res.StatusCode)
		}
	}
	if _, err := os.Stat(filepath.Join(root, "saves")); !os.IsNotExist(err) {
		t.Errorf("saves/ des Servers angelegt: %v", err)
	}
}

// Die Spiel-API der Engine nimmt Schreibanfragen nur von derselben Seite und nur über Loopback (B-507/AC-02).
func TestGameGuard(t *testing.T) {
	h := gameGuard(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) { w.WriteHeader(http.StatusOK) }))
	cases := []struct {
		name, method, remote, origin, site string
		want                               int
	}{
		{"fremde Origin", http.MethodPost, "127.0.0.1:5000", "http://evil.example", "", http.StatusForbidden},
		{"anderer Port", http.MethodPost, "127.0.0.1:5000", "http://127.0.0.1:9999", "", http.StatusForbidden},
		{"cross-site", http.MethodPost, "127.0.0.1:5000", "", "cross-site", http.StatusForbidden},
		{"same-site", http.MethodPost, "127.0.0.1:5000", "", "same-site", http.StatusForbidden},
		{"gleiche Seite", http.MethodPost, "127.0.0.1:5000", "http://127.0.0.1:8091", "same-origin", http.StatusOK},
		{"ohne Kopfzeilen", http.MethodPost, "127.0.0.1:5000", "", "", http.StatusOK},
		{"direkt", http.MethodPost, "127.0.0.1:5000", "", "none", http.StatusOK},
		{"GET fremd", http.MethodGet, "127.0.0.1:5000", "http://evil.example", "cross-site", http.StatusOK},
		{"nicht Loopback", http.MethodGet, "192.168.1.20:5000", "", "", http.StatusForbidden},
	}
	for _, c := range cases {
		req := httptest.NewRequest(c.method, "http://127.0.0.1:8091/api/save?slot=x", nil)
		req.RemoteAddr = c.remote
		if c.origin != "" {
			req.Header.Set("Origin", c.origin)
		}
		if c.site != "" {
			req.Header.Set("Sec-Fetch-Site", c.site)
		}
		rec := httptest.NewRecorder()
		h.ServeHTTP(rec, req)
		if rec.Code != c.want {
			t.Errorf("%s: %d, erwartet %d", c.name, rec.Code, c.want)
		}
	}
}

// Endet der Kontext, speichert die Engine ihre Räume; EngineWait meldet das (stopServer).
func TestEngineWaitNachEnde(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	_, studio := newHandler(ctx, fixture(t), nil)
	if studio.EngineWait(10 * time.Millisecond) {
		t.Error("Engine fertig, obwohl der Kontext läuft")
	}
	cancel()
	if !studio.EngineWait(5 * time.Second) {
		t.Error("Engine nach dem Ende nicht fertig")
	}
}

func TestCheckBuilt(t *testing.T) {
	if err := checkBuilt(fixture(t)); err != nil {
		t.Errorf("gebaut: %v", err)
	}
	if err := checkBuilt(t.TempDir()); err == nil || !strings.Contains(err.Error(), "task studio:frontend") {
		t.Errorf("ungebaut: %v", err)
	}
}

func TestListenFallsBackWhenBusy(t *testing.T) {
	first, err := listen("0")
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = first.Close() }()
	_, port, _ := strings.Cut(first.Addr().String(), ":")
	second, err := listen(port)
	if err != nil {
		t.Fatalf("belegter Port ohne Ausweichen: %v", err)
	}
	defer func() { _ = second.Close() }()
	if second.Addr().String() == first.Addr().String() {
		t.Error("zweiter Listener hat denselben Port")
	}
}

func TestWindowStateRoundTrip(t *testing.T) {
	path := filepath.Join(t.TempDir(), "k3c", "w.json")
	if _, ok := loadWindow(path); ok {
		t.Fatal("fehlende Datei darf nicht ok sein")
	}
	want := windowState{X: -1200, Y: 40, Width: 1500, Height: 900}
	if err := saveWindow(path, want); err != nil {
		t.Fatal(err)
	}
	if got, ok := loadWindow(path); !ok || got != want {
		t.Errorf("geladen %v ok=%v, erwartet %v", got, ok, want)
	}
	if err := saveWindow(path, windowState{Width: 10, Height: 10}); err != nil {
		t.Fatal(err)
	}
	if got, _ := loadWindow(path); got.Width != minWidth || got.Height != minHeight {
		t.Errorf("Minimum nicht angewandt: %v", got)
	}
}
