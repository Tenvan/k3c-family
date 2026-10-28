package level

import (
	"math"
	"strconv"
	"testing"
)

// TestLavaNichtAufLinien500Seeds: Kein Lava-Streifen (Chunk-Mitte ± widthUnits/2) liegt auf Mauer, Turm oder Tor
// einer Linie (B-291/AC-01); Biome ohne Lava bleiben unberührt.
func TestLavaNichtAufLinien500Seeds(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		if b.Lava.WidthUnits <= 0 {
			continue
		}
		lava := 0
		for seed := range 500 {
			l, err := Generate(b, strconv.Itoa(seed))
			if err != nil {
				t.Fatal(err)
			}
			for _, c := range l.Chunks {
				if c.Kind != "lava" {
					continue
				}
				lava++
				checkLavaAbseitsLinien(t, id, seed, b.Lava.WidthUnits, l, c)
			}
		}
		if lava == 0 {
			t.Fatalf("%s: kein Lava-Chunk in 500 Seeds", id)
		}
	}
}

// checkLavaAbseitsLinien prüft, dass ein Lava-Chunk keinen Mauer- oder Turm-Platz trifft.
func checkLavaAbseitsLinien(t *testing.T, id string, seed int, width float64, l Layout, c Chunk) {
	t.Helper()
	mid := c.StartUnits + l.ChunkWidthUnits/2
	for _, site := range l.Sites {
		if math.Abs(site.X-mid) <= width/2 {
			t.Fatalf("%s Seed %d: Lava bei %.1f trifft Platz %s bei %.1f", id, seed, mid, site.Kind, site.X)
		}
	}
}
