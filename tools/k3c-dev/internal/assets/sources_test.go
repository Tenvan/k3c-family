package assets

import (
	"context"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// recorded spielt aufgezeichnete Antworten ab (testdata/): OpenGameArt und Commons echt aufgezeichnet,
// freesound im Format der API-Doku (ohne Token kam nur 401). Kein Netz im Test.
func recorded(t *testing.T, failOGA bool) Searcher {
	t.Helper()
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		file := ""
		switch {
		case strings.HasPrefix(r.URL.Path, "/apiv2/") && r.URL.Query().Get("token") == "":
			w.WriteHeader(http.StatusUnauthorized)
			file = "freesound-401.json"
		case strings.HasPrefix(r.URL.Path, "/apiv2/"):
			file = "freesound.json"
		case r.URL.Path == "/w/api.php":
			file = "commons.json"
		case r.URL.Path == "/art-search-advanced" && failOGA:
			w.WriteHeader(http.StatusServiceUnavailable)
			return
		case r.URL.Path == "/art-search-advanced" && r.URL.Query().Get("field_art_type_tid[]") == "9":
			file = "oga-2d.html"
		case r.URL.Path == "/art-search-advanced":
			file = "oga-sound.html"
		}
		b, _ := os.ReadFile(filepath.Join("testdata", file))
		_, _ = w.Write(b)
	}))
	t.Cleanup(srv.Close)
	return Searcher{Client: srv.Client(), Freesound: srv.URL, Commons: srv.URL, OGA: srv.URL, Token: "test"}
}

func bySource(hits []Hit) map[string][]Hit {
	out := map[string][]Hit{}
	for _, h := range hits {
		out[h.Source] = append(out[h.Source], h)
	}
	return out
}

// B-298/AC-04: Vorschläge mit Link, Quelle und Lizenz aus drei Quellen, OpenGameArt ohne API.
func TestRechercheSound(t *testing.T) {
	hits, notes := recorded(t, false).Search(context.Background(), "coin", KindSound)
	if len(notes) != 0 {
		t.Fatalf("Hinweise: %v", notes)
	}
	src := bySource(hits)
	fs := src["freesound"]
	if len(fs) != 2 || fs[0].License != "CC0 1.0" || fs[1].License != "CC BY-NC 3.0" || fs[0].Artist != "Beetlemuse" || !strings.HasSuffix(fs[0].Preview, ".mp3") {
		t.Errorf("freesound: %+v", fs)
	}
	checkSoundOGA(t, src["OpenGameArt"])
	if c := src["Wikimedia Commons"]; len(c) != 2 || c[1].License != "CC BY-SA 3.0" || c[1].Artist != "devnewton" || c[1].Download == "" {
		t.Errorf("Commons: %+v", c)
	}
}

func checkSoundOGA(t *testing.T, oga []Hit) {
	t.Helper()
	if len(oga) != 3 || oga[0].Title != "Level up, power up, Coin get (13 Sounds)" || !strings.HasSuffix(oga[0].Link, "/content/level-up-power-up-coin-get-13-sounds") ||
		!strings.HasSuffix(oga[0].Preview, ".mp3") || oga[0].License != "" {
		t.Errorf("OpenGameArt: %+v", oga)
	}
}

func TestRechercheGrafikOhneFreesoundUndMitAusfall(t *testing.T) {
	s := recorded(t, true)
	s.Token = ""
	hits, notes := s.Search(context.Background(), "farm", KindGraphic)
	src := bySource(hits)
	if len(src["freesound"]) != 0 || len(src["Wikimedia Commons"]) != 2 || len(notes) != 1 || !strings.Contains(notes[0], "OpenGameArt: HTTP 503") {
		t.Errorf("Grafik mit OGA-Ausfall: %v %v", src, notes)
	}
	hits, _ = recorded(t, false).Search(context.Background(), "farm", KindGraphic)
	if oga := bySource(hits)["OpenGameArt"]; len(oga) != 3 || oga[0].Title != "Farm" || !strings.HasSuffix(oga[0].Preview, ".png") {
		t.Errorf("OGA 2D: %+v", oga)
	}
}

// Ohne Token meldet freesound einen Hinweis, die anderen Quellen liefern weiter (Token nie im Repo).
func TestFreesoundOhneToken(t *testing.T) {
	s := recorded(t, false)
	s.Token = ""
	hits, notes := s.Search(context.Background(), "coin", KindSound)
	if len(notes) != 1 || !strings.Contains(notes[0], "freesound: token fehlt") || len(hits) != 5 {
		t.Errorf("ohne Token: %d Treffer, %v", len(hits), notes)
	}
}
