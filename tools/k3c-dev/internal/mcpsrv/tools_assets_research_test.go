package mcpsrv

import (
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/tools/k3c-dev/internal/assets"
)

// B-298/AC-06: Recherche-Tools antworten; Netz ersetzt ein Testserver mit einer OpenGameArt-Zeile.
func TestAssetsResearchTools(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/art-search-advanced" {
			_, _ = w.Write([]byte(`<span class="art-preview-title"><a href="/content/coin">Coin</a></span><img src='https://x/coin.png'>` + "\n"))
			return
		}
		w.WriteHeader(http.StatusNotFound)
	}))
	defer srv.Close()
	old := newSearcher
	newSearcher = func(string) assets.Searcher {
		return assets.Searcher{Client: srv.Client(), Freesound: srv.URL, Commons: srv.URL, OGA: srv.URL}
	}
	defer func() { newSearcher = old }()
	cs := connect(t, New(Config{Version: "test", Root: assetRepo(t)}))
	text, isErr := callText(t, cs, "assets_research", map[string]any{"id": "farm", "query": "coin"})
	if isErr || !strings.Contains(text, "farm: 1 neu, 1 gespeichert · Hinweise: Wikimedia Commons: HTTP 404") || !strings.Contains(text, "pruefen · OpenGameArt · Lizenz ? · Coin") {
		t.Errorf("assets_research: %q", text)
	}
	steps := []struct {
		tool string
		args map[string]any
		want string
	}{
		{"assets_hit_add", map[string]any{"id": "farm", "titel": "Farm itch", "link": "https://x.itch.io/farm", "lizenz": "CC0"}, "farm: Treffer gespeichert"},
		{"assets_hit_add", map[string]any{"id": "farm", "titel": "Farm itch", "link": "https://x.itch.io/farm"}, "Link schon gespeichert"},
		{"assets_hits", map[string]any{"id": "farm"}, "erkannt · Agent · CC0 · Farm itch · https://x.itch.io/farm"},
	}
	for _, st := range steps {
		if text, isErr := callText(t, cs, st.tool, st.args); isErr || !strings.Contains(text, st.want) {
			t.Errorf("%s: %q", st.tool, text)
		}
	}
	hit := strings.Fields(strings.Split(text, "\n")[1])[0]
	if text, isErr := callText(t, cs, "assets_hit_drop", map[string]any{"id": "farm", "hit": hit}); isErr || !strings.Contains(text, "verworfen") {
		t.Errorf("assets_hit_drop: %q", text)
	}
	if text, isErr := callText(t, cs, "assets_hit_add", map[string]any{"id": "farm", "titel": "x", "link": "http://unsicher"}); !isErr || !strings.Contains(text, "https://") {
		t.Errorf("http abgelehnt: %q", text)
	}
}

// B-298/AC-05 über MCP: lokale Datei zu einem Treffer importieren; Ablage und Credit stimmen.
func TestAssetsImportTool(t *testing.T) {
	root := assetRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	if text, isErr := callText(t, cs, "assets_hit_add", map[string]any{"id": "farm", "titel": "Farm Haus", "link": "https://x.itch.io/farm"}); isErr {
		t.Fatal(text)
	}
	hits, _ := assets.HitsOf(root, "farm")
	src := filepath.Join(root, "public", "grafik", "town", "well.png")
	text, isErr := callText(t, cs, "assets_import", map[string]any{"id": "farm", "hit": hits[0].ID, "path": src})
	if isErr || !strings.Contains(text, "grafik/kandidaten/farm-haus.png importiert") {
		t.Fatalf("assets_import: %q", text)
	}
	credits, _ := os.ReadFile(filepath.Join(root, "public", "grafik", "CREDITS.md"))
	if !strings.Contains(string(credits), "| kandidaten/farm-haus.png | Farm Haus | unbekannt | unbekannt | https://x.itch.io/farm | Lizenz prüfen |") {
		t.Errorf("Credit: %s", credits)
	}
	outside := filepath.Join(t.TempDir(), "fremd.png")
	if err := os.WriteFile(outside, mustRead(t, src), 0o644); err != nil {
		t.Fatal(err)
	}
	if text, isErr := callText(t, cs, "assets_import", map[string]any{"id": "farm", "hit": hits[0].ID, "path": outside}); !isErr || !strings.Contains(text, "nur Dateien unterhalb der Repo-Wurzel") {
		t.Errorf("Pfad außerhalb des Checkouts: %q", text)
	}
	if text, isErr := callText(t, cs, "assets_import", map[string]any{"id": "farm", "hit": hits[0].ID}); !isErr || !strings.Contains(text, "manuell") {
		t.Errorf("Download ohne erkannte Lizenz: %q", text)
	}
}

func mustRead(t *testing.T, p string) []byte {
	t.Helper()
	b, err := os.ReadFile(p)
	if err != nil {
		t.Fatal(err)
	}
	return b
}
