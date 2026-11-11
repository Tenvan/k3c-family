package world

// Kamera je Zelle (B-795/AC-01): Wie im Browser (src/scenes/GameScene.ts › aimAtStage, cameraFollow.ts) folgt jede
// Kamera ihrem Monarchen weich und bleibt in den Grenzen der Stufe (setBounds); dazu eine kleine Totzone, damit kurze
// Schritte nicht das ganze Bild schieben. Beim ersten Bild und beim Wechsel der Stufe oder Insel springt sie ans Ziel.

import (
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

const (
	// deadZone ist die halbe Breite der Totzone um die Kamera-Mitte in Units: Steht das Ziel darin, bleibt die Kamera.
	deadZone = 1.0
	// followRate ist der Anteil des Abstands, den die Kamera je Update (30 Hz) aufholt: Phaser folgt mit lerp 0,1 je
	// Bild bei 60 Hz, das sind 1 − 0,9² = 0,19 je Update.
	followRate = 0.19
	// fadeSeconds ist die Dauer des Übergangs (Abblenden aus Schwarz) nach einem Stufen- oder Inselwechsel.
	fadeSeconds = 0.6
)

// stageKey erkennt die Stufe eines Spielers: Index, Tiefe und Level der Stufe, Seed ihrer Welt (Inselwechsel).
type stageKey struct {
	index, depth int
	level, seed  string
}

// seatState ist der Kamera-Zustand eines Spielers: zuletzt gesehene Stufe und Zeitpunkt (Frame.Clock) des Wechsels.
type seatState struct {
	key   stageKey
	since float64
	known bool
}

// cameras ist der Zustand der Kamera-Regel je Spieler (Seat.Player). Nur Follow (Update) schreibt, Draw liest.
type cameras struct {
	seats [view.MaxPlayers]seatState
}

// follow ist die Kamera-Regel (view.CameraRule): Ziel ist der Monarch, ohne ihn die Burg; ohne Welt bleibt die
// Kamera stehen. Beim ersten Bild (prev.Scale == 0) und beim Stufenwechsel springt sie, sonst holt sie weich auf.
func (s *cameras) follow(f *view.Frame, c view.Cell, prev view.Camera) view.Camera {
	k := c.Camera
	w := f.World(c)
	if w == nil {
		if prev.Scale != 0 {
			k.X = prev.X
		}
		return k
	}
	jump := s.enter(f, c.Player) || prev.Scale == 0
	x := targetX(f, c, w)
	if !jump {
		x = chase(prev.X, x)
	}
	k.X = clampX(x, halfUnits(k), w.WidthUnits)
	return k
}

// enter merkt sich die Stufe des Spielers p und meldet, ob sie neu ist (erstes Bild oder Wechsel); der Wechsel
// startet den Übergang zur Zeit Frame.Clock.
func (s *cameras) enter(f *view.Frame, p int) bool {
	if p < 0 || p >= len(s.seats) {
		return true
	}
	key, ok := keyOf(f, p)
	st := &s.seats[p]
	if !ok || (st.known && st.key == key) {
		return false
	}
	*st = seatState{key: key, since: f.Clock, known: true}
	return true
}

// fade liefert die Deckkraft des Übergangs für Spieler p zur Zeit clock (0 = kein Übergang).
func (s *cameras) fade(p int, clock float64) float64 {
	if p < 0 || p >= len(s.seats) || !s.seats[p].known {
		return 0
	}
	return fadeAlpha(clock - s.seats[p].since)
}

// keyOf liefert die Stufe des Spielers p im Frame.
func keyOf(f *view.Frame, p int) (stageKey, bool) {
	seat, ok := f.Seat(p)
	if !ok {
		return stageKey{}, false
	}
	st, ok := f.Stage(seat.Stage)
	if !ok || st.World == nil {
		return stageKey{}, false
	}
	return stageKey{index: st.Index, depth: st.Depth, level: st.Level, seed: st.World.Seed}, true
}

// targetX ist das Ziel der Kamera: der Monarch des Spielers, sonst die Burg der Stufe.
func targetX(f *view.Frame, c view.Cell, w *sim.World) float64 {
	if m := f.Monarch(c.Player); m != nil {
		return m.X
	}
	return w.Castle.X
}

// chase führt die Kamera von x auf das Ziel t zu: in der Totzone bleibt sie, sonst holt sie followRate des Abstands
// zum Rand der Totzone auf.
func chase(x, t float64) float64 {
	switch d := t - x; {
	case d > deadZone:
		return x + (d-deadZone)*followRate
	case d < -deadZone:
		return x + (d+deadZone)*followRate
	}
	return x
}

// clampX hält die Kamera-Mitte x so, dass die Zelle mit halber Breite half (Units) nicht über die Stufe 0 … width
// hinausschaut; ist die Stufe schmaler als die Zelle, steht die Kamera in ihrer Mitte.
func clampX(x, half, width float64) float64 {
	if width <= 0 || half <= 0 {
		return x
	}
	if width <= 2*half {
		return width / 2
	}
	return min(max(x, half), width-half)
}

// halfUnits ist die halbe sichtbare Breite der Kamera in Units (0 ohne Maßstab).
func halfUnits(k view.Camera) float64 {
	if k.Scale <= 0 {
		return 0
	}
	return k.Half / k.Scale
}

// fadeAlpha ist die Deckkraft des Übergangs elapsed Sekunden nach dem Wechsel: 1 → 0 linear über fadeSeconds.
func fadeAlpha(elapsed float64) float64 {
	if elapsed < 0 || elapsed >= fadeSeconds {
		return 0
	}
	return 1 - elapsed/fadeSeconds
}
