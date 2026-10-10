package net

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/engine/room"
)

// devCall schickt eine Anfrage an /api/dev und liefert Status und JSON-Antwort.
func devCall(t *testing.T, srv *httptest.Server, method, query, body string) (int, map[string]any) {
	t.Helper()
	req, _ := http.NewRequest(method, srv.URL+"/api/dev"+query, strings.NewReader(body))
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = res.Body.Close() }()
	var out map[string]any
	_ = json.NewDecoder(res.Body).Decode(&out)
	return res.StatusCode, out
}

// devPageRoom ist ein Server mit einem Raum (ein Monarch, Slot 0) und Dev-Mode nach Wunsch.
func devPageRoom(t *testing.T, dev bool) (*httptest.Server, *room.Room) {
	t.Helper()
	srv, m := devServer(t)
	m.Dev = dev
	x := hello(t, srv, "xbox")
	create(x, "dm", 0)
	return srv, m.Room(x.entered()["room"].(string))
}

// B-232/AC-02: Raumliste und Diagnose gehen auch ohne Dev-Mode; Aktionen sind dann verboten (403).
func TestDevSeiteOhneDevMode(t *testing.T) {
	srv, r := devPageRoom(t, false)
	code, body := devCall(t, srv, http.MethodGet, "", "")
	if rooms, _ := body["rooms"].([]any); code != 200 || body["dev"] != false || len(rooms) != 1 {
		t.Fatalf("Liste %d: %v", code, body)
	}
	code, body = devCall(t, srv, http.MethodGet, "?room="+r.Code, "")
	if s, _ := body["room"].(map[string]any); code != 200 || s["code"] != r.Code || s["timescale"] != float64(1) {
		t.Fatalf("Diagnose %d: %v", code, body)
	}
	if code, _ = devCall(t, srv, http.MethodPost, "?room="+r.Code, `{"action":"pause","paused":true}`); code != 403 {
		t.Fatalf("Aktion ohne Dev-Mode: %d", code)
	}
	if code, _ = devCall(t, srv, http.MethodGet, "?room=XXXX", ""); code != 404 {
		t.Fatalf("unbekannter Raum: %d", code)
	}
}

// B-232/AC-03, AC-05: Jede Aktion wirkt über POST /api/dev; die Antwort trägt die neue Diagnose.
func TestDevSeiteAktionen(t *testing.T) {
	srv, r := devPageRoom(t, true)
	q := "?room=" + r.Code
	for _, c := range []struct {
		body, field string
		want        any
	}{
		{`{"action":"gold","slot":0,"amount":5}`, "", nil},
		{`{"action":"material","slot":0,"amount":5,"resource":"wood"}`, "", nil},
		{`{"action":"timescale","factor":4}`, "timescale", float64(4)},
		{`{"action":"pause","paused":true}`, "paused", true},
		{`{"action":"wave","slot":0}`, "wave", float64(1)},
		{`{"action":"phase","phase":"night"}`, "", nil},
	} {
		code, body := devCall(t, srv, http.MethodPost, q, c.body)
		s, _ := body["room"].(map[string]any)
		if code != 200 || (c.field != "" && s[c.field] != c.want) {
			t.Fatalf("%s: %d %v", c.body, code, body)
		}
	}
	for _, bad := range []string{`{"action":"wave","slot":3}`, `{"action":"phase","phase":"mittag"}`, `{"action":"gold"}`, `kaputt`} {
		if code, _ := devCall(t, srv, http.MethodPost, q, bad); code != 400 {
			t.Fatalf("%s: %d, erwartet 400", bad, code)
		}
	}
	if code, _ := devCall(t, srv, http.MethodPost, q, `{"action":"pause","paused":false}`); code != 200 {
		t.Fatal("Pause nicht aufgehoben")
	}
	r.Tick() // die Nacht beginnt im nächsten Schritt: Phase und Nachtwelle
	if s := r.Summary(); s.Phase != "night" || s.Wave != 2 {
		t.Fatalf("Phase %s Welle %d", s.Phase, s.Wave)
	}
}

// B-232/AC-01: /dm liefert dm.html.
func TestDevSeiteUnterDm(t *testing.T) {
	root := t.TempDir()
	if err := os.WriteFile(filepath.Join(root, "dm.html"), []byte("<title>DM</title>"), 0o600); err != nil {
		t.Fatal(err)
	}
	srv := httptest.NewServer(NewHandler(Config{Dist: root}))
	t.Cleanup(srv.Close)
	res, err := http.Get(srv.URL + "/dm")
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = res.Body.Close() }()
	if res.StatusCode != 200 || !strings.HasPrefix(res.Header.Get("Content-Type"), "text/html") {
		t.Fatalf("/dm: %d %s", res.StatusCode, res.Header.Get("Content-Type"))
	}
}

// B-080/AC-02, K4.2: grade kommt über POST /api/dev im Raum an; ein unbekannter Grad ist 400.
func TestDevSeiteGrad(t *testing.T) {
	srv, r := devPageRoom(t, true)
	if code, _ := devCall(t, srv, http.MethodPost, "?room="+r.Code, `{"action":"grade","grade":"hard"}`); code != 200 {
		t.Fatalf("grade: %d", code)
	}
	if code, _ := devCall(t, srv, http.MethodPost, "?room="+r.Code, `{"action":"grade","grade":"gibtsnicht"}`); code != 400 {
		t.Fatalf("unbekannter Grad: %d", code)
	}
}
