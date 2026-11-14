package review

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"slices"
	"strings"
	"testing"
)

// generisch macht aus einem Ergebnis generisches JSON mit nach id sortierten Assets (so vergleichen die Tests).
func generisch(t *testing.T, e Ergebnis) any {
	t.Helper()
	e.Assets = slices.Clone(e.Assets)
	slices.SortFunc(e.Assets, func(a, b Asset) int { return strings.Compare(a.ID, b.ID) })
	raw, err := json.Marshal(e)
	if err != nil {
		t.Fatal(err)
	}
	var v any
	if err := json.Unmarshal(raw, &v); err != nil {
		t.Fatal(err)
	}
	return v
}

func fixtureReview(t *testing.T) Ergebnis {
	t.Helper()
	var e Ergebnis
	if err := json.Unmarshal(lies(t, fixture("ui/AssetsReview.json")), &e); err != nil {
		t.Fatal(err)
	}
	return e
}

// TestReviewHEAD1WieFixture: alle fünf Fälle gegen HEAD~1 exakt wie tools/paper/fixtures/ui/AssetsReview.json (AC-01).
func TestReviewHEAD1WieFixture(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	got, err := Vergleich(context.Background(), repo, "HEAD~1", &fakeFrames{CLI: cli()})
	if err != nil {
		t.Fatal(err)
	}
	want := fixtureReview(t)
	want.BasisCommit = basisCommit(t, repo) // Hash hängt von der echten Stil-Datei ab (B-738)
	if g, w := generisch(t, got), generisch(t, want); !reflect.DeepEqual(g, w) {
		gj, _ := json.MarshalIndent(g, "", " ")
		t.Fatalf("Review weicht ab:\n%s", gj)
	}
	reihenfolge := make([]string, len(got.Assets))
	for i, a := range got.Assets {
		reihenfolge[i] = a.ID
	}
	if !slices.Equal(reihenfolge, []string{"banner", "mauer", "monarch", "pferd", "fass"}) {
		t.Errorf("Reihenfolge %v, will ungueltig, geaendert, neu, geloescht je nach id", reihenfolge)
	}
}

// TestReviewStilwechselVorherBild (B-705/AC-02): nur die Stil-Datei geändert; das Vorher-Bild entsteht mit dem Stil
// der Basis (--stil des Schnappschusses), daher zeigt die echte CLI einen Pixel-Unterschied.
func TestReviewStilwechselVorherBild(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	gitIn(t, repo, "add", "-A")
	gitIn(t, repo, "commit", "-q", "--no-gpg-sign", "-m", "Arbeitsstand")
	pfad := filepath.Join(repo, filepath.FromSlash(stilPfad))
	var stil map[string]any
	if err := json.Unmarshal(lies(t, pfad), &stil); err != nil {
		t.Fatal(err)
	}
	stil["materialien"].(map[string]any)["stein"].(map[string]any)["basis"] = "#2050d0"
	raw, _ := json.MarshalIndent(stil, "", "  ")
	if err := os.WriteFile(pfad, raw, 0o644); err != nil {
		t.Fatal(err)
	}
	got, err := Vergleich(context.Background(), repo, "HEAD", cli())
	if err != nil {
		t.Fatal(err)
	}
	for _, a := range got.Assets {
		if a.ID == "mauer" {
			if a.Status != StatusGeaendert || !a.Stil || a.PixelAnteil <= 0 {
				t.Errorf("mauer: %+v", a)
			}
			return
		}
	}
	t.Fatalf("mauer fehlt: %+v", got.Assets)
}

// TestReviewHEAD: gegen HEAD nur monarch (geändert) und banner (ungültig).
func TestReviewHEAD(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	got, err := Vergleich(context.Background(), repo, "HEAD", &fakeFrames{CLI: cli()})
	if err != nil {
		t.Fatal(err)
	}
	status := map[string]string{}
	for _, a := range got.Assets {
		status[a.ID] = a.Status
	}
	if !reflect.DeepEqual(status, map[string]string{"monarch": StatusGeaendert, "banner": StatusUngueltig}) || got.Basis != "HEAD" {
		t.Fatalf("Review gegen HEAD: %+v", got)
	}
}

// TestReviewNurCRLF: nur Zeilenenden geändert → kein Eintrag (AC-02).
func TestReviewNurCRLF(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	pfad := filepath.Join(repo, "public/grafik/figuren/monarch.paper.json")
	alt, _, err := Show(context.Background(), repo, "HEAD", "public/grafik/figuren/monarch.paper.json")
	if err != nil {
		t.Fatal(err)
	}
	crlf := strings.ReplaceAll(string(alt), "\n", "\r\n")
	if err := os.WriteFile(pfad, []byte(crlf), 0o644); err != nil {
		t.Fatal(err)
	}
	got, err := Vergleich(context.Background(), repo, "HEAD", &fakeFrames{CLI: cli()})
	if err != nil {
		t.Fatal(err)
	}
	for _, a := range got.Assets {
		if a.ID == "monarch" {
			t.Fatalf("monarch nur mit CRLF gemeldet: %+v", a)
		}
	}
}

// TestReviewAusgabeGeaendert: nur das Manifest der Ausgabe anders → geändert ohne Felder.
func TestReviewAusgabeGeaendert(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	alt, _, err := Show(context.Background(), repo, "HEAD", "public/grafik/figuren/monarch.paper.json")
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(repo, "public/grafik/figuren/monarch.paper.json"), alt, 0o644); err != nil {
		t.Fatal(err)
	}
	manifest := filepath.Join(repo, "public/grafik/figuren/monarch/manifest.json")
	if err := os.MkdirAll(filepath.Dir(manifest), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(manifest, []byte(`{"id":"monarch","renderKey":"neu"}`), 0o644); err != nil {
		t.Fatal(err)
	}
	got, err := Vergleich(context.Background(), repo, "HEAD", &fakeFrames{CLI: cli()})
	if err != nil {
		t.Fatal(err)
	}
	for _, a := range got.Assets {
		if a.ID == "monarch" {
			if a.Status != StatusGeaendert || len(a.Felder) != 0 || a.Stil {
				t.Fatalf("monarch: %+v", a)
			}
			return
		}
	}
	t.Fatal("monarch fehlt trotz geänderter Ausgabe")
}

// TestGeaendertGegenHEAD: ohne CLI; gegen HEAD sind nur monarch und banner geändert (Grundlage von AssetsList).
func TestGeaendertGegenHEAD(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	got, err := Geaendert(context.Background(), repo, "HEAD")
	if err != nil {
		t.Fatal(err)
	}
	want := map[string]bool{
		"public/grafik/figuren/monarch.paper.json":   true,
		"public/grafik/requisiten/banner.paper.json": true,
		"public/grafik/reittiere/pferd.paper.json":   false,
		"public/grafik/gebaeude/mauer.paper.json":    false,
	}
	if !reflect.DeepEqual(got, want) {
		t.Fatalf("Geaendert = %v", got)
	}
}
