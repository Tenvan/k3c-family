package review

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"sync/atomic"
	"testing"
)

// fakeFrames ist die CLI-Attrappe, deren Frame die handgemachten Fixture-PNGs liefert: aus dem Cache (Basis) das
// alte, sonst das neue Bild. So entstehen die pixelAnteil-Werte der Fixture (beispiele.ts).
type fakeFrames struct {
	CLI
	aufrufe atomic.Int32
}

var fakeBilder = map[string][2]string{
	"monarch": {"monarch/monarch-galopp-0.png", "monarch/monarch-galopp-0-purpur.png"},
	"mauer":   {"mauer/mauer-1.png", "mauer/mauer-1-stil.png"},
}

func (f *fakeFrames) Frame(ctx context.Context, spec, animation string, frame int, variante, out string) (FrameAntwort, error) {
	f.aufrufe.Add(1)
	// Die echte CLI (RD1) lehnt die Basis HEAD~1 ab (Reittier fehlt dort); die Attrappe rendert ohne CLI.
	if bilder, ok := fakeBilder[strings.TrimSuffix(filepath.Base(spec), ".paper.json")]; ok {
		i := 1
		if strings.Contains(filepath.ToSlash(spec), "/basis/") {
			i = 0
		}
		raw, err := os.ReadFile(filepath.Join(repoRoot, "tools/paper/fixtures/render", bilder[i]))
		if err != nil {
			return FrameAntwort{}, err
		}
		var fx struct{ Teile map[string][]int }
		if err := json.Unmarshal(lies(nil, fixture("ui/AssetsFrame.json")), &fx); err != nil {
			return FrameAntwort{}, err
		}
		return FrameAntwort{Datei: out, Teile: fx.Teile}, os.WriteFile(out, raw, 0o644)
	}
	return f.CLI.Frame(ctx, spec, animation, frame, variante, out)
}

func cli() CLI { return CLI{Root: repoRoot} }

// TestCLIValidateUngueltig: banner mit fremdem Material wird abgelehnt.
func TestCLIValidateUngueltig(t *testing.T) {
	langsam(t)
	t.Parallel()
	repo := testRepo(t)
	fehler, err := cli().Validate(context.Background(), filepath.Join(repo, "public/grafik/requisiten/banner.paper.json"))
	if err != nil {
		t.Fatal(err)
	}
	want := []Fehler{{Feld: "teile[0].lagen[0].material", Regel: "material-unbekannt", Text: "Material „scharlach“ fehlt in der Stil-Datei"}}
	if !reflect.DeepEqual(fehler, want) {
		t.Fatalf("fehler = %+v", fehler)
	}
	ok, err := cli().Validate(context.Background(), filepath.Join(repo, "public/grafik/figuren/monarch.paper.json"))
	if err != nil || len(ok) != 0 {
		t.Fatalf("monarch: %v %+v", err, ok)
	}
}

// TestCLIDiffUmhang: Feld-Diff des Umhangs, kein Teil geändert.
func TestCLIDiffUmhang(t *testing.T) {
	langsam(t)
	t.Parallel()
	repo := testRepo(t)
	alt := filepath.Join(t.TempDir(), "monarch.paper.json")
	raw, _, err := Show(context.Background(), repo, "HEAD", "public/grafik/figuren/monarch.paper.json")
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(alt, raw, 0o644); err != nil {
		t.Fatal(err)
	}
	d, err := cli().Diff(context.Background(), alt, filepath.Join(repo, "public/grafik/figuren/monarch.paper.json"), "", "")
	if err != nil {
		t.Fatal(err)
	}
	want := []FeldAenderung{{Pfad: "umhang.material", Alt: "koenigsrot", Neu: "purpur"}}
	if !reflect.DeepEqual(d.Felder, want) || len(d.Teile) != 0 || d.Stil {
		t.Fatalf("diff = %+v", d)
	}
}

// TestCLISystemfehler: Exit 2 wird ein Go-Fehler mit stderr.
func TestCLISystemfehler(t *testing.T) {
	langsam(t)
	t.Parallel()
	_, err := cli().Validate(context.Background(), filepath.Join(t.TempDir(), "fehlt.paper.json"))
	var abgelehnt *Abgelehnt
	if err == nil || errors.As(err, &abgelehnt) || !strings.Contains(err.Error(), "Spec fehlt") {
		t.Fatalf("err = %v", err)
	}
}
