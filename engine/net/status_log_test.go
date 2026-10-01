package net

import (
	"encoding/json"
	"fmt"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"k3c/engine/room"
	"k3c/engine/store"
)

const diagToken = "geheim-123"

// diagServer ist ein Server mit Räumen, Token und Log-Ordner (leer = Log aus).
func diagServer(t *testing.T, logDir string) (*httptest.Server, *room.Manager) {
	t.Helper()
	m := room.NewManager(memSaves{})
	m.NewCode = func() string { return "FAMILIE" }
	root := t.TempDir()
	srv := httptest.NewServer(NewHandler(Config{Dist: root, StatusToken: diagToken, Version: "test", StartedAt: time.Now(),
		Saves: &store.Saves{Dir: filepath.Join(root, "saves")}, Reports: &store.Reports{Dir: filepath.Join(root, "reports")},
		Rooms: m, LogDir: logDir}))
	t.Cleanup(srv.Close)
	return srv, m
}

func diagGet(t *testing.T, srv *httptest.Server, path string) (int, map[string]any) {
	t.Helper()
	code, body := get(t, srv.URL+path, "Bearer "+diagToken)
	var out map[string]any
	_ = json.Unmarshal([]byte(body), &out)
	return code, out
}

// B-088/AC-01: Speicher im Status.
func TestStatusNenntSpeicher(t *testing.T) {
	srv, _ := diagServer(t, "")
	code, st := diagGet(t, srv, "/api/status")
	mem, _ := st["memory"].(map[string]any)
	if code != 200 || mem == nil || mem["heapMB"].(float64) <= 0 || mem["sysMB"].(float64) <= 0 {
		t.Errorf("memory: %d %v", code, st)
	}
}

// B-088/AC-01: Geräte im Raum-Status, nur mit Kennung, nie mit voller ID.
func TestRaumStatusNenntGeraeteMitKennung(t *testing.T) {
	srv, _ := diagServer(t, "")
	const device = "b1f4c2e0-5d7a-4c1e-9a33-7e2f0d6c8a15"
	c := hello(t, srv, device)
	create(c, "familie", 0, 1)
	c.entered()

	code, sum := diagGet(t, srv, "/api/status?room=FAMILIE")
	devices, _ := sum["devices"].([]any)
	if code != 200 || len(devices) != 1 {
		t.Fatalf("devices: %d %v", code, sum)
	}
	d := devices[0].(map[string]any)
	if d["id"] != "b1f4c2" || d["connected"] != true || fmt.Sprint(d["slots"]) != "[0 1]" {
		t.Errorf("Gerät: %v", d)
	}
	_, body := get(t, srv.URL+"/api/status?room=FAMILIE", "Bearer "+diagToken)
	if strings.Contains(body, device) {
		t.Errorf("volle Geräte-ID in der Antwort: %s", body)
	}
}

func writeLog(t *testing.T, dir, text string) {
	t.Helper()
	if err := os.WriteFile(filepath.Join(dir, logFileName), []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

func lines(m map[string]any) []string {
	var out []string
	for _, l := range m["lines"].([]any) {
		out = append(out, l.(string))
	}
	return out
}

// B-088/AC-02: Zeilen ab Cursor, mit neuem Cursor; eine halbe letzte Zeile bleibt liegen.
func TestLogEndpunktLiefertZeilenAbCursor(t *testing.T) {
	dir := t.TempDir()
	writeLog(t, dir, "{\"msg\":\"eins\"}\n{\"msg\":\"zwei\"}\n{\"msg\":\"drei\"}\n{\"msg\":\"halb")
	srv, _ := diagServer(t, dir)

	code, page := diagGet(t, srv, "/api/status/log?limit=2")
	if code != 200 || strings.Join(lines(page), "|") != `{"msg":"eins"}|{"msg":"zwei"}` {
		t.Fatalf("erste Seite: %d %v", code, page)
	}
	cursor := int64(page["cursor"].(float64))
	_, page = diagGet(t, srv, fmt.Sprintf("/api/status/log?since=%d", cursor))
	if strings.Join(lines(page), "|") != `{"msg":"drei"}` || page["truncated"] != false {
		t.Fatalf("zweite Seite: %v", page)
	}
	next := int64(page["cursor"].(float64))
	_, page = diagGet(t, srv, fmt.Sprintf("/api/status/log?since=%d", next))
	if len(lines(page)) != 0 || int64(page["cursor"].(float64)) != next {
		t.Errorf("nichts Neues, Cursor bleibt: %v", page)
	}
	writeLog(t, dir, "{\"msg\":\"eins\"}\n{\"msg\":\"zwei\"}\n{\"msg\":\"drei\"}\n{\"msg\":\"halb\"}\n")
	_, page = diagGet(t, srv, fmt.Sprintf("/api/status/log?since=%d", next))
	if strings.Join(lines(page), "|") != `{"msg":"halb"}` {
		t.Errorf("fertig geschriebene Zeile: %v", page)
	}
}

// B-088/AC-02: Datei kleiner geworden → ab Anfang mit truncated.
func TestLogEndpunktErkenntVerkleinerteDatei(t *testing.T) {
	dir := t.TempDir()
	writeLog(t, dir, "{\"msg\":\"neu\"}\n")
	srv, _ := diagServer(t, dir)
	_, page := diagGet(t, srv, "/api/status/log?since=99999")
	if page["truncated"] != true || strings.Join(lines(page), "|") != `{"msg":"neu"}` {
		t.Errorf("truncated: %v", page)
	}
}

// B-088/AC-02: ohne Log-Ordner oder -Datei „Log aus“.
func TestLogEndpunktOhneLog(t *testing.T) {
	for name, dir := range map[string]string{"ohne Ordner": "", "ohne Datei": t.TempDir()} {
		srv, _ := diagServer(t, dir)
		code, body := diagGet(t, srv, "/api/status/log")
		if code != 404 || body["error"] != "Log aus" {
			t.Errorf("%s: %d %v", name, code, body)
		}
	}
}

// B-088/AC-02: Zeilen über 64 KB werden übersprungen, Parameter geprüft.
func TestLogEndpunktGrenzenUndParameter(t *testing.T) {
	dir := t.TempDir()
	writeLog(t, dir, strings.Repeat("x", logMaxLine+1)+"\n{\"msg\":\"ok\"}\n")
	srv, _ := diagServer(t, dir)
	if _, page := diagGet(t, srv, "/api/status/log"); strings.Join(lines(page), "|") != `{"msg":"ok"}` {
		t.Errorf("lange Zeile: %v", page)
	}
	for _, q := range []string{"since=-1", "since=abc", "limit=0", "limit=-5", "limit=x"} {
		if code, _ := diagGet(t, srv, "/api/status/log?"+q); code != 400 {
			t.Errorf("%s: %d", q, code)
		}
	}
	if code, _ := diagGet(t, srv, "/api/status/log?limit=99999"); code != 200 {
		t.Errorf("limit wird gekappt: %d", code)
	}
}

// B-088/AC-04 (Lesewege): ohne Token am Server 404, falsch 401, falsche Methode 405.
func TestDiagnoseWegeSindGeschuetzt(t *testing.T) {
	dir := t.TempDir()
	writeLog(t, dir, "{}\n")
	srv, _ := diagServer(t, dir)
	for _, auth := range []string{"", "Bearer falsch", diagToken} {
		if code, _ := get(t, srv.URL+"/api/status/log", auth); code != 401 {
			t.Errorf("%q: %d", auth, code)
		}
	}
	req := httptest.NewRequest("POST", "/api/status/log", nil)
	req.Header.Set("Authorization", "Bearer "+diagToken)
	rec := httptest.NewRecorder()
	NewHandler(Config{StatusToken: diagToken, LogDir: dir}).ServeHTTP(rec, req)
	if rec.Code != 405 {
		t.Errorf("POST: %d", rec.Code)
	}
	off := httptest.NewServer(NewHandler(Config{LogDir: dir}))
	defer off.Close()
	if code, _ := get(t, off.URL+"/api/status/log", "Bearer x"); code != 404 {
		t.Errorf("Diagnose aus: %d", code)
	}
}
