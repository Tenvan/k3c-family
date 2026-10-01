package main

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	tea "charm.land/bubbletea/v2"
	"charm.land/lipgloss/v2"
)

const token = "geheim-123"

// statusJSON hat die Form von GET /api/status (engine/net/status.go, Sprint D1).
const statusJSON = `{"version":"v0.2.0","startedAt":"x","uptimeS":3720,"saves":2,"reports":1,
 "memory":{"heapMB":4.5,"sysMB":12.3,"numGC":7},
 "rooms":[
  {"code":"FAMILIE","name":"Familie","depth":0,"taken":2,"free":2,"running":true,"devices":2,"monarchs":["taken","taken","waiting","free"],"tick":900,"tickMs":{"last":0.4,"p99":1.25}},
  {"code":"TEST1","name":"ein sehr langer Name","depth":1,"taken":1,"free":3,"running":true,"devices":1,"monarchs":["taken","free","free","free"],"tick":30,"tickMs":{"last":0.2,"p99":0.5}}],
 "failures":[{"code":"X","name":"alt","error":"boom","at":"2026-10-01T10:00:00Z"}]}`

// fake antwortet wie der Server: Token Pflicht (leer = Diagnose aus, 404), falsches Token 401.
func fake(t *testing.T, serverToken string) *httptest.Server {
	t.Helper()
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch {
		case serverToken == "":
			http.NotFound(w, r)
		case r.Header.Get("Authorization") != "Bearer "+serverToken:
			http.Error(w, `{"error":"Token fehlt oder falsch"}`, http.StatusUnauthorized)
		default:
			_, _ = w.Write([]byte(statusJSON))
		}
	}))
	t.Cleanup(srv.Close)
	return srv
}

func env(url, tok string) func(string) string {
	return func(k string) string {
		switch k {
		case envURL:
			return url
		case envToken:
			return tok
		}
		return ""
	}
}

func parsed(t *testing.T) status {
	t.Helper()
	srv := fake(t, token)
	st, err := clientFromEnv(env(srv.URL, token)).status(t.Context())
	if err != nil {
		t.Fatal(err)
	}
	return st
}

// AC-01 (a): Kopfzeile, beide Räume mit Tick-Dauer, Abstürze; in schmaler Breite nichts über der Breite.
func TestRenderZeigtServerRaeumeAbstuerze(t *testing.T) {
	st := parsed(t)
	out := render(view{addr: "http://x:8080", st: &st, plain: true})
	for _, want := range []string{"K3C v0.2.0", "läuft 1 h 2 min", "4.5 MB Heap", "2 Spielstände", "FAMILIE", "TTWF", "0.40 ms (1.25)", "TEST1", "ein sehr lang…", "Abstürze", "X (alt) 2026-10-01T10:00:00Z: boom"} {
		if !strings.Contains(out, want) {
			t.Errorf("%q fehlt in\n%s", want, out)
		}
	}
	for _, width := range []int{20, 40, 80} {
		for _, line := range strings.Split(render(view{addr: "http://x:8080", st: &st, width: width}), "\n") {
			if lipgloss.Width(line) > width {
				t.Errorf("Breite %d: Zeile mit %d Zeichen: %q", width, lipgloss.Width(line), line)
			}
		}
	}
}

func TestRenderOhneRaeumeUndOhneZustand(t *testing.T) {
	if out := render(view{addr: "http://x", plain: true}); !strings.Contains(out, "Warte auf http://x") {
		t.Errorf("ohne Zustand: %q", out)
	}
	if out := render(view{addr: "x", st: &status{Version: "v"}, plain: true}); !strings.Contains(out, "Keine Räume") {
		t.Errorf("ohne Räume: %q", out)
	}
}

func runOnce(t *testing.T, url, tok string) (code int, stdout, stderr string) {
	t.Helper()
	var out, errb bytes.Buffer
	code = run([]string{"-once"}, env(url, tok), &out, &errb)
	return code, out.String(), errb.String()
}

// AC-01 (b): -once druckt Version und beide Räume.
func TestOnceDrucktZustand(t *testing.T) {
	srv := fake(t, token)
	code, out, errOut := runOnce(t, srv.URL, token)
	if code != 0 || errOut != "" || !strings.Contains(out, "v0.2.0") || !strings.Contains(out, "FAMILIE") || !strings.Contains(out, "TEST1") {
		t.Errorf("-once: %d\n%s\n%s", code, out, errOut)
	}
	if strings.Contains(out, "\x1b[") {
		t.Errorf("-once druckt Farbcodes: %q", out)
	}
}

// AC-01 (c) und (d): Fehlerfälle mit klarer Meldung, Token nie in der Ausgabe.
func TestOnceFehlerMeldungen(t *testing.T) {
	srv := fake(t, token)
	code, out, errOut := runOnce(t, srv.URL, "falsch-456")
	if code != 1 || out != "" || !strings.Contains(errOut, "401, Token prüfen") || strings.Contains(errOut, "falsch-456") {
		t.Errorf("falsches Token: %d %q %q", code, out, errOut)
	}
	if code, _, errOut = runOnce(t, fake(t, "").URL, token); code != 1 || !strings.Contains(errOut, "Diagnose am Server aus") {
		t.Errorf("Diagnose aus: %d %q", code, errOut)
	}
	dead := httptest.NewServer(http.NotFoundHandler())
	url := dead.URL
	dead.Close()
	if code, _, errOut = runOnce(t, url, token); code != 1 || !strings.Contains(errOut, "Server nicht erreichbar unter "+url) || strings.Contains(errOut, token) {
		t.Errorf("Server aus: %d %q", code, errOut)
	}
}

func TestAdresseAusPortUndStandard(t *testing.T) {
	get := func(m map[string]string) func(string) string { return func(k string) string { return m[k] } }
	if c := clientFromEnv(get(nil)); c.base != "http://127.0.0.1:8080" {
		t.Errorf("Standard: %s", c.base)
	}
	if c := clientFromEnv(get(map[string]string{envPort: "9123"})); c.base != "http://127.0.0.1:9123" {
		t.Errorf("K3C_HTTP_PORT: %s", c.base)
	}
	if c := clientFromEnv(get(map[string]string{envURL: "http://pi:8080", envPort: "1"})); c.base != "http://pi:8080" {
		t.Errorf("K3C_SERVER_URL: %s", c.base)
	}
}

// settle führt einen Befehl aus und liefert seine Nachricht.
func settle(cmd tea.Cmd) tea.Msg {
	if cmd == nil {
		return nil
	}
	return cmd()
}

// AC-01 (e): Antwort setzt den Zustand und plant den nächsten Abruf; Fehler setzt die Meldung und plant genau einen Versuch;
// ein Erfolg löscht die Meldung wieder.
func TestModellAbrufUndWiederholung(t *testing.T) {
	srv := fake(t, token)
	m := newModel(clientFromEnv(env(srv.URL, token)))
	m.refresh, m.retry = time.Millisecond, time.Millisecond

	msg := settle(m.Init())
	next, cmd := m.Update(msg)
	m = next.(model)
	if m.st == nil || m.st.Version != "v0.2.0" || m.err != "" {
		t.Fatalf("nach Antwort: %+v", m)
	}
	if _, ok := settle(cmd).(tickMsg); !ok {
		t.Fatal("nach einer Antwort fehlt der nächste Takt")
	}

	next, cmd = m.Update(statusMsg{err: failf("Server nicht erreichbar unter x")})
	m = next.(model)
	if m.err != "Server nicht erreichbar unter x" || m.st == nil {
		t.Fatalf("nach Fehler (letzter Zustand bleibt): %+v", m)
	}
	if _, ok := settle(cmd).(tickMsg); !ok {
		t.Fatal("nach einem Fehler fehlt der neue Versuch")
	}
	if !strings.Contains(render(view{addr: "a", st: m.st, err: m.err, plain: true}), "neuer Versuch in 2 s") {
		t.Error("Fehlerzeile ohne Hinweis auf den neuen Versuch")
	}

	next, cmd = m.Update(tickMsg{})
	next, _ = next.(model).Update(settle(cmd))
	if m = next.(model); m.err != "" {
		t.Errorf("Erfolg löscht die Meldung nicht: %q", m.err)
	}
}

// AC-01: der Takt der Anzeige ist eine Sekunde, der Wiederholungsversuch zwei.
func TestTaktwerte(t *testing.T) {
	m := newModel(clientFromEnv(env("http://x", "t")))
	if m.refresh != time.Second || m.retry != 2*time.Second {
		t.Errorf("Takt: %v / %v", m.refresh, m.retry)
	}
}

// AC-01 (f): q und Strg+C beenden, Fenstergröße wird übernommen, View schaltet den Alternativbildschirm ein.
func TestModellTastenUndGroesse(t *testing.T) {
	m := newModel(clientFromEnv(env("http://x", "t")))
	for _, key := range []tea.KeyPressMsg{{Code: 'q', Text: "q"}, {Code: 'c', Mod: tea.ModCtrl}} {
		_, cmd := m.Update(key)
		if _, quit := settle(cmd).(tea.QuitMsg); !quit {
			t.Errorf("%v beendet nicht", key)
		}
	}
	next, _ := m.Update(tea.WindowSizeMsg{Width: 33, Height: 10})
	if next.(model).width != 33 {
		t.Error("Breite nicht übernommen")
	}
	if v := m.View(); !v.AltScreen || !strings.Contains(v.Content, "q beenden") {
		t.Errorf("View: %+v", v)
	}
}
