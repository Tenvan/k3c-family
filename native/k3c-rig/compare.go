package main

// Bildvergleich (`--vergleich <ordner>`, B-778/AC-04): zeichnet den Rig-Monarchen in den Posen der Sheet-Frames in ein
// Bild von der Größe des Frames und legt es gegen den Sheet-Frame. Der Umhang läuft wie im Sheet (240 Schritte
// Einschwingen), ist also im Vergleich enthalten. Ausgabe: Kennzahlen als JSON und je Animation ein Differenzbild
// (Zeilen: Sheet, Rig, Differenz ×4; Spalten: Frames).

import (
	"encoding/json"
	"fmt"
	"image"
	"math"
	"os"
	"path/filepath"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-rig/paint"
	"k3c/native/k3c-rig/rig"
)

// Animationen des Vergleichs: zu Fuß und der Reiter mit Pferd.
var compareAnims = []string{"idle", "lauf", "trab"}

// Schwellen je Kanal (0..255), ab denen ein Pixel als abweichend zählt
const (
	diffLow  = 8
	diffHigh = 32
	diffGain = 4
)

type FrameDiff struct {
	Frame     int     `json:"frame"`
	MeanAbs   float64 `json:"mittel_abweichung"` // mittlere Abweichung je Kanal über die belegten Pixel
	Max       int     `json:"max_abweichung"`    // größte Abweichung eines Kanals
	ShareLow  float64 `json:"anteil_ueber_8"`    // Anteil belegter Pixel mit Abweichung > 8
	ShareHigh float64 `json:"anteil_ueber_32"`   // Anteil belegter Pixel mit Abweichung > 32
	Covered   int     `json:"belegte_pixel"`     // Pixel, die in Sheet oder Rig nicht leer sind
	OnlySheet int     `json:"nur_im_sheet"`      // deckend im Sheet, leer im Rig
	OnlyRig   int     `json:"nur_im_rig"`        // deckend im Rig, leer im Sheet
	sumAbs    float64
	low, high int
}

type AnimDiff struct {
	Animation string      `json:"animation"`
	FrameSize [2]int      `json:"frame"`
	Image     string      `json:"differenzbild"`
	Total     FrameDiff   `json:"gesamt"`
	Frames    []FrameDiff `json:"frames"`
}

type CompareReport struct {
	Variante  string     `json:"variante"`
	Schwellen [2]int     `json:"schwellen"`
	Hinweis   string     `json:"hinweis"`
	Anims     []AnimDiff `json:"animationen"`
}

type Compare struct {
	dir   string
	actor *paint.Actor
	ticks int
}

func NewCompare(g *Game) (*Compare, error) {
	if g.sheets == nil {
		return nil, fmt.Errorf("der Bildvergleich braucht die Sheets des Monarchen (erst `task gen:paper`)")
	}
	a, err := NewActor(g.assets, 0)
	return &Compare{dir: g.args.compare, actor: a}, err
}

func (c *Compare) Draw(*Game, *ebiten.Image) {}

// Update rechnet den ganzen Vergleich in einem Takt (das Fenster zeigt nichts) und beendet das Programm.
func (c *Compare) Update(g *Game, _ float64) error {
	if c.ticks++; c.ticks < 3 { // erst nach den ersten Bildern: dann steht die Grafik sicher
		return nil
	}
	g.quit = true
	report := CompareReport{Variante: variants[0], Schwellen: [2]int{diffLow, diffHigh},
		Hinweis: "Abweichung je Kanal 0..255, vormultipliziertes RGBA; belegte Pixel = im Sheet oder im Rig nicht leer"}
	for _, name := range compareAnims {
		d, err := c.compareAnim(g, name)
		if err != nil {
			return err
		}
		logf("✅ Vergleich %s: mittlere Abweichung %.2f, max %d, %.1f %% der belegten Pixel über %d", name, d.Total.MeanAbs, d.Total.Max, d.Total.ShareHigh*100, diffHigh)
		report.Anims = append(report.Anims, d)
	}
	text, err := json.MarshalIndent(report, "", "  ")
	if err != nil {
		return err
	}
	path := filepath.Join(c.dir, "vergleich.json")
	if err := os.WriteFile(path, text, 0o644); err != nil {
		return err
	}
	logf("💾 Bericht: %s", path)
	return nil
}

// chains liefert die Umhang-Kette je Frame wie im Sheet und stellt den Zustand des Monarchen ein.
func (c *Compare) chains(g *Game, name string, frames int) ([][]rig.Point, error) {
	c.actor.State = states[stateIndex(name)]
	if gait := c.actor.State.Gait; gait != "" {
		return rig.RideCloakFrames(g.assets.horse, g.assets.monarch, gait, frames)
	}
	return g.assets.monarch.FigureCloakFrames(name)
}

func (c *Compare) compareAnim(g *Game, name string) (AnimDiff, error) {
	sh, err := g.sheets.sheet(name, variants[0])
	if err != nil {
		return AnimDiff{}, err
	}
	n := len(sh.frames)
	chains, err := c.chains(g, name, n)
	if err != nil {
		return AnimDiff{}, err
	}
	out := AnimDiff{Animation: name, FrameSize: [2]int{sh.fw, sh.fh}, Image: "vergleich-" + name + ".png"}
	sheet := image.NewRGBA(image.Rect(0, 0, n*sh.fw, 3*sh.fh))
	off := ebiten.NewImage(sh.fw, sh.fh)
	defer off.Deallocate()
	pix := make([]byte, 4*sh.fw*sh.fh)
	sx, sy := float64(sh.fw)/sh.frame[2], float64(sh.fh)/sh.frame[3]
	for i := range n {
		if err := c.actor.SetFrame(i, n, chains[i]); err != nil {
			return out, err
		}
		off.Clear()
		g.painter.Dst, g.painter.View = off, rig.M{sx, 0, 0, sy, -sh.frame[0] * sx, -sh.frame[1] * sy}
		c.actor.Draw(g.painter)
		off.ReadPixels(pix)
		d := diffFrame(sh, i, pix, sheet)
		d.Frame = i
		out.Total.add(d)
		out.Frames = append(out.Frames, d.finish())
	}
	out.Total = out.Total.finish()
	return out, paint.WritePNG(filepath.Join(c.dir, out.Image), sheet)
}

// diffFrame vergleicht Frame n des Sheets mit dem Rig-Bild und füllt die drei Zeilen des Differenzbilds.
func diffFrame(sh *Sheet, n int, rigPix []byte, out *image.RGBA) FrameDiff {
	var d FrameDiff
	for y := range sh.fh {
		for x := range sh.fw {
			a := sh.pix.Pix[sh.pix.PixOffset(n*sh.fw+x, y):][:4]
			b := rigPix[4*(y*sh.fw+x):][:4]
			copy(out.Pix[out.PixOffset(n*sh.fw+x, y):], a)
			copy(out.Pix[out.PixOffset(n*sh.fw+x, sh.fh+y):], b)
			top := d.pixel(a, b)
			v := uint8(min(255, top*diffGain))
			copy(out.Pix[out.PixOffset(n*sh.fw+x, 2*sh.fh+y):], []byte{v, v, v, 255})
		}
	}
	return d
}

// pixel zählt einen Bildpunkt und liefert seine größte Abweichung über die vier Kanäle.
func (d *FrameDiff) pixel(a, b []byte) int {
	if a[3] == 0 && b[3] == 0 {
		return 0
	}
	d.Covered++
	top := 0
	for k := range 4 {
		diff := int(math.Abs(float64(int(a[k]) - int(b[k]))))
		d.sumAbs += float64(diff)
		top = max(top, diff)
	}
	if top > diffLow {
		d.low++
	}
	if top > diffHigh {
		d.high++
	}
	if a[3] >= 128 && b[3] == 0 {
		d.OnlySheet++
	}
	if b[3] >= 128 && a[3] == 0 {
		d.OnlyRig++
	}
	d.Max = max(d.Max, top)
	return top
}

func (d *FrameDiff) add(o FrameDiff) {
	d.Covered += o.Covered
	d.sumAbs += o.sumAbs
	d.low += o.low
	d.high += o.high
	d.OnlySheet += o.OnlySheet
	d.OnlyRig += o.OnlyRig
	d.Max = max(d.Max, o.Max)
}

// finish rechnet die Anteile aus den Zählern (auf 4 Stellen gerundet).
func (d FrameDiff) finish() FrameDiff {
	if d.Covered > 0 {
		c := float64(d.Covered)
		d.MeanAbs = round4(d.sumAbs / (4 * c))
		d.ShareLow, d.ShareHigh = round4(float64(d.low)/c), round4(float64(d.high)/c)
	}
	return d
}

func round4(v float64) float64 { return math.Round(v*1e4) / 1e4 }
