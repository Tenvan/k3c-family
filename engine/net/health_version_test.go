package net

import (
	"encoding/json"
	"testing"
)

// AC-03 (F5.2): /api/health meldet ok und die Server-Version, ok bleibt für Healthcheck und Landingpage.
func TestHealthMeldetVersion(t *testing.T) {
	srv, _ := testServer(t)
	res, body := do(t, "GET", srv.URL+"/api/health", "")
	var h struct {
		OK      bool   `json:"ok"`
		Version string `json:"version"`
	}
	if err := json.Unmarshal([]byte(body), &h); err != nil || res.StatusCode != 200 || !h.OK || h.Version != "v9.9.9" {
		t.Errorf("health: %d %q", res.StatusCode, body)
	}
}
