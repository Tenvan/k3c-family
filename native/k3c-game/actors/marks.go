package actors

// Anzeigen an den Figuren (NG12, B-806): Lebensbalken, Beute-Gold der Gegner, Last der Bauern und der Punkt unter
// dem Monarchen. Spiegel von src/scenes/render/worldRenderer.ts (bar, createTroop, createEnemy, createPlayer).
// Alles hier ist reine Rechnung in Spiel-Pixeln des Browsers (32 je Unit) relativ zum Fußpunkt der Figur, x nach
// rechts, y nach oben; markdraw.go zeichnet sie.

import (
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// barHeightPx ist die Höhe jedes Lebensbalkens (worldRenderer.ts › bar).
const barHeightPx = 8

// barLook ist Breite, Abstand der Balken-Mitte über der Figuren-Oberkante und Füllfarbe eines Balkens.
type barLook struct {
	width, above float64
	color        uint32
}

// Balken je Figurenart (createTroop, createEnemy, createPlayer).
var (
	troopBar  = barLook{36, 16, 0x52b788}
	enemyBar  = barLook{44, 32, 0xe63946}
	playerBar = barLook{50, 50, 0x52b788}
)

// Farben und Maße der übrigen Anzeigen.
const (
	barBack      = 0x000000 // Balken-Hintergrund, Deckkraft barBackAlpha
	barBackAlpha = 0.6

	lootRadius = 10.0
	lootFill   = 0xffd166
	lootEdge   = 0x9c6644
	lootAbove  = 12.0 // Mitte des Goldkreises über der Oberkante

	loadW, loadH = 16.0, 24.0
	loadFill     = 0x6b4226
	loadEdge     = 0x3b2414
	loadHeight   = 0.55 // Mitte der Last in Anteilen der Figurenhöhe
	loadSide     = 14.0 // seitlicher Versatz, hinter dem Rücken
	edgeWidth    = 2.0  // Randbreite von Goldkreis und Last

	markerW, markerH = 56.0, 12.0
	markerDrop       = 4.0 // der Punkt sitzt so weit unter dem Boden
	markerAlpha      = 0.8
)

// playerColors sind die Farben der Spieler (src/core/constants.ts › PLAYER_COLORS).
var playerColors = [...]uint32{0x3a86ff, 0xff006e, 0xfb5607, 0x8338ec}

// PlayerColor liefert die Farbe des Monarchen mit Spieler-Index i (zyklisch).
func PlayerColor(i int) uint32 {
	n := len(playerColors)
	return playerColors[(i%n+n)%n]
}

// BarVisible: Der Balken erscheint nur bei verletzter Figur (hp < maxHp); ohne Lebenswert (maxHp ≤ 0) nie.
func BarVisible(hp, maxHP float64) bool { return maxHP > 0 && hp < maxHP }

// PlayerBarVisible: Monarchen zeigen den Balken nur lebendig (respawnIn ≤ 0).
func PlayerBarVisible(hp, maxHP, respawnIn float64) bool {
	return BarVisible(hp, maxHP) && respawnIn <= 0
}

// BarFraction ist der gefüllte Anteil 0 … 1 (Phaser.Math.Clamp(hp / maxHp)).
func BarFraction(hp, maxHP float64) float64 {
	if maxHP <= 0 {
		return 0
	}
	return min(max(hp/maxHP, 0), 1)
}

// HasLoot: Der Gegner trägt geklautes Gold (carriedGold > 0) und zeigt den Goldkreis.
func HasLoot(carriedGold int) bool { return carriedGold > 0 }

// Carrying: Der Bauer trägt etwas zum Lager (Auftrag carry) und zeigt seine Last.
func Carrying(j *sim.Job) bool { return j != nil && j.Type == "carry" }

// LoadOffset ist der seitliche Versatz der Last in px: hinter dem Rücken, also bei Blick nach links rechts davon.
func LoadOffset(left bool) float64 {
	if left {
		return loadSide
	}
	return -loadSide
}

// Seated: Der Monarch mit Spieler-Index monarch gehört einem lokalen Spieler (Frame.Seats); freie Monarchen
// (niemand steuert sie) bleiben unsichtbar (worldRenderer.ts › sync(hidden), GameScene.ts).
func Seated(seats []view.Seat, monarch int) bool {
	for _, s := range seats {
		if s.Monarch == monarch {
			return true
		}
	}
	return false
}

// box ist ein Rechteck um die Mitte (cx, cy) in px, y nach oben.
type box struct{ cx, cy, w, h float64 }

// barBoxes liefert Hintergrund und Füllung eines Balkens über einer Figur mit Oberkante topPx und gefülltem Anteil
// frac; die Füllung wächst von der linken Kante des Hintergrunds.
func barBoxes(l barLook, topPx, frac float64) (back, fill box) {
	cy := topPx + l.above
	return box{0, cy, l.width, barHeightPx}, box{-l.width / 2 * (1 - frac), cy, l.width * frac, barHeightPx}
}

// lootCenter ist die Mitte des Goldkreises über einer Figur mit Oberkante topPx.
func lootCenter(topPx float64) (cx, cy float64) { return 0, topPx + lootAbove }

// loadBox ist das Rechteck der Last einer Figur der Höhe topPx, die nach links (left) schaut.
func loadBox(topPx float64, left bool) box {
	return box{LoadOffset(left), topPx * loadHeight, loadW, loadH}
}

// markerBox ist die Ellipse des Spieler-Punkts unter den Füßen.
func markerBox() box { return box{0, -markerDrop, markerW, markerH} }
