package main

import (
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"

	"k3c/tools/workkit/applog"
	"k3c/tools/workkit/uikit"
)

// playtestApp ist eine App mit zwei Sessions unter playtests/ (B-1218).
func playtestApp(t *testing.T) *App {
	t.Helper()
	a := uiApp(t)
	a.log = &applog.Log{Logger: applog.Discard()}
	dir := filepath.Join(a.root, "playtests", "2026-11-21-1")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	for name, body := range map[string]string{"session.json": `{"startMs":1,"endMs":60001,"stand":"aufgenommen"}`, "video.mkv": "0123456789", "voice.wav": "abc"} {
		if err := os.WriteFile(filepath.Join(dir, name), []byte(body), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	return a
}

// Die Bindings listen, planen und löschen erst mit Bestätigung; über RPC erreichbar.
func TestPlaytestBindings(t *testing.T) {
	a := playtestApp(t)
	list, err := a.PlaytestList()
	if err != nil || len(list) != 1 || list[0].Stage != "aufgenommen" {
		t.Fatalf("list = %+v, err %v", list, err)
	}
	if _, err := a.PlaytestDelete("video", "2026-11-21-1", false); err == nil {
		t.Fatal("ohne Bestätigung gelöscht")
	}
	video := filepath.Join(a.root, "playtests", "2026-11-21-1", "video.mkv")
	if _, err := os.Stat(video); err != nil {
		t.Fatal("Video fehlt ohne Bestätigung")
	}
	if p, err := a.PlaytestDelete("video", "2026-11-21-1", true); err != nil || p.Total != 10 {
		t.Fatalf("plan = %+v, err %v", p, err)
	}
	if _, err := os.Stat(video); err == nil {
		t.Fatal("Video noch da")
	}
	code, out := uiCall(t, a, uikit.Prefix+"rpc/PlaytestList", "[]")
	if res, _ := out["result"].([]any); code != http.StatusOK || len(res) != 1 {
		t.Errorf("RPC: %d %v", code, out)
	}
}

// Der Player-Weg liefert nur voice.wav und video.mkv, mit Bereichsanfragen, und hält den Schutz der Oberfläche ein.
func TestPlaytestMedia(t *testing.T) {
	a := playtestApp(t)
	get := func(path, host string, hdr map[string]string) *httptest.ResponseRecorder {
		r := httptest.NewRequest(http.MethodGet, path, nil)
		r.RemoteAddr, r.Host = "127.0.0.1:50000", host
		for k, v := range hdr {
			r.Header.Set(k, v)
		}
		w := httptest.NewRecorder()
		a.uiHandler().ServeHTTP(w, r)
		return w
	}
	base := playtestMediaPrefix + "2026-11-21-1/"
	if w := get(base+"voice.wav", "localhost:5191", nil); w.Code != http.StatusOK || w.Body.String() != "abc" {
		t.Errorf("voice: %d %q", w.Code, w.Body.String())
	}
	if w := get(base+"video.mkv", "localhost:5191", map[string]string{"Range": "bytes=2-4"}); w.Code != http.StatusPartialContent || w.Body.String() != "234" {
		t.Errorf("video Range: %d %q", w.Code, w.Body.String())
	}
	if w := get(base+"session.json", "localhost:5191", nil); w.Code != http.StatusNotFound {
		t.Errorf("session.json: %d", w.Code)
	}
	if w := get(base+"voice.wav", "fremd.example:80", nil); w.Code != http.StatusForbidden {
		t.Errorf("fremder Host: %d", w.Code)
	}
	if w := get(base+"voice.wav", "localhost:5191", map[string]string{"Sec-Fetch-Site": "cross-site"}); w.Code != http.StatusForbidden {
		t.Errorf("fremde Seite: %d", w.Code)
	}
}
