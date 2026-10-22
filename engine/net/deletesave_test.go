package net

import (
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"testing"

	"k3c/engine/room"
	"k3c/engine/store"
)

// BD5.1, B-506/AC-01: DELETE /api/save löscht einen Stand (200), fehlend → 404, ungültig → 400, in einem Raum offen → 409.
func TestSaveLoeschenUeberHTTP(t *testing.T) {
	saves := &store.Saves{Dir: filepath.Join(t.TempDir(), "saves")}
	m := room.NewManager(saves)
	srv := httptest.NewServer(NewHandler(Config{Saves: saves, Rooms: m}))
	t.Cleanup(srv.Close)
	if _, err := saves.Store("nord", []byte(`{"version":8,"campaignId":"nord","world":"standard"}`)); err != nil {
		t.Fatal(err)
	}
	c := hello(t, srv, "xbox")
	create(c, "offen", 0)
	c.entered()

	for _, tc := range []struct {
		slot string
		code int
		body string
	}{
		{"nord", http.StatusOK, `"ok":true`},
		{"nord", http.StatusNotFound, "Kein Spielstand"},
		{"offen", http.StatusConflict, "Spielstand ist in einem Raum offen"},
		{"..%2Fx", http.StatusBadRequest, "Ungültiger Slot"},
	} {
		res, body := do(t, http.MethodDelete, srv.URL+"/api/save?slot="+tc.slot, "")
		if res.StatusCode != tc.code || !strings.Contains(body, tc.body) {
			t.Fatalf("%s: %d %s", tc.slot, res.StatusCode, body)
		}
	}
	for _, s := range saves.List() {
		if s.Name == "nord" {
			t.Fatalf("Liste: %+v", saves.List())
		}
	}
	if b, _ := saves.Backups("nord"); len(b) != 1 {
		t.Fatalf("Sicherung fehlt: %v", b)
	}
	m.Close() // speichert den offenen Raum jetzt, nicht erst während des Aufräumens von TempDir
}
