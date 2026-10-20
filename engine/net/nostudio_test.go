package net

import (
	"net/http"
	"testing"
)

// Der Spielserver hat keine Studio-Seite mehr (B-507/AC-03): Oberfläche und API des Studios antworten 404, auch im
// Dev-Mode; das Studio ist ein eigenes Fenster mit eigenem Server.
func TestKeineStudioSeite(t *testing.T) {
	srv, _ := testServer(t)
	cases := []struct{ method, path string }{
		{"GET", "/studio/"},
		{"GET", "/studio/index.html"},
		{"GET", "/studio.html"},
		{"GET", "/api/studio/ping"},
		{"POST", "/api/studio/rpc/worldList"},
	}
	for _, c := range cases {
		if res, _ := do(t, c.method, srv.URL+c.path, ""); res.StatusCode != http.StatusNotFound {
			t.Errorf("%s %s: %d, erwartet 404", c.method, c.path, res.StatusCode)
		}
	}
}
