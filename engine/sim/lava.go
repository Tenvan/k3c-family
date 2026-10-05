package sim

import "math"

// Lava (B-115, Beschluss Q28): Spieler, Truppen und Bauern auf einem Lava-Streifen nehmen Schaden je Sekunde
// (data/biomes/*.json › lava). Der Streifen liegt mitten im Chunk der Art „lava“. Gegner nehmen keinen Schaden
// (Startwert, bestätigt der Balancing-Workshop BR1).

// onLava: Liegt x auf einem Lava-Streifen der Stufe?
func onLava(w *World, x float64) bool {
	half := w.Biome.Lava.WidthUnits / 2
	for _, c := range w.Level.Chunks {
		if c.Kind == "lava" && math.Abs(x-(c.StartUnits+w.Level.ChunkWidthUnits/2)) <= half {
			return true
		}
	}
	return false
}

// stepLava zieht allen Figuren auf Lava einmal je Sekunde den Schaden einer Sekunde ab (über applyDamage: Tod,
// playerDown, Treffer). Je Sekunde statt je Tick, weil ein Treffer am Spieler mindestens 1 Schaden macht.
func stepLava(w *World, dt float64) {
	dps := w.Biome.Lava.DamagePerSecond
	if dps <= 0 {
		return
	}
	w.lavaIn -= dt
	if w.lavaIn > 0 {
		return
	}
	w.lavaIn++
	var ids []int
	for _, p := range w.Players {
		if isAlive(p) && !p.Free && onLava(w, p.X) {
			ids = append(ids, p.ID)
		}
	}
	for _, t := range w.Troops {
		if t.HP > 0 && onLava(w, t.X) {
			ids = append(ids, t.ID)
		}
	}
	for _, id := range ids {
		applyDamage(w, id, dps)
	}
}
