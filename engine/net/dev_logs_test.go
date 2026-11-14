package net

import (
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"net/url"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"k3c/engine/room"
	"k3c/engine/store"
)

// getH ist get mit Kopfzeilen: Status, Antworttext, Header.
func getH(t *testing.T, u string) (int, string, http.Header) {
	t.Helper()
	resp, err := http.Get(u)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = resp.Body.Close() }()
	b, _ := io.ReadAll(resp.Body)
	return resp.StatusCode, string(b), resp.Header
}

// B-770/AC-01: Die Liste nennt alle vorhandenen Log-Dateien, /api/dev/log liest Server- und Client-Log (und dessen alte Generation).
func TestDevLogs_listeUndDateien(t *testing.T) {
	srv, logDir, _ := readServer(t, true)
	writeFile(t, logDir, logFileName, "server WebGL\nserver Pad\n")
	writeFile(t, logDir, "k3c-client.jsonl", "client Kontext lost\nclient fps\n")
	writeFile(t, logDir, "k3c-client.1.jsonl", "alt eins\n")
	code, body := get(t, srv.URL+"/api/dev/logs", "")
	if code != http.StatusOK || !strings.HasPrefix(body, "server 24 ") || !strings.Contains(body, "\nclient 31 ") || !strings.Contains(body, "\nclient.1 9 ") {
		t.Errorf("Liste: %d %q", code, body)
	}
	for file, want := range map[string]string{"": "server WebGL\nserver Pad", "server": "server WebGL\nserver Pad", "client": "client Kontext lost\nclient fps", "client.1": "alt eins"} {
		if code, body := get(t, srv.URL+"/api/dev/log?file="+file, ""); code != http.StatusOK || body != want {
			t.Errorf("file=%q: %d %q, erwartet %q", file, code, body, want)
		}
	}
	if _, body := get(t, srv.URL+"/api/dev/log?file=client&pattern=kontext", ""); body != "client Kontext lost" {
		t.Errorf("Filter im Client-Log: %q", body)
	}
}

// B-770/AC-01: Nur feste Namen, nie ein Pfad: alles andere ist 400, auch codierte Pfade.
func TestDevLogs_pfadeAbgelehnt(t *testing.T) {
	srv, logDir, _ := readServer(t, true)
	writeFile(t, logDir, logFileName, "x\n")
	for _, name := range []string{"../k3c-server.jsonl", "k3c-server.jsonl", "..", "client/../server", "C:\\x", "unbekannt"} {
		if code, _ := get(t, srv.URL+"/api/dev/log?file="+url.QueryEscape(name), ""); code != http.StatusBadRequest {
			t.Errorf("file=%q: %d, erwartet 400", name, code)
		}
	}
}

// B-770/AC-01: Mit since kommen nur neue Zeilen, X-Log-Size ist der nächste Cursor; hinter dem Dateiende beginnt die Antwort von vorn.
func TestDevLogs_cursor(t *testing.T) {
	srv, logDir, _ := readServer(t, true)
	writeFile(t, logDir, logFileName, "eins\nzwei\n")
	code, body, h := getH(t, srv.URL+"/api/dev/log?since=0")
	if code != http.StatusOK || body != "eins\nzwei" || h.Get("X-Log-Size") != "10" || h.Get("X-Log-Truncated") != "" {
		t.Fatalf("since=0: %d %q size=%s trunc=%q", code, body, h.Get("X-Log-Size"), h.Get("X-Log-Truncated"))
	}
	if err := appendFile(filepath.Join(logDir, logFileName), "drei\nvier\n"); err != nil {
		t.Fatal(err)
	}
	code, body, h = getH(t, srv.URL+"/api/dev/log?since=10")
	if code != http.StatusOK || body != "drei\nvier" || h.Get("X-Log-Size") != "20" {
		t.Errorf("since=10: %d %q size=%s", code, body, h.Get("X-Log-Size"))
	}
	if _, body, _ = getH(t, srv.URL+"/api/dev/log?since=20"); body != "" {
		t.Errorf("nichts Neues: %q", body)
	}
	code, body, h = getH(t, srv.URL+"/api/dev/log?since=999")
	if code != http.StatusOK || body != "eins\nzwei\ndrei\nvier" || h.Get("X-Log-Truncated") != "1" {
		t.Errorf("Cursor hinter dem Ende: %d %q trunc=%q", code, body, h.Get("X-Log-Truncated"))
	}
	if code, _ := get(t, srv.URL+"/api/dev/log?since=abc", ""); code != http.StatusBadRequest {
		t.Errorf("since=abc: %d, erwartet 400", code)
	}
}

// B-770 Review: Ist die Datei größer als das Fenster, fällt die angeschnittene erste Zeile weg und partial ist gesetzt.
func TestLinesSince_fensterKuerzt(t *testing.T) {
	path := filepath.Join(t.TempDir(), "x.jsonl")
	if err := os.WriteFile(path, []byte("ganz lange erste Zeile\nzweite\ndritte\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	w, err := linesSince(path, 0, 12)
	if err != nil || !w.partial || w.truncated || w.cursor != 37 || len(w.lines) != 1 || w.lines[0] != "dritte" {
		t.Errorf("Fenster: %+v err=%v, erwartet nur 'dritte', partial, Cursor 37", w, err)
	}
}

// B-770 Review: Eine noch halb geschriebene letzte Zeile wird nicht geliefert, der Cursor bleibt davor und die Zeile kommt beim nächsten Mal ganz.
func TestLinesSince_halbeLetzteZeile(t *testing.T) {
	path := filepath.Join(t.TempDir(), "x.jsonl")
	if err := os.WriteFile(path, []byte("a\nhalb"), 0o644); err != nil {
		t.Fatal(err)
	}
	w, err := linesSince(path, 0, 1<<20)
	if err != nil || len(w.lines) != 1 || w.lines[0] != "a" || w.cursor != 2 {
		t.Fatalf("erste Runde: %+v err=%v, erwartet ['a'], Cursor 2", w, err)
	}
	if err := appendFile(path, "er Rest\n"); err != nil {
		t.Fatal(err)
	}
	w, err = linesSince(path, w.cursor, 1<<20)
	if err != nil || len(w.lines) != 1 || w.lines[0] != "halber Rest" || w.cursor != 14 {
		t.Errorf("zweite Runde: %+v err=%v, erwartet ['halber Rest'], Cursor 14", w, err)
	}
}

// B-770 Review: Ein Cursor über int64 ist ein Fehler der Anfrage (400), kein Überlauf.
func TestDevLogs_cursorUeberInt64(t *testing.T) {
	srv, logDir, _ := readServer(t, true)
	writeFile(t, logDir, logFileName, "x\n")
	if code, _ := get(t, srv.URL+"/api/dev/log?since=99999999999999999999", ""); code != http.StatusBadRequest {
		t.Errorf("since über int64: %d, erwartet 400", code)
	}
}

// B-770 Review: Kürzt limit die Antwort, steht X-Log-Partial im Kopf, damit ein pollender Client den Verlust sieht.
func TestDevLogs_limitMeldetPartial(t *testing.T) {
	srv, logDir, _ := readServer(t, true)
	writeFile(t, logDir, logFileName, "eins\nzwei\ndrei\n")
	_, body, h := getH(t, srv.URL+"/api/dev/log?since=0&limit=2")
	if body != "zwei\ndrei" || h.Get("X-Log-Partial") != "1" || h.Get("X-Log-Size") != "15" {
		t.Errorf("limit 2: %q partial=%q size=%s", body, h.Get("X-Log-Partial"), h.Get("X-Log-Size"))
	}
	if _, _, h = getH(t, srv.URL+"/api/dev/log?since=0"); h.Get("X-Log-Partial") != "" {
		t.Errorf("ohne Kürzung darf X-Log-Partial nicht stehen: %q", h.Get("X-Log-Partial"))
	}
}

func appendFile(path, text string) error {
	f, err := os.OpenFile(path, os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		return err
	}
	defer func() { _ = f.Close() }()
	_, err = f.WriteString(text)
	return err
}

// B-770/AC-02: Ohne Dev-Mode 403, falsche Methode 405, ohne Log-Ordner 404 mit Hinweis auf K3C_LOG_DIR, fehlende Datei 404.
func TestDevLogs_fehlerfaelle(t *testing.T) {
	off, _, _ := readServer(t, false)
	if code, _ := get(t, off.URL+"/api/dev/logs", ""); code != http.StatusForbidden {
		t.Errorf("logs ohne Dev-Mode: %d, erwartet 403", code)
	}
	on, logDir, _ := readServer(t, true)
	resp, err := http.Post(on.URL+"/api/dev/logs", "text/plain", strings.NewReader("x"))
	if err != nil {
		t.Fatal(err)
	}
	_ = resp.Body.Close()
	if resp.StatusCode != http.StatusMethodNotAllowed {
		t.Errorf("POST logs: %d, erwartet 405", resp.StatusCode)
	}
	if code, _ := get(t, on.URL+"/api/dev/log?file=client", ""); code != http.StatusNotFound {
		t.Errorf("fehlende Datei: %d, erwartet 404", code)
	}
	writeFile(t, logDir, logFileName, "x\n")
	noLog := httptest.NewServer(NewHandler(Config{Dist: t.TempDir(), Version: "test", StartedAt: time.Now(), Saves: &store.Saves{Dir: t.TempDir()},
		Reports: &store.Reports{Dir: t.TempDir()}, Rooms: devRooms()}))
	t.Cleanup(noLog.Close)
	for _, p := range []string{"/api/dev/log", "/api/dev/logs"} {
		code, body := get(t, noLog.URL+p, "")
		if code != http.StatusNotFound || !strings.Contains(body, "K3C_LOG_DIR") {
			t.Errorf("%s ohne Log-Ordner: %d %q, erwartet 404 mit K3C_LOG_DIR", p, code, body)
		}
	}
}

func devRooms() *room.Manager {
	m := room.NewManager(memSaves{})
	m.Dev = true
	return m
}

// B-770/AC-03: /api/dev/status nennt, ob geloggt wird, und welche Dateien es gibt (nur Namen, kein Pfad).
func TestDevLogs_statusNenntLog(t *testing.T) {
	srv, logDir, _ := readServer(t, true)
	writeFile(t, logDir, logFileName, "x\n")
	writeFile(t, logDir, "k3c-client.jsonl", "y\n")
	_, body := get(t, srv.URL+"/api/dev/status", "")
	if on, files := statusLog(t, body); !on || strings.Join(files, ",") != "server,client" {
		t.Errorf("status: on=%v files=%v", on, files)
	}
	if strings.Contains(body, logDir) {
		t.Errorf("der Log-Pfad steht im Status: %s", body)
	}
	noLog := httptest.NewServer(NewHandler(Config{Dist: t.TempDir(), Version: "test", StartedAt: time.Now(), Saves: &store.Saves{Dir: t.TempDir()},
		Reports: &store.Reports{Dir: t.TempDir()}, Rooms: devRooms()}))
	t.Cleanup(noLog.Close)
	_, body = get(t, noLog.URL+"/api/dev/status", "")
	if on, files := statusLog(t, body); on || len(files) != 0 {
		t.Errorf("status ohne Log-Ordner: on=%v files=%v", on, files)
	}
}

// statusLog liest den Log-Teil aus der Antwort von /api/dev/status.
func statusLog(t *testing.T, body string) (on bool, files []string) {
	t.Helper()
	var st struct {
		Log struct {
			On    bool     `json:"on"`
			Files []string `json:"files"`
		} `json:"log"`
	}
	if err := json.Unmarshal([]byte(body), &st); err != nil {
		t.Fatalf("Status kein JSON: %v: %s", err, body)
	}
	return st.Log.On, st.Log.Files
}
