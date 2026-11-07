package assetsui

import (
	"context"
	"reflect"
	"slices"
	"sync/atomic"
	"testing"

	"k3c/tools/k3c-workbench/internal/review"
)

// zaehlCLI zählt die Aufrufe von Validate (B-732).
type zaehlCLI struct {
	review.CLI
	n *atomic.Int32
}

func (z zaehlCLI) Validate(ctx context.Context, spec string) ([]review.Fehler, error) {
	z.n.Add(1)
	return z.CLI.Validate(ctx, spec)
}

// TestAssetsIndexOhnePruefung: der Index ruft keinen Validate; AssetsCheck aller Pfade ergibt AssetsList (B-732/AC-02).
func TestAssetsIndexOhnePruefung(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	var n atomic.Int32
	d := Dienst{Root: repo, R: zaehlCLI{CLI: review.CLI{Root: root}, n: &n}}
	index, err := d.AssetsIndex()
	if err != nil {
		t.Fatal(err)
	}
	if n.Load() != 0 {
		t.Fatalf("AssetsIndex rief Validate %d-mal", n.Load())
	}
	pfade := make([]string, len(index))
	for i, e := range index {
		pfade[i] = e.Pfad
		if e.ID == "" || e.Gruppe == "" {
			t.Fatalf("Eintrag ohne ID oder Gruppe: %+v", e)
		}
	}
	got, err := d.AssetsCheck(pfade)
	if err != nil {
		t.Fatal(err)
	}
	want, err := d.AssetsList()
	if err != nil {
		t.Fatal(err)
	}
	// AssetsCheck ermittelt `geaendert` nicht, das liefert der Index
	for i := range got {
		if got[i].Geaendert {
			t.Fatalf("AssetsCheck meldet geaendert: %+v", got[i])
		}
		got[i].Geaendert = index[slices.IndexFunc(index, func(e AssetEintrag) bool { return e.Pfad == got[i].Pfad })].Geaendert
	}
	if g, w := nachID(roundtrip(t, got)), nachID(roundtrip(t, want)); !reflect.DeepEqual(g, w) {
		t.Fatalf("AssetsCheck = %v\nwill %v", g, w)
	}
	// nach der Prüfung liefert der Index das Ergebnis aus dem Memo, nichts bleibt offen
	again, err := d.AssetsIndex()
	if err != nil {
		t.Fatal(err)
	}
	if g, w := nachID(roundtrip(t, again)), nachID(roundtrip(t, want)); !reflect.DeepEqual(g, w) {
		t.Fatalf("AssetsIndex nach Prüfung = %v\nwill %v", g, w)
	}
}

// TestAssetsCheckFremderPfad: ein Pfad außerhalb des Arbeitsstands wird nicht geprüft, sein Eintrag meldet `fehlt`.
func TestAssetsCheckFremderPfad(t *testing.T) {
	t.Parallel()
	var n atomic.Int32
	d := Dienst{Root: testRepo(t), R: zaehlCLI{CLI: review.CLI{Root: root}, n: &n}}
	got, err := d.AssetsCheck([]string{"../../etc/passwd.paper.json"})
	if err != nil {
		t.Fatal(err)
	}
	if n.Load() != 0 || len(got) != 1 || got[0].Gueltig || got[0].Fehler[0].Regel != "fehlt" {
		t.Fatalf("AssetsCheck = %+v, Validate %d-mal", got, n.Load())
	}
}
