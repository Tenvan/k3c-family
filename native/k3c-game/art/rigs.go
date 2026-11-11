package art

// Rig-Bibliothek: Figuren aus Teilen (native/k3c-rig/paint). Fachpakete dürfen paint nicht importieren, deshalb
// reicht art die nötigen Typen durch und baut Actor und Painter über die Library.

import (
	"fmt"

	"k3c/native/k3c-game/view"
	"k3c/native/k3c-rig/paint"
	"k3c/native/k3c-rig/rig"
)

type (
	// Painter zeichnet Rig-Figuren in ein Ziel (Dst) mit einer Sicht (View).
	Painter = paint.Painter
	// Actor ist eine Figur aus Rig-Teilen mit Zustand (Animation, Variante, Waffe).
	Actor = paint.Actor
	// State ist Animation bzw. Gangart einer Figur.
	State = paint.State
	// M ist eine 2D-Affin-Matrix wie SVG matrix(a b c d e f) (Painter.View, RigView).
	M = rig.M
	// Point ist ein Punkt in Rig-Einheiten (Actor.SetFrame: Kette des Umhangs).
	Point = rig.Point
)

// ActorFrame liefert den Rahmen einer Figur (x, y, Breite, Höhe in Rig-Einheiten, Fußpunkt bei 0,0): beritten bzw.
// Vierbeiner der des Reittiers, zu Fuß der des Reiters (wie native/k3c-ebiten/rigs.go).
func ActorFrame(a *Actor) [4]float64 {
	if a.Mount != nil {
		return a.Mount.Frame
	}
	if a.Rider != nil {
		return a.Rider.Frame
	}
	return [4]float64{}
}

// RigView liefert die Sicht-Matrix (Painter.View) einer Rig-Figur: Fußpunkt bei Welt-x wx (Units) auf der Boden-Linie
// groundY (Fenster-y in px, view.Cell.GroundY), k ist die Kamera der Zelle. size sind Welt-Units je Rig-Einheit
// (Spielhöhe der Figur / Rahmenhöhe), frame ihr Rahmen (ActorFrame). left spiegelt wie das Sheet im eigenen Rahmen
// (nicht um den Fußpunkt), damit Rig und Sheet an derselben Stelle stehen. Reine Rechnung, ohne Allokation.
func RigView(k view.Camera, wx, groundY, size float64, frame [4]float64, left bool) M {
	u := size * k.Scale // Bildpunkte je Rig-Einheit
	fx, sx := k.ScreenX(wx), u
	if left {
		fx, sx = fx+(frame[2]+2*frame[0])*u, -u
	}
	return M{sx, 0, 0, u, fx, groundY}
}

// NewPainter legt einen Painter an (Puffer werden wiederverwendet; einer je Zeichner genügt).
func (l *Library) NewPainter() (*Painter, error) {
	if l == nil || l.rigs == nil {
		return nil, fmt.Errorf("%w: Rig-Bibliothek nicht geladen (RigDir leer)", ErrMissing)
	}
	return paint.NewPainter(l.rigs.White), nil
}

// NewActor legt eine Figur an: rider und mount sind Rig-Ordner `gruppe/id` ("" = keiner), variant z. B. "spieler2".
// Fehlende Teil-Bilder melden den Pfad.
func (l *Library) NewActor(rider, mount, variant string) (*Actor, error) {
	if l == nil || l.rigs == nil {
		return nil, fmt.Errorf("%w: Rig-Bibliothek nicht geladen (RigDir leer)", ErrMissing)
	}
	r, err := l.rig(rider)
	if err != nil {
		return nil, err
	}
	m, err := l.rig(mount)
	if err != nil {
		return nil, err
	}
	return paint.NewActor(l.rigs, r, m, variant)
}

// rig lädt das Rig `gruppe/id` ("" = nil).
func (l *Library) rig(rel string) (*rig.Rig, error) {
	if rel == "" {
		return nil, nil
	}
	return l.rigs.Rig(rel)
}
