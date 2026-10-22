package net

import (
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
	"time"
)

// publicServer baut dist/ und public/ mit je einigen Dateien und startet den Handler mit beiden.
func publicServer(t *testing.T) (srv *httptest.Server, public string) {
	t.Helper()
	root := t.TempDir()
	dist, public := filepath.Join(root, "dist"), filepath.Join(root, "public")
	for p, text := range map[string]string{
		filepath.Join(dist, "index.html"):                   "<html>build",
		filepath.Join(dist, "grafik", "a.png"):              "alt",
		filepath.Join(dist, "assets", "app-abc123.js"):      "build-js",
		filepath.Join(public, "grafik", "a.png"):            "neu",
		filepath.Join(public, "index.html"):                 "<html>public",
		filepath.Join(public, "app.js"):                     "nicht aus public",
		filepath.Join(public, "nur-public.ogg"):             "klang",
		filepath.Join(public, ".omc", "state", "wert.json"): "intern",
	} {
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	srv = httptest.NewServer(NewHandler(Config{Dist: dist, Public: public}))
	t.Cleanup(srv.Close)
	return srv, public
}

// WZ25.2 (B-509/AC-02): Dateien aus public/ gehen dem Build vor und kommen ohne `task build` an.
func TestPublicVorDemBuild(t *testing.T) {
	srv, _ := publicServer(t)
	res, body := do(t, "GET", srv.URL+"/grafik/a.png", "")
	if body != "neu" || res.Header.Get("Cache-Control") != "no-cache" || res.Header.Get("Content-Type") != "image/png" {
		t.Errorf("Grafik: %q, %v", body, res.Header)
	}
	if _, body := do(t, "GET", srv.URL+"/nur-public.ogg", ""); body != "klang" {
		t.Errorf("Datei nur in public/: %q", body)
	}
}

// HTML und Skripte kommen aus dem Build, gehashte Assets bleiben dauerhaft gecacht, nichts verlässt public/ oder liegt in
// einem Punkt-Ordner.
func TestPublicLiefertNurDasRichtige(t *testing.T) {
	srv, _ := publicServer(t)
	if _, body := do(t, "GET", srv.URL+"/", ""); body != "<html>build" {
		t.Errorf("HTML kommt aus dem Build: %q", body)
	}
	if _, body := do(t, "GET", srv.URL+"/app.js", ""); body == "nicht aus public" {
		t.Error("Skripte kommen aus dem Build, nie aus public/")
	}
	if res, _ := do(t, "GET", srv.URL+"/assets/app-abc123.js", ""); res.Header.Get("Cache-Control") == "no-cache" {
		t.Error("gehashte Assets bleiben dauerhaft gecacht")
	}
	for _, p := range []string{"/..%2f..%2fgeheim.png", "/.omc/state/wert.json"} {
		if res, body := do(t, "GET", srv.URL+p, ""); res.StatusCode == 200 || body == "intern" {
			t.Errorf("%s ausgeliefert", p)
		}
	}
}

// Caching nach Inhalt: eine geänderte Datei kommt bei der bedingten Anfrage sofort, eine unveränderte gibt 304.
func TestPublicCachingNachInhalt(t *testing.T) {
	srv, public := publicServer(t)
	res, _ := do(t, "GET", srv.URL+"/grafik/a.png", "")
	path := filepath.Join(public, "grafik", "a.png")
	if err := os.WriteFile(path, []byte("neuer"), 0o644); err != nil {
		t.Fatal(err)
	}
	later := time.Now().Add(time.Hour)
	if err := os.Chtimes(path, later, later); err != nil {
		t.Fatal(err)
	}
	conditional := func(since string) *http.Response {
		req, _ := http.NewRequest("GET", srv.URL+"/grafik/a.png", nil)
		req.Header.Set("If-Modified-Since", since)
		got, err := http.DefaultClient.Do(req)
		if err != nil {
			t.Fatal(err)
		}
		t.Cleanup(func() { _ = got.Body.Close() })
		return got
	}
	got := conditional(res.Header.Get("Last-Modified"))
	if got.StatusCode != 200 {
		t.Errorf("geänderte Datei: Status %d, erwartet 200 (neue Fassung)", got.StatusCode)
	}
	if same := conditional(got.Header.Get("Last-Modified")); same.StatusCode != http.StatusNotModified {
		t.Errorf("unveränderte Datei: Status %d, erwartet 304", same.StatusCode)
	}
}
