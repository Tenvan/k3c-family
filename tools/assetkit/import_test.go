package assets

import (
	"bytes"
	"context"
	"image"
	"image/png"
	"net/http"
	"net/http/httptest"
	"net/url"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func pngBytes(t *testing.T) []byte {
	t.Helper()
	var buf bytes.Buffer
	if err := png.Encode(&buf, image.NewNRGBA(image.Rect(0, 0, 16, 16))); err != nil {
		t.Fatal(err)
	}
	return buf.Bytes()
}

// importRepo: Mini-Repo mit Credit-Datei und zwei Treffern zu farm (itch.io: prüfen, Pixabay: ausgeschlossen).
func importRepo(t *testing.T) (root, itch, pixabay string) {
	t.Helper()
	root = repo(t)
	writeFile(t, filepath.Join(root, "public", "grafik", "CREDITS.md"), "# Grafik\n\n| Ordner | Pack | Urheber | Lizenz | Quelle |\n|---|---|---|---|---|\n| town | Town | Zuno | CC0 1.0 | https://x |\n")
	hits := []Hit{{Title: "../../Farm Häuschen", Link: "https://x.itch.io/farm", Source: "Agent", Artist: "Anna"},
		{Title: "Farm", Link: "https://pixabay.com/farm", Source: "Agent", License: "Pixabay"}}
	if _, err := AddHits(root, "farm", hits, day); err != nil {
		t.Fatal(err)
	}
	got, _ := HitsOf(root, "farm")
	return root, got[0].ID, got[1].ID
}

func srcFile(t *testing.T, name string, data []byte) string {
	t.Helper()
	p := filepath.Join(t.TempDir(), name)
	writeFile(t, p, string(data))
	return p
}

// B-298/AC-05: Ablage, Credit-Eintrag und „Lizenz prüfen“; Bestätigen setzt den Status in Treffer und Credit.
func TestImportManuell(t *testing.T) {
	root, itch, _ := importRepo(t)
	rel, err := ImportFile(root, "farm", itch, srcFile(t, "download.PNG", pngBytes(t)), day)
	if err != nil || rel != "grafik/kandidaten/farm-haeuschen.png" {
		t.Fatalf("Import: %q %v", rel, err)
	}
	credits, _ := os.ReadFile(filepath.Join(root, "public", "grafik", "CREDITS.md"))
	row := "| kandidaten/farm-haeuschen.png | ../../Farm Häuschen | Anna | unbekannt | https://x.itch.io/farm | Lizenz prüfen |"
	if !strings.Contains(string(credits), importSection) || !strings.Contains(string(credits), row) {
		t.Errorf("Credit:\n%s", credits)
	}
	reg, _ := Load(root)
	if c := reg.Find("farm").Choices; len(c) != 1 || c[0].File != rel {
		t.Errorf("Zuordnung: %+v", c)
	}
	if err := ConfirmLicense(root, "farm", itch, day); err != nil {
		t.Fatal(err)
	}
	credits, _ = os.ReadFile(filepath.Join(root, "public", "grafik", "CREDITS.md"))
	hits, _ := HitsOf(root, "farm")
	if !strings.Contains(string(credits), "| bestätigt 🧑 2026-10-14 |") || hits[0].LicenseStatus != LicenseConfirmed || hits[0].File != rel {
		t.Errorf("nach Bestätigen: %+v\n%s", hits[0], credits)
	}
	list, _ := Catalog(root)
	for _, a := range list {
		if a.File == rel && a.Artist != "Anna" {
			t.Errorf("Katalog nutzt die Credit-Zeile der Datei nicht: %+v", a)
		}
	}
}

// Vorhandene Datei bleibt, Pfadanteile im Titel landen nie außerhalb von kandidaten/.
func TestImportZweimalUndPfad(t *testing.T) {
	root, itch, _ := importRepo(t)
	src := srcFile(t, "a.png", pngBytes(t))
	first, _ := ImportFile(root, "farm", itch, src, day)
	second, err := ImportFile(root, "farm", itch, src, day)
	if err != nil || first == second || second != "grafik/kandidaten/farm-haeuschen-2.png" {
		t.Errorf("zweiter Import: %q %q %v", first, second, err)
	}
	if _, err := os.Stat(filepath.Join(root, "public", "farm-haeuschen.png")); err == nil {
		t.Error("Datei außerhalb von kandidaten/")
	}
}

func TestImportAbgelehnt(t *testing.T) {
	root, itch, pixabay := importRepo(t)
	big := append([]byte("\x89PNG\r\n\x1a\n"), make([]byte, maxImportBytes)...)
	cases := map[string]struct{ hit, src string }{
		"nicht übernehmbar":        {pixabay, srcFile(t, "a.png", pngBytes(t))},
		"erlaubt sind .png":        {itch, srcFile(t, "a.jpg", pngBytes(t))},
		"a.svg: erlaubt sind .png": {itch, srcFile(t, "a.svg", []byte("<svg><script>alert(1)</script></svg>"))}, // B-514: kein SVG importierbar
		"inhalt ist kein png":      {itch, srcFile(t, "a.png", []byte("geheim"))},
		"größer als 20 MB":         {itch, srcFile(t, "big.png", big)},
		"nicht gefunden":           {"nichts", srcFile(t, "a.png", pngBytes(t))},
	}
	for want, c := range cases {
		if _, err := ImportFile(root, "farm", c.hit, c.src, day); err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%s: %v", want, err)
		}
	}
}

// tlsClient liefert den Testclient ohne Umgebungs-Proxy: Virenscanner mit Traffic-Filter (Norton) hängen sich per
// Proxy-Umgebung auch an Loopback-Verbindungen und tauschen das Zertifikat aus (B-474).
func tlsClient(srv *httptest.Server) *http.Client {
	c := srv.Client()
	if tr, ok := c.Transport.(*http.Transport); ok {
		tr.Proxy = nil
	}
	return c
}

// Direkte Übernahme nur bei https und erkannter Lizenz.
func TestDownloadHit(t *testing.T) {
	root, itch, _ := importRepo(t)
	srv := httptest.NewTLSServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) { _, _ = w.Write(pngBytes(t)) }))
	defer srv.Close()
	if _, err := AddHits(root, "farm", []Hit{{Title: "Intern", Link: "https://commons.wikimedia.org/i", Source: "Agent", License: "CC0 1.0",
		Download: srv.URL + "/intern.png"}}, day); err != nil {
		t.Fatal(err)
	}
	intern, _ := HitsOf(root, "farm")
	if _, err := DownloadHit(context.Background(), root, "farm", intern[2].ID, tlsClient(srv), day); err == nil || !strings.Contains(err.Error(), "download nur von upload.wikimedia.org") {
		t.Fatalf("fremder Host nicht abgelehnt: %v", err)
	}
	allowTestServer(t, srv)
	if _, err := AddHits(root, "farm", []Hit{{Title: "Coin video game.png", Link: "https://commons.wikimedia.org/c", Source: "Wikimedia Commons",
		License: "CC BY-SA 3.0", Download: srv.URL + "/coin.png"}}, day); err != nil {
		t.Fatal(err)
	}
	hits, _ := HitsOf(root, "farm")
	rel, err := DownloadHit(context.Background(), root, "farm", hits[3].ID, tlsClient(srv), day)
	if err != nil || rel != "grafik/kandidaten/coin-video-game-png.png" {
		t.Fatalf("Download: %q %v", rel, err)
	}
	credits, _ := os.ReadFile(filepath.Join(root, "public", "grafik", "CREDITS.md"))
	if !strings.Contains(string(credits), "| CC BY-SA 3.0 | https://commons.wikimedia.org/c | Lizenz erkannt (Wikimedia Commons) |") {
		t.Errorf("Credit:\n%s", credits)
	}
	if _, err := DownloadHit(context.Background(), root, "farm", itch, tlsClient(srv), day); err == nil || !strings.Contains(err.Error(), "manuell") {
		t.Errorf("ohne Download: %v", err)
	}
}

// B-678 AC-03: Ein Treffer-Download von einem erlaubten Host folgt Weiterleitungen (Client ohne Vorrat-Host-Prüfung).
func TestDownloadHitFolgtWeiterleitung(t *testing.T) {
	root, _, _ := importRepo(t)
	srv := httptest.NewTLSServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/alt.png" {
			http.Redirect(w, r, "/neu.png", http.StatusFound)
			return
		}
		_, _ = w.Write(pngBytes(t))
	}))
	defer srv.Close()
	allowTestServer(t, srv)
	if _, err := AddHits(root, "farm", []Hit{{Title: "Weiter.png", Link: "https://commons.wikimedia.org/w", Source: "Wikimedia Commons",
		License: "CC0 1.0", Download: srv.URL + "/alt.png"}}, day); err != nil {
		t.Fatal(err)
	}
	hits, _ := HitsOf(root, "farm")
	if _, err := DownloadHit(context.Background(), root, "farm", hits[len(hits)-1].ID, tlsClient(srv), day); err != nil {
		t.Fatalf("Weiterleitung: %v", err)
	}
}

// B-688 AC-01: Weiterleitung auf einen nicht erlaubten Host oder eine lokale Adresse bricht ab, es entsteht keine Datei.
func TestDownloadHitWeiterleitungAufFremdenHostAbgelehnt(t *testing.T) {
	for _, ziel := range []string{"https://localhost:1/x.png", "https://192.168.0.5/x.png", "http://127.0.0.1/x.png"} {
		root, _, _ := importRepo(t)
		srv := httptest.NewTLSServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			http.Redirect(w, r, ziel, http.StatusFound)
		}))
		allowTestServer(t, srv)
		if _, err := AddHits(root, "farm", []Hit{{Title: "Weg.png", Link: "https://commons.wikimedia.org/w", Source: "Wikimedia Commons",
			License: "CC0 1.0", Download: srv.URL + "/alt.png"}}, day); err != nil {
			t.Fatal(err)
		}
		hits, _ := HitsOf(root, "farm")
		_, err := DownloadHit(context.Background(), root, "farm", hits[len(hits)-1].ID, tlsClient(srv), day)
		srv.Close()
		if err == nil || !strings.Contains(err.Error(), "abgelehnt") {
			t.Errorf("%s: %v", ziel, err)
		}
		if files, _ := filepath.Glob(filepath.Join(root, "public", "grafik", "kandidaten", "*")); len(files) != 0 {
			t.Errorf("%s: Datei entstanden: %v", ziel, files)
		}
	}
}

// allowTestServer erlaubt Host und Port des Testservers bis zum Testende.
func allowTestServer(t *testing.T, srv *httptest.Server) {
	t.Helper()
	u, _ := url.Parse(srv.URL)
	DownloadHosts[u.Hostname()], DownloadPorts[u.Port()] = true, true
	t.Cleanup(func() { delete(DownloadHosts, u.Hostname()); delete(DownloadPorts, u.Port()) })
}

// B-694: Fremder Port und Zugangsdaten in der URL gelten auch auf einem erlaubten Host nicht.
func TestDownloadAllowedPortUndUserinfo(t *testing.T) {
	for raw, want := range map[string]bool{
		"https://upload.wikimedia.org/a.png":         true,
		"https://upload.wikimedia.org:443/a.png":     true,
		"https://upload.wikimedia.org:8443/a.png":    false,
		"https://user:pw@upload.wikimedia.org/a.png": false,
		"https://localhost/a.png":                    false,
	} {
		u, _ := url.Parse(raw)
		if got := downloadAllowed(u); got != want {
			t.Errorf("%s: %v, erwartet %v", raw, got, want)
		}
	}
}
