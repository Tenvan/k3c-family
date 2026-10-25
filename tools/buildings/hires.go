package main

import (
	"fmt"
	"regexp"
	"sort"
	"strconv"
)

// rasterHintergrund ist das feine Raster der Hintergrund-Schichten in vierfacher Auflösung (GEB18, B-618): Koordinaten in
// 1/128 Kachel, gerendert in Dichte 128 (jedes Raster-Pixel ein Bild-Pixel) und 32 (Spiel-Dichte, Details entfallen).
const rasterHintergrund = 128

// rasterVon liefert das Koordinaten-Raster des Gebäudes in Teilen je Kachel; ohne Angabe das Master-Raster 1/32.
func rasterVon(b building) int {
	if b.Raster == 0 {
		return master
	}
	return b.Raster
}

// pruefeRaster: `raster` ist 32 oder 128, 128 nur für Hintergrund-Schichten; `detail`-Bausteine nur im feinen Raster.
func pruefeRaster(b building) error {
	r := rasterVon(b)
	switch {
	case r != master && r != rasterHintergrund:
		return fmt.Errorf("raster %d nicht erlaubt (%d oder %d)", r, master, rasterHintergrund)
	case r == rasterHintergrund && b.Klasse != klasseHintergrund:
		return fmt.Errorf("raster %d nur für Klasse %q", r, klasseHintergrund)
	}
	if r == master {
		for i, s := range b.Bausteine {
			if s.Detail {
				return fmt.Errorf("baustein %d (%s): detail nur mit raster %d", i, s.Art, rasterHintergrund)
			}
		}
	}
	return nil
}

// entfaellt: `fein` fehlt unter der Spiel-Dichte (32); `detail` fehlt, sobald es in Dichte d schmaler oder niedriger als
// 1 px wäre (in 128 immer da, in 64 ab 2 Raster-Pixeln, in 32 ab 4).
func entfaellt(s baustein, d, raster int) bool {
	if d < master && s.Fein {
		return true
	}
	return s.Detail && d < raster && zuDuenn(s, d)
}

// zuDuenn: Breite oder Höhe des Bausteins ergibt in Dichte d weniger als 1 px.
func zuDuenn(s baustein, d int) bool {
	return s.Rechteck[2]*float64(d) < 1 || s.Rechteck[3]*float64(d) < 1
}

// hintergrundID: `hg-<biom>-<fern|mitte|nah>` mit optionaler Variante `-v<n>`; ohne Endung ist es die bisherige Fassung (Variante 0, GEB7).
var hintergrundID = regexp.MustCompile(`^hg-([a-z]+)-(fern|mitte|nah)(?:-v([1-9][0-9]*))?$`)

// hintergrundTeile zerlegt eine Hintergrund-ID in Biom, Schicht und Variante.
func hintergrundTeile(id string) (biom, schicht string, variante int, ok bool) {
	m := hintergrundID.FindStringSubmatch(id)
	if m == nil {
		return "", "", 0, false
	}
	variante = 0
	if m[3] != "" {
		variante, _ = strconv.Atoi(m[3])
	}
	return m[1], m[2], variante, true
}

// hintergrundGruppen ordnet Hintergrund-IDs nach Biom → Variante → Schicht → ID; andere IDs entfallen.
func hintergrundGruppen(ids []string) map[string]map[int]map[string]string {
	out := map[string]map[int]map[string]string{}
	for _, id := range ids {
		biom, schicht, v, ok := hintergrundTeile(id)
		if !ok {
			continue
		}
		if out[biom] == nil {
			out[biom] = map[int]map[string]string{}
		}
		if out[biom][v] == nil {
			out[biom][v] = map[string]string{}
		}
		out[biom][v][schicht] = id
	}
	return out
}

// fehlendeVarianten nennt je Biom, wie viele vollständige Varianten (fern, mitte und nah, ab `-v1`) fehlen, bis `min` erreicht ist;
// die bisherige Fassung ohne Endung zählt nicht; leer = jedes Biom hat genug (B-618/AC-02).
func fehlendeVarianten(ids []string, min int) []string {
	var out []string
	for biom, varianten := range hintergrundGruppen(ids) {
		voll := 0
		for v, s := range varianten {
			if v >= 1 && len(s) == 3 {
				voll++
			}
		}
		if voll < min {
			out = append(out, fmt.Sprintf("%s: %d von %d Varianten vollständig", biom, voll, min))
		}
	}
	sort.Strings(out)
	return out
}
