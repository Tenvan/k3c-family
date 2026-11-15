package world

import (
	"encoding/json"
	"strings"
	"testing"
)

// B-963/AC-02: etappe je Insel per Set schreiben (Feldreihenfolge id, name, etappe, depths), Lücke lehnt ab, Load liefert sie.
func TestSetEtappe(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "etappen", "Etappen", ""); err != nil {
		t.Fatal(err)
	}
	patch := `{"islands":[{"id":"island1","etappe":1,"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}},{"id":"island2","etappe":2}]}`
	if _, err := e.Set(ctx, "etappen", json.RawMessage(patch)); err != nil {
		t.Fatal(err)
	}
	text := readWorld(t, e, "etappen")
	for _, want := range []string{`"name": "Insel 1", "etappe": 1, "depths"`, `"name": "island2", "etappe": 2, "depths"`} {
		if !strings.Contains(text, want) {
			t.Fatalf("fehlt %q:\n%s", want, text)
		}
	}
	d := e.Load("etappen")
	if d.Error != "" || d.Islands[1].Etappe == nil || *d.Islands[1].Etappe != 2 {
		t.Fatalf("Load = %+v", d)
	}
	if info := d.BiomeInfo["forest"]; info.PortalHP <= 0 || info.Palette["sky"] == "" {
		t.Fatalf("BiomeInfo forest = %+v", info)
	}
	// Etappe ändern und wieder entfernen.
	if _, err := e.Set(ctx, "etappen", json.RawMessage(`{"islands":[{"id":"island2","etappe":null}]}`)); err != nil {
		t.Fatal(err)
	}
	if strings.Contains(readWorld(t, e, "etappen"), `"etappe": 2`) {
		t.Fatal("etappe nicht entfernt")
	}
	// Lücke: Fehler, Datei bleibt.
	before := readWorld(t, e, "etappen")
	_, err := e.Set(ctx, "etappen", json.RawMessage(`{"islands":[{"id":"island2","etappe":3}]}`))
	if err == nil || !strings.Contains(err.Error(), "etappe") {
		t.Fatalf("Lücke erwartet, Fehler = %v", err)
	}
	if readWorld(t, e, "etappen") != before {
		t.Fatal("Datei trotz Fehler geändert")
	}
}

func TestSetEtappeView(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "etappen", "Etappen", ""); err != nil {
		t.Fatal(err)
	}
	three := 3
	if v := e.SetEtappeView(ctx, "etappen", "island1", &three); v.Error == "" || !strings.Contains(v.Error, "etappe") {
		t.Fatalf("Fehler erwartet: %+v", v)
	}
	one := 1
	if v := e.SetEtappeView(ctx, "etappen", "island1", &one); v.Error != "" {
		t.Fatalf("Fehler: %+v", v)
	}
}

// Review B-963: unbekannte Insel legt keine Insel an; die Etappe ist eine ganze Zahl.
func TestSetEtappeViewOnlyExistingIslandAndInteger(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "etappen", "Etappen", ""); err != nil {
		t.Fatal(err)
	}
	before := readWorld(t, e, "etappen")
	one := 1
	if v := e.SetEtappeView(ctx, "etappen", "gibtsnicht", &one); v.Error == "" {
		t.Fatalf("unbekannte Insel angenommen: %+v", v)
	}
	if readWorld(t, e, "etappen") != before {
		t.Fatal("Datei trotz Fehler geändert")
	}
	if _, err := e.Set(ctx, "etappen", json.RawMessage(`{"islands":[{"id":"island1","etappe":1.5}]}`)); err == nil {
		t.Fatal("nicht ganze Zahl angenommen")
	}
	if readWorld(t, e, "etappen") != before {
		t.Fatal("Datei trotz Fehler geändert")
	}
}
