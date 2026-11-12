package view

import "image"

// WorldShare ist der Anteil der Welt an der Zellenhöhe (obere zwei Drittel, Boden-Linie an der Grenze); der Rest ist
// der Fluss (wie NV3, native/k3c-ebiten).
const WorldShare = 2.0 / 3.0

// Camera bildet Welt-x (Units) auf Fenster-x (px) einer Zelle ab. Reine Rechnung; X setzt game je Update über die
// CameraRule, Scale, Center und Half folgen aus der Lage der Zelle.
type Camera struct {
	X      float64 // Welt-x in Units in der Mitte der Zelle (CameraRule bzw. --cam)
	Scale  float64 // Bildpunkte je Unit
	Center float64 // Fenster-x der Zellenmitte in px
	Half   float64 // halbe Zellenbreite in px
}

// NewCamera legt die Kamera mit Welt-x x und Maßstab scale auf das Rechteck r.
func NewCamera(r image.Rectangle, x, scale float64) Camera {
	half := float64(r.Dx()) / 2
	return Camera{X: x, Scale: scale, Center: float64(r.Min.X) + half, Half: half}
}

// ScreenX liefert das Fenster-x des Welt-x wx (Boden-Ebene, Parallaxe 1).
func (k Camera) ScreenX(wx float64) float64 { return (wx-k.X)*k.Scale + k.Center }

// ParallaxX liefert das Fenster-x des Welt-x wx auf einer Ebene mit Scroll-Faktor factor (0 = steht, 1 = Boden).
func (k Camera) ParallaxX(wx, factor float64) float64 { return (wx-k.X*factor)*k.Scale + k.Center }

// WorldX ist die Umkehrung von ScreenX.
func (k Camera) WorldX(sx float64) float64 {
	if k.Scale == 0 {
		return k.X
	}
	return (sx-k.Center)/k.Scale + k.X
}

// Span liefert den sichtbaren Welt-Bereich in Units (links, rechts).
func (k Camera) Span() (left, right float64) {
	if k.Scale == 0 {
		return k.X, k.X
	}
	w := k.Half / k.Scale
	return k.X - w, k.X + w
}

// Visible meldet, ob ein Objekt bei Welt-x x mit halber Breite half (Units) in die Zelle ragt.
func (k Camera) Visible(x, half float64) bool {
	l, r := k.Span()
	return x+half >= l && x-half <= r
}

// Cell ist eine Zelle des Split-Screens: Rechteck im Fenster, Spieler und Kamera. Welt in den oberen zwei Dritteln,
// Fluss im unteren Drittel.
// Zelle i zeigt Frame.Seats[i]; Index und Player sind verschieden, sobald Spieler 0 fehlt.
type Cell struct {
	Index  int             // Lage: 0 = oben, 1 = unten (Zahl der Zellen = len(Frame.Seats)); -1 = WindowCell
	Player int             // lokaler Spieler (Seat.Player) dieser Zelle, nicht Index; -1 = keiner (Fenster)
	Rect   image.Rectangle // Lage im Fenster in px
	Camera Camera
}

// Anzeige oben rechts (game: FPS bzw. im Aufnahme-Modus der Takt): Debug-Schrift statsW × statsH px, statsScale-fach,
// statsMargin px vom Rand.
const (
	statsW      = 76
	statsH      = 18
	statsScale  = 5
	statsMargin = 16
)

// StatsRect ist die Fläche der Anzeige oben rechts im Fenster win; game zeichnet dort, die Drawer halten sie frei.
// Sie liegt in der obersten Zelle (gleiche Oberkante und Breite wie das Fenster).
func StatsRect(win image.Rectangle) image.Rectangle {
	w, h := statsW*statsScale, statsH*statsScale
	x := win.Max.X - w - statsMargin
	return image.Rect(x, win.Min.Y+statsMargin, x+w, win.Min.Y+statsMargin+h)
}

// StatsTextSize ist die Größe des Textbilds der Anzeige vor der Vergrößerung (Debug-Schrift) und der Faktor.
func StatsTextSize() (w, h, scale int) { return statsW, statsH, statsScale }

// statsLine ist die Höhe einer weiteren Textzeile der Debug-Schrift in px.
const statsLine = 16

// StatsRectLines ist StatsRect mit lines Textzeilen (Leistungsanzeige, Settings.Stats: zwei Zeilen); die weiteren Zeilen
// reichen nach unten über StatsRect hinaus. lines < 1 zählt als 1.
func StatsRectLines(win image.Rectangle, lines int) image.Rectangle {
	r := StatsRect(win)
	r.Max.Y += (max(lines, 1) - 1) * statsLine * statsScale
	return r
}

// StatsTextSizeLines ist StatsTextSize für lines Textzeilen.
func StatsTextSizeLines(lines int) (w, h, scale int) {
	w, h, scale = StatsTextSize()
	return w, h + (max(lines, 1)-1)*statsLine, scale
}

// WindowCell ist die Zelle für LayerOverlay und Screens: das ganze Fenster, ohne Spieler.
func WindowCell(win image.Rectangle) Cell {
	return Cell{Index: -1, Player: -1, Rect: win, Camera: NewCamera(win, 0, 1)}
}

// GroundY ist das Fenster-y der Boden-Linie (Grenze Welt/Fluss).
func (c Cell) GroundY() int {
	return c.Rect.Min.Y + int(float64(c.Rect.Dy())*WorldShare+0.5)
}

// WorldRect ist der Welt-Teil der Zelle (obere zwei Drittel).
func (c Cell) WorldRect() image.Rectangle {
	return image.Rect(c.Rect.Min.X, c.Rect.Min.Y, c.Rect.Max.X, c.GroundY())
}

// RiverRect ist der Fluss-Teil der Zelle (unteres Drittel).
func (c Cell) RiverRect() image.Rectangle {
	return image.Rect(c.Rect.Min.X, c.GroundY(), c.Rect.Max.X, c.Rect.Max.Y)
}

// ScreenY liefert das Fenster-y einer Höhe h in Units über dem Boden.
func (c Cell) ScreenY(h float64) float64 { return float64(c.GroundY()) - h*c.Camera.Scale }
