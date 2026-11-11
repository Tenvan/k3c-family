package main

// Zwei Spieler: je ein Gamepad (Tastatur als Ersatz), nur horizontale Bewegung. Die Sicht je Spieler (Split-Screen
// oben/unten) zeichnet main.go; hier stehen Eingabe, Bewegung und der Gangwechsel.

import (
	"fmt"
	"math"
	"sort"

	"github.com/hajimehoshi/ebiten/v2"
)

const (
	// Galopp in Spiel-Pixeln je Sekunde (10 Units/s)
	speed = 320.0
	// Sichtbare Höhe der Welt je Bildschirmhälfte
	viewH = 600.0
	// Kamera-Mitte über dem Boden
	cameraY = 240.0
	// Totzone des linken Sticks
	deadzone = 0.25
)

// Reittiere der beiden Monarchen (Animationen `schritt-<tier>` / `galopp-<tier>` im Monarch-Manifest)
var mounts = [2]string{"schimmel", "rappe"}

type Player struct {
	index  int
	sprite *Sprite
	moving bool
}

func gaitAnim(gait string, index int) string {
	return fmt.Sprintf("%s-%s@spieler%d", gait, mounts[index], index+1)
}

// sortedPads: Gamepad 1 → Spieler 1, Gamepad 2 → Spieler 2, nach ID sortiert.
func sortedPads() []ebiten.GamepadID {
	ids := ebiten.AppendGamepadIDs(nil)
	sort.Slice(ids, func(a, b int) bool { return ids[a] < ids[b] })
	return ids
}

// inputAxis: Richtung aus Gamepad (linker Stick oder Steuerkreuz, Standard-Layout) oder Tastatur.
func inputAxis(index int, pads []ebiten.GamepadID) float64 {
	x := 0.0
	if index < len(pads) {
		id := pads[index]
		if ebiten.IsStandardGamepadLayoutAvailable(id) {
			if stick := ebiten.StandardGamepadAxisValue(id, ebiten.StandardGamepadAxisLeftStickHorizontal); math.Abs(stick) > deadzone {
				x = stick
			}
			if ebiten.IsStandardGamepadButtonPressed(id, ebiten.StandardGamepadButtonLeftLeft) {
				x = -1
			}
			if ebiten.IsStandardGamepadButtonPressed(id, ebiten.StandardGamepadButtonLeftRight) {
				x = 1
			}
		} else if stick := ebiten.GamepadAxisValue(id, 0); math.Abs(stick) > deadzone {
			x = stick // unbekanntes Pad: erste Achse, kein Steuerkreuz
		}
	}
	left, right := ebiten.KeyA, ebiten.KeyD
	if index == 1 {
		left, right = ebiten.KeyArrowLeft, ebiten.KeyArrowRight
	}
	if ebiten.IsKeyPressed(left) {
		x = -1
	}
	if ebiten.IsKeyPressed(right) {
		x = 1
	}
	return x
}

// autoAxis: automatisches Hin und Her (4 s je Richtung, Spieler 2 gegenläufig).
func autoAxis(index int, t float64) float64 {
	if (int64(math.Floor(t/4))+int64(index))%2 == 0 {
		return 1
	}
	return -1
}

// update bewegt den Monarchen und wechselt den Gang: Schritt im Stand (angehalten), Galopp in Bewegung.
func (pl *Player) update(p *Paper, dt, elapsed float64, auto bool, pads []ebiten.GamepadID) error {
	x := inputAxis(pl.index, pads)
	if auto {
		x = autoAxis(pl.index, elapsed)
	}
	sp := pl.sprite
	pl.moving = math.Abs(x) > 0.01
	if pl.moving {
		sp.flip = x < 0
		sp.x = math.Min(math.Max(sp.x+x*speed*dt, 100), worldW-100)
	}
	gait := "schritt"
	if pl.moving {
		gait = "galopp"
	}
	s, err := p.sheet(sp.asset, gaitAnim(gait, pl.index))
	if err != nil {
		return err
	}
	if sp.sheet != s {
		sp.sheet, sp.frame = s, 0
		sp.scale = p.scaleFor(sp.asset, sp.height, s)
	}
	sp.playing = pl.moving
	return nil
}
