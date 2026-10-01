package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync"
	"testing"
	"time"

	tea "charm.land/bubbletea/v2"
)

const roomJSON = `{"code":"FAMILIE","depth":0,"tick":900,"phase":"day","day":2,"gold":[12,7],"troops":{"peasant":3,"archer":1},"enemies":0,"castleHp":950,
 "wave":1,"devices":[{"id":"aaaaaa","connected":true,"slots":[0,1]},{"id":"bbbbbb","connected":false,"slots":[2]}]}`

// diag ist ein Server im Antwortformat von Sprint D1; er merkt sich jede Anfrage und lässt Aktionen mit einem Fehler antworten.
type diag struct {
	*httptest.Server
	mu       sync.Mutex
	requests []string
	actErr   int      // Statuscode für POST-Aktionen (0 = ok)
	logPages []string // JSON-Antworten von /api/status/log, der Reihe nach (die letzte wiederholt sich)
	logCalls int
}

func newDiag(t *testing.T) *diag {
	t.Helper()
	d := &diag{}
	d.Server = httptest.NewServer(http.HandlerFunc(d.serve))
	t.Cleanup(d.Close)
	return d
}

func (d *diag) serve(w http.ResponseWriter, r *http.Request) {
	d.mu.Lock()
	defer d.mu.Unlock()
	d.requests = append(d.requests, r.Method+" "+r.URL.RequestURI())
	if r.Header.Get("Authorization") != "Bearer "+token {
		http.Error(w, `{"error":"Token fehlt oder falsch"}`, http.StatusUnauthorized)
		return
	}
	switch {
	case r.URL.Path == "/api/status/log":
		if len(d.logPages) == 0 {
			http.Error(w, `{"error":"Log aus"}`, http.StatusNotFound)
			return
		}
		page := d.logPages[min(d.logCalls, len(d.logPages)-1)]
		d.logCalls++
		_, _ = w.Write([]byte(page))
	case r.Method == http.MethodPost && d.actErr != 0:
		http.Error(w, `{"error":"Raum ist geschlossen"}`, d.actErr)
	case r.Method == http.MethodPost:
		_, _ = w.Write([]byte(`{"ok":true}`))
	case r.URL.Query().Get("room") == "FAMILIE":
		_, _ = w.Write([]byte(roomJSON))
	case r.URL.Query().Get("room") != "":
		http.Error(w, `{"error":"Raum nicht gefunden"}`, http.StatusNotFound)
	default:
		_, _ = w.Write([]byte(statusJSON))
	}
}

func (d *diag) posts() []string {
	d.mu.Lock()
	defer d.mu.Unlock()
	var out []string
	for _, r := range d.requests {
		if strings.HasPrefix(r, "POST ") {
			out = append(out, r)
		}
	}
	return out
}

// drive treibt das Modell: Nachricht rein, Befehl ausführen (Takte schlafen nur Millisekunden), Antwort wieder rein.
type drive struct {
	t *testing.T
	m model
}

func newDrive(t *testing.T, d *diag) *drive {
	t.Helper()
	m := newModel(clientFromEnv(env(d.URL, token)))
	m.refresh, m.retry = time.Millisecond, time.Millisecond
	return &drive{t, m}
}

// send schickt eine Nachricht und liefert den Befehl dazu.
func (d *drive) send(msg tea.Msg) tea.Cmd {
	next, cmd := d.m.Update(msg)
	d.m = next.(model)
	return cmd
}

// settle führt den Befehl aus, gibt das Ergebnis ins Modell und führt auch die Folgebefehle aus (Abruf nach Aktion, Nachholen des Logs),
// bis nichts mehr kommt oder ein Takt ansteht (er wird nicht weitergegeben). Liefert die letzte Nachricht.
func (d *drive) settle(cmd tea.Cmd) tea.Msg {
	var msg tea.Msg
	for cmd != nil {
		msg = cmd()
		if _, tick := msg.(tickMsg); tick {
			return msg
		}
		cmd = d.send(msg)
	}
	return msg
}

// key drückt eine Taste und führt den Befehl aus, der daraus entsteht (Abruf oder Aktion samt Folgeabruf).
func (d *drive) key(k string) tea.Msg {
	var msg tea.KeyPressMsg
	switch k {
	case "enter":
		msg = tea.KeyPressMsg{Code: tea.KeyEnter}
	case "esc":
		msg = tea.KeyPressMsg{Code: tea.KeyEscape}
	case "up":
		msg = tea.KeyPressMsg{Code: tea.KeyUp}
	case "down":
		msg = tea.KeyPressMsg{Code: tea.KeyDown}
	default:
		msg = tea.KeyPressMsg{Code: rune(k[0]), Text: k}
	}
	return d.settle(d.send(msg))
}

func (d *drive) text() string { return d.m.View().Content }

// loaded: Übersicht mit geladenem Status.
func loaded(t *testing.T, s *diag) *drive {
	t.Helper()
	d := newDrive(t, s)
	d.settle(d.m.Init())
	if d.m.st == nil {
		t.Fatal("Status nicht geladen")
	}
	return d
}

func opened(t *testing.T, s *diag) *drive {
	t.Helper()
	d := loaded(t, s)
	d.key("enter")
	return d
}

// AC-02 (a): Enter auf einer Raumzeile lädt ?room= und zeigt Gold, Truppen und Geräte.
func TestRaumAnsehen(t *testing.T) {
	s := newDiag(t)
	d := loaded(t, s)
	d.key("down")
	if d.m.sel != 1 {
		t.Fatalf("Auswahl: %d", d.m.sel)
	}
	d.key("up")
	d.key("enter")
	if d.m.screen != scrRoom || d.m.detail == nil {
		t.Fatalf("Raumansicht: %+v", d.m.screen)
	}
	out := d.text()
	for _, want := range []string{"Raum FAMILIE", "Tag 2 (day)", "Gold je Monarch: 12, 7", "archer 1, peasant 3", "Burg 950 HP", "aaaaaa", "verbunden", "bbbbbb", "getrennt"} {
		if !strings.Contains(out, want) {
			t.Errorf("%q fehlt in\n%s", want, out)
		}
	}
	if got := s.requests[len(s.requests)-1]; got != "GET /api/status?room=FAMILIE" {
		t.Errorf("Anfrage: %s", got)
	}
}

// AC-02 (b): d + y sendet genau einen POST mit Raum und Kennung des gewählten Geräts; d + n sendet nichts.
func TestGeraetTrennenMitBestaetigung(t *testing.T) {
	s := newDiag(t)
	d := opened(t, s)
	d.key("d")
	if !d.m.confirm || !strings.Contains(d.text(), "Gerät aaaaaa trennen? (y/n)") {
		t.Fatalf("keine Rückfrage:\n%s", d.text())
	}
	d.key("n")
	if d.m.confirm || len(s.posts()) != 0 {
		t.Fatalf("Abbruch sendete etwas: %v", s.posts())
	}
	d.key("d")
	d.key("x") // andere Tasten ändern während der Rückfrage nichts
	if !d.m.confirm || len(s.posts()) != 0 {
		t.Fatal("Rückfrage verloren")
	}
	d.key("y")
	if got := s.posts(); len(got) != 1 || got[0] != "POST /api/status/disconnect?room=FAMILIE&device=aaaaaa" {
		t.Fatalf("POSTs: %v", got)
	}
	if d.m.flash != "Gerät aaaaaa getrennt" || d.m.screen != scrRoom {
		t.Errorf("Ergebnis: %q", d.m.flash)
	}
	d.key("down") // zweites Gerät ist getrennt: keine Rückfrage
	d.key("d")
	if d.m.confirm || d.m.flash != "Kein verbundenes Gerät gewählt" {
		t.Errorf("getrenntes Gerät: confirm=%v flash=%q", d.m.confirm, d.m.flash)
	}
}

// AC-02 (c): s sichert, Serverfehler erscheinen als Meldung, die Anzeige bleibt.
func TestSpielstandSichern(t *testing.T) {
	s := newDiag(t)
	d := opened(t, s)
	d.key("s")
	if got := s.posts(); len(got) != 1 || got[0] != "POST /api/status/save?room=FAMILIE" || d.m.flash != "Spielstand von FAMILIE gesichert" {
		t.Fatalf("save: %v %q", got, d.m.flash)
	}
	s.actErr = http.StatusConflict
	d.key("s")
	if d.m.flash != "Raum ist geschlossen" || d.m.screen != scrRoom || d.m.detail == nil {
		t.Errorf("Serverfehler: %q", d.m.flash)
	}
}

func page(cursor int64, truncated bool, lines ...string) string {
	b, _ := json.Marshal(logPage{Cursor: cursor, Truncated: truncated, Lines: lines})
	return string(b)
}

const (
	lineInfo  = `{"time":"2026-10-01T16:01:11.453+02:00","level":"INFO","msg":"K3C läuft","ns":"main","port":"8080"}`
	lineError = `{"time":"2026-10-01T16:02:00Z","level":"ERROR","msg":"Spielstand nicht gespeichert","ns":"room","room":"FAMILIE"}`
)

// AC-02 (d): Log öffnen zeigt die Zeilen, Nachladen hängt nur Neues an (Cursor), truncated und „Log aus“ werden gemeldet.
func TestLogFolgen(t *testing.T) {
	s := newDiag(t)
	s.logPages = []string{page(120, false, lineInfo, lineError), page(120, false), page(150, false, `{"level":"WARN","msg":"neu"}`), page(10, true, lineInfo)}
	d := loaded(t, s)
	d.key("l")
	if d.m.screen != scrLog || len(d.m.log.lines) != 2 || d.m.log.cursor != 120 {
		t.Fatalf("Log: %+v", d.m.log)
	}
	if out := d.text(); !strings.Contains(out, "K3C läuft") || !strings.Contains(out, "Spielstand nicht gespeichert") || !strings.Contains(out, "folgt") {
		t.Errorf("Anzeige:\n%s", out)
	}
	for range 2 { // nichts Neues, dann eine neue Zeile
		cmd := d.settle(d.m.next())
		tick, ok := cmd.(tickMsg)
		if !ok {
			t.Fatalf("Takt: %T", cmd)
		}
		d.settle(d.send(tick))
	}
	if len(d.m.log.lines) != 3 || d.m.log.cursor != 150 {
		t.Fatalf("nach Nachladen: %d Zeilen, Cursor %d", len(d.m.log.lines), d.m.log.cursor)
	}
	if want := "GET /api/status/log?since=120&limit=500"; !containsRequest(s, want) {
		t.Errorf("Anfragen: %v", s.requests)
	}
	d.settle(d.send(tickMsg{d.m.chain})) // Datei kleiner geworden
	if d.m.log.note != "Log neu begonnen" || len(d.m.log.lines) != 1 {
		t.Errorf("truncated: %+v", d.m.log)
	}
}

func containsRequest(s *diag, want string) bool {
	s.mu.Lock()
	defer s.mu.Unlock()
	for _, r := range s.requests {
		if r == want {
			return true
		}
	}
	return false
}

func TestLogAusUndZurueck(t *testing.T) {
	s := newDiag(t) // ohne logPages: 404 „Log aus“
	d := opened(t, s)
	d.key("l")
	if d.m.screen != scrLog || d.m.err != "Log aus" || !strings.Contains(d.text(), "Log aus") {
		t.Fatalf("Log aus: %q\n%s", d.m.err, d.text())
	}
	d.key("esc")
	if d.m.screen != scrRoom {
		t.Errorf("Esc aus dem Log führt zurück in den Raum, nicht in %v", d.m.screen)
	}
	d.key("esc")
	if d.m.screen != scrOverview {
		t.Errorf("Esc aus dem Raum führt in die Übersicht, nicht in %v", d.m.screen)
	}
	if _, quit := d.settle(d.send(tea.KeyPressMsg{Code: 'q', Text: "q"})).(tea.QuitMsg); !quit {
		t.Error("q in der Übersicht beendet nicht")
	}
}

// AC-02 (d): Beim Öffnen wird das Log seitenweise nachgeholt (höchstens 20 Seiten), im Speicher bleiben die letzten 500 Zeilen.
func TestLogNachholenUndKuerzen(t *testing.T) {
	var full []string
	for i := range logPageSize {
		full = append(full, fmt.Sprintf(`{"level":"INFO","msg":"zeile %d"}`, i))
	}
	s := newDiag(t)
	s.logPages = []string{page(1000, false, full...)} // jede Antwort ist eine volle Seite
	d := loaded(t, s)
	d.key("l")
	if d.m.log.pages != logCatchUp || len(d.m.log.lines) != logKeep {
		t.Errorf("Nachholen: %d Seiten, %d Zeilen", d.m.log.pages, len(d.m.log.lines))
	}
	if s.logCalls != logCatchUp {
		t.Errorf("%d Abrufe statt %d", s.logCalls, logCatchUp)
	}
}

// AC-02 (d): blättern hält das Folgen an, f schaltet es wieder ein.
func TestLogBlaettern(t *testing.T) {
	s := newDiag(t)
	s.logPages = []string{page(50, false, lineInfo, lineError, lineInfo)}
	d := loaded(t, s)
	d.key("l")
	d.key("up")
	if d.m.log.follow || d.m.log.off != 1 || !strings.Contains(d.text(), "angehalten") {
		t.Errorf("blättern: %+v", d.m.log)
	}
	d.key("f")
	if !d.m.log.follow || d.m.log.off != 0 {
		t.Errorf("folgen: %+v", d.m.log)
	}
}

// AC-02 (e): ERROR/WARN sind gefärbt, eine kaputte Zeile bleibt unverändert sichtbar.
func TestLogZeilenFormat(t *testing.T) {
	out := formatLogLine(lineError)
	for _, want := range []string{"16:02:00", "ERROR", "room", "Spielstand nicht gespeichert", "room=FAMILIE", "\x1b["} {
		if !strings.Contains(out, want) {
			t.Errorf("%q fehlt in %q", want, out)
		}
	}
	if info := formatLogLine(lineInfo); !strings.Contains(info, "K3C läuft") || !strings.Contains(info, "port=8080") {
		t.Errorf("INFO: %q", info)
	}
	if bad := formatLogLine(`{kaputt`); !strings.Contains(bad, "{kaputt") {
		t.Errorf("kaputte Zeile: %q", bad)
	}
}

// Ein Raum, der verschwindet, bringt zurück in die Übersicht; Ergebnisse einer alten Ansicht werden ignoriert.
func TestRaumVerschwindetUndAlteErgebnisse(t *testing.T) {
	s := newDiag(t)
	d := opened(t, s)
	d.send(roomMsg{err: failf("Raum nicht gefunden"), chain: d.m.chain})
	if d.m.screen != scrOverview || !strings.Contains(d.m.flash, "nicht mehr da") {
		t.Errorf("Raum weg: screen=%v flash=%q", d.m.screen, d.m.flash)
	}
	old := d.m.chain - 1
	if cmd := d.send(roomMsg{d: roomDetail{Code: "ALT"}, chain: old}); cmd != nil || d.m.detail != nil && d.m.detail.Code == "ALT" {
		t.Error("altes Ergebnis wurde übernommen")
	}
	if cmd := d.send(logMsg{page: logPage{Lines: []string{"x"}}, chain: old}); cmd != nil || len(d.m.log.lines) != 0 {
		t.Error("altes Log-Ergebnis wurde übernommen")
	}
}

// AC-02 (g): Das Token steht in keiner Ansicht, auch nicht in Fehlermeldungen.
func TestTokenNieSichtbar(t *testing.T) {
	s := newDiag(t)
	s.logPages = []string{page(5, false, lineInfo)}
	d := opened(t, s)
	screens := []string{d.text()}
	d.key("l")
	screens = append(screens, d.text())
	d.key("esc")
	d.m.c.token = "falsch-456"
	d.key("s")
	screens = append(screens, d.text(), d.m.flash)
	for _, out := range screens {
		if strings.Contains(out, token) || strings.Contains(out, "falsch-456") {
			t.Errorf("Token sichtbar in\n%s", out)
		}
	}
	if !strings.Contains(d.m.flash, "401, Token prüfen") {
		t.Errorf("Meldung bei falschem Token: %q", d.m.flash)
	}
}
