package hud

// Aufnahme der Tasten-Icons im Fenster (B-911, NA1.5): oben je Eingabe-Profil und Sprache alle Glyphen und die
// Bedienhinweise von Skill-Menü und Seekarte, dazu überlange Beschriftungen; unten das Skill-Menü zweier Spieler
// (Profil pfeile und Pad). Schreibt ein Bild für die Sichtprüfung und zählt die Allokationen eines Aufrufs wie die
// Aufnahme der Seekarte (seekarte_shot_test.go): nur ganze Vielfache der Zahl je Zeichenbefehl.
//
//	K3C_GLYPH_SHOT=<bild.png> go test ./hud/   (im Ordner native/k3c-game; Grafik vorher mit `task gen:paper`)

import (
	"errors"
	"fmt"
	"image"
	"image/color"
	"math"
	"os"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

const glyphShotEnv = "K3C_GLYPH_SHOT"

// shotGlyph ist eine Glyph der Aufnahme mit den Maßen ihrer Beschriftung bei Glyph-Höhe shotGlyphPx.
type shotGlyph struct {
	g      glyph
	tw, th float64
}

// shotRow ist eine Zeile der Aufnahme: Titel, Glyphen und zwei Bedienhinweise.
type shotRow struct {
	title     string
	glyphs    []shotGlyph
	menu, sea keyLine
}

const shotGlyphPx = 44.0

type glyphShotGame struct {
	h            *hud
	rows         []shotRow
	cells        [2]view.Cell
	shot         *ebiten.Image
	frame        int
	drawN, callN float64
	out          string
	err          error
}

func (g *glyphShotGame) Layout(int, int) (int, int) { return shotW, shotH }

func (g *glyphShotGame) Update() error {
	if g.frame <= shotWarm {
		return nil
	}
	g.err = writeSeaShot(g.shot, g.out)
	return ebiten.Termination
}

func (g *glyphShotGame) draw() {
	h, f := g.h, g.h.font()
	y := 16.0
	for i := range g.rows {
		r := &g.rows[i]
		h.paint.text(g.shot, f, r.title, 20, y+8, 24, yellow)
		x := 300.0
		for _, s := range r.glyphs {
			x += h.iconGlyph(g.shot, f, h.glyphIcon(s.g), s.g, x, y+shotGlyphPx/2, shotGlyphPx, s.tw, s.th) + 12
		}
		h.drawHint(g.shot, f, &r.menu, r.menu.plain, 300, y+56, 24)
		h.drawHint(g.shot, f, &r.sea, r.sea.plain, 1100, y+56, 24)
		y += 104
	}
	for i := range g.cells {
		h.drawMenu(g.shot, &h.seats[i], g.cells[i], scaleOf(g.cells[i], h.cells))
	}
}

func (g *glyphShotGame) Draw(screen *ebiten.Image) {
	g.frame++
	g.shot.Fill(color.RGBA{0x3c, 0x6e, 0x47, 0xff})
	g.h.paint.stencils()
	if g.frame == shotWarm {
		g.drawN = testing.AllocsPerRun(20, g.draw)
		g.callN = testing.AllocsPerRun(200, func() { g.h.paint.fill(g.shot, rect{0, 0, 2, 2}, white) })
		g.shot.Fill(color.RGBA{0x3c, 0x6e, 0x47, 0xff})
	}
	g.draw()
	screen.DrawImage(g.shot, nil)
}

// glyphShotRow baut die Zeile für Sprache lang und Profil profile: alle Glyphen des Profils und die zwei Hinweise.
func glyphShotRow(f *art.Font, lang, profile string, extra ...glyph) shotRow {
	r := shotRow{title: profile + " / " + lang, menu: menuLine(lang, profile), sea: seaLine(lang, profile, true)}
	add := func(g glyph) {
		s := shotGlyph{g: g}
		if isKeyGlyph(g) {
			s.tw, s.th = f.Measure(g.label, shotGlyphPx*glyphLabelK)
		}
		r.glyphs = append(r.glyphs, s)
	}
	if profile == view.ProfileGamepad {
		for _, name := range padGlyphs {
			add(newGlyph("pad:"+name, name))
		}
	} else {
		for _, a := range KeyActions {
			add(glyphOf(lang, a, profile))
		}
	}
	for _, g := range extra {
		add(g)
	}
	return r
}

// runGlyphShot zeichnet die Tasten-Icons, schreibt das Bild nach out und prüft die Allokationen; Rückgabe ist der
// Exit-Code.
func runGlyphShot(out string) int {
	cfg := art.Config{GrafikDir: repoGrafik}
	if _, err := os.Stat(repoFont); err == nil {
		cfg.FontPath = repoFont
	}
	lib, err := art.New(cfg)
	if err != nil {
		fmt.Println("❌ Grafik:", err)
		return 1
	}
	h := New(lib).(*hud)
	if !h.keyIcons {
		fmt.Println("❌ Tasten-Icons fehlen (erst `task gen:paper`)")
		return 1
	}
	if err := lib.SetLevels(map[string][]string{"forest": h.Assets("forest", nil)}); err != nil {
		fmt.Println("❌ Grafik:", err)
		return 1
	}
	h.cells, h.lang = 2, "de"
	f := h.font()
	g := &glyphShotGame{h: h, out: out, shot: ebiten.NewImage(shotW, shotH)}
	g.rows = []shotRow{
		glyphShotRow(f, "de", view.ProfileWASD),
		glyphShotRow(f, "de", view.ProfileArrows),
		glyphShotRow(f, "en", view.ProfileArrows, newGlyph("key:SHIFTRIGHT", keyLabel("en", "SHIFTRIGHT"))),
		glyphShotRow(f, "de", view.ProfileGamepad, newGlyph("key:X", "Umschalttaste rechts, sehr lang"), newGlyph("key:NUM7", keyLabel("de", "NUM7"))),
		glyphShotRow(f, "en", view.ProfileGamepad),
	}
	es := menuEntries(nil, []string{"taunt"}, 1, only("heal"), true)
	slots := slotViews([]string{"fireball", "taunt", "heal", ""}, nil)
	for i, profile := range []string{view.ProfileArrows, view.ProfileGamepad} {
		g.cells[i] = view.Cell{Index: i, Player: i, Rect: image.Rect(i*shotW/2, shotH/2, (i+1)*shotW/2, shotH)}
		h.seats[i].menuV = buildMenu(newMenuKey(h.lang, profile, i, 1, es, slots), es)
	}
	ebiten.SetVsyncEnabled(false)
	ebiten.SetWindowSize(shotW/2, shotH/2)
	ebiten.SetWindowTitle("K3C Tasten-Icons")
	if err := ebiten.RunGame(g); err != nil && !errors.Is(err, ebiten.Termination) || g.err != nil {
		fmt.Println("❌ Aufnahme:", err, g.err)
		return 1
	}
	fmt.Printf("💾 Bild %s; Allokationen je Aufruf %.0f, je Zeichenbefehl %.0f\n", out, g.drawN, g.callN)
	if g.callN == 0 && g.drawN != 0 || g.callN > 0 && math.Mod(g.drawN, g.callN) != 0 {
		fmt.Println("❌ Zeichnen der Tasten-Icons allokiert über die Zeichenbefehle hinaus")
		return 1
	}
	return 0
}
