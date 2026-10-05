package net

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"k3c/engine/room"
)

func metricsServer(t *testing.T, token string) (*httptest.Server, *room.Manager) {
	t.Helper()
	m := room.NewManager(memSaves{})
	srv := httptest.NewServer(NewHandler(Config{Rooms: m, StatusToken: token}))
	t.Cleanup(srv.Close)
	return srv, m
}

// B-281/AC-02: Schutz wie /api/status: ohne gesetztes Token 404, falsches oder fehlendes Token 401, falsche Methode 405.
func TestMetricsNurMitToken(t *testing.T) {
	off, _ := metricsServer(t, "")
	if code, _ := get(t, off.URL+"/api/metrics", "Bearer egal"); code != http.StatusNotFound {
		t.Errorf("ohne Token: %d", code)
	}
	srv, _ := metricsServer(t, "geheim-123")
	for _, auth := range []string{"", "Bearer falsch", "geheim-123"} {
		if code, _ := get(t, srv.URL+"/api/metrics", auth); code != http.StatusUnauthorized {
			t.Errorf("%q: %d", auth, code)
		}
	}
	req, _ := http.NewRequest(http.MethodPost, srv.URL+"/api/metrics", nil)
	req.Header.Set("Authorization", "Bearer geheim-123")
	res, err := http.DefaultClient.Do(req)
	if err != nil || res.StatusCode != http.StatusMethodNotAllowed {
		t.Errorf("POST: %v %v", res, err)
	}
	if code, _ := get(t, srv.URL+"/api/metrics", "Bearer geheim-123"); code != http.StatusOK {
		t.Errorf("mit Token: %d", code)
	}
}

// Ein Client-Fehler aus /api/clientlog wird ein Diagnose-Ereignis ohne Raum; Warnungen nicht.
func TestClientFehlerAlsEreignis(t *testing.T) {
	srv, m := metricsServer(t, "geheim-123")
	body := `{"entries":[{"level":"warn","msg":"langsam","device":"handy-0001"},{"level":"error","msg":"kaputt","device":"handy-0001"}]}`
	res, err := http.Post(srv.URL+"/api/clientlog", "application/json", strings.NewReader(body))
	if err != nil || res.StatusCode != http.StatusNoContent {
		t.Fatalf("clientlog: %v %v", res, err)
	}
	ev := m.Monitor.Since(0, time.Now()).Events
	if len(ev) != 1 || ev[0].Kind != "client" || ev[0].Room != "" || ev[0].Text != "handy-00: kaputt" {
		t.Errorf("Ereignisse %+v", ev)
	}
}

// B-281/AC-02: since liefert nur neuere Punkte und Ereignisse (Delta), dazu startedAt; ohne gültiges since alles.
func TestMetricsDelta(t *testing.T) {
	srv, m := metricsServer(t, "geheim-123")
	now := time.UnixMilli(2000)
	m.Now = func() time.Time { return now }
	m.Monitor.Device("geraet01", "KRNZ", room.DevicePoint{T: 1000, RTTMs: 12})
	m.Monitor.Device("geraet01", "KRNZ", room.DevicePoint{T: 2000, RTTMs: 15, Queue: 2})
	m.Monitor.Event(time.UnixMilli(1500), "KRNZ", "drop", "geraet01")
	m.Sample()
	read := func(query string) room.Metrics { return readMetrics(t, srv.URL+"/api/metrics"+query) }
	d := read("?since=1500")
	if p := d.Devices["geraet01"].Points; len(p) != 1 || p[0].T != 2000 || p[0].Queue != 2 || d.Devices["geraet01"].Room != "KRNZ" {
		t.Errorf("Delta Gerät: %+v", d.Devices)
	}
	if len(d.Events) != 0 || len(d.Server) != 1 || d.StartedAt <= 0 || d.Now < d.StartedAt {
		t.Errorf("Delta: %+v", d)
	}
	if d = read("?since=2000"); len(d.Devices) != 0 || len(d.Server) != 0 {
		t.Errorf("nichts Neues: %+v", d)
	}
	if all := read("?since=kaputt"); len(all.Devices["geraet01"].Points) != 2 || len(all.Events) != 1 || all.Events[0].Kind != "drop" {
		t.Errorf("alles: %+v", all)
	}
}

func readMetrics(t *testing.T, url string) room.Metrics {
	t.Helper()
	code, body := get(t, url, "Bearer geheim-123")
	var out room.Metrics
	if code != http.StatusOK || json.Unmarshal([]byte(body), &out) != nil {
		t.Fatalf("%s: %d %s", url, code, body)
	}
	return out
}
