package devtools

// Zeichnen des Cheat-Menüs in der Zelle des Spielers: Titel, Reiter der Gruppen, sichtbarer Ausschnitt der Einträge um
// die Auswahl, Ergebnis der letzten Aktion und Bedienhinweis. Schriftgröße nach der Zellenhöhe (TV-tauglich).

import (
	"fmt"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// cheatRows ist die Zahl der sichtbaren Einträge.
const cheatRows = 9

// cheatHint ist der Bedienhinweis unter den Einträgen.
const cheatHint = "links/rechts Gruppe · hoch/runter Eintrag · A/Leertaste auslösen · B/F9 schließen"

// window liefert den ersten sichtbaren Eintrag, damit row bei n Einträgen in einem Fenster von size Zeilen liegt.
func window(row, n, size int) int {
	if n <= size {
		return 0
	}
	return min(max(0, row-size/2), n-size)
}

// drawCheats zeichnet das Menü st in die Zelle c.
func (o *overlay) drawCheats(dst *ebiten.Image, font *art.Font, c view.Cell, st *cheatState) {
	r := c.Rect
	px := max(14, float64(r.Dy())/24)
	lineH := px * art.LineSpacing
	gap := px * 0.6
	w := min(float64(r.Dx())-2*gap, px*24)
	h := lineH*float64(cheatRows+4) + 2*gap
	x, y := float64(r.Min.X)+gap, float64(r.Min.Y)+gap
	box(dst, x, y, w, h)
	g := cheatGroups[st.group]
	first := window(st.row, len(g.entries), cheatRows)
	sel := y + gap + lineH*float64(2+st.row-first)
	fill(dst, x, sel, w, lineH, selColor)
	if font == nil {
		return
	}
	font.Draw(dst, fmt.Sprintf("Cheats · Spieler %d", c.Player+1), x+gap, y+gap, px, keyColor)
	tx := x + gap
	for i, gr := range cheatGroups {
		clr := dimColor
		if i == st.group {
			clr = textColor
		}
		font.Draw(dst, gr.name, tx, y+gap+lineH, px, clr)
		tw, _ := font.Measure(gr.name, px)
		tx += tw + gap*1.5
	}
	for i := first; i < min(len(g.entries), first+cheatRows); i++ {
		font.Draw(dst, o.toggles.label(&g.entries[i]), x+gap*2, y+gap+lineH*float64(2+i-first), px, textColor)
	}
	small := px * 0.8
	if st.status != "" {
		clr := textColor
		if st.bad {
			clr = badColor
		}
		font.Draw(dst, st.status, x+gap, y+gap+lineH*float64(cheatRows+2), small, clr)
	}
	font.Draw(dst, cheatHint, x+gap, y+gap+lineH*float64(cheatRows+3), small, dimColor)
}
