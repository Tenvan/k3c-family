package sim

import "math"

// Gold-Fontäne und Gold aus Bäumen und Gegnern (docs/rules/wirtschaft.md § 9, B-952, B-953):
//   - Eine Truhe wirft ihr Gold als Münzen im Bogen auf den Boden (fountainCoins): jede Münze startet gestaffelt (LaunchAt) an der
//     Truhe, fliegt zu einem Zielpunkt (±spreadUnits) und liegt ab LandsAt aufsammelbar; Zeiten und Ziele kommen aus engine/rng, deterministisch.
//   - Ein gefällter Baum wirft treeGold Münzen (unit_common.go › gather), das Gold der Gegner skaliert enemyGoldFactor.

// fountainCoins lässt count Münzen aus der Truhe bei x als Fontäne auf den Boden fallen.
func fountainCoins(w *World, x float64, count int) {
	f := w.values().Economy.ChestFountain
	for i := range count {
		to := math.Min(w.WidthUnits, math.Max(0, x+(w.rng.Next()*2-1)*f.SpreadUnits))
		launch := w.Time + float64(i)*f.StaggerSeconds
		flight := f.FlightSeconds * (0.8 + 0.4*w.rng.Next())
		w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: to, FromX: x, LaunchAt: launch, LandsAt: launch + flight})
	}
}

// scaledGold skaliert das Gold eines Gegners mit economy.enemyGoldFactor (gerundet; Faktor 0 = unverändert).
func scaledGold(w *World, gold int) int {
	f := w.values().Economy.EnemyGoldFactor
	if f <= 0 {
		return gold
	}
	return int(math.Round(float64(gold) * f))
}
