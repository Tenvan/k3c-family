package hud

// Zeichnen der Geschichtszeile (story.go, B-974): ein bis zwei Zeilen mittig auf einer dunklen Fläche, Deckkraft nach
// dem Ablauf der Einblendung. Passt die längere Zeile nicht in die Zelle, wird die Schrift kleiner (fitPx), nicht
// umbrochen und nicht gekürzt.

import (
	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
)

// storyFit sind Schriftgröße und Maße der beiden Zeilen eines Eintrags, so dass die längere in die Breite cellW passt.
type storyFit struct{ px, wa, ha, wb, hb float64 }

// fitStory misst den Eintrag e in der Größe der Rolle (gemerkt je Spieler: einmal je Text, nicht je Bild) und rechnet
// Größe und Maße im Verhältnis herunter, wenn die längere Zeile nicht in cellW passt.
func (h *hud) fitStory(e storyEntry, f *art.Font, p int, k, cellW float64) storyFit {
	base := fontPx(roleStory, h.cells) * k
	wa, ha := h.paint.measureText(f, p, slotStoryA, e.a, base)
	wb, hb := h.paint.measureText(f, p, slotStoryB, e.b, base)
	px := fitPx(base, max(wa, wb)+2*boxPadX*k, cellW-2*gap*k)
	s := px / base
	return storyFit{px, wa * s, ha * s, wb * s, hb * s}
}

// storySize trägt die Fläche des laufenden Eintrags des Spielers in das Layout ein (ohne Eintrag: nichts).
func (h *hud) storySize(st *seatState, f *art.Font, p int, k, cellW float64, in *items) {
	if st.story.showing.a == "" {
		return
	}
	if m := h.fitStory(st.story.showing, f, p, k, cellW); max(m.wa, m.wb) > 0 {
		in[itemStory].W, in[itemStory].H = storyBox(m.wa, m.ha, m.wb, m.hb, m.px, k)
	}
}

// storyBox ist die Fläche um zwei Zeilen der Breiten wa und wb (hb == 0: nur eine Zeile) in der Schriftgröße px.
func storyBox(wa, ha, wb, hb, px, k float64) (w, h float64) {
	h = ha
	if hb > 0 {
		h = art.LineSpacing*px + hb
	}
	return max(wa, wb) + 2*boxPadX*k, h + 2*boxPadY*k
}

// drawStory zeichnet den laufenden Eintrag auf seiner Fläche r, jede Zeile mittig.
func (h *hud) drawStory(dst *ebiten.Image, st *seatState, f *art.Font, p int, r rect, k, cellW float64) {
	a := st.story.alpha()
	if a <= 0 {
		return
	}
	e, pt := st.story.showing, &h.paint
	pt.fill(dst, r, rgb(0, boxAlpha*a))
	y := r.Y + boxPadY*k
	m := h.fitStory(e, f, p, k, cellW)
	pt.outline.Draw(dst, f, e.a, r.X+(r.W-m.wa)/2, y, m.px, roleStory.clr, art.Outline8, a)
	if e.b != "" {
		pt.outline.Draw(dst, f, e.b, r.X+(r.W-m.wb)/2, y+art.LineSpacing*m.px, m.px, roleStory.clr, art.Outline8, a)
	}
}
