package net

import (
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"k3c/engine/room"
	"k3c/engine/store"
)

// readServer ist ein Server mit Log-Ordner und Berichte-Ordner; dev schaltet den Dev-Mode (K3C_DEV) ein. Kein Token.
func readServer(t *testing.T, dev bool) (srv *httptest.Server, logDir, reportsDir string) {
	t.Helper()
	m := room.NewManager(memSaves{})
	m.Dev = dev
	root := t.TempDir()
	logDir, reportsDir = filepath.Join(root, "logs"), filepath.Join(root, "reports")
	for _, d := range []string{logDir, reportsDir} {
		if err := os.MkdirAll(d, 0o755); err != nil {
			t.Fatal(err)
		}
	}
	srv = httptest.NewServer(NewHandler(Config{Dist: root, Version: "test", StartedAt: time.Now(), Saves: &store.Saves{Dir: filepath.Join(root, "saves")},
		Reports: &store.Reports{Dir: reportsDir}, Rooms: m, LogDir: logDir}))
	t.Cleanup(srv.Close)
	return srv, logDir, reportsDir
}

func writeFile(t *testing.T, dir, name, text string) {
	t.Helper()
	if err := os.WriteFile(filepath.Join(dir, name), []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

// RX1: Ohne Dev-Mode lesen die Endpunkte nichts (403), mit Dev-Mode brauchen sie kein Token; schreiben geht nie.
func TestDevRead_nurImDevMode(t *testing.T) {
	off, _, _ := readServer(t, false)
	for _, p := range []string{"/api/dev/status", "/api/dev/log", "/api/dev/reports"} {
		if code, _ := get(t, off.URL+p, ""); code != http.StatusForbidden {
			t.Errorf("%s ohne Dev-Mode: %d, erwartet 403", p, code)
		}
	}
	on, _, _ := readServer(t, true)
	if code, body := get(t, on.URL+"/api/dev/status", ""); code != http.StatusOK || !strings.Contains(body, `"version":"test"`) {
		t.Errorf("status im Dev-Mode: %d %s", code, body)
	}
	resp, err := http.Post(on.URL+"/api/dev/log", "text/plain", strings.NewReader("x"))
	if err != nil {
		t.Fatal(err)
	}
	_ = resp.Body.Close()
	if resp.StatusCode != http.StatusMethodNotAllowed {
		t.Errorf("POST: %d, erwartet 405", resp.StatusCode)
	}
}

// RX1: Das Log kommt von hinten, gefiltert nach Muster (ohne Groß-/Kleinschreibung), älteste Zeile zuerst, mit Zähler der Treffer.
func TestDevRead_log(t *testing.T) {
	srv, logDir, _ := readServer(t, true)
	writeFile(t, logDir, logFileName, "eins WebGL\nzwei Canvas\ndrei webgl Kontext\nvier Pad\n")
	code, body := get(t, srv.URL+"/api/dev/log?pattern=webgl", "")
	if code != http.StatusOK || body != "eins WebGL\ndrei webgl Kontext" {
		t.Errorf("Muster: %d %q", code, body)
	}
	if _, body := get(t, srv.URL+"/api/dev/log?limit=2", ""); body != "drei webgl Kontext\nvier Pad" {
		t.Errorf("limit 2 gibt die letzten zwei: %q", body)
	}
	if code, _ := get(t, srv.URL+"/api/dev/log?pattern=(", ""); code != http.StatusBadRequest {
		t.Errorf("ungültiger Regex: %d, erwartet 400", code)
	}
	if code, _ := get(t, srv.URL+"/api/dev/log?limit=0", ""); code != http.StatusBadRequest {
		t.Errorf("limit 0: %d, erwartet 400", code)
	}
	if code, _ := get(t, srv.URL+"/api/dev/log?pattern="+strings.Repeat("a", devPatternMax+1), ""); code != http.StatusBadRequest {
		t.Errorf("zu langes Muster: %d, erwartet 400", code)
	}
}

// RX1: Ohne Log-Datei antwortet der Endpunkt 404 statt zu scheitern; ein angeschnittener Anfang zählt nicht als Zeile.
func TestDevRead_logLeerUndAngeschnitten(t *testing.T) {
	srv, logDir, _ := readServer(t, true)
	if code, _ := get(t, srv.URL+"/api/dev/log", ""); code != http.StatusNotFound {
		t.Errorf("ohne Datei: %d, erwartet 404", code)
	}
	writeFile(t, logDir, logFileName, "ganz lange erste Zeile\nzweite\ndritte\n")
	lines, size, err := tailLines(filepath.Join(logDir, logFileName), 12)
	if err != nil || size != 37|| len(lines) != 1 || lines[0] != "dritte" {
		t.Errorf("tailLines: %v size=%d err=%v, erwartet nur 'dritte'", lines, size, err)
	}
}

// RX1: Berichte werden gelistet (neueste zuerst) und gelesen; Pfade, versteckte und fremde Dateien nicht.
func TestDevRead_berichte(t *testing.T) {
	srv, _, reports := readServer(t, true)
	writeFile(t, reports, "gamepad-2026-11-08.json", `{"a":1}`)
	writeFile(t, reports, "gamepad-2026-11-09.json", `{"kind":"texture-probe"}`)
	writeFile(t, reports, "notiz.txt", "x")
	_, list := get(t, srv.URL+"/api/dev/reports", "")
	lines := strings.Split(strings.TrimSpace(list), "\n")
	if len(lines) != 2 || !strings.HasPrefix(lines[0], "gamepad-2026-11-09.json 24 ") || !strings.HasPrefix(lines[1], "gamepad-2026-11-08.json ") {
		t.Errorf("Liste: %q", list)
	}
	if code, body := get(t, srv.URL+"/api/dev/reports?name=gamepad-2026-11-09.json", ""); code != http.StatusOK || body != `{"kind":"texture-probe"}` {
		t.Errorf("lesen: %d %q", code, body)
	}
	for _, bad := range []string{"../x.json", "..%2Fx.json", "notiz.txt", ".versteckt.json", "a%5Cb.json", "unterordner/x.json"} {
		if code, _ := get(t, srv.URL+"/api/dev/reports?name="+bad, ""); code != http.StatusBadRequest {
			t.Errorf("name=%s: %d, erwartet 400", bad, code)
		}
	}
	if code, _ := get(t, srv.URL+"/api/dev/reports?name=fehlt.json", ""); code != http.StatusNotFound {
		t.Errorf("fehlender Bericht: %d, erwartet 404", code)
	}
}

// RX1: lastMatches kappt die Antwort bei der Byte-Grenze und zählt trotzdem alle Treffer.
func TestLastMatches_kapptBeiByteGrenze(t *testing.T) {
	lines := []string{"aaaa", "bbbb", "cccc"}
	hits, matched := lastMatches(lines, nil, 10, 10)
	if matched != 3 || len(hits) != 2 || hits[0] != "bbbb" || hits[1] != "cccc" {
		t.Errorf("hits=%v matched=%d, erwartet bbbb,cccc und 3 Treffer", hits, matched)
	}
}
