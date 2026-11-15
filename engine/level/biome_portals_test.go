package level

import "testing"

func TestPortalWerteSindPflicht(t *testing.T) {
	var b Biome
	if b.checkPortals() == nil {
		t.Error("Biom ohne Portal-HP geladen")
	}
	b.Portals.HP = 100
	if b.checkPortals() == nil {
		t.Error("Biom ohne Verteidiger-Takt geladen")
	}
	b.Portals.Defense.IntervalSeconds = 20
	if err := b.checkPortals(); err != nil {
		t.Error(err)
	}
	for _, id := range []string{"forest", "cave", "mine", "ironhold", "crystal"} {
		if _, err := LoadBiome(id); err != nil {
			t.Error(err)
		}
	}
}
