package assets

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"net/url"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"sync/atomic"
	"testing"
)

var (
	oggData = []byte("OggS" + strings.Repeat("x", 60))
	zipData = []byte("PK\x03\x04" + strings.Repeat("z", 20))
)

// vorratRepo: Mini-Repo mit Slot sfx:coin, Credit-Datei und einer Vorschlagsliste, deren Dateien ein lokaler
// TLS-Testserver liefert (kein Internet im Test); hits zählt die Anfragen.
func vorratRepo(t *testing.T) (string, *http.Client, *atomic.Int32) {
	t.Helper()
	root := repo(t)
	writeFile(t, filepath.Join(root, "public", "audio", "README.md"), "# Audio\n")
	hits := &atomic.Int32{}
	srv := httptest.NewTLSServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		hits.Add(1)
		switch r.URL.Path {
		case "/coin.ogg":
			_, _ = w.Write(oggData)
		case "/pack.zip":
			_, _ = w.Write(zipData)
		case "/fake.ogg":
			_, _ = w.Write([]byte("<html>" + strings.Repeat("y", 58)))
		default:
			http.NotFound(w, r)
		}
	}))
	t.Cleanup(srv.Close)
	allowTestHost(t)
	p := func(id, file string, size int, license string) Suggestion {
		return Suggestion{ID: id, Slot: "sfx:coin", Kind: KindSound, Title: "Münze " + id, Source: "OpenGameArt",
			Page: "https://opengameart.org/content/" + id, FileURL: srv.URL + "/" + file, Artist: "Anna", License: license,
			FileName: file, Size: int64(size), Priority: 1, Status: "offen"}
	}
	list := Suggestions{List: []Suggestion{
		p("v1", "coin.ogg", len(oggData), "CC0"),
		p("v2", "pack.zip", len(zipData), "CC-BY 4.0"),
		p("v3", "fake.ogg", len(oggData), "CC0"),
		p("v4", "coin.ogg", len(oggData), "CC-BY-NC 4.0"),
		p("v5", "missing.ogg", 10, "CC0"),
		p("v6", "coin.ogg", 99, "CC0"),
	}}
	b, _ := json.Marshal(list)
	writeFile(t, filepath.Join(root, SuggestionsPath), string(b))
	return root, srv.Client(), hits
}

func suggestion(t *testing.T, root, id string) Suggestion {
	t.Helper()
	list, err := LoadSuggestions(root)
	if err != nil {
		t.Fatal(err)
	}
	for _, s := range list.List {
		if s.ID == id {
			return s
		}
	}
	t.Fatalf("%s fehlt", id)
	return Suggestion{}
}

// B-601/AC-02: Laden in den Vorrat mit Index (Quelle, Lizenz, Größe, Prüfsumme); erneutes Laden ist ein No-op.
func TestVorratDownload(t *testing.T) {
	root, client, hits := vorratRepo(t)
	var last int64
	e, err := VorratDownload(context.Background(), root, suggestion(t, root, "v1"), client, day, func(n int64) { last = n })
	if err != nil {
		t.Fatal(err)
	}
	if e.File != "audio/opengameart/v1-coin.ogg" || e.Size != int64(len(oggData)) || len(e.SHA256) != 64 || e.License != "CC0" {
		t.Errorf("Eintrag %+v", e)
	}
	if last != int64(len(oggData)) {
		t.Errorf("Fortschritt %d", last)
	}
	idx, _ := LoadVorrat(root)
	if len(idx.Entries) != 1 || idx.Entries[0].Source != "OpenGameArt" || idx.Entries[0].Loaded == "" {
		t.Errorf("Index %+v", idx.Entries)
	}
	if _, err := VorratDownload(context.Background(), root, suggestion(t, root, "v1"), client, day, nil); err != nil || hits.Load() != 1 {
		t.Errorf("zweites Laden: %v, %d Anfragen", err, hits.Load())
	}
	states, _ := VorratStates(root, []Suggestion{suggestion(t, root, "v1"), suggestion(t, root, "v3"), suggestion(t, root, "v4")})
	if states["v1"].Status != VorratLoaded || states["v3"].Status != VorratOpen || states["v4"].Status != VorratBlocked {
		t.Errorf("Status %+v", states)
	}
}

// Falsche Signatur, falsche Größe, HTTP-Fehler und verbotene Lizenz lassen nichts im Vorrat zurück.
func TestVorratDownloadFehler(t *testing.T) {
	root, client, hits := vorratRepo(t)
	for _, id := range []string{"v3", "v4", "v5", "v6"} {
		if _, err := VorratDownload(context.Background(), root, suggestion(t, root, id), client, day, nil); err == nil {
			t.Errorf("%s: Fehler erwartet", id)
		}
	}
	if hits.Load() != 3 {
		t.Errorf("gesperrte Lizenz darf nicht laden: %d Anfragen", hits.Load())
	}
	var files []string
	_ = filepath.WalkDir(filepath.Join(root, ".work"), func(p string, d os.DirEntry, err error) error {
		if err == nil && !d.IsDir() {
			files = append(files, p)
		}
		return nil
	})
	if len(files) != 0 {
		t.Errorf("Reste im Vorrat: %v", files)
	}
	bad := suggestion(t, root, "v1")
	bad.FileURL = strings.Replace(bad.FileURL, "https:", "http:", 1)
	if _, _, err := CheckVorratDownload(bad); err == nil {
		t.Error("http soll abgelehnt werden")
	}
	bad.FileURL, bad.FileName = "https://x/a.gif", "a.gif"
	if _, _, err := CheckVorratDownload(bad); err == nil {
		t.Error("gif soll abgelehnt werden")
	}
}

// B-601/AC-02: Übernahme kopiert nach public/, legt Credit (und damit den Katalog-Eintrag) und die Zuordnung an;
// Pakete verlangen die Auswahl von Hand; Aufräumen entfernt nur Unverwendetes.
func TestVorratAdoptUndAufraeumen(t *testing.T) {
	root, client, _ := vorratRepo(t)
	for _, id := range []string{"v1", "v2"} {
		if _, err := VorratDownload(context.Background(), root, suggestion(t, root, id), client, day, nil); err != nil {
			t.Fatal(err)
		}
	}
	if _, err := VorratAdopt(root, "v2", day); err == nil || !strings.Contains(err.Error(), "von Hand") {
		t.Errorf("Paket: %v", err)
	}
	rel, err := VorratAdopt(root, "v1", day)
	if err != nil || rel != "audio/kandidaten/muenze-v1.ogg" {
		t.Fatalf("Übernahme %q %v", rel, err)
	}
	if again, err := VorratAdopt(root, "v1", day); err != nil || again != rel {
		t.Errorf("zweite Übernahme %q %v", again, err)
	}
	credits, _ := os.ReadFile(filepath.Join(root, "public", "audio", "README.md"))
	if !strings.Contains(string(credits), "| kandidaten/muenze-v1.ogg | Münze v1 | Anna | CC0 | https://opengameart.org/content/v1 |") {
		t.Errorf("Credit:\n%s", credits)
	}
	cat, _ := Catalog(root)
	if found := Search(cat, "muenze-v1", KindSound, 0); len(found) != 1 || found[0].License != "CC0" {
		t.Errorf("Katalog %+v", found)
	}
	reg, _ := Load(root)
	if c := reg.Find("sfx:coin").Choices; len(c) != 1 || c[0].File != rel {
		t.Errorf("Zuordnung %+v", c)
	}
	checkCleanup(t, root)
}

// checkCleanup: v1 ist übernommen und zugeordnet, v2 nur geladen; das Aufräumen entfernt nur v2.
func checkCleanup(t *testing.T, root string) {
	t.Helper()
	preview, _ := VorratCleanupPreview(root)
	if len(preview) != 1 || preview[0].ID != "v2" {
		t.Errorf("Vorschau %+v", preview)
	}
	removed, err := VorratCleanup(root)
	if err != nil || len(removed) != 1 {
		t.Fatalf("Aufräumen %+v %v", removed, err)
	}
	states, _ := VorratStates(root, []Suggestion{suggestion(t, root, "v1"), suggestion(t, root, "v2")})
	if states["v2"].Status != VorratOpen || states["v1"].Status != VorratLoaded || !states["v1"].Used {
		t.Errorf("Status nach Aufräumen %+v", states)
	}
}

// allowTestHost lässt den lokalen Testserver (127.0.0.1) als Quelle zu, nur für diesen Test.
func allowTestHost(t *testing.T) {
	t.Helper()
	old := VorratHosts
	VorratHosts = append(slices.Clone(old), "127.0.0.1")
	t.Cleanup(func() { VorratHosts = old })
}

// Fremde Hosts, Klartext und Umleitungen auf nicht erlaubte Ziele lädt der Vorrat nicht (SSRF, Review WZ29.6).
func TestVorratNurErlaubteHosts(t *testing.T) {
	for _, raw := range []string{"https://127.0.0.1/a.png", "http://opengameart.org/a.png", "https://evil.example/a.png", "https://opengameart.org.evil.example/a.png"} {
		u, _ := url.Parse(raw)
		if VorratURLAllowed(u) == nil {
			t.Errorf("%s erlaubt", raw)
		}
	}
	for _, raw := range []string{"https://opengameart.org/sites/default/files/a.png", "https://cdn.freesound.org/previews/a.mp3"} {
		u, _ := url.Parse(raw)
		if err := VorratURLAllowed(u); err != nil {
			t.Errorf("%s: %v", raw, err)
		}
	}
	req, _ := http.NewRequest(http.MethodGet, "https://192.168.1.10/x", nil)
	if VorratRedirect(req, nil) == nil {
		t.Error("Umleitung ins Heimnetz erlaubt")
	}
}
