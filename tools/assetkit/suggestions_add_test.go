package assets

import (
	"context"
	"errors"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// addRepo: Mini-Repo (Slots farm, sfx:coin) ohne Vorschlagsliste; der TLS-Testserver liefert nur HEAD-Größen.
func addRepo(t *testing.T) (string, string, *http.Client) {
	t.Helper()
	root := repo(t)
	srv := httptest.NewTLSServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodHead || strings.HasSuffix(r.URL.Path, "missing.png") {
			http.NotFound(w, r)
			return
		}
		w.Header().Set("Content-Length", "1234")
	}))
	t.Cleanup(srv.Close)
	allowTestHost(t)
	return root, srv.URL, srv.Client()
}

func cand(base, file, slot, kind, license string) Suggestion {
	return Suggestion{Slot: slot, Kind: kind, Title: "Titel " + file, Source: "Kenney", Page: "https://kenney.nl/assets/x",
		FileURL: base + "/" + file, License: license}
}

// B-673/AC-05: ungültige Einträge (Lizenz, Quelle, Slot, Endung, Art, doppelte URL) stehen mit Grund in Skipped, gültige
// werden mit neuer ID und Größe per HEAD in einem Schritt geschrieben.
func TestAddSuggestions(t *testing.T) {
	root, base, client := addRepo(t)
	size := HeadSize(client)
	bad := map[string]Suggestion{
		"NC":     cand(base, "nc.png", "farm", KindGraphic, "CC-BY-NC 4.0"),
		"Slot":   cand(base, "slot.png", "weg", KindGraphic, "CC0"),
		"Endung": cand(base, "x.exe", "farm", KindGraphic, "CC0"),
		"Art":    cand(base, "art.png", "farm", KindSound, "CC0"),
		"HEAD":   cand(base, "missing.png", "farm", KindGraphic, "CC0"),
	}
	foreign := cand(base, "q.png", "farm", KindGraphic, "CC0")
	foreign.Source = "Wikimedia Commons"
	in := []Suggestion{cand(base, "ok.png", "farm", KindGraphic, "CC0"), cand(base, "ok.png", "farm", KindGraphic, "CC0"), foreign}
	for _, c := range bad {
		in = append(in, c)
	}
	res, err := AddSuggestions(context.Background(), root, in, size)
	if err != nil {
		t.Fatal(err)
	}
	if len(res.Added) != 1 || res.Added[0].ID != "v001" || res.Added[0].Size != 1234 || res.Added[0].Status != "offen" || res.Bytes != 1234 {
		t.Fatalf("übernommen: %+v", res)
	}
	if len(res.Skipped) != 7 {
		t.Fatalf("übersprungen: %+v", res.Skipped)
	}
	wantReasons(t, res.Skipped, "schon in der Liste", "Quelle nicht erlaubt", "nicht erlaubt (Q70)", "nicht im Register", "endung", "passt nicht", "Größe nicht ermittelbar")
	list, _ := LoadSuggestions(root)
	if len(list.List) != 1 {
		t.Fatalf("geschrieben: %+v", list.List)
	}
	secondRun(t, root, base, size)
}

// secondRun: ein zweiter Lauf vergibt die nächste ID und lehnt die Datei-URL als doppelt ab.
func secondRun(t *testing.T, root, base string, size SizeFunc) {
	t.Helper()
	again, err := AddSuggestions(context.Background(), root, []Suggestion{cand(base, "ok.png", "farm", KindGraphic, "CC0"), cand(base, "two.png", "sfx:coin", KindSound, "CC0")}, size)
	if err != nil || len(again.Added) != 1 || again.Added[0].ID != "v002" || len(again.Skipped) != 1 {
		t.Fatalf("zweiter Lauf: %+v %v", again, err)
	}
	if left, _ := filepath.Glob(filepath.Join(root, "data", ".assets-*")); len(left) != 0 {
		t.Errorf("Zwischendateien bleiben: %v", left)
	}
}

// wantReasons prüft, dass jeder Teiltext in einem der Gründe vorkommt.
func wantReasons(t *testing.T, skipped []SkippedSuggestion, want ...string) {
	t.Helper()
	text := ""
	for _, s := range skipped {
		text += s.Reason + "|"
	}
	for _, w := range want {
		if !strings.Contains(text, w) {
			t.Errorf("Grund %q fehlt in %s", w, text)
		}
	}
}

// Ändert sich die Liste während der Größenabfrage, bricht das Anhängen ab und schreibt nichts (B-673, Fehlerfälle).
func TestAddSuggestionsChangedMeanwhile(t *testing.T) {
	root, base, _ := addRepo(t)
	size := func(context.Context, string) (int64, error) {
		writeFile(t, filepath.Join(root, SuggestionsPath), `{"_comment":"extern","vorschlaege":[]}`)
		return 10, nil
	}
	_, err := AddSuggestions(context.Background(), root, []Suggestion{cand(base, "a.png", "farm", KindGraphic, "CC0")}, size)
	if !errors.Is(err, ErrSuggestionsChanged) {
		t.Fatalf("Fehler: %v", err)
	}
	b, _ := os.ReadFile(filepath.Join(root, SuggestionsPath))
	if !strings.Contains(string(b), "extern") {
		t.Errorf("Datei überschrieben: %s", b)
	}
}

// seedHits legt Recherche-Treffer zu farm an (Datei-URLs auf dem Testserver).
func seedHits(t *testing.T, root, base string) {
	t.Helper()
	hit := func(title, source, file, license, status string) Hit {
		h := Hit{Title: title, Link: "https://example.org/" + title, Source: source, License: license, Artist: "Anna", LicenseStatus: status}
		if file != "" {
			h.Download = base + "/" + file
		}
		return h
	}
	for _, h := range []Hit{
		hit("gut", "Kenney", "gut.png", "CC0", LicenseKnown),
		hit("bestaetigt", "OpenGameArt", "b.png", "CC BY 4.0", LicenseConfirmed),
		hit("ohne", "Kenney", "", "CC0", LicenseKnown),
		hit("wiki", "Wikimedia Commons", "w.png", "CC0", LicenseKnown),
		hit("pruefen", "OpenGameArt", "p.png", "", LicenseCheck),
		hit("nc", "OpenGameArt", "n.png", "CC-BY-NC 3.0", LicenseKnown),
	} {
		if _, err := AddHits(root, "farm", []Hit{h}, day); err != nil {
			t.Fatal(err)
		}
	}
}

// B-673/AC-04: Aus Recherche nur Treffer mit Datei-URL, erlaubter Quelle und Lizenz erkannt/bestätigt; Plan schreibt nichts,
// Übernahme schreibt ohne Duplikate, nur die bestätigten URLs.
func TestImportResearch(t *testing.T) {
	root, base, client := addRepo(t)
	seedHits(t, root, base)
	size := HeadSize(client)
	plan, err := PlanResearchImport(context.Background(), root, "", size)
	if err != nil {
		t.Fatal(err)
	}
	if len(plan.Added) != 2 || len(plan.Skipped) != 4 || plan.Bytes != 2468 {
		t.Fatalf("Plan: %+v", plan)
	}
	if _, err := os.Stat(filepath.Join(root, SuggestionsPath)); err == nil {
		t.Fatal("Plan hat geschrieben")
	}
	if p, _ := PlanResearchImport(context.Background(), root, "Kenney", size); len(p.Added) != 1 || p.Added[0].License != "CC0" {
		t.Fatalf("Filter Quelle: %+v", p)
	}
	res, err := ImportResearch(context.Background(), root, "", []string{base + "/gut.png"}, size)
	if err != nil || len(res.Added) != 1 {
		t.Fatalf("Übernahme: %+v %v", res, err)
	}
	again, _ := ImportResearch(context.Background(), root, "", nil, size)
	if len(again.Added) != 1 || again.Added[0].License != "CC-BY 4.0" || again.Added[0].ID != "v002" {
		t.Fatalf("zweite Übernahme ohne Duplikat: %+v", again)
	}
	wantReasons(t, again.Skipped, "keine direkte Datei-URL", "Quelle nicht erlaubt", "Lizenz nicht geklärt", "nicht erlaubt (Q70)", "schon in der Liste")
}
