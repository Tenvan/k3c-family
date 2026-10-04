package net

import (
	"io"
	"net/http"
	"strings"
	"testing"
)

// (e) S2.2, B-147/AC-04: GET /api/saves liefert Zeitpunkt, Version, Tag, Phase und Tiefen; andere Methoden → 405.
func TestSavesListeUeberHTTP(t *testing.T) {
	srv, _ := testServer(t)
	get := func() (int, string) {
		t.Helper()
		res, err := http.Get(srv.URL + "/api/saves")
		if err != nil {
			t.Fatal(err)
		}
		defer func() { _ = res.Body.Close() }()
		body, _ := io.ReadAll(res.Body)
		return res.StatusCode, strings.TrimSpace(string(body))
	}
	if code, body := get(); code != http.StatusOK || body != "[]" {
		t.Fatalf("leer: %d %s", code, body)
	}
	save := `{"version":4,"campaignId":"familie","savedAt":"2026-10-04T22:00:00Z","day":2,"phase":"night",` +
		`"players":[{"index":0,"depth":1},{"index":1,"depth":0}]}`
	res, err := http.Post(srv.URL+"/api/save?slot=familie", "application/json", strings.NewReader(save))
	if err != nil {
		t.Fatal(err)
	}
	_ = res.Body.Close()
	want := `[{"name":"familie","savedAt":"2026-10-04T22:00:00Z","version":4,"day":2,"phase":"night","depths":[0,1]}]`
	if code, body := get(); code != http.StatusOK || body != want {
		t.Fatalf("Liste: %d %s", code, body)
	}
	res, err = http.Post(srv.URL+"/api/saves", "application/json", strings.NewReader("{}"))
	if err != nil {
		t.Fatal(err)
	}
	_ = res.Body.Close()
	if res.StatusCode != http.StatusMethodNotAllowed {
		t.Fatalf("POST: %d", res.StatusCode)
	}
}
