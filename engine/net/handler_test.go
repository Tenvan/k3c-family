package net

import (
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/engine/store"
)

// testServer baut einen Server über einem Wegwerf-Build (index.html, game.html, gamepad-test.html, assets/app.js).
func testServer(t *testing.T) (*httptest.Server, string) {
	t.Helper()
	root := t.TempDir()
	dist := filepath.Join(root, "dist")
	for name, text := range map[string]string{
		"index.html": "<h1>Start</h1>", "game.html": "<h1>Spiel</h1>", "gamepad-test.html": "<h1>Test</h1>",
		"assets/app.js": "console.log(1)", "assets/daten.bin": "x",
	} {
		p := filepath.Join(dist, filepath.FromSlash(name))
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	if err := os.WriteFile(filepath.Join(root, "geheim.txt"), []byte("geheim"), 0o644); err != nil {
		t.Fatal(err)
	}
	srv := httptest.NewServer(NewHandler(Config{Dist: dist, Saves: &store.Saves{Dir: filepath.Join(root, "saves")},
		Reports: &store.Reports{Dir: filepath.Join(root, "reports")}}))
	t.Cleanup(srv.Close)
	return srv, root
}

func do(t *testing.T, method, url, body string) (*http.Response, string) {
	t.Helper()
	req, err := http.NewRequest(method, url, strings.NewReader(body))
	if err != nil {
		t.Fatal(err)
	}
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = res.Body.Close() }()
	data, _ := io.ReadAll(res.Body)
	return res, string(data)
}

// Was der alte Smoke-Test in ci.yml prüfte (B-020/AC-01).
func TestWieDerSmokeTest(t *testing.T) {
	srv, root := testServer(t)
	for _, page := range []string{"/", "/game.html", "/gamepad-test.html"} {
		if res, _ := do(t, "GET", srv.URL+page, ""); res.StatusCode != 200 {
			t.Errorf("%s: %d", page, res.StatusCode)
		}
	}
	if res, _ := do(t, "POST", srv.URL+"/api/report", "kaputt"); res.StatusCode != 400 {
		t.Errorf("Bericht kaputt: %d", res.StatusCode)
	}
	if res, body := do(t, "POST", srv.URL+"/api/report", `{"ci":true}`); res.StatusCode != 200 || !strings.Contains(body, `"ok":true`) {
		t.Errorf("Bericht: %d %s", res.StatusCode, body)
	}
	if files, _ := filepath.Glob(filepath.Join(root, "reports", "gamepad-*.json")); len(files) != 1 {
		t.Errorf("Berichte: %v", files)
	}
	if res, _ := do(t, "GET", srv.URL+"/api/save", ""); res.StatusCode != 404 {
		t.Errorf("leerer Spielstand: %d", res.StatusCode)
	}
	if res, body := do(t, "POST", srv.URL+"/api/save", `{"version":1,"campaignId":"ci"}`); res.StatusCode != 200 || body != `{"backup":null,"ok":true,"slot":"autosave"}`+"\n" {
		t.Errorf("Speichern: %d %q", res.StatusCode, body)
	}
	if _, body := do(t, "GET", srv.URL+"/api/save", ""); !strings.Contains(body, `"campaignId":"ci"`) {
		t.Errorf("Laden: %s", body)
	}
}

func TestAuslieferung(t *testing.T) {
	srv, _ := testServer(t)
	res, _ := do(t, "GET", srv.URL+"/", "")
	if res.Header.Get("Content-Type") != "text/html; charset=utf-8" || res.Header.Get("Cache-Control") != "no-cache" {
		t.Errorf("HTML-Header: %v", res.Header)
	}
	res, body := do(t, "GET", srv.URL+"/assets/app.js", "")
	if body != "console.log(1)" || res.Header.Get("Cache-Control") != "public, max-age=31536000, immutable" ||
		res.Header.Get("Content-Type") != "text/javascript; charset=utf-8" {
		t.Errorf("Asset: %q %v", body, res.Header)
	}
	if res, _ := do(t, "GET", srv.URL+"/assets/daten.bin", ""); res.Header.Get("Content-Type") != "application/octet-stream" {
		t.Errorf("unbekannte Endung: %v", res.Header)
	}
	if res, body := do(t, "HEAD", srv.URL+"/game.html", ""); res.StatusCode != 200 || body != "" {
		t.Errorf("HEAD: %d %q", res.StatusCode, body)
	}
	if res, body := do(t, "GET", srv.URL+"/fehlt.html", ""); res.StatusCode != 404 || !strings.Contains(body, "Nicht gefunden") {
		t.Errorf("fehlt: %d %q", res.StatusCode, body)
	}
	if res, _ := do(t, "DELETE", srv.URL+"/", ""); res.StatusCode != 405 {
		t.Errorf("DELETE: %d", res.StatusCode)
	}
	if res, _ := do(t, "GET", srv.URL+"/api/health", ""); res.StatusCode != 200 {
		t.Errorf("health: %d", res.StatusCode)
	}
}

// Kein Pfad verlässt dist/ (auch nicht über kodierte Punkte oder \ unter Windows).
func TestAuslieferungBleibtInDist(t *testing.T) {
	srv, _ := testServer(t)
	for _, p := range []string{"/../geheim.txt", "/%2e%2e/geheim.txt", "/assets/..%2f..%2fgeheim.txt", "/..%5cgeheim.txt", "/%5c..%5c..%5cgeheim.txt"} {
		if _, body := do(t, "GET", srv.URL+p, ""); strings.Contains(body, "geheim") {
			t.Errorf("%s liefert eine Datei außerhalb von dist/", p)
		}
	}
	dist := filepath.Join(t.TempDir(), "dist")
	for _, p := range []string{`/..\geheim`, "/a/../../x", "/x\x00y"} {
		if file, ok := resolve(dist, p); ok && !strings.HasPrefix(file, dist) {
			t.Errorf("resolve(%q) = %q", p, file)
		}
	}
}

func TestSpielstandFehler(t *testing.T) {
	srv, _ := testServer(t)
	cases := map[string][3]string{
		"Slot":   {"POST", "/api/save?slot=../x", `{"version":1,"campaignId":"ci"}`},
		"JSON":   {"POST", "/api/save", "kaputt"},
		"Felder": {"POST", "/api/save", `{"version":"1"}`},
	}
	for name, c := range cases {
		if res, _ := do(t, c[0], srv.URL+c[1], c[2]); res.StatusCode != 400 {
			t.Errorf("%s: %d", name, res.StatusCode)
		}
	}
	big := `{"version":1,"campaignId":"` + strings.Repeat("x", store.MaxSaveBytes) + `"}`
	if res, _ := do(t, "POST", srv.URL+"/api/save", big); res.StatusCode != 413 {
		t.Errorf("zu groß: %d", res.StatusCode)
	}
	if res, _ := do(t, "PUT", srv.URL+"/api/save", "{}"); res.StatusCode != 405 {
		t.Errorf("PUT: %d", res.StatusCode)
	}
	if res, _ := do(t, "GET", srv.URL+"/api/report", ""); res.StatusCode != 405 {
		t.Errorf("GET Bericht: %d", res.StatusCode)
	}
}
