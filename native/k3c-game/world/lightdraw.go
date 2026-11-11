package world

// Licht zeichnen (B-795/AC-03, Vorbild src/scenes/render/lightView.ts): eine Lichtkarte multipliziert die ganze Zelle
// (Welt und Fluss, über Gebäuden und Figuren, unter den Effekten), gerechnet in lightColumns Spalten und dazwischen
// linear verlaufen; darüber die Fackeln (Halter immer, Flamme mit ihrem Schein) und der Übergang nach einem
// Stufenwechsel (Abblenden aus Schwarz). Am vollen Tag ohne Übergang zeichnet es nichts.

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

const (
	// lightColumns ist die Zahl der Spalten der Lichtkarte je Zelle.
	lightColumns = 64
	// Fackel (lightView.ts, in Units): Flammen-Mitte über dem Boden, Halter, Flammen-Radius.
	torchLift   = 40.0 / 32
	torchStickW = 4.0 / 32
	torchStickH = 16.0 / 32
	torchFlameR = 5.0 / 32
)

var (
	torchStick = color.RGBA{0x5a, 0x3a, 0x22, 255}
	black      = color.RGBA{0, 0, 0, 255}
	// multiply: Ziel × Quelle (Lichtkarte), Deckkraft des Ziels bleibt.
	multiply = ebiten.Blend{
		BlendFactorSourceRGB:        ebiten.BlendFactorDestinationColor,
		BlendFactorSourceAlpha:      ebiten.BlendFactorZero,
		BlendFactorDestinationRGB:   ebiten.BlendFactorZero,
		BlendFactorDestinationAlpha: ebiten.BlendFactorOne,
		BlendOperationRGB:           ebiten.BlendOperationAdd,
		BlendOperationAlpha:         ebiten.BlendOperationAdd,
	}
)

// lightMesh hält die Puffer der Lichtkarte (einmal angelegt, je Zelle neu gefüllt).
type lightMesh struct {
	verts   []ebiten.Vertex
	indices []uint16
	op      ebiten.DrawTrianglesOptions
}

// ensure legt die Puffer beim ersten Gebrauch an: Spalte i hat Ecken 2i (oben) und 2i+1 (unten).
func (m *lightMesh) ensure() {
	if m.verts != nil {
		return
	}
	m.verts = make([]ebiten.Vertex, 2*(lightColumns+1))
	m.indices = make([]uint16, 0, 6*lightColumns)
	for i := range uint16(lightColumns) {
		a := 2 * i
		m.indices = append(m.indices, a, a+1, a+2, a+1, a+3, a+2)
	}
	m.op.Blend = multiply
}

// columnX ist das Fenster-x der Spalten-Kante i (0 … lightColumns) der Zelle.
func columnX(c view.Cell, i int) float64 {
	return float64(c.Rect.Min.X) + float64(c.Rect.Dx())*float64(i)/lightColumns
}

// drawLight zeichnet Lichtkarte, Fackeln und Übergang der Zelle.
func (w *World) drawLight(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	st := stageOf(f, c)
	if st == nil {
		return
	}
	wd := st.World
	if daylight(wd.Cycle) < 1 {
		w.drawLightMap(dst, c, wd)
	}
	w.drawTorches(dst, c, wd)
	if a := w.cams.fade(c.Player, f.Clock); a > 0 {
		w.fill(dst, float64(c.Rect.Min.X), float64(c.Rect.Min.Y), float64(c.Rect.Dx()), float64(c.Rect.Dy()), black, 1, a)
	}
}

// drawLightMap multipliziert die Zelle mit der Lichtfarbe je Spalte.
func (w *World) drawLightMap(dst *ebiten.Image, c view.Cell, wd *sim.World) {
	m := &w.mesh
	m.ensure()
	top, bottom := float32(c.Rect.Min.Y), float32(c.Rect.Max.Y)
	for i := 0; i <= lightColumns; i++ {
		sx := columnX(c, i)
		r, g, b := w.light.colorAt(c.Camera.WorldX(sx), wd)
		for j, y := range [2]float32{top, bottom} {
			m.verts[2*i+j] = ebiten.Vertex{
				DstX: float32(sx), DstY: y, SrcX: 0.5, SrcY: 0.5,
				ColorR: float32(r), ColorG: float32(g), ColorB: float32(b), ColorA: 1,
			}
		}
	}
	dst.DrawTriangles(m.verts, m.indices, w.pixel(), &m.op)
}

// drawTorches zeichnet die sichtbaren Fackeln: Halter aus Holz, darüber die Flamme in glowColor mit der Deckkraft
// ihres Scheins (am Tag aus).
func (w *World) drawTorches(dst *ebiten.Image, c view.Cell, wd *sim.World) {
	offs := w.light.Torches
	if len(offs) == 0 {
		return
	}
	k := c.Camera
	a := glowAlpha(w.light.torch.Strength, wd.Cycle)
	flame := color.RGBA{uint8(w.light.glow[0] * 255), uint8(w.light.glow[1] * 255), uint8(w.light.glow[2] * 255), 255}
	top := c.ScreenY(torchLift)
	for _, s := range wd.Sites {
		if s.State != "built" {
			continue
		}
		side := torchSide(s.X, wd.HubX)
		for _, o := range offs[s.Kind] {
			x := s.X + side*o
			if !Visible(k, x, torchFlameR) {
				continue
			}
			sx := k.ScreenX(x)
			w.fill(dst, sx-torchStickW*k.Scale/2, top, torchStickW*k.Scale, torchStickH*k.Scale, torchStick, 1, 1)
			r := torchFlameR * k.Scale
			w.fill(dst, sx-r, top-r/2-r, 2*r, 2*r, flame, 1, a)
		}
	}
}
