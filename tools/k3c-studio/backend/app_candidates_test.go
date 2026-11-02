package backend

import (
	"errors"
	"net/http/httptest"
	"net/url"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	assets "k3c/tools/assetkit"
)

const candidateCredits = "# Audio\n\n| Ordner | Pack | Urheber | Lizenz | Quelle |\n|---|---|---|---|---|\n" +
	"| `gesperrt.ogg` | Gesperrt | A | CC BY-ND 4.0 | https://example.org/nd |\n" +
	"| `nc.ogg` | NC | A | CC BY-NC 4.0 | https://example.org/nc |\n" +
	"| `sampling.ogg` | Sampling | A | Sampling+ 1.0 | https://example.org/sampling |\n"

// candidatesApp: sourcesApp (Slot sfx:coin, Vorschläge q1 bis q3, TLS-Testserver) mit der Zuordnung audio/alt.ogg und den
// Katalog-Dateien extra (frei), gesperrt (ND) und nc; Treffer dürfen von 127.0.0.1 laden.
func candidatesApp(t *testing.T, probe string) *App {
	t.Helper()
	a, srv := sourcesApp(t, probe)
	assets.DownloadHosts["127.0.0.1"] = true
	t.Cleanup(func() { delete(assets.DownloadHosts, "127.0.0.1") })
	allowTestPort(t, srv)
	for _, name := range []string{"alt", "extra", "gesperrt", "nc", "sampling"} {
		write(t, filepath.Join(a.root, "public", "audio", name+".ogg"), string(testOgg))
	}
	write(t, filepath.Join(a.root, "public", "audio", "README.md"), candidateCredits)
	write(t, filepath.Join(a.root, assets.RegisterPath), `{"_comment": "", "slots": [{"id": "sfx:coin", "art": "sound", `+
		`"kategorie": "sfx", "beschreibung": "Münze", "bus": "sfx", "wahl": [{"datei": "audio/alt.ogg"}]}]}`)
	return a
}

func candidateIDs(v AssetCandidatesView) string {
	ids := make([]string, len(v.Candidates))
	for i, c := range v.Candidates {
		ids[i] = c.ID
	}
	return strings.Join(ids, ",")
}

func candidateOf(t *testing.T, v AssetCandidatesView, id string) assets.Candidate {
	t.Helper()
	for _, c := range v.Candidates {
		if c.ID == id {
			return c
		}
	}
	t.Fatalf("kandidat %s fehlt in %s", id, candidateIDs(v))
	return assets.Candidate{}
}

func addTestHit(t *testing.T, a *App, name, license, download string) string {
	t.Helper()
	link := "https://kenney.nl/" + name
	if name == "h4" {
		link = "https://pixabay.com/h4"
	}
	h := assets.Hit{Title: name, Link: link, Source: "Kenney", License: license, Download: download}
	if _, err := assets.AddHits(a.root, "sfx:coin", []assets.Hit{h}, time.Now()); err != nil {
		t.Fatal(err)
	}
	hits, _ := assets.HitsOf(a.root, "sfx:coin")
	for _, x := range hits {
		if x.Link == link {
			return "hit:" + x.ID
		}
	}
	t.Fatalf("treffer %s fehlt", name)
	return ""
}

func choiceFiles(t *testing.T, a *App) []string {
	t.Helper()
	reg, err := assets.Load(a.root)
	if err != nil {
		t.Fatal(err)
	}
	var out []string
	for _, c := range reg.Find("sfx:coin").Choices {
		out = append(out, c.File)
	}
	return out
}

func noVorrat(t *testing.T, a *App) {
	t.Helper()
	if v, err := assets.LoadVorrat(a.root); err != nil || len(v.Entries) != 0 {
		t.Errorf("vorrat nicht leer: %+v %v", v, err)
	}
}

// B-677/AC-01: alle Stufen in einer Liste, Dubletten (gleiche Datei-URL) mit allen Herkünften, Reihenfolge Zuordnung,
// Vorrat, Netz, Katalog; Lizenz-Marken und Gründe je Zeile.
func TestAssetCandidatesListe(t *testing.T) {
	a := candidatesApp(t, "")
	srvURL := strings.TrimSuffix(proposalURL(t, a, "q1"), "/coin.ogg")
	if err := a.SourceDownload([]string{"q1"}, int64(len(testOgg))); err != nil {
		t.Fatal(err)
	}
	waitIdle(t, a)
	addTestHit(t, a, "h1", "CC0", srvURL+"/weg.ogg") // Dublette zu q2
	h2 := addTestHit(t, a, "h2", "CC0", "")
	h3 := addTestHit(t, a, "h3", "Custom", srvURL+"/h3.ogg")
	addTestHit(t, a, "h4", "CC0", "")                 // Pixabay: ausgeschlossen
	addTestHit(t, a, "h5", "CC0", srvURL+"/coin.ogg") // Dublette zu q1
	v, err := a.AssetCandidates("sfx:coin", "ogg", 2)
	if err != nil || v.Offline || v.Stand == "" {
		t.Fatalf("liste: %+v %v", v, err)
	}
	if got, want := candidateIDs(v), "wahl:audio/alt.ogg,sug:q1,sug:q2,"+h2+","+h3+",cat:audio/extra.ogg,cat:audio/gesperrt.ogg"; got != want {
		t.Fatalf("reihenfolge\n got %s\nwant %s", got, want)
	}
	checkCandidateStages(t, v)
	checkCandidateHits(t, v, h2, h3)
	checkCandidateCatalog(t, v)
}

// checkCandidateStages: Zuordnung, Vorrat (q1 mit Treffer h5) und Netz (q2 mit Treffer h1) mit allen Herkünften.
func checkCandidateStages(t *testing.T, v AssetCandidatesView) {
	t.Helper()
	alt, q1, q2 := candidateOf(t, v, "wahl:audio/alt.ogg"), candidateOf(t, v, "sug:q1"), candidateOf(t, v, "sug:q2")
	if alt.Pref != 1 || alt.Stage != assets.StageGame || alt.Assignable || alt.Mark != assets.MarkFree || alt.Preview != "/audio/alt.ogg" {
		t.Errorf("zuordnung %+v", alt)
	}
	if q1.Stage != assets.StageStock || strings.Join(q1.Origins, ",") != "vorrat,recherche" || !q1.Assignable || !strings.HasPrefix(q1.Preview, vorratURL) {
		t.Errorf("q1 %+v", q1)
	}
	if q2.Stage != assets.StageNet || strings.Join(q2.Origins, ",") != "vorschlag,recherche" || !q2.Assignable || q2.Preview != "" || q2.Size != int64(len(testOgg)) {
		t.Errorf("q2 %+v", q2)
	}
}

// proposalURL ist die Datei-URL des Vorschlags id aus der Liste.
func proposalURL(t *testing.T, a *App, id string) string {
	t.Helper()
	list, _ := assets.LoadSuggestions(a.root)
	for _, s := range list.List {
		if s.ID == id {
			return s.FileURL
		}
	}
	t.Fatalf("vorschlag %s fehlt", id)
	return ""
}

func checkCandidateHits(t *testing.T, v AssetCandidatesView, h2, h3 string) {
	t.Helper()
	c2, c3 := candidateOf(t, v, h2), candidateOf(t, v, h3)
	if c2.Assignable || !strings.Contains(c2.Reason, "kein direkter Download") || c2.Mark != assets.MarkFree {
		t.Errorf("treffer ohne download %+v", c2)
	}
	if c3.Assignable || c3.Mark != assets.MarkCheck || c3.Reason != "Lizenz prüfen" || !c3.ConfirmLicense {
		t.Errorf("treffer lizenz prüfen %+v", c3)
	}
}

func checkCandidateCatalog(t *testing.T, v AssetCandidatesView) {
	t.Helper()
	extra, nd := candidateOf(t, v, "cat:audio/extra.ogg"), candidateOf(t, v, "cat:audio/gesperrt.ogg")
	if extra.Pref != 0 || !extra.Assignable || extra.Mark != assets.MarkFree || extra.Preview != "/audio/extra.ogg" {
		t.Errorf("katalog %+v", extra)
	}
	if nd.Assignable || nd.Mark != assets.MarkBlocked || !strings.Contains(nd.Reason, "ND") || nd.Source != "https://example.org/nd" {
		t.Errorf("katalog ND %+v", nd)
	}
}

// B-677/AC-02: Ein Netz-Vorschlag wird in einem Aufruf geladen, übernommen und als letzte Präferenz zugeordnet.
func TestAssetCandidateAssignNetz(t *testing.T) {
	a := candidatesApp(t, "")
	v, _ := a.AssetCandidates("sfx:coin", "zzz", 5)
	saved, err := a.AssetCandidateAssign("sfx:coin", "sug:q1", v.Stand, int64(len(testOgg)))
	if err != nil {
		t.Fatal(err)
	}
	files := choiceFiles(t, a)
	if len(files) != 2 || !strings.HasPrefix(files[1], "audio/kandidaten/muenze") || len(saved.Slot.Choices) != 2 {
		t.Fatalf("zuordnungen %v / %+v", files, saved.Slot.Choices)
	}
	if now, _ := assets.Stamp(a.root); saved.Stand != now || saved.Stand == v.Stand {
		t.Errorf("stand %q (jetzt %q, vorher %q)", saved.Stand, now, v.Stand)
	}
	v2, _ := a.AssetCandidates("sfx:coin", "zzz", 5)
	if got := candidateIDs(v2); got != "wahl:audio/alt.ogg,wahl:"+files[1]+",sug:q2" {
		t.Errorf("nach dem Zuordnen %s", got)
	}
	if st := a.sourceState(); st.busy {
		t.Error("download-platz nicht freigegeben")
	}
}

// B-677/AC-03: Falsch bestätigte Größe und veralteter Stand laden nichts und schreiben nichts.
func TestAssetCandidateAssignPruefungen(t *testing.T) {
	a := candidatesApp(t, "")
	hit := addTestHit(t, a, "h6", "CC0", strings.TrimSuffix(proposalURL(t, a, "q1"), "/coin.ogg")+"/coin.ogg")
	v, _ := a.AssetCandidates("sfx:coin", "zzz", 5)
	for _, n := range []int64{0, 1, int64(len(testOgg)) + 1} {
		if _, err := a.AssetCandidateAssign("sfx:coin", "sug:q1", v.Stand, n); err == nil || !strings.Contains(err.Error(), "neu bestätigen") {
			t.Errorf("größe %d: %v", n, err)
		}
	}
	for _, id := range []string{"sug:q1", hit, "cat:audio/extra.ogg"} {
		if _, err := a.AssetCandidateAssign("sfx:coin", id, "alt", int64(len(testOgg))); !errors.Is(err, assets.ErrStale) {
			t.Errorf("%s ohne aktuellen stand: %v", id, err)
		}
	}
	if _, err := a.AssetCandidateAssign("sfx:coin", "sug:q1", "", int64(len(testOgg))); !errors.Is(err, assets.ErrStale) {
		t.Errorf("leerer stand: %v", err)
	}
	noVorrat(t, a)
	if _, err := os.Stat(filepath.Join(a.root, ".work")); !os.IsNotExist(err) {
		t.Errorf("etwas geladen: %v", err)
	}
	if now, _ := assets.Stamp(a.root); now != v.Stand || len(choiceFiles(t, a)) != 1 {
		t.Error("register geändert")
	}
}

// B-677/AC-04: Gesperrte Lizenz, offline, schon zugeordnet und fremde Eingaben werden abgelehnt.
func TestAssetCandidateAssignAbgelehnt(t *testing.T) {
	a := candidatesApp(t, "https://127.0.0.1:1/")
	v, err := a.AssetCandidates("sfx:coin", "ogg", 10)
	if err != nil || !v.Offline {
		t.Fatalf("offline erwartet: %+v %v", v, err)
	}
	if q1 := candidateOf(t, v, "sug:q1"); q1.Assignable || q1.Reason != "offline" {
		t.Errorf("q1 offline %+v", q1)
	}
	size := int64(len(testOgg))
	for id, want := range map[string]string{
		"sug:q1":                 "offline",
		"sug:q3":                 "gesperrt",
		"cat:audio/gesperrt.ogg": "nicht erlaubt",
		"cat:audio/nc.ogg":       "nicht erlaubt",
		"cat:audio/fehlt.ogg":    "nicht im Katalog",
		"wahl:audio/alt.ogg":     "schon zugeordnet",
		"cat:audio/alt.ogg":      "schon zugeordnet",
		"sug:erfunden":           "unbekannt",
		"xyz:1":                  "unbekannt",
	} {
		if _, err := a.AssetCandidateAssign("sfx:coin", id, v.Stand, size); err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%s: %v, erwartet %q", id, err, want)
		}
	}
	if _, err := a.AssetCandidateAssign("nix", "sug:q1", v.Stand, size); err == nil {
		t.Error("unbekannter slot")
	}
	if _, err := a.AssetCandidates("nix", "", 1); err == nil {
		t.Error("liste zu unbekanntem slot")
	}
	noVorrat(t, a)
	if len(choiceFiles(t, a)) != 1 {
		t.Error("zuordnung geschrieben")
	}
}

// B-677/AC-02: Treffer mit Lizenz „prüfen“ oder ohne direkten Download sind nicht zuordenbar; ein Treffer mit
// Download wird geladen und zugeordnet.
func TestAssetCandidateAssignTreffer(t *testing.T) {
	a := candidatesApp(t, "")
	base := strings.TrimSuffix(proposalURL(t, a, "q1"), "/coin.ogg")
	check, none := addTestHit(t, a, "h3", "Custom", base+"/coin.ogg"), addTestHit(t, a, "h2", "CC0", "")
	good := addTestHit(t, a, "h7", "CC0", base+"/coin.ogg")
	v, _ := a.AssetCandidates("sfx:coin", "zzz", 5)
	for id, want := range map[string]string{check: "Lizenz prüfen", none: "kein direkter Download"} {
		if _, err := a.AssetCandidateAssign("sfx:coin", id, v.Stand, 0); err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%s: %v, erwartet %q", id, err, want)
		}
	}
	if len(choiceFiles(t, a)) != 1 {
		t.Fatal("abgelehnter treffer zugeordnet")
	}
	saved, err := a.AssetCandidateAssign("sfx:coin", good, v.Stand, 0)
	if err != nil || len(saved.Slot.Choices) != 2 || !strings.HasPrefix(saved.Slot.Choices[1].File, "audio/kandidaten/h7") {
		t.Fatalf("treffer: %+v %v", saved, err)
	}
	if now, _ := assets.Stamp(a.root); saved.Stand != now {
		t.Error("stand nicht aktuell")
	}
	if hits, _ := assets.HitsOf(a.root, "sfx:coin"); hits[2].File != saved.Slot.Choices[1].File {
		t.Errorf("treffer nicht als importiert vermerkt: %+v", hits[2])
	}
}

// B-677/AC-02: Ein Katalog-Treffer wird als letzte Präferenz angehängt.
func TestAssetCandidateAssignKatalog(t *testing.T) {
	a := candidatesApp(t, "")
	v, _ := a.AssetCandidates("sfx:coin", "extra", 5)
	saved, err := a.AssetCandidateAssign("sfx:coin", "cat:audio/extra.ogg", v.Stand, 0)
	if err != nil || len(saved.Slot.Choices) != 2 || saved.Slot.Choices[0].File != "audio/alt.ogg" || saved.Slot.Choices[1].File != "audio/extra.ogg" {
		t.Fatalf("katalog: %+v %v", saved.Slot.Choices, err)
	}
	if _, err := a.AssetCandidateAssign("sfx:coin", "cat:audio/nc.ogg", saved.Stand, 0); err == nil {
		t.Error("NC soll gesperrt bleiben")
	}
}

// B-677/AC-03: Scheitert der Download, bleibt die Zuordnung unverändert; läuft ein Batch-Download, wird nicht parallel geladen.
func TestAssetCandidateAssignDownloadFehler(t *testing.T) {
	a := candidatesApp(t, "")
	base := strings.TrimSuffix(proposalURL(t, a, "q1"), "/coin.ogg")
	bad := addTestHit(t, a, "h8", "CC0", base+"/nix.ogg")
	v, _ := a.AssetCandidates("sfx:coin", "zzz", 5)
	size := int64(len(testOgg))
	for _, id := range []string{"sug:q2", bad} {
		if _, err := a.AssetCandidateAssign("sfx:coin", id, v.Stand, size); err == nil || !strings.Contains(err.Error(), "404") {
			t.Errorf("%s: %v", id, err)
		}
	}
	noVorrat(t, a)
	if now, _ := assets.Stamp(a.root); now != v.Stand || len(choiceFiles(t, a)) != 1 {
		t.Error("zuordnung trotz fehler")
	}
	st := a.sourceState()
	st.mu.Lock()
	st.busy = true
	st.mu.Unlock()
	if _, err := a.AssetCandidateAssign("sfx:coin", "sug:q1", v.Stand, size); err == nil || !strings.Contains(err.Error(), "läuft schon ein Download") {
		t.Errorf("während eines downloads: %v", err)
	}
}

// Die Methoden sind über den RPC-Pfad erreichbar (Präfix AssetCandidate).
func TestAssetCandidatesRPC(t *testing.T) {
	a := candidatesApp(t, "")
	rec, ans := call(t, a.Handler(false), "POST", Prefix+"rpc/AssetCandidates", `["sfx:coin","extra",5]`, "127.0.0.1:5000", true)
	if rec.Code != 200 || !strings.Contains(string(ans.Result), `"id":"cat:audio/extra.ogg"`) || !strings.Contains(string(ans.Result), `"stufe":"spiel"`) {
		t.Fatalf("rpc: %d %s %s", rec.Code, rec.Body, ans.Error)
	}
	rec, _ = call(t, a.Handler(false), "POST", Prefix+"rpc/AssetCandidateAssign", `["sfx:coin","wahl:audio/alt.ogg","x",0]`, "127.0.0.1:5000", true)
	if rec.Code == 404 {
		t.Error("AssetCandidateAssign nicht erreichbar")
	}
}

// Review B-677: Sampling+ bleibt gesperrt, auch als Treffer nach „Lizenz bestätigen“ und als Katalog-Eintrag.
func TestAssetCandidateSamplingGesperrt(t *testing.T) {
	a := candidatesApp(t, "")
	base := strings.TrimSuffix(proposalURL(t, a, "q1"), "/coin.ogg")
	hit := addTestHit(t, a, "h9", "Sampling+ 1.0", base+"/h9.ogg")
	if err := a.AssetsLicenseOK("sfx:coin", strings.TrimPrefix(hit, "hit:")); err != nil {
		t.Fatal(err)
	}
	v, _ := a.AssetCandidates("sfx:coin", "sampling", 5)
	h, cat := candidateOf(t, v, hit), candidateOf(t, v, "cat:audio/sampling.ogg")
	if h.Assignable || h.ConfirmLicense || h.Mark != assets.MarkBlocked || !strings.Contains(h.Reason, "nicht erlaubt") {
		t.Errorf("treffer %+v", h)
	}
	if cat.Assignable || cat.Mark != assets.MarkBlocked || !strings.Contains(cat.Reason, "nicht erlaubt") {
		t.Errorf("katalog %+v", cat)
	}
	for _, id := range []string{hit, "cat:audio/sampling.ogg"} {
		if _, err := a.AssetCandidateAssign("sfx:coin", id, v.Stand, 0); err == nil || !strings.Contains(err.Error(), "nicht erlaubt") {
			t.Errorf("%s: %v", id, err)
		}
	}
	noVorrat(t, a)
	if len(choiceFiles(t, a)) != 1 {
		t.Error("gesperrte Lizenz zugeordnet")
	}
}

// Review B-677: Den Download-Platz braucht nur ein Download; ein schon geladener Vorrat-Eintrag wird auch während eines
// Downloads der Seite Quellen übernommen.
func TestAssetCandidateAssignVorratOhneDownloadPlatz(t *testing.T) {
	a := candidatesApp(t, "")
	size := int64(len(testOgg))
	if err := a.SourceDownload([]string{"q1"}, size); err != nil {
		t.Fatal(err)
	}
	waitIdle(t, a)
	v, _ := a.AssetCandidates("sfx:coin", "zzz", 5)
	st := a.sourceState()
	st.mu.Lock()
	st.busy = true
	st.mu.Unlock()
	if saved, err := a.AssetCandidateAssign("sfx:coin", "sug:q1", v.Stand, 0); err != nil || len(saved.Slot.Choices) != 2 {
		t.Fatalf("vorrat bei laufendem download: %+v %v", saved, err)
	}
	if !st.busy {
		t.Error("fremder download-platz freigegeben")
	}
}

// allowTestPort erlaubt den Port des TLS-Testservers bis zum Testende (B-694).
func allowTestPort(t *testing.T, srv *httptest.Server) {
	t.Helper()
	u, _ := url.Parse(srv.URL)
	assets.DownloadPorts[u.Port()] = true
	t.Cleanup(func() { delete(assets.DownloadPorts, u.Port()) })
}
