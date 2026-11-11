package view

import "image"

// WorldShare ist der Anteil der Welt an der Zellenhöhe (obere zwei Drittel, Boden-Linie an der Grenze); der Rest ist
// der Fluss (wie NV3, native/k3c-ebiten).
const WorldShare = 2.0 / 3.0

// Camera bildet Welt-x (Units) auf Fenster-x (px) einer Zelle ab. Reine Rechnung, Werte setzt game je Bild.
type Camera struct {
	X      float64 // Welt-x in Units in der Mitte der Zelle (folgt dem Monarchen des Spielers)
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
type Cell struct {
	Index  int             // 0 … Zahl der Zellen − 1; -1 = WindowCell
	Player int             // lokaler Spieler (Seat.Player) dieser Zelle; -1 = keiner (Fenster)
	Rect   image.Rectangle // Lage im Fenster in px
	Camera Camera
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
