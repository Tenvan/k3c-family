package net

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/engine/level"
	"k3c/engine/room"
)

func levelServer(t *testing.T) *httptest.Server {
	t.Helper()
	srv, _ := testServer(t)
	return srv
}

// erwartet baut dieselbe Antwort direkt aus engine/level, wie der Endpunkt sie liefern soll.
func erwartet(t *testing.T, biomeID, seed string) string {
	t.Helper()
	b, err := level.LoadBiome(biomeID)
	if err != nil {
		t.Fatal(err)
	}
	l, err := level.Generate(b, seed)
	if err != nil {
		t.Fatal(err)
	}
	warn := level.Validate(l, b)
	if warn == nil {
		warn = []string{}
	}
	raw, err := json.Marshal(levelResponse{Layout: l, Warnings: warn})
	if err != nil {
		t.Fatal(err)
	}
	return string(raw)
}

func sameJSON(t *testing.T, got, want string) {
	t.Helper()
	var a, b any
	if err := json.Unmarshal([]byte(got), &a); err != nil {
		t.Fatalf("Antwort ist kein JSON: %v\n%s", err, got)
	}
	if err := json.Unmarshal([]byte(want), &b); err != nil {
		t.Fatal(err)
	}
	ga, _ := json.Marshal(a)
	gb, _ := json.Marshal(b)
	if string(ga) != string(gb) {
		t.Errorf("Antwort weicht von engine/level ab:\n%s\n%s", ga, gb)
	}
}

// B-091/AC-01: Seed und Biom ergeben dasselbe Level wie engine/level (Felder und Warnungen), zweimal identisch.
func TestLevelStimmtMitEngineUeberein(t *testing.T) {
	srv := levelServer(t)
	for _, biome := range []string{"forest", "cave", "mine"} {
		res, first := do(t, "GET", srv.URL+"/api/level?seed=test&biome="+biome, "")
		if res.StatusCode != 200 || !strings.HasPrefix(res.Header.Get("Content-Type"), "application/json") {
			t.Fatalf("%s: %d %q", biome, res.StatusCode, res.Header.Get("Content-Type"))
		}
		sameJSON(t, first, erwartet(t, biome, "test"))
		if _, second := do(t, "GET", srv.URL+"/api/level?seed=test&biome="+biome, ""); second != first {
			t.Errorf("%s: zweiter Aufruf weicht ab", biome)
		}
	}
}

func TestLevelWarningsSindImmerEinArray(t *testing.T) {
	srv := levelServer(t)
	_, body := do(t, "GET", srv.URL+"/api/level?seed=test", "")
	var got struct {
		Warnings *[]string `json:"warnings"`
	}
	if err := json.Unmarshal([]byte(body), &got); err != nil || got.Warnings == nil {
		t.Fatalf("warnings fehlt oder ist null: %v\n%s", err, body)
	}
}

// Entschieden (🧑): ohne Seed gilt "k3c", ohne Biom "forest".
func TestLevelStandardwerte(t *testing.T) {
	srv := levelServer(t)
	_, body := do(t, "GET", srv.URL+"/api/level", "")
	sameJSON(t, body, erwartet(t, "forest", "k3c"))
	_, body = do(t, "GET", srv.URL+"/api/level?seed=a", "")
	sameJSON(t, body, erwartet(t, "forest", "a"))
	_, body = do(t, "GET", srv.URL+"/api/level?biome=cave", "")
	sameJSON(t, body, erwartet(t, "cave", "k3c"))
}

// B-091/AC-02: unbekanntes Biom und zu langer Seed → 400, andere Methoden → 405.
func TestLevelLehntEingabenAb(t *testing.T) {
	srv := levelServer(t)
	for _, query := range []string{"biome=xyz", "biome=../forest", "biome=forest.json", "biome=FOREST", "seed=" + strings.Repeat("a", 65), "seed=" + strings.Repeat("ä", 65)} {
		res, body := do(t, "GET", srv.URL+"/api/level?"+query, "")
		if res.StatusCode != 400 || !strings.Contains(body, `"error"`) {
			t.Errorf("%.40s: %d %s", query, res.StatusCode, body)
		}
	}
	for _, query := range []string{"seed=" + strings.Repeat("a", 64), "seed=" + strings.Repeat("ä", 64)} {
		if res, _ := do(t, "GET", srv.URL+"/api/level?"+query, ""); res.StatusCode != 200 {
			t.Errorf("%.40s: %d, erwartet 200", query, res.StatusCode)
		}
	}
	for _, method := range []string{"POST", "PUT", "DELETE"} {
		if res, _ := do(t, method, srv.URL+"/api/level?seed=test", "{}"); res.StatusCode != http.StatusMethodNotAllowed {
			t.Errorf("%s: %d, erwartet 405", method, res.StatusCode)
		}
	}
}

// B-091/AC-03: kein Raum, keine Datei. (Die Räume liest `Manager.Rooms()`, dieselbe Quelle wie /api/status.)
func TestLevelLegtNichtsAn(t *testing.T) {
	root := t.TempDir()
	m := room.NewManager(memSaves{})
	srv := httptest.NewServer(NewHandler(Config{Dist: root, Rooms: m, Version: "test"}))
	t.Cleanup(srv.Close)
	for _, query := range []string{"", "?seed=test&biome=cave", "?biome=xyz"} {
		do(t, "GET", srv.URL+"/api/level"+query, "")
	}
	if rooms := m.Rooms(); len(rooms) != 0 {
		t.Errorf("Räume nach /api/level: %+v", rooms)
	}
	entries, err := os.ReadDir(root)
	if err != nil || len(entries) != 0 {
		names := []string{}
		for _, e := range entries {
			names = append(names, filepath.Join(root, e.Name()))
		}
		t.Errorf("Dateien nach /api/level: %v (%v)", names, err)
	}
}
