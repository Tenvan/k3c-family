package props

import "k3c/engine/sim"

// Fontäne der Truhen (B-953): Münzen fliegen von der Truhe im Bogen zum Zielpunkt und liegen danach am Boden. Die Bahn
// rechnet die Anzeige aus den Feldern der Münze (FromX, LaunchAt, LandsAt); die Simulation entscheidet allein, wann sie
// aufsammelbar ist.

// fountainHeight ist die Scheitelhöhe des Bogens in Spiel-Pixeln.
const fountainHeight = 96

// coinPose liefert Ort und Höhe (Spiel-Pixel über dem Boden) der Münze zum Zeitpunkt now; shown ist false, solange die
// Münze noch in der Truhe liegt (vor LaunchAt). Eine Münze ohne Fontäne liegt bei X.
func coinPose(c *sim.Coin, now float64) (x, lift float64, shown bool) {
	if c.LandsAt <= c.LaunchAt || now >= c.LandsAt {
		return c.X, 0, true
	}
	if now < c.LaunchAt {
		return c.FromX, 0, false
	}
	u := (now - c.LaunchAt) / (c.LandsAt - c.LaunchAt)
	return c.FromX + (c.X-c.FromX)*u, 4 * fountainHeight * u * (1 - u), true
}
