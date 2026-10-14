package mcpsrv

import (
	"net/http"
	"net/http/httptest"
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
