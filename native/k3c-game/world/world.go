// Package world zeichnet je Zelle Himmel, Kulisse (fern, mitte, nah), Boden, Fluss und Licht und führt die Kamera
// (B-795). Rechnung (Kamera, Culling, Schichten, Fluss, Licht) liegt in reinen Funktionen ohne ebiten.Image; die
// Drawer sind dünn und legen je Bild nichts an.
package world

import (
	"image"
	"image/color"
	"os"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// World hält den gemeinsamen Zustand der Drawer: Daten (Paletten, Kulissen, Licht), Kamera-Zustand je Spieler und
// die wiederverwendeten Zeichen-Puffer.
type World struct {
	lib    *art.Library
	looks  looks
	light  lightData
	cams   cameras
	river  riverTile
	water  riverWater // Fluss mit Shader (water.go)
	start  riverMode  // Fassung ohne gewählte Einstellung (K3C_RIVER)
	mesh   lightMesh
	px     *ebiten.Image           // weißer Bildpunkt für Flächen (lazy, erst beim ersten Zeichnen)
	op     ebiten.DrawImageOptions // Sheets und Fluss: linear, ohne Mipmaps
	fillOp ebiten.DrawImageOptions // Flächen aus dem Bildpunkt
}

// New liefert die Drawer der Welt: fern (mit Himmel), mitte, nah, Boden, Fluss und Licht. lib (Grafik) darf nil sein,
// dann bleiben die Kulissen leer. Der ferne Drawer führt zugleich die Kamera (view.CameraRule) und meldet die
// Kulissen-Grafik je Level (view.AssetSource).
func New(lib *art.Library) []view.Drawer {
	w := &World{lib: lib, looks: loadLooks(), light: loadLight()}
	w.op.Filter = ebiten.FilterLinear
	w.op.DisableMipmaps = true
	w.start = riverModeOf(os.Getenv(riverEnv))
	w.water.init(w.start, riverKage)
	return []view.Drawer{
		far{w},
		layerDrawer{w, view.LayerMid, w.drawMid},
		layerDrawer{w, view.LayerNear, w.drawNear},
		layerDrawer{w, view.LayerGround, w.drawGround},
		layerDrawer{w, view.LayerRiver, w.drawRiver},
		layerDrawer{w, view.LayerRiver, w.drawLight},
	}
}

// far ist die ferne Ebene mit Himmel; sie trägt die Kamera-Regel und die Asset-Quelle der Welt.
type far struct{ w *World }

func (far) Layer() view.Layer { return view.LayerFar }

func (d far) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) { d.w.drawFar(dst, f, c) }

// Follow ist die Kamera-Regel (Totzone, Grenzen der Stufe, Sprung beim Stufenwechsel; camera.go).
func (d far) Follow(f *view.Frame, c view.Cell, prev view.Camera) view.Camera {
	return d.w.cams.follow(f, c, prev)
}

// Assets nennt die Kulissen-Grafik des Levels (alle Varianten der drei Schichten).
func (d far) Assets(level string, _ *sim.World) []string { return d.w.looks.assets(level) }

// layerDrawer zeichnet eine Ebene über eine Methode der Welt.
type layerDrawer struct {
	w     *World
	layer view.Layer
	draw  func(dst *ebiten.Image, f *view.Frame, c view.Cell)
}

func (d layerDrawer) Layer() view.Layer { return d.layer }

func (d layerDrawer) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) { d.draw(dst, f, c) }

// pixel liefert den weißen Bildpunkt für Flächen (einmal angelegt).
func (w *World) pixel() *ebiten.Image {
	if w.px == nil {
		w.px = ebiten.NewImage(1, 1)
		w.px.Fill(color.White)
	}
	return w.px
}

// fill füllt das Rechteck (Fenster-px) mit der Farbe clr, skaliert mit dem Faktor k (0 … 1, Helligkeit) und der
// Deckkraft a; ohne Allokation.
func (w *World) fill(dst *ebiten.Image, x, y, wd, h float64, clr color.RGBA, k, a float64) {
	if wd <= 0 || h <= 0 || a <= 0 {
		return
	}
	op := &w.fillOp
	op.GeoM.Reset()
	op.GeoM.Scale(wd, h)
	op.GeoM.Translate(x, y)
	op.ColorScale.Reset()
	op.ColorScale.Scale(float32(float64(clr.R)/255*k*a), float32(float64(clr.G)/255*k*a), float32(float64(clr.B)/255*k*a), float32(a))
	dst.DrawImage(w.pixel(), op)
}

// fillRect füllt ein ganzzahliges Rechteck.
func (w *World) fillRect(dst *ebiten.Image, r image.Rectangle, clr color.RGBA, k float64) {
	w.fill(dst, float64(r.Min.X), float64(r.Min.Y), float64(r.Dx()), float64(r.Dy()), clr, k, 1)
}
