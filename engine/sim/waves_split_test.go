package sim

import (
	"math"
	"testing"
)

// B-650: Boss-HP und Wellengröße hängen an getrennten Feldern; je Feld ein eigener Wert.
func TestBossHpUndWellenSindGetrennt(t *testing.T) {
	old := waves
	t.Cleanup(func() { waves = old })
	boss, wave := 0.1, 0.5
	waves.BossHpPerExtraPlayer, waves.PerExtraPlayer = &boss, wave

	one, three := bossHpOf(t, 1), bossHpOf(t, 3)
	if want := one * (1 + 2*boss); math.Abs(three-want) > 1 {
		t.Errorf("Boss-HP 3 Spieler %.0f, erwartet %.0f (bossHpPerExtraPlayer %.1f, nicht perExtraPlayer)", three, want, boss)
	}
	base := waveCount(islandWith(t, "opt-a", "normal", 1), 1)
	want := int(math.Round(float64(base) * (1 + wave*2)))
	if got := waveCount(islandWith(t, "opt-a", "normal", 3), 1); got != want {
		t.Errorf("Welle 3 Spieler: %d Gegner, erwartet %d (perExtraPlayer %.1f, nicht bossHpPerExtraPlayer)", got, want, wave)
	}
}

func bossHpOf(t *testing.T, players int) float64 {
	t.Helper()
	isl := bossIsland(t, players)
	w := isl.Stages[0]
	return spawnBoss(w, miniBoss(0), w.Portals[0]).MaxHP
}

func TestCheckWaves(t *testing.T) {
	neg, ok := -0.1, 0.25
	for name, w := range map[string]wavesData{
		"fehlt":            {PerExtraPlayer: 0.25},
		"negativ":          {PerExtraPlayer: 0.25, BossHpPerExtraPlayer: &neg},
		"Wellenfaktor neg": {PerExtraPlayer: -1, BossHpPerExtraPlayer: &ok},
	} {
		if checkWaves(w) == nil {
			t.Errorf("%s: Fehler erwartet", name)
		}
	}
	if err := checkWaves(wavesData{PerExtraPlayer: 0.25, BossHpPerExtraPlayer: &ok}); err != nil {
		t.Errorf("gültig: %v", err)
	}
}
