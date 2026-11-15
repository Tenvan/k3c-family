package view

// Zustände einer gefallenen Krone (sim.Crown.State). Die Engine exportiert sie nicht; die Werte sind Spiegel von
// engine/sim/crown.go (flying, ground, enemy, player) und stehen hier einmal für alle Zeichner (props, actors).
const (
	CrownFlying = "flying" // fliegt im Bogen
	CrownGround = "ground" // liegt am Boden
	CrownEnemy  = "enemy"  // ein Gegner trägt sie zum Portal
	CrownPlayer = "player" // ein Mitspieler trägt sie zum Besitzer
)

// CrownParts ist die Form der Krone in Spiel-Pixeln (32 je Unit), gemeinsam für props (am Boden, im Flug) und actors (am Kopf):
// je Teil Versatz (dx, dy) der Mitte von der Mitte des Bands und Größe (w, h); Band zuerst, dann drei Zacken.
var CrownParts = [4][4]float64{{0, 0, 20, 7}, {-7, 6, 5, 7}, {0, 7, 5, 9}, {7, 6, 5, 7}}
