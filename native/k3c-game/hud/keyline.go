package hud

// Bedienhinweis mit Tasten-Icons (B-911): eine Zeile aus Text und Glyphen, gebaut aus der Vorlage eines Textes
// („{move} wählen · {ok} bestätigen“) und den Tasten des Eingabe-Profils. Skill-Menü und Seekarte zeichnen ihren
// Hinweis damit; sind die Icons der Zeile nicht geladen, steht derselbe Hinweis als reiner Text (plain) wie vor den
// Icons. Gebaut wird nur bei Änderung, gemessen einmal bei lineRef; je Bild rechnet die Zeile nur um.

import (
	"image/color"
	"math"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
)

const (
	lineSegs = 10 // Teile je Zeile höchstens (Text und Glyphen im Wechsel)
	lineRef  = 64 // Schriftgröße, bei der die Teile gemessen sind
)

// lineGlyphPx ist die Glyph-Höhe in einer Zeile der Schriftgröße size: höher als in den Menüzeilen (glyphPx), damit die
// Beschriftung der Kappen neben dem kleineren Hinweistext lesbar bleibt.
func lineGlyphPx(size float64) float64 { return math.Ceil(size * 1.5) }

// lineSeg ist ein Teil der Zeile: g.key "" = Text g.label, sonst die Glyph; tw und th sind Breite und Höhe von g.label
// bei lineRef.
type lineSeg struct {
	g      glyph
	tw, th float64
}

// keyLine ist ein Bedienhinweis: n Teile und derselbe Hinweis als Text. n 0 = nur Text.
type keyLine struct {
	segs     [lineSegs]lineSeg
	n        int
	plain    string
	measured bool
}

// lineArg ist der Wert eines Platzhalters der Vorlage: Glyphen und Text (key "") in Reihenfolge.
type lineArg struct {
	name  string
	parts []glyph
}

func arg(name string, parts ...glyph) lineArg { return lineArg{name, parts} }

// textPart ist ein Text-Teil ohne Bild.
func textPart(s string) glyph { return glyph{label: s} }

// newKeyLine zerlegt die Vorlage tmpl an ihren Platzhaltern `{name}` in Text und die Teile aus args; ein unbekannter
// Platzhalter bleibt als Text stehen. Mehr als lineSegs Teile: nur Text.
func newKeyLine(tmpl string, args ...lineArg) keyLine {
	var l keyLine
	var plain strings.Builder
	for tmpl != "" {
		i := strings.IndexByte(tmpl, '{')
		j := strings.IndexByte(tmpl, '}')
		if i < 0 || j < i {
			l.add(&plain, textPart(tmpl))
			break
		}
		l.add(&plain, textPart(tmpl[:i]))
		name := tmpl[i+1 : j]
		k := 0
		for k < len(args) && args[k].name != name {
			k++
		}
		if k == len(args) {
			l.add(&plain, textPart(tmpl[i:j+1]))
		} else {
			for _, g := range args[k].parts {
				l.add(&plain, g)
			}
		}
		tmpl = tmpl[j+1:]
	}
	l.plain = plain.String()
	if l.n > lineSegs {
		l.n = 0
	}
	return l
}

// add hängt den Teil g an: Text ohne Ränder (den Abstand setzt die Zeile), Text nach Text wächst zusammen.
func (l *keyLine) add(plain *strings.Builder, g glyph) {
	plain.WriteString(g.label)
	if g.key == "" {
		g.label = strings.TrimSpace(g.label)
		if g.label == "" {
			return
		}
		if l.n > 0 && l.n <= lineSegs && l.segs[l.n-1].g.key == "" {
			l.segs[l.n-1].g.label += " " + g.label
			return
		}
	}
	if l.n < lineSegs {
		l.segs[l.n].g = g
	}
	l.n++
}

// lineIcons meldet, ob die Zeile mit Icons gezeichnet wird: Sie hat Teile und jede Glyph ein geladenes Icon.
func (h *hud) lineIcons(l *keyLine) bool {
	for i := range l.n {
		if g := l.segs[i].g; g.key != "" && h.glyphIcon(g) == nil {
			return false
		}
	}
	return l.n > 0
}

// measure misst die Teile einmal bei lineRef.
func (l *keyLine) measure(f *art.Font) {
	if l.measured {
		return
	}
	for i := range l.n {
		s := &l.segs[i]
		s.tw, s.th = f.Measure(s.g.label, lineRef)
	}
	l.measured = true
}

// segWidth ist die Breite von Teil i bei Schriftgröße size und Glyph-Höhe gh; für eine Tastenkappe dazu Breite und Höhe
// ihrer Beschriftung bei gh·glyphLabelK. Die Debug-Schrift hat eine feste Größe, ihre Maße gelten unverändert.
func (l *keyLine) segWidth(i int, f *art.Font, size, gh float64) (w, tw, th float64) {
	s := &l.segs[i]
	r := size / lineRef
	if isKeyGlyph(s.g) {
		r = gh * glyphLabelK / lineRef
	}
	if f.Fallback {
		r = 1
	}
	tw, th = s.tw*r, s.th*r
	switch {
	case s.g.key == "":
		return tw, tw, th
	case isKeyGlyph(s.g):
		w, _ = capBox(gh, tw, s.g.icon == capWide, f.Fallback)
		return w, tw, th
	}
	return gh, 0, 0
}

// lineWidth ist die Breite der Zeile l bei Schriftgröße size; false = ohne Icons, es gilt der Text l.plain.
func (h *hud) lineWidth(f *art.Font, l *keyLine, size float64) (float64, bool) {
	if f == nil || !h.lineIcons(l) {
		return 0, false
	}
	l.measure(f)
	w, gh := 0.0, lineGlyphPx(size)
	for i := range l.n {
		sw, _, _ := l.segWidth(i, f, size, gh)
		w += sw
	}
	return w + float64(l.n-1)*spaceW(size), true
}

// drawLine zeichnet die Zeile l mit der linken oberen Ecke des Textes bei (x, y) in Schriftgröße size, Glyphen mittig
// zur Textzeile; false = ohne Icons nichts gezeichnet (der Aufrufer zeichnet l.plain).
func (h *hud) drawLine(dst *ebiten.Image, f *art.Font, l *keyLine, x, y, size float64, clr color.RGBA) bool {
	if f == nil || !h.lineIcons(l) {
		return false
	}
	l.measure(f)
	gh := lineGlyphPx(size)
	for i := range l.n {
		g := l.segs[i].g
		w, tw, th := l.segWidth(i, f, size, gh)
		if g.key == "" {
			h.paint.text(dst, f, g.label, x, y, size, clr)
		} else {
			h.iconGlyph(dst, f, h.glyphIcon(g), g, x, y+size*0.6, gh, tw, th)
		}
		x += w + spaceW(size)
	}
	return true
}

// drawHint zeichnet den Bedienhinweis: die Zeile l mit Icons, sonst den Text plain.
func (h *hud) drawHint(dst *ebiten.Image, f *art.Font, l *keyLine, plain string, x, y, size float64) {
	if !h.drawLine(dst, f, l, x, y, size, white) {
		h.paint.text(dst, f, plain, x, y, size, white)
	}
}
