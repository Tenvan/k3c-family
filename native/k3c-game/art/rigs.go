package art

// Rig-Bibliothek: Figuren aus Teilen (native/k3c-rig/paint). Fachpakete dürfen paint nicht importieren, deshalb
// reicht art die nötigen Typen durch und baut Actor und Painter über die Library.

import (
	"fmt"

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
)

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
