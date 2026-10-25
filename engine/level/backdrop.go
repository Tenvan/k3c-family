package level

import (
	"math"

	"k3c/engine/rng"
)

// BackdropTileUnits ist die Breite einer Hintergrund-Kachel in Units (Regeldatei § 10: Schichten 24 × 15 Kacheln).
const BackdropTileUnits = 24.0

// backdrops legt je Hintergrund-Kachel eine Variante 1 … n fest (GEB18, B-618): deterministisch nach Biom und Seed aus dem
// eigenen Strom `<biom>:<seed>:backdrop`, damit der Level-Strom und alle bisherigen Level unverändert bleiben. Benachbarte
// Kacheln haben verschiedene Varianten (bei n ≥ 2). Die Folge reicht über die ganze Weltbreite plus eine Kachel; der Client
// nimmt für Kachel k den Eintrag k (Schichten mit Scroll-Faktor < 1 brauchen weniger). n ≤ 0 = keine Varianten.
func backdrops(biomeID, seed string, n int, widthUnits float64) []int {
	if n <= 0 {
		return nil
	}
	r := rng.New(biomeID + ":" + seed + ":backdrop")
	out := make([]int, int(math.Ceil(widthUnits/BackdropTileUnits))+1)
	for i := range out {
		if i == 0 || n == 1 {
			out[i] = r.Int(1, n)
			continue
		}
		v := r.Int(1, n-1) // eine der n-1 Varianten außer der vorigen
		if v >= out[i-1] {
			v++
		}
		out[i] = v
	}
	return out
}
