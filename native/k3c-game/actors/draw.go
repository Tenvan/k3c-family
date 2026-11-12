package actors

// Zeichnen der Figuren einer Zelle: hinten nach vorn Truppen, Gegner, Monarchen (Reihenfolge der Simulation), nur
// sichtbare. Draw ändert keinen Zustand: Rig-Figuren zeichnen die Pose, die Listen im Takt gerechnet hat (pose.go),
// Sheets wählen nur ihr Bild. Beide mit der Deckkraft der Figur (fg.alpha). Ohne Allokation je Bild.

import (
	"math"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

func (d *drawer) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if d.cat == nil || d.lib == nil {
		return
	}
	w := f.World(c)
	s, ok := f.Seat(c.Player)
	if w == nil || !ok {
		return
	}
	if d.ink == nil {
		d.ink = newInk()
	}
	for _, t := range w.Troops {
		if fg := d.drawFigure(dst, c, key{s.Stage, classTroop, t.ID}, t.X); fg != nil {
			d.troopMarks(dst, c, fg, t)
		}
	}
	for _, e := range w.Enemies {
		if fg := d.drawFigure(dst, c, key{s.Stage, classEnemy, e.ID}, e.X); fg != nil {
			d.enemyMarks(dst, c, fg, e)
		}
	}
	d.drawPlayers(dst, f, c, s.Stage, w)
}

// drawPlayers zeichnet die Monarchen der Zelle: Punkt, Figur, Balken. Freie Monarchen (kein Platz in Frame.Seats)
// bleiben unsichtbar.
func (d *drawer) drawPlayers(dst *ebiten.Image, f *view.Frame, c view.Cell, stage int, w *sim.World) {
	for _, p := range w.Players {
		if p == nil || !Seated(f.Seats, p.Index) {
			continue
		}
		k := key{stage, classPlayer, p.ID}
		if fg := d.figs[k]; fg != nil && !fg.broken {
			d.playerMarker(dst, c, fg, p)
		}
		if fg := d.drawFigure(dst, c, k, p.X); fg != nil {
			d.playerBarMark(dst, c, fg, p)
		}
	}
}

// drawFigure zeichnet die Figur k mit Fußpunkt bei Welt-x x, wenn sie in die Zelle ragt. Rückgabe: die Figur, wenn sie
// zeichenbar ist (für ihre Anzeigen, auch außerhalb der Zelle), sonst nil (unbekannt, Rig fehlt, noch ohne Pose).
func (d *drawer) drawFigure(dst *ebiten.Image, c view.Cell, k key, x float64) *figure {
	fg := d.figs[k]
	if fg == nil || fg.broken {
		return nil
	}
	y := c.ScreenY(fg.lift)
	if fg.actor == nil {
		d.drawSheet(dst, c.Camera, fg, x, y)
		return fg
	}
	if d.painter == nil {
		return nil
	}
	size, frame := rigSize(fg)
	if !c.Camera.Visible(x, (math.Abs(frame[0])+math.Abs(frame[2]))*size) {
		return fg
	}
	if !fg.posed {
		return nil // noch ohne Pose aus Listen oder Pose fehlerhaft (gemeldet)
	}
	d.painter.Dst, d.painter.Alpha = dst, fg.alpha
	d.painter.View = art.RigView(c.Camera, x, y, size, frame, fg.left)
	fg.actor.Draw(d.painter)
	return fg
}

// rigSize liefert Welt-Units je Rig-Einheit (Spielhöhe / Rahmenhöhe der eigenen Figur; beritten bleibt der Maßstab
// des Reiters) und den Rahmen der gezeichneten Figur (beritten bzw. Vierbeiner der des Reittiers).
func rigSize(fg *figure) (float64, [4]float64) {
	a := fg.actor
	base, frame := a.Rider, art.ActorFrame(a)
	if base == nil {
		base = a.Mount
	}
	if fg.pose.Gait == "" && a.Rider != nil {
		frame = a.Rider.Frame // zu Fuß (Monarch tot oder beim Respawn)
	}
	if base.Frame[3] <= 0 {
		return 0, frame
	}
	return fg.look.Height / base.Frame[3], frame
}

// drawSheet zeichnet eine Sheet-Figur: Fußpunkt aus dem Rahmen, gespiegelt im eigenen Bild, mit Deckkraft.
func (d *drawer) drawSheet(dst *ebiten.Image, k view.Camera, fg *figure, x, y float64) {
	s := d.lib.Sheet(fg.look.Asset, fg.pose.Anim)
	if s == nil || s.FH <= 0 {
		return
	}
	px := fg.look.Height * k.Scale / float64(s.FH) // Bildpunkte je Sheet-Pixel
	fw, fh := float64(s.FW), float64(s.FH)
	if !k.Visible(x, fw*px/k.Scale) {
		return
	}
	i := SheetFrame(AnimTime(fg.pose, d.now, fg.shift), s.FPS, len(s.Frames), fg.pose.Dur > 0)
	g := &d.op.GeoM
	g.Reset()
	if fg.left {
		g.Scale(-1, 1)
		g.Translate(fw, 0)
	}
	g.Translate(-s.OriginX*fw, -s.OriginY*fh)
	g.Scale(px, px)
	g.Translate(k.ScreenX(x), y)
	d.op.ColorScale.Reset()
	d.op.ColorScale.ScaleAlpha(float32(fg.alpha))
	if img := s.Frame(i); img != nil {
		dst.DrawImage(img, &d.op)
	}
}
