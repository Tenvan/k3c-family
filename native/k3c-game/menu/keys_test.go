package menu

import (
	"image"
	"strings"
	"testing"

	"k3c/native/k3c-game/view"
)

func TestMove(t *testing.T) {
	offOne := func(i int) bool { return i == 1 }
	cases := []struct {
		name        string
		sel, dir, n int
		wrap        bool
		off         func(int) bool
		want        int
	}{
		{"runter", 0, 1, 3, false, nil, 1},
		{"unten bleibt", 2, 1, 3, false, nil, 2},
		{"oben bleibt", 0, -1, 3, false, nil, 0},
		{"im Kreis nach oben", 0, -1, 3, true, nil, 2},
		{"im Kreis nach unten", 2, 1, 3, true, nil, 0},
		{"überspringt ausgegraut", 0, 1, 3, false, offOne, 2},
		{"nur ausgegraut darüber", 2, -1, 3, false, func(i int) bool { return i != 2 }, 2},
		{"leere Liste", 0, 1, 0, false, nil, 0},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := move(c.sel, c.dir, c.n, c.wrap, c.off); got != c.want {
				t.Errorf("move = %d, erwartet %d", got, c.want)
			}
		})
	}
}

func TestWindowAndCycle(t *testing.T) {
	cases := []struct{ sel, n, vis, want int }{
		{0, 3, 7, 0}, {0, 12, 7, 0}, {5, 12, 7, 2}, {11, 12, 7, 5}, {3, 12, 0, 0},
	}
	for _, c := range cases {
		if got := window(c.sel, c.n, c.vis); got != c.want {
			t.Errorf("window(%d, %d, %d) = %d, erwartet %d", c.sel, c.n, c.vis, got, c.want)
		}
	}
	if cycle(3, 1, 4) != 0 || cycle(0, -1, 4) != 3 || cycle(1, 1, 0) != 0 {
		t.Error("cycle springt nicht im Kreis")
	}
}

func TestKeyOf(t *testing.T) {
	want := map[view.Action]key{
		view.ActionUp: keyUp, view.ActionDown: keyDown, view.ActionLeft: keyLeft, view.ActionRight: keyRight,
		view.ActionConfirm: keyOK, view.ActionBack: keyBack, view.ActionMenu: keyBack, view.ActionHome: keyHome,
		view.ActionView: keyDelete, view.ActionSkillMenu: keyNone, view.ActionFullscreen: keyNone, view.ActionNextPage: keyNone,
	}
	for a, k := range want {
		if got := keyOf(a); got != k {
			t.Errorf("Aktion %d: %d, erwartet %d", a, got, k)
		}
	}
}

func TestRunRemembersOperatingDevice(t *testing.T) {
	sc := newMain(newMenus())
	s := &fakeSession{}
	sc.Update(press(pad2, view.ActionDown), s)
	sc.fill(view.DefaultLanguage)
	if !strings.HasSuffix(sc.pg.hint, "bedient: Pad 2") {
		t.Errorf("Hinweis %q", sc.pg.hint)
	}
	sc.Update(press(view.DeviceID{Kind: view.DeviceKeyboard, Index: 1}, view.ActionUp), s)
	sc.fill(view.LanguageEnglish)
	if !strings.HasSuffix(sc.pg.hint, "operated by: Keyboard right") {
		t.Errorf("hint %q", sc.pg.hint)
	}
}

func TestTextsComplete(t *testing.T) {
	for k := range textCount {
		for l, s := range texts[k] {
			if s == "" {
				t.Errorf("Text %d Sprache %d fehlt", k, l)
			}
			for _, r := range s {
				if r > 0xff {
					t.Errorf("Text %q: Zeichen %q fehlt in der Debug-Schrift (nur Latin-1)", s, r)
				}
			}
		}
	}
	if tr("fr", txtNew) != "Neues Spiel" || tr1(view.LanguageEnglish, txtPlayer, "2") != "Player 2" {
		t.Error("Sprache oder Platzhalter falsch")
	}
}

func TestGeomFitsWindow(t *testing.T) {
	for _, win := range []image.Rectangle{image.Rect(0, 0, 1280, 720), image.Rect(0, 0, 1920, 1080), image.Rect(0, 0, 320, 180)} {
		g := geomOf(win)
		_, y, w, h := g.rowRect(maxRows - 1)
		if y+h > g.noteY || g.rowX < 0 || g.rowX+w > float64(win.Max.X) || g.titleY >= g.subY || g.subY+g.smallPx > g.rowTop {
			t.Errorf("%v: Zeilen ragen über die Meldung (%.0f > %.0f) oder aus dem Fenster", win, y+h, g.noteY)
		}
		ax, ay, aw, ah := g.askRect()
		if ax < 0 || ay < 0 || ax+aw > float64(win.Max.X) || ay+ah > float64(win.Max.Y) {
			t.Errorf("%v: Rückfrage ragt aus dem Fenster", win)
		}
	}
	if textScale(10) != 1 || textScale(43) != 3 {
		t.Error("textScale")
	}
}
