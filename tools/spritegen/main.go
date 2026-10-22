// spritegen zeichnet die Figuren-Sheets für Eisenkäfer und Kristallspinne (GR8.1, B-329).
// Deterministisch, nur DB32-Farben, ohne Zufall: gleiche Eingabe ergibt byte-gleiche PNG.
// Aufruf: go run ./tools/spritegen [-out public/sprites]; mit -monarch die Monarch-Animationen (monarch.go).
package main

import (
	"flag"
	"fmt"
	"image"
	"image/color"
	"image/png"
	"math"
	"os"
	"path/filepath"
)

const (
	frameW  = 32
	frameH  = 24
	groundY = 22
)

// DB32-Farben (Namen nach Lospec), nur die hier benutzten.
var (
	cOutline = rgb(0x222034)
	cDark    = rgb(0x595652)
	cMid     = rgb(0x696a6a)
	cLight   = rgb(0x9badb7)
	cShine   = rgb(0xcbdbfc)
	cEye     = rgb(0xd95763)
	cBlueD   = rgb(0x3f3f74)
	cBlue    = rgb(0x306082)
	cBlueL   = rgb(0x5fcde4)
	cLimb    = rgb(0x323c39)
)

func rgb(v uint32) color.NRGBA {
	return color.NRGBA{R: uint8(v >> 16), G: uint8(v >> 8), B: uint8(v), A: 0xff}
}

type frame struct{ img *image.NRGBA }

func newFrame() *frame { return &frame{image.NewNRGBA(image.Rect(0, 0, frameW, frameH))} }

func (f *frame) set(x, y int, c color.NRGBA) {
	if x >= 0 && y >= 0 && x < frameW && y < frameH {
		f.img.SetNRGBA(x, y, c)
	}
}

// ellipse füllt eine Ellipse; row(y, dx) liefert die Farbe je Pixel (Schattierung).
func (f *frame) ellipse(cx, cy, rx, ry float64, shade func(dx, dy float64) color.NRGBA) {
	for y := int(cy - ry - 1); y <= int(cy+ry+1); y++ {
		for x := int(cx - rx - 1); x <= int(cx+rx+1); x++ {
			dx, dy := (float64(x)-cx)/rx, (float64(y)-cy)/ry
			if dx*dx+dy*dy <= 1 {
				f.set(x, y, shade(dx, dy))
			}
		}
	}
}

func (f *frame) line(x0, y0, x1, y1 int, c color.NRGBA) {
	dx, dy := abs(x1-x0), -abs(y1-y0)
	sx, sy := sign(x1-x0), sign(y1-y0)
	err := dx + dy
	for {
		f.set(x0, y0, c)
		if x0 == x1 && y0 == y1 {
			return
		}
		e2 := 2 * err
		if e2 >= dy {
			err += dy
			x0 += sx
		}
		if e2 <= dx {
			err += dx
			y0 += sy
		}
	}
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func sign(a int) int {
	switch {
	case a < 0:
		return -1
	case a > 0:
		return 1
	}
	return 0
}

// outline legt eine dunkle Kontur um alle gefüllten Pixel.
func (f *frame) outline() {
	src := image.NewNRGBA(f.img.Rect)
	copy(src.Pix, f.img.Pix)
	for y := 0; y < frameH; y++ {
		for x := 0; x < frameW; x++ {
			if src.NRGBAAt(x, y).A != 0 {
				continue
			}
			for _, d := range [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}} {
				nx, ny := x+d[0], y+d[1]
				if nx >= 0 && ny >= 0 && nx < frameW && ny < frameH && src.NRGBAAt(nx, ny).A != 0 {
					f.img.SetNRGBA(x, y, cOutline)
					break
				}
			}
		}
	}
}

// pose beschreibt eine Bewegungsphase: p in [0,1) Takt, lunge = Vorstoß in Pixeln (Angriff).
type pose struct {
	p     float64
	bob   int
	lunge int
}

type anim struct {
	name   string
	frames int
	pose   func(i, n int) pose
}

var anims = []anim{
	{"idle", 4, func(i, n int) pose { return pose{p: float64(i) / float64(n), bob: []int{0, 0, 1, 1}[i]} }},
	{"run", 6, func(i, n int) pose { return pose{p: float64(i) / float64(n), bob: i % 2} }},
	{"attack", 6, func(i, n int) pose { return pose{p: float64(i) / float64(n), lunge: []int{0, -2, 1, 4, 3, 0}[i]} }},
}

func drawBeetle(f *frame, ps pose) {
	by := 14 - ps.bob
	bx := 14 + ps.lunge
	// Beine: drei Paare, gegenläufiger Takt
	for k, lx := range []int{bx - 6, bx - 1, bx + 4} {
		sw := int(math.Round(2 * math.Sin(2*math.Pi*(ps.p+float64(k)/3))))
		f.line(lx, by+4, lx+sw-1, groundY, cLimb)
		f.line(lx+1, by+4, lx+sw+1, groundY, cLimb)
	}
	// Panzer
	f.ellipse(float64(bx), float64(by+1), 10, 7, func(dx, dy float64) color.NRGBA {
		switch {
		case dy > 0.55:
			return cDark
		case dx < -0.1 && dy < -0.15:
			return cShine
		case dx < 0.35 && dy < 0.2:
			return cLight
		}
		return cMid
	})
	f.line(bx, by-5, bx, by+5, cDark) // Naht der Deckflügel
	// Kopf und Zangen
	hx := bx + 10
	f.ellipse(float64(hx), float64(by+3), 3.2, 3, func(dx, dy float64) color.NRGBA {
		if dy < -0.2 {
			return cLight
		}
		return cMid
	})
	f.set(hx+1, by+2, cEye)
	open := 0
	if ps.lunge > 1 {
		open = 2
	}
	f.line(hx+3, by+4, hx+5, by+3-open, cShine)
	f.line(hx+3, by+6, hx+5, by+7+open, cShine)
	f.outline()
}

func drawSpider(f *frame, ps pose) {
	by := 9 - ps.bob
	bx := 14 + ps.lunge
	// vier Beinpaare: Gelenk oben, Fuß am Boden; gegenläufiger Takt
	for k := 0; k < 4; k++ {
		ph := ps.p + float64(k%2)*0.5
		sw := int(math.Round(1.5 * math.Sin(2*math.Pi*ph)))
		lift := 0
		if ps.lunge > 1 && k >= 2 {
			lift = -3 // vordere Beine heben beim Angriff
		}
		fx := bx - 13 + k*7 + sw
		jx := bx - 10 + k*6
		f.line(bx-4+k*2, by+2, jx, by-3+lift/2, cBlueD)
		f.line(jx, by-3+lift/2, fx, groundY+lift, cBlueD)
	}
	// Hinterleib und Kopf aus Kristall
	f.ellipse(float64(bx-3), float64(by+1), 6, 5, func(dx, dy float64) color.NRGBA {
		switch {
		case dx < -0.2 && dy < -0.2:
			return cShine
		case dy < 0.1:
			return cBlueL
		case dy < 0.55:
			return cBlue
		}
		return cBlueD
	})
	f.ellipse(float64(bx+5), float64(by+2), 3.5, 3, func(dx, dy float64) color.NRGBA {
		if dy < 0 {
			return cBlueL
		}
		return cBlue
	})
	f.set(bx+6, by+1, cEye)
	f.set(bx+4, by+1, cEye)
	f.outline()
}

func sheet(draw func(*frame, pose), a anim) *image.NRGBA {
	out := image.NewNRGBA(image.Rect(0, 0, frameW*a.frames, frameH))
	for i := 0; i < a.frames; i++ {
		f := newFrame()
		draw(f, a.pose(i, a.frames))
		for y := 0; y < frameH; y++ {
			for x := 0; x < frameW; x++ {
				out.SetNRGBA(i*frameW+x, y, f.img.NRGBAAt(x, y))
			}
		}
	}
	return out
}

func write(path string, img image.Image) error {
	fh, err := os.Create(path)
	if err != nil {
		return err
	}
	if err := png.Encode(fh, img); err != nil {
		_ = fh.Close()
		return err
	}
	return fh.Close()
}

func main() {
	out := flag.String("out", "public/sprites", "Ausgabeordner")
	monarch := flag.Bool("monarch", false, "nur die Monarch-Animationen aus den König-Sheets ableiten (GR9.2)")
	flag.Parse()
	if *monarch {
		if err := runMonarch(*out); err != nil {
			fmt.Fprintln(os.Stderr, "❌", err)
			os.Exit(1)
		}
		fmt.Println("✅ Monarch-Sheets geschrieben:", *out)
		return
	}
	figures := map[string]func(*frame, pose){"iron-beetle": drawBeetle, "crystal-spider": drawSpider}
	for _, name := range []string{"iron-beetle", "crystal-spider"} {
		dir := filepath.Join(*out, name)
		if err := os.MkdirAll(dir, 0o755); err != nil {
			fmt.Fprintln(os.Stderr, "❌", err)
			os.Exit(1)
		}
		for _, a := range anims {
			if err := write(filepath.Join(dir, a.name+".png"), sheet(figures[name], a)); err != nil {
				fmt.Fprintln(os.Stderr, "❌", err)
				os.Exit(1)
			}
		}
	}
	fmt.Println("✅ Sheets geschrieben:", *out)
}
