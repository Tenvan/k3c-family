package net

import (
	"encoding/json"
	"net/http/httptest"
	"path/filepath"
	"testing"
	"time"

	"k3c/engine/store"
)

// B-175/AC-04: mit Quelle steht cpu im Status, ohne Quelle fehlt das Feld.
func TestStatusNenntCPU(t *testing.T) {
	old := systemCPU
	t.Cleanup(func() { systemCPU = old })
	for _, tc := range []struct {
		name     string
		sys, cfg func() (float64, bool)
		want     any
	}{
		{"Quelle in Config", nil, func() (float64, bool) { return 42.5, true }, 42.5},
		{"Systemquelle", func() (float64, bool) { return 7, true }, nil, 7.0},
		{"keine Quelle", nil, nil, nil},
		{"Quelle ohne Wert", nil, func() (float64, bool) { return 0, false }, nil},
	} {
		systemCPU = tc.sys
		root := t.TempDir()
		srv := httptest.NewServer(NewHandler(Config{Dist: root, StatusToken: "tok", CPU: tc.cfg,
			Saves: &store.Saves{Dir: filepath.Join(root, "saves")}, Reports: &store.Reports{Dir: filepath.Join(root, "reports")}}))
		code, body := get(t, srv.URL+"/api/status", "Bearer tok")
		srv.Close()
		var st map[string]any
		if code != 200 || json.Unmarshal([]byte(body), &st) != nil {
			t.Fatalf("%s: %d %s", tc.name, code, body)
		}
		if got, ok := st["cpu"]; got != tc.want || ok != (tc.want != nil) {
			t.Errorf("%s: cpu = %v (da: %v), erwartet %v", tc.name, got, ok, tc.want)
		}
	}
}

// Das Messgerät rechnet kumulative CPU-Zeit in Prozent einer CPU um; ohne Quelle gibt es keins.
func TestCPUMeterProzent(t *testing.T) {
	var used time.Duration
	f := newCPUMeter(func() (time.Duration, bool) { return used, true })
	if f == nil {
		t.Fatal("Quelle fehlt")
	}
	f() // erster Aufruf setzt den Vorwert
	time.Sleep(100 * time.Millisecond)
	used += 50 * time.Millisecond
	if pct, ok := f(); !ok || pct < 20 || pct > 60 {
		t.Errorf("cpu = %v, %v (erwartet ca. 50)", pct, ok)
	}
	if newCPUMeter(func() (time.Duration, bool) { return 0, false }) != nil {
		t.Error("ohne Quelle muss nil kommen")
	}
}
