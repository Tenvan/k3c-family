package main

import (
	"reflect"
	"strings"
	"testing"
)

// hgFein ist eine kleine Hintergrund-Schicht im feinen Raster (1/128 Kachel): Boden, ein Stamm und ein Detail von 1/128 Kachel.
func hgFein() building {
	return building{
		ID: "hg-probe-fern-v2", Klasse: klasseHintergrund, Raster: rasterHintergrund,
		Vorlage: &vorlage{Motiv: "Probe", Teile: []string{"boden"}},
		Kacheln: kacheln{B: 2, H: 1},
		Bausteine: []baustein{
			{Art: "boden", Rechteck: [4]float64{0, 0.75, 2, 0.25}, Farbe: "dell"},
			{Art: "stamm", Rechteck: [4]float64{0.5, 0.25, 0.25, 0.5}, Farbe: "rope"},
			{Art: "glanz", Rechteck: [4]float64{0.5078125, 0.2578125, 0.0078125, 0.0078125}, Farbe: "white", Detail: true},
		},
	}
}

// TestRasterFein: im feinen Raster rendert Dichte 128 jedes Raster-Pixel 1:1 (vierfache Auflösung), Dichte 32 ohne Details (B-618/AC-01).
func TestRasterFein(t *testing.T) {
	b := hgFein()
	if err := pruefeRaster(b); err != nil {
		t.Fatal(err)
	}
	if got := dichtenFuer(b.Klasse, rasterVon(b)); !reflect.DeepEqual(got, []int{32, 128}) {
		t.Fatalf("Dichten %v, erwartet [32 128]", got)
	}
	hi, err := render(b, 128)
	if err != nil {
		t.Fatal(err)
	}
	lo, err := render(b, 32)
	if err != nil {
		t.Fatal(err)
	}
	if hi.Bounds().Dx() != 4*lo.Bounds().Dx() || hi.Bounds().Dx() != 256 {
		t.Fatalf("Breite 128: %d, 32: %d; erwartet 256 und ein Viertel davon", hi.Bounds().Dx(), lo.Bounds().Dx())
	}
	_, idx := palette()
	if hi.ColorIndexAt(65, 33) != idx["white"] {
		t.Error("Detail fehlt in Dichte 128")
	}
	if lo.ColorIndexAt(16, 8) != idx["rope"] {
		t.Error("Dichte 32: an der Stelle des Details muss der Stamm stehen (Detail entfällt)")
	}
}

// TestRasterRegeln: 1/128-Werte nur im feinen Raster, feines Raster nur für Hintergründe, detail nur im feinen Raster,
// dünne Bausteine ohne detail scheitern in Dichte 32.
func TestRasterRegeln(t *testing.T) {
	b := hgFein()
	b.Bausteine[2].Detail = false
	if _, err := render(b, 32); err == nil || !strings.Contains(err.Error(), "detail") {
		t.Errorf("dünner Baustein ohne detail in Dichte 32: %v", err)
	}
	grob := hgFein()
	grob.Raster = 0
	if err := pruefeRaster(grob); err == nil {
		t.Error("detail ohne raster 128 muss scheitern")
	}
	grob.Bausteine = grob.Bausteine[:2]
	if _, err := render(grob, 32); err != nil {
		t.Fatalf("1/4-Kachel-Werte im Master-Raster: %v", err)
	}
	grob.Bausteine[1].Rechteck[0] = 0.5078125
	if _, err := render(grob, 32); err == nil {
		t.Error("1/128-Wert im Master-Raster muss scheitern")
	}
	haus := hgFein()
	haus.Klasse = "haus"
	if err := pruefeRaster(haus); err == nil {
		t.Error("raster 128 nur für Hintergründe")
	}
	schief := hgFein()
	schief.Raster = 64
	if err := pruefeRaster(schief); err == nil {
		t.Error("raster 64 nicht erlaubt")
	}
	if got := dichtenFuer(klasseHintergrund, master); !reflect.DeepEqual(got, []int{16, 32}) {
		t.Errorf("Hintergrund im Master-Raster: Dichten %v, erwartet [16 32] wie bisher", got)
	}
}

// TestWiederholungFein: der Schritt einer Wiederholung darf im feinen Raster 1/128 Kachel sein.
func TestWiederholungFein(t *testing.T) {
	s := []baustein{{Art: "gras", Rechteck: [4]float64{0, 0, 0.0078125, 0.0078125}, Farbe: "dell", Wiederholung: &wiederholung{Schritt: 0.0234375, Anzahl: 4}}}
	if _, err := aufloesen("probe", s, rasterHintergrund); err != nil {
		t.Fatal(err)
	}
	if _, err := aufloesen("probe", s, master); err == nil {
		t.Error("1/128-Schritt im Master-Raster muss scheitern")
	}
}

// TestHintergrundVarianten: IDs mit `-v<n>` gehören zur Variante n, ohne Endung zur Variante 1; vollständig = fern, mitte, nah.
func TestHintergrundVarianten(t *testing.T) {
	if biom, schicht, v, ok := hintergrundTeile("hg-wald-mitte-v3"); !ok || biom != "wald" || schicht != "mitte" || v != 3 {
		t.Errorf("hg-wald-mitte-v3 → %s %s %d %v", biom, schicht, v, ok)
	}
	if _, _, v, ok := hintergrundTeile("hg-wald-nah"); !ok || v != 1 {
		t.Errorf("ohne Endung: Variante %d, ok %v", v, ok)
	}
	for _, id := range []string{"hg-wald-oben", "hg-wald-nah-v0", "wall-1", "hg-Wald-nah"} {
		if _, _, _, ok := hintergrundTeile(id); ok {
			t.Errorf("%s darf keine Hintergrund-ID sein", id)
		}
	}
	ids := []string{"hg-wald-fern", "hg-wald-mitte", "hg-wald-nah", "hg-wald-fern-v2", "hg-wald-mitte-v2", "hg-wald-nah-v2", "hg-mine-fern-v1", "castle"}
	want := []string{"mine: 0 von 2 Varianten vollständig"}
	if got := fehlendeVarianten(ids, 2); !reflect.DeepEqual(got, want) {
		t.Errorf("fehlende Varianten %v, erwartet %v", got, want)
	}
	if got := fehlendeVarianten(ids[:6], 2); len(got) != 0 {
		t.Errorf("Wald hat zwei vollständige Varianten, gemeldet: %v", got)
	}
}
