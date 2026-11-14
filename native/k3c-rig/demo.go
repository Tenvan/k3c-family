package main

// Vorführ-Modus: zwei Monarchen untereinander, je Zeile links das Sheet und rechts dieselbe Pose aus Rig-Teilen.

import (
	"fmt"
	"image"
	"image/color"
	"math"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"

	"k3c/native/k3c-rig/paint"
	"k3c/native/k3c-rig/rig"
)

// Vergrößerung gegenüber der Pixeldichte der Sheets (1 = Sheet-Pixel 1:1, 2 = Pixeldichte der Teil-Bilder)
var zooms = []float64{1, 1.5, 2, 3}

var (
	groundColor = color.RGBA{74, 56, 36, 255}
	lineColor   = color.RGBA{255, 255, 255, 90}
)

const help = "Tastatur: Tab/Shift+Tab oder 1..0 Zustand | F Farbe | W Waffe | R Panzer | Pfeile links/rechts Blick, hoch/runter Tempo | Z Zoom | P Spieler | Esc Ende\n" +
	"Gamepad:  A weiter, LB zurueck | RB Farbe | X Waffe | Y Panzer | Steuerkreuz/Stick links/rechts Blick, hoch/runter Tempo | Menu Zoom"

type Demo struct {
	actors   [2]*paint.Actor
	zoom     int
	keyboard int // Spieler, den die Tastatur steuert
	pads     []ebiten.GamepadID
}

func NewDemo(g *Game) (*Demo, error) {
	d := &Demo{zoom: 2}
	for i := range d.actors {
		a, err := NewActor(g.assets, i)
		if err != nil {
			return nil, err
		}
		d.actors[i] = a
	}
	return d, nil
}

func (d *Demo) Update(g *Game, dt float64) error {
	d.pads = sortedPads()
	if d.input(g) {
		g.quit = true
		return nil
	}
	for _, a := range d.actors {
		if err := a.Update(dt); err != nil {
			return err
		}
	}
	return nil
}

// scale: Bildpunkte je Einheit; die Sheets haben die halbe Dichte der Teil-Bilder.
func (d *Demo) scale(g *Game) float64 { return zooms[d.zoom] * g.assets.monarch.Density / 2 }

func (d *Demo) Draw(g *Game, screen *ebiten.Image) {
	half := g.h / 2
	for i, a := range d.actors {
		row := image.Rect(0, i*half, g.w, (i+1)*half)
		d.drawRow(g, screen, a, row)
		ebitenutil.DebugPrintAt(screen, d.label(i, a), 16, row.Min.Y+12)
	}
	fill(screen, g.assets.White, 0, float64(half)-1, float64(g.w), 2, lineColor)
	fill(screen, g.assets.White, float64(g.w)/2-1, 0, 2, float64(g.h), lineColor)
	ebitenutil.DebugPrintAt(screen, fmt.Sprintf("%.0f FPS | Zoom %.1fx | Tastatur: Spieler %d\n%s", ebiten.ActualFPS(), zooms[d.zoom], d.keyboard+1, help), 16, g.h-56)
}

// drawRow zeichnet eine Zeile: links das Sheet, rechts das Rig, beide mit den Füßen auf derselben Bodenlinie.
func (d *Demo) drawRow(g *Game, screen *ebiten.Image, a *paint.Actor, row image.Rectangle) {
	s := d.scale(g)
	ground := float64(row.Max.Y) - 28
	fill(screen, g.assets.White, 0, ground, float64(g.w), float64(row.Max.Y)-ground, groundColor)
	mid := (row.Min.X + row.Max.X) / 2
	left := screen.SubImage(image.Rect(row.Min.X, row.Min.Y, mid, row.Max.Y)).(*ebiten.Image)
	right := screen.SubImage(image.Rect(mid, row.Min.Y, row.Max.X, row.Max.Y)).(*ebiten.Image)
	x := float64(g.w) / 4
	if g.sheets == nil {
		ebitenutil.DebugPrintAt(screen, "Sheets fehlen (task gen:paper)", row.Min.X+16, row.Min.Y+40)
	} else if sh, err := g.sheets.sheet(a.State.Name, a.Variant); err != nil {
		ebitenutil.DebugPrintAt(screen, "Sheet fehlt: "+a.State.Name, row.Min.X+16, row.Min.Y+40)
	} else {
		sh.draw(left, int(math.Floor(a.Phase()*float64(len(sh.frames)))), x, ground, s, a.Left)
	}
	g.painter.Dst, g.painter.View = right, viewAt(x+float64(g.w)/2, ground, s, a.Left)
	a.Draw(g.painter)
}

// viewAt: Sicht mit dem Ursprung der Figur bei (x, y), scale Bildpunkten je Einheit und optionaler Spiegelung.
func viewAt(x, y, scale float64, left bool) rig.M {
	sx := scale
	if left {
		sx = -scale
	}
	return rig.M{sx, 0, 0, scale, x, y}
}

func (d *Demo) label(i int, a *paint.Actor) string {
	name := func(s string) string {
		if s == "" {
			return "-"
		}
		return s
	}
	return fmt.Sprintf("Spieler %d | Zustand %s | Farbe %s | Waffe %s | Panzer %s | Tempo %.2f\nlinks: Sheet            rechts: Rig-Teile",
		i+1, a.State.Name, a.Variant, name(a.Weapon), name(a.Armor), a.Tempo)
}

// fill zeichnet ein Rechteck in einer Farbe (ein weißer Bildpunkt, gestreckt und eingefärbt).
func fill(dst, white *ebiten.Image, x, y, w, h float64, c color.RGBA) {
	op := &ebiten.DrawImageOptions{}
	op.GeoM.Scale(w, h)
	op.GeoM.Translate(x, y)
	op.ColorScale.ScaleWithColor(c)
	dst.DrawImage(white, op)
}
