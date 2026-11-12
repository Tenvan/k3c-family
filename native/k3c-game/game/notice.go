package game

// Meldung im Fenster (B-810): Fehler und Ladeanzeige als große Debug-Schrift in der Mitte des Fensters. Die Debug-Schrift
// braucht weder die Schrift-Datei noch Grafik, die Meldung erscheint also auch, wenn genau die fehlen; sie kennt nur
// ASCII, Umlaute und Emojis werden vorher umgeschrieben bzw. weggelassen. Text und Maßstab sind reine Funktionen, das
// Textbild entsteht einmal beim ersten Zeichnen.

import (
	"image/color"
	"strings"
	"unicode/utf8"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"
)

// Maße der Debug-Schrift (ebitenutil.DebugPrintAt): Zeichenbreite und Zeilenhöhe in px; noticePad ist der Rand des Bilds.
const (
	noticeCols    = 56
	noticeCharW   = 6
	noticeLineH   = 16
	noticePad     = 8
	noticeMaxZoom = 6
)

var (
	noticeBack = color.RGBA{16, 16, 24, 255}
	umlauts    = strings.NewReplacer("ä", "ae", "ö", "oe", "ü", "ue", "Ä", "Ae", "Ö", "Oe", "Ü", "Ue", "ß", "ss")
)

// asciiText schreibt Umlaute um (ä → ae, ß → ss) und lässt alle anderen Zeichen außerhalb von ASCII weg (Emojis); eine
// Lücke vor dem weggelassenen Zeichen bleibt nicht stehen.
func asciiText(s string) string {
	var b strings.Builder
	for _, r := range umlauts.Replace(s) {
		if r < utf8.RuneSelf {
			b.WriteRune(r)
		}
	}
	return strings.TrimSpace(b.String())
}

// wrapText bricht s in Zeilen von höchstens cols Zeichen: an Leerzeichen, ein längeres Wort (Pfad) hart. Leere Zeilen
// bleiben, damit Absätze erhalten bleiben.
func wrapText(s string, cols int) []string {
	if s == "" {
		return []string{""}
	}
	var out []string
	line := ""
	flush := func() {
		out = append(out, line)
		line = ""
	}
	for _, w := range strings.Fields(s) {
		for len(w) > cols {
			if line != "" {
				flush()
			}
			out = append(out, w[:cols])
			w = w[cols:]
		}
		switch {
		case line == "":
			line = w
		case len(line)+1+len(w) <= cols:
			line += " " + w
		default:
			flush()
			line = w
		}
	}
	if line != "" {
		flush()
	}
	return out
}

// noticeLines macht aus den Absätzen die Zeilen der Meldung (ASCII, umbrochen).
func noticeLines(paras []string) []string {
	var out []string
	for _, p := range paras {
		out = append(out, wrapText(asciiText(p), noticeCols)...)
	}
	return out
}

// capLines kürzt die Zeilen auf höchstens n; die letzte Zeile ist dann „...“.
func capLines(lines []string, n int) []string {
	if len(lines) <= n {
		return lines
	}
	return append(lines[:n-1:n-1], "...")
}

// noticeScale ist der ganzzahlige Maßstab, mit dem ein Textbild von iw × ih px in ein Fenster von w × h px passt
// (höchstens 90 % des Fensters, mindestens 1, höchstens noticeMaxZoom).
func noticeScale(w, h, iw, ih int) int {
	if iw <= 0 || ih <= 0 {
		return 1
	}
	return max(1, min(noticeMaxZoom, w*9/10/iw, h*9/10/ih))
}

// notice ist eine Meldung: ihre Zeilen und das Textbild (lazy).
type notice struct {
	lines []string
	img   *ebiten.Image
	op    ebiten.DrawImageOptions
}

// newNotice baut die Meldung aus Absätzen (jeder wird umbrochen).
func newNotice(paras ...string) *notice { return &notice{lines: noticeLines(paras)} }

// size ist die Größe des Textbilds in px.
func (n *notice) size() (w, h int) {
	return noticeCols*noticeCharW + 2*noticePad, len(n.lines)*noticeLineH + 2*noticePad
}

// draw füllt den Bildschirm dunkel und zeichnet die Meldung vergrößert in die Mitte; legt nach dem ersten Bild nichts an.
func (n *notice) draw(screen *ebiten.Image) {
	iw, ih := n.size()
	if n.img == nil {
		n.img = ebiten.NewImage(iw, ih)
		for i, l := range n.lines {
			ebitenutil.DebugPrintAt(n.img, l, noticePad, noticePad+i*noticeLineH)
		}
	}
	screen.Fill(noticeBack)
	b := screen.Bounds()
	k := noticeScale(b.Dx(), b.Dy(), iw, ih)
	n.op.GeoM.Reset()
	n.op.GeoM.Scale(float64(k), float64(k))
	n.op.GeoM.Translate(float64(b.Min.X+(b.Dx()-iw*k)/2), float64(b.Min.Y+(b.Dy()-ih*k)/2))
	screen.DrawImage(n.img, &n.op)
}
