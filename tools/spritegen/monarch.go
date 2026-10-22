package main

// Monarch-Animationen (GR9.2, B-505): leitet aus idle.png und attack.png der König-Sheets (LuizMelo, CC0) die
// Streifen give, hit, death und respawn ab. Deterministisch, ohne Zufall, Nearest-Neighbor; Frame-Größen bleiben.
// Aufruf: go run ./tools/spritegen -monarch [-out public/sprites]

import (
	"fmt"
	"image"
	"image/png"
	"math"
	"os"
	"path/filepath"
)

// kingSheet: Frame-Größe, Fußpunkt (Pixel, wie originX/originY in data/sprites.json) und Zahl der Attack-Frames für give.
type kingSheet struct {
	name         string
	fw, fh       int
	footX, footY int
	give         int
}

var kings = []kingSheet{
	{name: "medieval-king", fw: 155, fh: 155, footX: 66, footY: 116, give: 3},
	{name: "medieval-king-2", fw: 160, fh: 111, footX: 81, footY: 105, give: 2},
}

// Reihenfolge der erzeugten Streifen; Frames und fps stehen in data/sprites.json.
var monarchAnims = []string{"give", "hit", "death", "respawn"}

var (
	deathAngles  = []float64{15, 35, 60, 80, 90}    // Kippen nach vorn um den Fußpunkt, letzter Frame hält
	hitFlash     = []float64{0.75, 0.45, 0.15}      // Mischung mit Weiß je Frame
	hitPush      = []int{-3, -2, -1}                // Rückstoß in Pixeln (Blick nach rechts)
	respawnShown = []float64{0.2, 0.4, 0.6, 0.8, 1} // sichtbarer Anteil der Figur von unten
	respawnFlash = []float64{0.8, 0.6, 0.4, 0.2, 0}
)

func readPNG(path string) (*image.NRGBA, error) {
	fh, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer func() { _ = fh.Close() }()
	src, err := png.Decode(fh)
	if err != nil {
		return nil, fmt.Errorf("%s: %w", filepath.ToSlash(path), err)
	}
	out := image.NewNRGBA(image.Rect(0, 0, src.Bounds().Dx(), src.Bounds().Dy()))
	for y := 0; y < out.Rect.Dy(); y++ {
		for x := 0; x < out.Rect.Dx(); x++ {
			out.Set(x, y, src.At(src.Bounds().Min.X+x, src.Bounds().Min.Y+y))
		}
	}
	return out, nil
}

// cutFrame liefert Frame i eines Streifens als eigenes Bild.
func cutFrame(strip *image.NRGBA, i, fw, fh int) *image.NRGBA {
	out := image.NewNRGBA(image.Rect(0, 0, fw, fh))
	for y := 0; y < fh; y++ {
		copy(out.Pix[y*out.Stride:y*out.Stride+fw*4], strip.Pix[y*strip.Stride+i*fw*4:y*strip.Stride+(i+1)*fw*4])
	}
	return out
}

// whiten mischt jedes sichtbare Pixel mit Weiß (t = 0 unverändert, 1 = weiß); Alpha bleibt.
func whiten(src *image.NRGBA, t float64) *image.NRGBA {
	out := image.NewNRGBA(src.Rect)
	copy(out.Pix, src.Pix)
	for i := 0; i < len(out.Pix); i += 4 {
		if out.Pix[i+3] == 0 {
			continue
		}
		for c := 0; c < 3; c++ {
			v := float64(out.Pix[i+c])
			out.Pix[i+c] = uint8(math.Round(v + (255-v)*t))
		}
	}
	return out
}

// shift verschiebt das Bild waagerecht um dx Pixel (herausgeschobene Pixel fallen weg).
func shift(src *image.NRGBA, dx int) *image.NRGBA {
	out := image.NewNRGBA(src.Rect)
	w, h := src.Rect.Dx(), src.Rect.Dy()
	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			if nx := x + dx; nx >= 0 && nx < w {
				out.SetNRGBA(nx, y, src.NRGBAAt(x, y))
			}
		}
	}
	return out
}

// opaqueBounds: kleinstes Rechteck um alle sichtbaren Pixel (leer bei leerem Bild).
func opaqueBounds(img *image.NRGBA) image.Rectangle {
	var r image.Rectangle
	b := img.Rect
	for y := b.Min.Y; y < b.Max.Y; y++ {
		for x := b.Min.X; x < b.Max.X; x++ {
			if img.Pix[img.PixOffset(x, y)+3] != 0 {
				r = r.Union(image.Rect(x, y, x+1, y+1))
			}
		}
	}
	return r
}

// rotateBig dreht src um den Fußpunkt (Uhrzeigersinn auf dem Bildschirm = Kippen nach vorn) auf einer dreifach großen
// Fläche, damit nichts abgeschnitten wird. Der Fußpunkt liegt dort bei (fw+footX, fh+footY).
func rotateBig(src *image.NRGBA, k kingSheet, deg float64) *image.NRGBA {
	big := image.NewNRGBA(image.Rect(0, 0, 3*k.fw, 3*k.fh))
	cx, cy := float64(k.fw+k.footX), float64(k.fh+k.footY)
	sin, cos := math.Sincos(deg * math.Pi / 180)
	for y := 0; y < big.Rect.Dy(); y++ {
		for x := 0; x < big.Rect.Dx(); x++ {
			dx, dy := float64(x)+0.5-cx, float64(y)+0.5-cy
			sx := int(math.Floor(dx*cos + dy*sin + float64(k.footX)))
			sy := int(math.Floor(-dx*sin + dy*cos + float64(k.footY)))
			if image.Pt(sx, sy).In(src.Rect) {
				big.SetNRGBA(x, y, src.NRGBAAt(sx, sy))
			}
		}
	}
	return big
}

// tilt kippt den Frame um den Fußpunkt und schneidet ihn wieder auf Frame-Größe zu: Was unter den Boden ragt, wird
// angehoben, was seitlich herausragt, eingerückt. Fehler, wenn die Figur trotzdem nicht in den Frame passt.
func tilt(src *image.NRGBA, k kingSheet, deg float64) (*image.NRGBA, error) {
	big := rotateBig(src, k, deg)
	bb := opaqueBounds(big)
	ox, oy := k.fw, k.fh
	if ground := k.fh + k.footY + 1; bb.Max.Y > ground {
		oy += bb.Max.Y - ground
	}
	if bb.Min.X < ox {
		ox = bb.Min.X
	}
	if bb.Max.X > ox+k.fw {
		ox = bb.Max.X - k.fw
	}
	crop := image.Rect(ox, oy, ox+k.fw, oy+k.fh)
	if !bb.In(crop) {
		return nil, fmt.Errorf("%s: Kippen um %.0f° passt nicht in %dx%d (Figur %v)", k.name, deg, k.fw, k.fh, bb.Size())
	}
	out := image.NewNRGBA(image.Rect(0, 0, k.fw, k.fh))
	for y := 0; y < k.fh; y++ {
		copy(out.Pix[y*out.Stride:(y+1)*out.Stride], big.Pix[big.PixOffset(ox, oy+y):big.PixOffset(ox+k.fw, oy+y)])
	}
	return out, nil
}

// reveal zeigt nur den unteren Anteil shown (0..1) der Figur, gemessen vom Fußpunkt bis zur Oberkante.
func reveal(src *image.NRGBA, footY int, shown float64) *image.NRGBA {
	top := opaqueBounds(src).Min.Y
	cut := footY + 1 - int(math.Ceil(shown*float64(footY+1-top)))
	out := image.NewNRGBA(src.Rect)
	for y := max(cut, 0); y < src.Rect.Dy(); y++ {
		copy(out.Pix[y*out.Stride:(y+1)*out.Stride], src.Pix[y*src.Stride:(y+1)*src.Stride])
	}
	return out
}

// deriveMonarch erzeugt die Frames aller vier Animationen aus idle- und attack-Streifen.
func deriveMonarch(k kingSheet, idleStrip, attackStrip *image.NRGBA) (map[string][]*image.NRGBA, error) {
	idle := cutFrame(idleStrip, 0, k.fw, k.fh)
	out := map[string][]*image.NRGBA{}
	for i := 0; i < k.give; i++ {
		out["give"] = append(out["give"], cutFrame(attackStrip, i, k.fw, k.fh))
	}
	out["give"] = append(out["give"], idle) // zurück in die Ruhe
	for i, t := range hitFlash {
		out["hit"] = append(out["hit"], shift(whiten(idle, t), hitPush[i]))
	}
	for _, deg := range deathAngles {
		f, err := tilt(idle, k, deg)
		if err != nil {
			return nil, err
		}
		out["death"] = append(out["death"], f)
	}
	for i, s := range respawnShown {
		out["respawn"] = append(out["respawn"], whiten(reveal(idle, k.footY, s), respawnFlash[i]))
	}
	return out, nil
}

// joinFrames legt Frames gleicher Größe zu einem waagerechten Streifen zusammen.
func joinFrames(frames []*image.NRGBA) *image.NRGBA {
	fw, fh := frames[0].Rect.Dx(), frames[0].Rect.Dy()
	out := image.NewNRGBA(image.Rect(0, 0, fw*len(frames), fh))
	for i, f := range frames {
		for y := 0; y < fh; y++ {
			copy(out.Pix[y*out.Stride+i*fw*4:y*out.Stride+(i+1)*fw*4], f.Pix[y*f.Stride:y*f.Stride+fw*4])
		}
	}
	return out
}

// runMonarch schreibt give/hit/death/respawn.png in die Ordner der König-Sheets unter dir.
func runMonarch(dir string) error {
	for _, k := range kings {
		idle, err := readPNG(filepath.Join(dir, k.name, "idle.png"))
		if err != nil {
			return err
		}
		attack, err := readPNG(filepath.Join(dir, k.name, "attack.png"))
		if err != nil {
			return err
		}
		anims, err := deriveMonarch(k, idle, attack)
		if err != nil {
			return err
		}
		for _, name := range monarchAnims {
			if err := write(filepath.Join(dir, k.name, name+".png"), joinFrames(anims[name])); err != nil {
				return err
			}
			fmt.Printf("🔨 %s/%s.png: %d Frames\n", k.name, name, len(anims[name]))
		}
	}
	return nil
}
