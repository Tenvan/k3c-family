package hud

import (
	"testing"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// measureMenu misst ein Skill-Menü mit der Schrift f wie drawMenu.
func measureMenu(f *art.Font, v *menuView, base, k float64) menuSizes {
	m := menuSizes{n: v.n}
	m.w[0], m.h[0] = f.Measure(v.title, base)
	for i := range v.n {
		m.w[i+1], m.h[i+1] = f.Measure(v.rows[i].text, base)
		if v.rows[i].glyph.key != "" {
			m.w[i+1] += glyphPx(base) + 8*k
			m.h[i+1] = max(m.h[i+1], glyphPx(base))
		}
	}
	m.w[v.n+1], m.h[v.n+1] = f.Measure(v.footer, base)
	return m
}

// Skill-Menü ohne Überlappung (B-813 AC-02): Mit der echten Schrift liegen Kopfzeile, Zeilen und Hinweis samt Kontur
// (1 px) untereinander ohne Berührung und im Kasten, an Tastatur und Pad, bei 1 und 2 Spielern in 720p und 1080p.
func TestSkillMenueOhneUeberlappung(t *testing.T) {
	font, err := art.LoadFont("../../../public/schrift/k3c.ttf", true)
	if err != nil {
		t.Skipf("Schrift fehlt: %v", err)
	}
	es := menuEntries(nil, []string{"taunt"}, 1, only("heal"), true)
	slots := slotViews([]string{"fireball", "taunt", "", ""}, nil)
	for _, dev := range []view.DeviceID{keys1, pad} {
		v := buildMenu(newMenuKey("de", dev, 0, 1, es, slots), es)
		for _, win := range [][3]float64{{1280, 720, 2}, {1920, 1080, 1}, {1920, 1080, 2}} {
			cell := rect{0, 0, win[0], win[1] / win[2]}
			k := win[1] / refHeight
			base := menuPxHalf * k
			if cell.W/k >= 1400 {
				base = menuPxFull * k
			}
			m := measureMenu(font, &v, base, k)
			b := layoutMenu(cell, k, base, &m, false)
			checkMenuBox(t, b, v.n, win)
		}
	}
}

// checkMenuBox prüft, dass die Zeilen (um die Kontur erweitert) einander nicht berühren und im Kasten liegen.
func checkMenuBox(t *testing.T, b menuBox, n int, win [3]float64) {
	t.Helper()
	parts := append([]rect{b.title}, b.rows[:n]...)
	parts = append(parts, b.footer)
	grow := func(r rect) rect { return rect{r.X - 1, r.Y - 1, r.W + 2, r.H + 2} }
	for i := 1; i < len(parts); i++ {
		if grow(parts[i-1]).overlaps(grow(parts[i])) {
			t.Errorf("%v: Teil %d %+v überlappt Teil %d %+v", win, i-1, parts[i-1], i, parts[i])
		}
	}
	for i, r := range parts {
		if !r.inside(b.box) {
			t.Errorf("%v: Teil %d %+v nicht im Kasten %+v", win, i, r, b.box)
		}
	}
}
