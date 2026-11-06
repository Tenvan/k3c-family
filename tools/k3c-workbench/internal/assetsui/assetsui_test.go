package assetsui

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"reflect"
	"slices"
	"strings"
	"testing"

	"k3c/tools/k3c-workbench/internal/review"
)

// nachID sortiert eine generische JSON-Liste von Objekten nach "id".
func nachID(v any) any {
	liste, _ := v.([]any)
	liste = slices.Clone(liste)
	slices.SortFunc(liste, func(a, b any) int {
		return strings.Compare(a.(map[string]any)["id"].(string), b.(map[string]any)["id"].(string))
	})
	return liste
}

// TestAssetsListTestRepo: Katalog aus dem Checkout; geaendert gegen HEAD, daher pferd und mauer false (AC-07, B-701).
func TestAssetsListTestRepo(t *testing.T) {
	t.Parallel()
	got, err := dienst(testRepo(t), false).AssetsList()
	if err != nil {
		t.Fatal(err)
	}
	want := fixture(t, "AssetsList").([]any)
	for _, e := range want {
		if m := e.(map[string]any); m["id"] == "pferd" || m["id"] == "mauer" {
			m["geaendert"] = false
		}
	}
	if g, w := nachID(roundtrip(t, got)), nachID(want); !reflect.DeepEqual(g, w) {
		t.Fatalf("AssetsList = %v\nwill %v", g, w)
	}
}

// TestAssetsGet: Spec ohne $schema; im Test-Repo trägt der Umhang purpur (AC-07).
func TestAssetsGet(t *testing.T) {
	t.Parallel()
	got, err := dienst(testRepo(t), false).AssetsGet("monarch")
	if err != nil {
		t.Fatal(err)
	}
	want := fixture(t, "AssetsGet").(map[string]any)
	want["umhang"].(map[string]any)["material"] = "purpur"
	if !reflect.DeepEqual(roundtrip(t, got), want) {
		t.Fatal("AssetsGet weicht von der Fixture ab")
	}
}

// TestAssetsGetFehlt: Vertragsfehler für eine unbekannte id.
func TestAssetsGetFehlt(t *testing.T) {
	t.Parallel()
	_, err := dienst(testRepo(t), false).AssetsGet("niemand")
	if err == nil || err.Error() != "Asset „niemand“ fehlt" {
		t.Fatalf("err = %v", err)
	}
}

// TestAssetsStyle und TestAssetsPrompts: echte Dateien der Repo-Wurzel = Fixtures (AC-07).
func TestAssetsStyle(t *testing.T) {
	t.Parallel()
	got, err := dienst(root, false).AssetsStyle()
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(roundtrip(t, got), fixture(t, "AssetsStyle")) {
		t.Fatal("AssetsStyle weicht ab")
	}
}

func TestAssetsPrompts(t *testing.T) {
	t.Parallel()
	got, err := dienst(root, false).AssetsPrompts()
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(roundtrip(t, got), fixture(t, "AssetsPrompts")) {
		t.Fatal("AssetsPrompts weicht ab")
	}
}

// TestAssetsReviewTestRepo: gegen HEAD~1 wie AssetsReview.json; banner ungültig mit Fehlertext, die übrigen vollständig (AC-05).
func TestAssetsReviewTestRepo(t *testing.T) {
	t.Parallel()
	got, err := dienst(testRepo(t), true).AssetsReview("HEAD~1")
	if err != nil {
		t.Fatal(err)
	}
	g, w := roundtrip(t, got).(map[string]any), fixture(t, "AssetsReview").(map[string]any)
	g["assets"], w["assets"] = nachID(g["assets"]), nachID(w["assets"])
	if !reflect.DeepEqual(g, w) {
		t.Fatalf("AssetsReview = %v", g)
	}
}

// TestAssetsReviewLeer: leere Basis = HEAD.
func TestAssetsReviewLeer(t *testing.T) {
	t.Parallel()
	got, err := dienst(testRepo(t), true).AssetsReview("")
	if err != nil {
		t.Fatal(err)
	}
	if got.Basis != "HEAD" || len(got.Assets) != 2 {
		t.Fatalf("Review = %+v", got)
	}
}

// TestAssetsBasisUngueltig: Vertragsfehlertext; beim Pixel-Diff ist leer keine Basis.
func TestAssetsBasisUngueltig(t *testing.T) {
	t.Parallel()
	d := dienst(testRepo(t), true)
	_, e1 := d.AssetsReview("main")
	_, e2 := d.AssetsFrame("monarch", "reiten", 0, "", "main")
	_, e3 := d.AssetsPixelDiff("monarch", "reiten", 0, "")
	_, e4 := d.AssetsPixelDiff("monarch", "reiten", 0, "; rm -rf")
	for i, err := range []error{e1, e2, e3, e4} {
		if err == nil || !strings.HasSuffix(err.Error(), "ungültig: HEAD, HEAD~1, Commit-SHA oder origin/develop") {
			t.Errorf("Aufruf %d: %v", i, err)
		}
	}
}

func dekodiere(t *testing.T, url string) []byte {
	t.Helper()
	const prefix = "data:image/png;base64,"
	if !strings.HasPrefix(url, prefix) {
		t.Fatalf("keine PNG-Data-URL: %.40s", url)
	}
	raw, err := base64.StdEncoding.DecodeString(strings.TrimPrefix(url, prefix))
	if err != nil {
		t.Fatal(err)
	}
	return raw
}

// TestAssetsFrame: Frame des Arbeitsstands als Data-URL mit den Teil-Boxen der Fixture.
func TestAssetsFrame(t *testing.T) {
	t.Parallel()
	got, err := dienst(testRepo(t), false).AssetsFrame("monarch", "reiten", 0, "", "")
	if err != nil {
		t.Fatal(err)
	}
	png, err := os.ReadFile(filepath.Join(root, "tools/paper/fixtures/render/monarch/monarch-galopp-0.png"))
	if err != nil {
		t.Fatal(err)
	}
	if !slices.Equal(dekodiere(t, got.Bild), png) {
		t.Error("Bild ist nicht das Frame der CLI")
	}
	want := fixture(t, "AssetsFrame").(map[string]any)["teile"]
	if !reflect.DeepEqual(roundtrip(t, got.Teile), want) {
		t.Errorf("teile = %v", got.Teile)
	}
}

// TestAssetsFrameBasis: Frame der Basis kommt beim zweiten Aufruf aus dem Cache; gelöschte Assets gibt es nur dort.
func TestAssetsFrameBasis(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	d := dienst(repo, false)
	erst, err := d.AssetsFrame("monarch", "reiten", 0, "", "HEAD~1")
	if err != nil {
		t.Fatal(err)
	}
	d.R = keinFrame{CLI: review.CLI{Root: root}} // der zweite Aufruf darf nicht rendern
	zweit, err := d.AssetsFrame("monarch", "reiten", 0, "", "HEAD~1")
	if err != nil || zweit.Bild != erst.Bild {
		t.Fatalf("Cache: %v", err)
	}
	// fass gibt es nur in der Basis und hat keine Animation: gefunden, aber ohne Frame (B-701 D-Zeile).
	_, err = dienst(repo, false).AssetsFrame("fass", "", 0, "", "HEAD~1")
	if err == nil || err.Error() != "Asset „fass“ hat keine Animation" {
		t.Fatalf("gelöschtes Asset ohne Animation: %v", err)
	}
}

// TestAssetsFrameAnimationUnbekannt: Text nennt die vorhandenen Animationen.
func TestAssetsFrameAnimationUnbekannt(t *testing.T) {
	t.Parallel()
	_, err := dienst(testRepo(t), false).AssetsFrame("monarch", "flug", 0, "", "")
	if err == nil || !strings.Contains(err.Error(), "„flug“") || !strings.Contains(err.Error(), "idle") || !strings.Contains(err.Error(), "reiten") {
		t.Fatalf("err = %v", err)
	}
}

// TestAssetsFrameLeereAnimation: leer = erste Animation der Spec.
func TestAssetsFrameLeereAnimation(t *testing.T) {
	t.Parallel()
	d := dienst(testRepo(t), false)
	leer, err := d.AssetsFrame("monarch", "", 0, "", "")
	if err != nil {
		t.Fatal(err)
	}
	idle, err := d.AssetsFrame("monarch", "idle", 0, "", "")
	if err != nil || leer.Bild != idle.Bild {
		t.Fatalf("leere Animation ≠ idle: %v", err)
	}
}

// TestAssetsPixelDiff: monarch gegen HEAD~1 wie AssetsPixelDiff.json (anteil, boxen, overlay).
func TestAssetsPixelDiff(t *testing.T) {
	t.Parallel()
	got, err := dienst(testRepo(t), true).AssetsPixelDiff("monarch", "reiten", 0, "HEAD~1")
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(roundtrip(t, got), fixture(t, "AssetsPixelDiff")) {
		t.Fatalf("PixelDiff: anteil %v, boxen %v", got.Anteil, got.Boxen)
	}
}

// keinFrame lässt jeden Render scheitern (gleiche Cache-Kennung wie die Attrappe).
type keinFrame struct{ review.CLI }

func (keinFrame) Frame(context.Context, string, string, int, string, string) (review.FrameAntwort, error) {
	return review.FrameAntwort{}, errors.New("Frame gerendert, obwohl er im Cache liegt")
}

// TestAssetsReviewOhneAenderung: sauberer Checkout → "assets": [] statt null (Review-Befund 1).
func TestAssetsReviewOhneAenderung(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	for _, args := range [][]string{{"add", "-A"}, {"-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q", "-m", "alles"}} {
		cmd := exec.Command("git", args...)
		cmd.Dir = repo
		if out, err := cmd.CombinedOutput(); err != nil {
			t.Fatalf("git %v: %v %s", args, err, out)
		}
	}
	got, err := dienst(repo, true).AssetsReview("HEAD")
	if err != nil {
		t.Fatal(err)
	}
	raw, _ := json.Marshal(got)
	if !strings.Contains(string(raw), `"assets":[]`) {
		t.Fatalf("Review = %s", raw)
	}
}

// kaputteCLI lässt validate scheitern (Absturz von bun).
type kaputteCLI struct{ review.CLI }

func (kaputteCLI) Validate(context.Context, string) ([]review.Fehler, error) {
	return nil, errors.New("bun abgestürzt")
}

// TestAssetsListCLIFehler: ein CLI-Absturz trifft nur den Eintrag (gueltig false, Regel cli), nicht den Katalog (Befund 2);
// ein schon geprüfter Stand kommt aus dem Speicher und braucht die CLI nicht (Befund 3).
func TestAssetsListCLIFehler(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	if err := os.WriteFile(filepath.Join(repo, "public/grafik/requisiten/neu1.paper.json"), []byte(`{"id":"neu1","rig":"requisite","x":1}`), 0o644); err != nil {
		t.Fatal(err)
	}
	if _, err := dienst(repo, false).AssetsList(); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(repo, "public/grafik/requisiten/neu2.paper.json"), []byte(`{"id":"neu2","rig":"requisite","x":2}`), 0o644); err != nil {
		t.Fatal(err)
	}
	liste, err := Dienst{Root: repo, R: kaputteCLI{CLI: review.CLI{Root: root}}}.AssetsList()
	if err != nil {
		t.Fatal(err)
	}
	for _, e := range liste {
		switch e.ID {
		case "neu2":
			if e.Gueltig || len(e.Fehler) != 1 || e.Fehler[0].Regel != "cli" {
				t.Errorf("neu2: %+v", e)
			}
		case "monarch":
			if !e.Gueltig {
				t.Errorf("monarch nicht aus dem Speicher: %+v", e)
			}
		}
	}
}

// TestPfadZuExakt: die id muss dem Dateinamen gleichen, kein Suffix-Treffer über Gruppen hinweg.
func TestPfadZuExakt(t *testing.T) {
	t.Parallel()
	_, err := dienst(testRepo(t), false).AssetsGet("figuren/monarch")
	if err == nil {
		t.Fatal("id mit Gruppe angenommen")
	}
}

// TestAssetsFrameAusserhalb: ein Frame jenseits der Animation ist ein Fehler, kein beliebiges Bild (Gesamt-Review 3).
func TestAssetsFrameAusserhalb(t *testing.T) {
	t.Parallel()
	d := dienst(testRepo(t), false)
	_, err := d.AssetsFrame("monarch", "idle", 8, "", "")
	if err == nil || err.Error() != "Frame 8 fehlt: Animation „idle“ hat 8 Frames" {
		t.Fatalf("err = %v", err)
	}
	if _, err := d.AssetsPixelDiff("monarch", "idle", 8, "HEAD~1"); err == nil {
		t.Fatal("Pixel-Diff jenseits der Animation angenommen")
	}
}
