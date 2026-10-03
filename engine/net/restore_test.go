package net

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"k3c/engine/store"
)

// restoreServer ist ein Server mit Token und mitgeschriebenem Log; zwei Speicherstände (n=1, n=2) liefern eine Sicherung.
func restoreServer(t *testing.T, token string) (*httptest.Server, *bytes.Buffer, string) {
	t.Helper()
	root, buf := t.TempDir(), &bytes.Buffer{}
	srv := httptest.NewServer(NewHandler(Config{Dist: root, StatusToken: token, Log: slog.New(slog.NewTextHandler(buf, nil)),
		Saves: &store.Saves{Dir: filepath.Join(root, "saves")}, Reports: &store.Reports{Dir: filepath.Join(root, "reports")}}))
	t.Cleanup(srv.Close)
	for i := 1; i <= 2; i++ {
		if code := postSave(t, srv, fmt.Sprintf(`{"version":1,"campaignId":"a","n":%d}`, i)); code != 200 {
			t.Fatalf("save %d: %d", i, code)
		}
		time.Sleep(2 * time.Millisecond)
	}
	code, body := get(t, srv.URL+"/api/save/backups?slot=autosave", "")
	var list []store.Backup
	if code != 200 || json.Unmarshal([]byte(body), &list) != nil || len(list) != 1 {
		t.Fatalf("Sicherungen: %d %s", code, body)
	}
	return srv, buf, list[0].Name
}

// postSave ist POST /api/save ohne Token, wie es das Spiel schickt.
func postSave(t *testing.T, srv *httptest.Server, body string) int {
	t.Helper()
	res, err := http.Post(srv.URL+"/api/save", "application/json", strings.NewReader(body))
	if err != nil {
		t.Fatal(err)
	}
	_ = res.Body.Close()
	return res.StatusCode
}

// restored meldet, ob der Spielstand wieder der aus der Sicherung (n=1) ist.
func restored(t *testing.T, srv *httptest.Server) bool {
	t.Helper()
	_, body := get(t, srv.URL+"/api/save", "")
	return strings.Contains(body, `"n":1`)
}

// B-143/AC-01: Restore nur mit Token; abgelehnt bleibt der Spielstand unverändert, ohne gesetztes Token ist Restore aus.
func TestRestoreNurMitToken(t *testing.T) {
	srv, _, backup := restoreServer(t, diagToken)
	path := "/api/save/restore?backup=" + backup
	for _, auth := range []string{"", "Bearer falsch", diagToken} {
		if code, _ := post(t, srv, path, auth); code != 401 {
			t.Errorf("%q: %d", auth, code)
		}
	}
	if restored(t, srv) {
		t.Fatal("abgelehnt, aber zurückgespielt")
	}
	if code, _ := post(t, srv, path, "Bearer "+diagToken); code != 200 || !restored(t, srv) {
		t.Errorf("mit Token: %d", code)
	}

	off, _, backup := restoreServer(t, "")
	if code, _ := post(t, off, "/api/save/restore?backup="+backup, "Bearer x"); code != 404 || restored(t, off) {
		t.Errorf("ohne Variable: %d", code)
	}
}

// B-143/AC-02: Mit gesetztem Token laufen Lobby, /ws und Spielstände ohne Header weiter.
func TestRestoreSpielbetriebOhneToken(t *testing.T) {
	srv, _ := diagServer(t, "")
	c := hello(t, srv, "xbox") // welcome und rooms (Raumliste der Lobby)
	create(c, "familie", 0)
	c.entered()
	if code := postSave(t, srv, `{"version":1,"campaignId":"a"}`); code != 200 {
		t.Errorf("POST /api/save: %d", code)
	}
	if code, body := get(t, srv.URL+"/api/save", ""); code != 200 || !strings.Contains(body, `"campaignId":"a"`) {
		t.Errorf("GET /api/save: %d %s", code, body)
	}
}

// B-143/AC-03: Jede Ablehnung genau ein Logeintrag mit Pfad, Aufrufer und Grund, ohne Token.
func TestRestoreAblehnungImLog(t *testing.T) {
	srv, buf, backup := restoreServer(t, diagToken)
	if code, _ := post(t, srv, "/api/save/restore?backup="+backup, "Bearer falsch-xyz"); code != 401 {
		t.Fatalf("falsches Token: %d", code)
	}
	srv.Close() // wartet auf den Handler, danach ist das Log vollständig
	log := buf.String()
	if n := strings.Count(log, "restore abgelehnt"); n != 1 {
		t.Fatalf("%d Einträge:\n%s", n, log)
	}
	for _, want := range []string{"ns=save", "path=/api/save/restore", "caller=127.0.0.1 ", `reason="Token fehlt oder falsch"`} {
		if !strings.Contains(log, want) {
			t.Errorf("fehlt %q:\n%s", want, log)
		}
	}
	for _, secret := range []string{"falsch-xyz", diagToken, "Bearer"} {
		if strings.Contains(log, secret) {
			t.Errorf("%q im Log:\n%s", secret, log)
		}
	}
}
