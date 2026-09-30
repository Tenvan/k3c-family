package net

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"k3c/engine/store"
)

func statusServer(t *testing.T, token string) *httptest.Server {
	t.Helper()
	root := t.TempDir()
	srv := httptest.NewServer(NewHandler(Config{Dist: root, StatusToken: token, Version: "test",
		StartedAt: time.Now().Add(-time.Minute), Saves: &store.Saves{Dir: filepath.Join(root, "saves")},
		Reports: &store.Reports{Dir: filepath.Join(root, "reports")}}))
	t.Cleanup(srv.Close)
	return srv
}

func get(t *testing.T, url, auth string) (int, string) {
	t.Helper()
	req, _ := http.NewRequest("GET", url, nil)
	if auth != "" {
		req.Header.Set("Authorization", auth)
	}
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = res.Body.Close() }()
	data, _ := io.ReadAll(res.Body)
	return res.StatusCode, string(data)
}

// B-027/AC-01 und AC-02: ohne gültiges Token 401, mit Token der Status; ohne Variable ist die Diagnose aus.
func TestStatusNurMitToken(t *testing.T) {
	srv := statusServer(t, "geheim-123")
	for _, auth := range []string{"", "Bearer falsch", "geheim-123", "Bearer geheim-1234", "bearer geheim-123"} {
		if code, _ := get(t, srv.URL+"/api/status", auth); code != 401 {
			t.Errorf("%q: %d", auth, code)
		}
	}
	code, body := get(t, srv.URL+"/api/status", "Bearer geheim-123")
	var st map[string]any
	if code != 200 || json.Unmarshal([]byte(body), &st) != nil || st["version"] != "test" || st["uptimeS"].(float64) < 59 {
		t.Errorf("mit Token: %d %s", code, body)
	}
	if code, _ := get(t, statusServer(t, "").URL+"/api/status", "Bearer x"); code != 404 {
		t.Errorf("ohne Variable: %d", code)
	}
}

// B-028/AC-02: Sicherungen per API listen und wiederherstellen.
func TestSicherungenPerAPI(t *testing.T) {
	srv := statusServer(t, "")
	for i := 1; i <= 3; i++ {
		res, err := http.Post(srv.URL+"/api/save", "application/json",
			strings.NewReader(fmt.Sprintf(`{"version":1,"campaignId":"a","n":%d}`, i)))
		if err != nil || res.StatusCode != 200 {
			t.Fatal(err, res.StatusCode)
		}
		_ = res.Body.Close()
		time.Sleep(2 * time.Millisecond)
	}
	code, body := get(t, srv.URL+"/api/save/backups?slot=autosave", "")
	var list []store.Backup
	if code != 200 || json.Unmarshal([]byte(body), &list) != nil || len(list) != 2 {
		t.Fatalf("Liste: %d %s", code, body)
	}
	res, _ := http.Post(srv.URL+"/api/save/restore?backup="+list[1].Name, "", nil)
	if res.StatusCode != 200 {
		t.Fatalf("restore: %d", res.StatusCode)
	}
	if _, body := get(t, srv.URL+"/api/save", ""); !strings.Contains(body, `"n":1`) {
		t.Errorf("nach restore: %s", body)
	}
	for _, q := range []string{"backup=../autosave.json", "backup=fehlt.json", "slot=../x&backup=a"} {
		res, _ := http.Post(srv.URL+"/api/save/restore?"+q, "", nil)
		if res.StatusCode != 404 && res.StatusCode != 400 {
			t.Errorf("%s: %d", q, res.StatusCode)
		}
	}
	if code, _ := get(t, srv.URL+"/api/save/restore", ""); code != 405 {
		t.Errorf("GET restore: %d", code)
	}
}
