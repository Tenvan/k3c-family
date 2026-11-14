package hud

// Schriftrollen (Spiegel von src/scenes/render/fontRules.ts, Beschluss Q03, docs/rules/bedienung.md § 2): Größe in px
// bezogen auf 1080 Bildpunkte Fensterhöhe, Art (Pflicht- oder Nebeninfo) und Farbe. Das native Fenster skaliert die
// Rollen mit seiner Höhe (scaleOf), wie der Browser mit FIT auf 1920 × 1080 skaliert.

import (
	"image/color"
	"math"

	"k3c/native/k3c-game/view"
)

// refHeight ist die Bezugshöhe der Größen (Browser: GAME_HEIGHT).
const refHeight = 1080.0

// fontArt: Pflicht-Info (Gold, Vorrat, HP, Warnungen, Meldungen) oder Nebeninfo (Hinweise, Tastenhilfen).
type fontArt int

const (
	artInfo fontArt = iota
	artSide
)

// fontRole ist eine Schriftrolle.
type fontRole struct {
	px  float64
	art fontArt
	clr color.RGBA
}

var (
	white    = color.RGBA{255, 255, 255, 255}
	black    = color.RGBA{0, 0, 0, 255}
	yellow   = color.RGBA{0xff, 0xd1, 0x66, 255}
	pinkish  = color.RGBA{0xff, 0x8f, 0xa3, 255}
	citizenC = color.RGBA{0xe9, 0xc4, 0x6a, 255}
)

// Rollen wie FONTS im Browser (nur die, die das HUD zeichnet).
var (
	rolePlayer   = fontRole{28, artInfo, white}
	roleShared   = fontRole{28, artInfo, white}
	roleFight    = fontRole{28, artInfo, pinkish}
	roleBanner   = fontRole{28, artInfo, white}
	roleBoss     = fontRole{28, artInfo, yellow}
	roleTravel   = fontRole{40, artInfo, yellow}
	roleJoin     = fontRole{26, artSide, white}
	roleHint     = fontRole{24, artSide, white}
	roleRoomInfo = fontRole{24, artSide, white}
	roleCitizen  = fontRole{32, artInfo, citizenC}
)

// minFontPx ist die Mindestgröße bei 1080 px Höhe: Pflicht-Info 28 px in Zellen über die ganze Breite (beide nativen
// Layouts), 24 px sonst; Nebeninfo 24 px bei 1 und 2 Spielern, 20 px darüber.
func minFontPx(seats int, fullWidth bool, art fontArt) float64 {
	if art == artInfo {
		if fullWidth {
			return 28
		}
		return 24
	}
	if seats <= 2 {
		return 24
	}
	return 20
}

// scaleOf ist der Maßstab des HUD in der Zelle c: Fensterhöhe / 1080. Die Zellen liegen übereinander (game/layout.go),
// die Fensterhöhe ist also Zellenhöhe × Zahl der Zellen.
func scaleOf(c view.Cell, cells int) float64 {
	h := float64(c.Rect.Dy() * max(1, cells))
	if h <= 0 {
		return 1
	}
	return h / refHeight
}

// luminance ist die relative Helligkeit nach WCAG.
func luminance(c color.RGBA) float64 {
	ch := func(v uint8) float64 {
		x := float64(v) / 255
		if x <= 0.03928 {
			return x / 12.92
		}
		return math.Pow((x+0.055)/1.055, 2.4)
	}
	return 0.2126*ch(c.R) + 0.7152*ch(c.G) + 0.0722*ch(c.B)
}

// contrastRatio ist das Kontrastverhältnis zweier Farben nach WCAG (1 bis 21).
func contrastRatio(a, b color.RGBA) float64 {
	la, lb := luminance(a), luminance(b)
	return (max(la, lb) + 0.05) / (min(la, lb) + 0.05)
}

// rgb macht aus 0xRRGGBB eine Farbe mit Deckkraft alpha (0..1, vormultipliziert).
func rgb(hex uint32, alpha float64) color.RGBA {
	a := math.Max(0, math.Min(1, alpha))
	return color.RGBA{uint8(float64(hex>>16&0xff) * a), uint8(float64(hex>>8&0xff) * a), uint8(float64(hex&0xff) * a), uint8(255 * a)}
}

// fontPx ist die Größe der Rolle bei 1080, nie unter der Mindestgröße ihrer Art (beide nativen Layouts: Zellen über die
// ganze Breite).
func fontPx(r fontRole, cells int) float64 { return max(r.px, minFontPx(cells, true, r.art)) }
