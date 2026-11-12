package review

import (
	"context"
	"os"
	"path/filepath"
	"sync"
	"testing"
)

// TestBildAnimationMitVariante: Animationsschlüssel wie `idle@spieler2` sind erlaubt, Pfadteile nicht (Review-Befund 1).
func TestBildAnimationMitVariante(t *testing.T) {
	t.Parallel()
	for _, ok := range []string{"idle", "idle@spieler2", "stand.s2", "reiten-2"} {
		if !reName.MatchString(ok) {
			t.Errorf("%q abgelehnt", ok)
		}
	}
	for _, bad := range []string{"", "..", "a/b", `a\b`, "-x", "a..b", "C:x"} {
		if gueltigerName(bad) {
			t.Errorf("%q angenommen", bad)
		}
	}
}

// TestBildParallel: gleichzeitige Aufrufe für denselben Frame scheitern nicht an einer gemeinsamen Temp-Datei (Befund 2).
func TestBildParallel(t *testing.T) {
	langsam(t)
	t.Parallel()
	repo := testRepo(t)
	q, ok, err := QuelleArbeitsstand(repo, "public/grafik/figuren/monarch.paper.json")
	if err != nil || !ok {
		t.Fatal(err)
	}
	var wg sync.WaitGroup
	fehler := make([]error, 4)
	for i := range fehler {
		wg.Go(func() { _, fehler[i] = Bild(context.Background(), cli(), repo, q, "reiten", 0, "") })
	}
	wg.Wait()
	for _, err := range fehler {
		if err != nil {
			t.Fatal(err)
		}
	}
}

// TestBildAusSchnappschuss: die CLI rendert die gehashten Bytes, nicht die spätere Datei auf der Platte (Befund 3).
func TestBildAusSchnappschuss(t *testing.T) {
	langsam(t)
	t.Parallel()
	repo := testRepo(t)
	q, _, err := QuelleArbeitsstand(repo, "public/grafik/figuren/monarch.paper.json")
	if err != nil {
		t.Fatal(err)
	}
	datei, err := q.SpecDatei(repo, cli())
	if err != nil {
		t.Fatal(err)
	}
	if filepath.Dir(filepath.Dir(filepath.Dir(filepath.Dir(filepath.Dir(datei))))) != q.ordner(repo, cli()) {
		t.Fatalf("Spec für die CLI liegt nicht im Cache: %s", datei)
	}
}

// TestReviewKaputteSpec: nicht lesbares JSON wird ein ungültiger Eintrag, der Rest des Reviews bleibt (Befund 4).
func TestReviewKaputteSpec(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	if err := os.WriteFile(filepath.Join(repo, "public/grafik/requisiten/kaputt.paper.json"), []byte("{nicht json"), 0o644); err != nil {
		t.Fatal(err)
	}
	got, err := Vergleich(context.Background(), repo, "HEAD", &fakeFrames{CLI: cli()})
	if err != nil {
		t.Fatal(err)
	}
	status := map[string]string{}
	for _, a := range got.Assets {
		status[a.ID] = a.Status
		if a.ID == "kaputt" && (len(a.Fehler) != 1 || a.Fehler[0].Regel != "schema") {
			t.Errorf("kaputt: %+v", a)
		}
	}
	if status["kaputt"] != StatusUngueltig || status["monarch"] != StatusGeaendert || status["banner"] != StatusUngueltig {
		t.Fatalf("Status = %v", status)
	}
}

// TestShowGitFehler: ein kaputter Commit ist ein Fehler, nicht „fehlt in der Basis“ (Befund 5).
func TestShowGitFehler(t *testing.T) {
	langsam(t)
	t.Parallel()
	repo := testRepo(t)
	if _, ok, err := Show(context.Background(), repo, "HEAD", "public/grafik/fehlt.paper.json"); ok || err != nil {
		t.Fatalf("fehlende Datei: ok %v, err %v", ok, err)
	}
	if _, _, err := Show(context.Background(), filepath.Join(repo, "kein-repo"), "HEAD", "public/grafik/stil.paper.json"); err == nil {
		t.Fatal("Git-Fehler verschluckt")
	}
}

// TestIstSpecHaertung: nur echte Spec-Pfade, keine Pfadteile wie .. oder Backslash (Befund 6).
func TestIstSpecHaertung(t *testing.T) {
	t.Parallel()
	for _, bad := range []string{"public/grafik/../x.paper.json", `public/grafik/a\..\x.paper.json`, "public/grafik/a/..paper.json",
		"public/grafik/a/index.paper.json", "public/grafik/A:/x.paper.json"} {
		if istSpec(bad) {
			t.Errorf("%q gilt als Spec", bad)
		}
	}
	if !istSpec("public/grafik/figuren/monarch.paper.json") {
		t.Error("monarch abgelehnt")
	}
}

// TestCacheKennung: die echte CLI (RD1) rendert unter RendererVersion, ohne Renderer ebenso (Befund 7).
func TestCacheKennung(t *testing.T) {
	t.Parallel()
	if k := kennung(cli()); k != RendererVersion {
		t.Fatalf("echte CLI: %s", k)
	}
	if k := kennung(nil); k != RendererVersion {
		t.Fatalf("ohne Renderer: %s", k)
	}
}

// TestSchnappschussMitReittier: der Schnappschuss enthält das Reittier der Spec, damit die echte CLI (RD1, Regel
// reittier-unbekannt) es neben der Spec findet; im Arbeitsstand aus dem Checkout, in der Basis aus dem Commit.
func TestSchnappschussMitReittier(t *testing.T) {
	langsam(t)
	t.Parallel()
	repo := testRepo(t)
	q, _, err := QuelleArbeitsstand(repo, "public/grafik/figuren/monarch.paper.json")
	if err != nil {
		t.Fatal(err)
	}
	if _, ok := q.Beilagen["public/grafik/reittiere/pferd.paper.json"]; !ok {
		t.Fatalf("Beilagen = %v", q.Beilagen)
	}
	spec, err := q.SpecDatei(repo, cli())
	if err != nil {
		t.Fatal(err)
	}
	pferd := filepath.Join(filepath.Dir(filepath.Dir(spec)), "reittiere", "pferd.paper.json")
	if _, err := os.Stat(pferd); err != nil {
		t.Fatalf("Reittier fehlt im Schnappschuss: %v", err)
	}
	b, ok, err := QuelleBasis(context.Background(), repo, "HEAD", "public/grafik/figuren/monarch.paper.json")
	if err != nil || !ok {
		t.Fatal(err)
	}
	if _, ok := b.Beilagen["public/grafik/reittiere/pferd.paper.json"]; !ok {
		t.Fatalf("Basis-Beilagen = %v", b.Beilagen)
	}
	if b.ordner(repo, cli()) == q.ordner(repo, cli()) {
		t.Fatal("Basis und Arbeitsstand teilen den Cache-Ordner")
	}
}
