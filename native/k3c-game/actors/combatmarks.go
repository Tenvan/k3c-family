package actors

// Anzeigen der Kampfregeln an den Figuren (KS5, B-934, docs/rules/kampf.md): Krone am Kopf des Monarchen und auf dem Kopf
// ihres Trägers (Gegner oder Mitspieler), Monarch ohne Krone, Banner am Monarchen, Schild des Gruppenführers und die
// dezente Gruppenmarke der Truppe. Krone, Banner und Schild sind Papierschnitt-Bilder (B-940: icons/icon-krone,
// objekte/banner-angriff und objekte/schild-gruppenfuehrer, Varianten je Spieler); die Gruppenmarke bleibt eine Scheibe.
// Fliegende und liegende Kronen, Portal-HP und Zahlstellen zeichnet props.

import (
	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// Assets der Anzeigen (data/paper.json › objekte) und die Animationen ihrer Varianten (Spieler 1 bis 4, 0 = ohne).
const (
	crownAsset  = "icons/icon-krone"
	bannerAsset = "objekte/banner-angriff"
	shieldAsset = "objekte/schild-gruppenfuehrer"
)

var variantAnims = [5]string{"idle", "idle@spieler1", "idle@spieler2", "idle@spieler3", "idle@spieler4"}

// variantAnim ist die Animation der Variante für Spieler n (1 bis 4); sonst die Grundvariante.
func variantAnim(n int) string {
	if n < 1 || n >= len(variantAnims) {
		return variantAnims[0]
	}
	return variantAnims[n]
}

// Maße in Spiel-Pixeln (wie marks.go).
const (
	crownScale = 0.75 // das 32-px-Icon ist größer als die alte Form; am Kopf auf 24 px verkleinert
	crownAbove = 10.0 // Fuß der Krone über der Oberkante der Figur

	bareAlpha = 0.3 // Deckkraft des Spieler-Punkts ohne Krone, relativ zu markerAlpha

	bannerOffset = 1.6 // Abstand des Stocks in Vielfachen des Last-Versatzes (hinter dem Rücken)

	shieldSide   = 16.0 // seitlicher Versatz des Schilds
	shieldCenter = 12.0 // Mitte des 28-px-Bilds über seinem Fuß

	groupDot      = 6.0
	groupDotAlpha = 0.7
)

// markAssets nennt die Papier-Bilder der Anzeigen, die actors für jedes Level anmeldet (sortiert).
var markAssets = []string{bannerAsset, crownAsset, shieldAsset}

// sprite zeichnet Bild i des Sheets mit dem Fuß bei (ox, oy) px vom Bezugspunkt (oy nach oben), um scale vergrößert,
// bei flip gespiegelt, mit Deckkraft alpha. Ohne Allokation (Optionen und Matrix liegen im ink).
func (k *ink) sprite(dst *ebiten.Image, a anchor, s *art.Sheet, i int, ox, oy, scale float64, flip bool, alpha float64) {
	img := s.Frame(i)
	if img == nil || s.FW <= 0 || s.FH <= 0 {
		return
	}
	fw, fh, px := float64(s.FW), float64(s.FH), s.UnitsPerPx*a.s*scale
	g := &k.op.GeoM
	g.Reset()
	if flip {
		g.Scale(-1, 1)
		g.Translate(fw, 0)
	}
	g.Translate(-s.OriginX*fw, -s.OriginY*fh)
	g.Scale(px, px)
	g.Translate(a.sx+ox*a.s, a.sy-oy*a.s)
	k.op.ColorScale.Reset()
	k.op.ColorScale.ScaleAlpha(float32(alpha))
	dst.DrawImage(img, &k.op)
}

// crown zeichnet das Kronen-Icon mit dem Fuß bei (cx, cy); alpha ist die Deckkraft der Figur.
func (d *drawer) crown(dst *ebiten.Image, a anchor, cx, cy, alpha float64) {
	if s := d.lib.Sheet(crownAsset, variantAnims[0]); s != nil {
		d.ink.sprite(dst, a, s, 0, cx, cy, crownScale, false, alpha)
	}
}

// crownCarrier: Der Monarch p trägt die Krone eines Mitspielers (CrownPlayer, Träger ist er).
func crownCarrier(w *sim.World, p *sim.Player) bool {
	for _, o := range w.Players {
		if o != nil && o.Crown != nil && o.Crown.State == sim.CrownPlayer && o.Crown.Carrier == p.ID {
			return true
		}
	}
	return false
}

// bannerFlip: Der Wimpel (im Bild nach rechts) zeigt vom Stock weg, den der Monarch hinter dem Rücken trägt.
func bannerFlip(left bool) bool { return LoadOffset(left) < 0 }

// playerMarks zeichnet Krone (am Kopf bzw. getragen) und Banner des Monarchen.
func (d *drawer) playerMarks(dst *ebiten.Image, c view.Cell, fg *figure, p *sim.Player, w *sim.World) {
	a, ok := marksAnchor(c, fg, p.X)
	if !ok {
		return
	}
	top := d.topPx(fg)
	if p.Crown == nil || crownCarrier(w, p) {
		d.crown(dst, a, 0, top+crownAbove, fg.ui)
	}
	if p.Banner != 0 {
		if s := d.lib.Sheet(bannerAsset, variantAnim(p.Index+1)); s != nil {
			d.ink.sprite(dst, a, s, SheetFrame(d.now, s.FPS, len(s.Frames), false), LoadOffset(fg.left)*bannerOffset, 0, 1, bannerFlip(fg.left), fg.ui)
		}
	}
}

// leaderMarks zeichnet das Schild des Gruppenführers, sonst bei zugeordneter Truppe die dezente Gruppenmarke über dem Kopf.
// Gruppen gehören keinem Spieler (t.Group ist eine Truppen-ID): das Schild nutzt die Grundvariante.
func (d *drawer) leaderMarks(dst *ebiten.Image, a anchor, fg *figure, t *sim.Troop, top float64) {
	switch {
	case t.Leader:
		if s := d.lib.Sheet(shieldAsset, variantAnims[0]); s != nil {
			d.ink.sprite(dst, a, s, 0, -LoadOffset(fg.left)*shieldSide/loadSide, top*loadHeight-shieldCenter, 1, false, fg.ui)
		}
	case t.Group != 0:
		d.ink.oval(dst, a, box{0, top + groupDot, groupDot, groupDot}, tint(PlayerColor(t.Group), groupDotAlpha*fg.ui))
	}
}
