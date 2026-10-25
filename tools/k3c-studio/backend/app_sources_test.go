package backend

import (
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"

	assets "k3c/tools/assetkit"
)

var testOgg = []byte("OggS" + strings.Repeat("o", 40))

// sourcesApp: Studio mit Slot sfx:coin und zwei Einträgen, deren Dateien ein lokaler TLS-Testserver liefert; die Probe
// geht an probe (kein Internet im Test).
func sourcesApp(t *testing.T, probe string) (*App, *httptest.Server) {
	t.Helper()
	root := studioRepo(t)
	write(t, filepath.Join(root, assets.RegisterPath), `{"_comment": "", "slots": [{"id": "sfx:coin", "art": "sound", "kategorie": "sfx", "beschreibung": "Münze", "bus": "sfx", "wahl": []}]}`)
	write(t, filepath.Join(root, "public", "audio", "README.md"), "# Audio\n")
	srv := httptest.NewTLSServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/coin.ogg" {
			_, _ = w.Write(testOgg)
			return
		}
		http.NotFound(w, r)
	}))
	t.Cleanup(srv.Close)
	old := assets.VorratHosts
	assets.VorratHosts = append(slices.Clone(old), "127.0.0.1")
	t.Cleanup(func() { assets.VorratHosts = old })
	p := func(id, file, license string) assets.Suggestion {
		return assets.Suggestion{ID: id, Slot: "sfx:coin", Kind: assets.KindSound, Title: "Münze", Source: "Kenney",
			Page: "https://kenney.nl/" + id, FileURL: srv.URL + "/" + file, License: license, FileName: file,
			Size: int64(len(testOgg)), Priority: 1, Status: "offen"}
	}
	b, _ := json.Marshal(assets.Suggestions{List: []assets.Suggestion{p("q1", "coin.ogg", "CC0"), p("q2", "weg.ogg", "CC0"), p("q3", "coin.ogg", "CC-BY-NC")}})
	write(t, filepath.Join(root, assets.SuggestionsPath), string(b))
	a := &App{root: root, bus: newBus()}
	st := a.sourceState()
	st.client = srv.Client()
	st.probes = []string{probe}
	if probe == "" {
		st.probes = []string{srv.URL}
	}
	return a, srv
}

// waitIdle wartet, bis der Download im Hintergrund fertig ist.
func waitIdle(t *testing.T, a *App) []SourceProgress {
	t.Helper()
	for range 200 {
		st := a.sourceState()
		st.mu.Lock()
		busy := st.busy
		st.mu.Unlock()
		if !busy {
			return a.SourceProgress()
		}
		time.Sleep(10 * time.Millisecond)
	}
	t.Fatal("Download endet nicht")
	return nil
}

func proposalByID(t *testing.T, v SourcesView, id string) SourceProposal {
	t.Helper()
	for _, p := range v.Proposals {
		if p.ID == id {
			return p
		}
	}
	t.Fatalf("%s fehlt", id)
	return SourceProposal{}
}

// B-601/AC-02: Download in den Vorrat mit Fortschritt, Fehler bleibt am Eintrag; Vorschau unter /vorrat/ nur lesend;
// Übernahme ordnet zu; Aufräumen lässt Verwendetes stehen.
func TestSourcesDownloadUndAdopt(t *testing.T) {
	a, _ := sourcesApp(t, "")
	if err := a.SourceDownload([]string{"q3"}, int64(len(testOgg))); err == nil {
		t.Error("gesperrte Lizenz soll abgelehnt werden")
	}
	if err := a.SourceDownload([]string{"q1", "q2"}, 2*int64(len(testOgg))); err != nil {
		t.Fatal(err)
	}
	prog := waitIdle(t, a)
	if len(prog) != 2 || prog[0].Status != "fertig" || prog[0].Loaded != int64(len(testOgg)) || prog[1].Status != "fehler" {
		t.Fatalf("Fortschritt %+v", prog)
	}
	checkVorratServed(t, a, strings.TrimPrefix(checkSourcesView(t, a), Prefix))
	if err := a.SourceAdopt("q1"); err != nil {
		t.Fatal(err)
	}
	if v, _ := a.Sources(); !proposalByID(t, v, "q1").Used || v.Sources[1].Used != 1 {
		t.Errorf("nach Übernahme %+v", proposalByID(t, v, "q1"))
	}
	if c, err := a.SourceCleanupPreview(); err != nil || len(c.Files) != 0 {
		t.Errorf("Verwendetes bleibt: %+v %v", c, err)
	}
}

// checkSourcesView: q1 geladen mit Vorschau und Prüfsumme, q2 offen mit Fehler, q3 gesperrt; liefert die Vorschau von q1.
func checkSourcesView(t *testing.T, a *App) string {
	t.Helper()
	v, err := a.Sources()
	if err != nil {
		t.Fatal(err)
	}
	q1, q2, q3 := proposalByID(t, v, "q1"), proposalByID(t, v, "q2"), proposalByID(t, v, "q3")
	if v.Offline || q1.Status != "geladen" || q1.Preview != vorratURL+"audio/kenney/q1-coin.ogg" || len(q1.Checksum) != 64 {
		t.Errorf("q1 %+v offline=%v", q1, v.Offline)
	}
	if q2.Status != "offen" || !strings.Contains(q2.Error, "404") || q3.Status != "gesperrt" || q3.LicenseOK {
		t.Errorf("q2 %+v, q3 %+v", q2, q3)
	}
	if kenney := v.Sources[1]; kenney.Name != "Kenney" || kenney.Proposed != 3 || kenney.Loaded != 1 || kenney.Rules == "" {
		t.Errorf("Quelle %+v", kenney)
	}
	return q1.Preview
}

// checkVorratServed: Die geladene Datei kommt unter /api/studio/vorrat/ an Loopback, POST und .. nicht.
func checkVorratServed(t *testing.T, a *App, rel string) {
	t.Helper()
	h := a.Handler(false)
	get := func(method, p string) *httptest.ResponseRecorder {
		req := httptest.NewRequest(method, p, nil)
		req.RemoteAddr, req.Host = "127.0.0.1:5000", "127.0.0.1:8091"
		req.Header.Set(header, "1")
		rec := httptest.NewRecorder()
		h.ServeHTTP(rec, req)
		return rec
	}
	rec := get(http.MethodGet, Prefix+rel)
	if body, _ := io.ReadAll(rec.Body); rec.Code != http.StatusOK || string(body) != string(testOgg) {
		t.Errorf("GET %s: %d", rel, rec.Code)
	}
	if rec := get(http.MethodGet, Prefix+"vorrat/../../data/assets.json"); rec.Code == http.StatusOK {
		t.Error("Pfad außerhalb des Vorrats ausgeliefert")
	}
	if rec := get(http.MethodPost, Prefix+rel); rec.Code == http.StatusOK {
		t.Error("POST auf den Vorrat erlaubt")
	}
}

// B-601/AC-03: Scheitert die Probe, ist die Seite offline und der Download wird abgelehnt; Aufräumen entfernt
// Geladenes, die Einträge sind wieder offen.
func TestSourcesOfflineUndAufraeumen(t *testing.T) {
	a, _ := sourcesApp(t, "")
	if err := a.SourceDownload([]string{"q1"}, int64(len(testOgg))); err != nil {
		t.Fatal(err)
	}
	waitIdle(t, a)
	c, err := a.SourceCleanupPreview()
	if err != nil || len(c.Files) != 1 || c.Files[0].Name != "q1-coin.ogg" || c.Size != int64(len(testOgg)) {
		t.Fatalf("Vorschau %+v %v", c, err)
	}
	if c, err := a.SourceCleanup(); err != nil || len(c.Files) != 1 {
		t.Fatalf("Aufräumen %+v %v", c, err)
	}
	if v, _ := a.Sources(); proposalByID(t, v, "q1").Status != "offen" {
		t.Error("q1 soll wieder offen sein")
	}
	off, _ := sourcesApp(t, "https://127.0.0.1:1/")
	if v, err := off.Sources(); err != nil || !v.Offline {
		t.Errorf("offline erwartet: %v", err)
	}
	if err := off.SourceDownload([]string{"q1"}, int64(len(testOgg))); err == nil || !strings.Contains(err.Error(), "offline") {
		t.Errorf("Download offline: %v", err)
	}
}

// Ohne passende bestätigte Gesamtgröße startet kein Download (Bestätigung auch im Backend, Review WZ29.6).
func TestSourceDownloadBrauchtBestaetigung(t *testing.T) {
	a, _ := sourcesApp(t, "")
	if err := a.SourceDownload([]string{"q1"}, 0); err == nil || !strings.Contains(err.Error(), "bestätigt") {
		t.Fatalf("Download ohne Bestätigung: %v", err)
	}
}
