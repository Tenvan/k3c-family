package hud

import (
	"strings"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// Burgfall-Banner (B-813 AC-01): Im Modus „lost“ (castleFallen und gameOver im selben Takt) nennt die Meldung die
// Verluste dieses Modus, sonst die des Stufenverlusts; beide ganz, ohne „…“.
func TestBurgfallBannerVerluste(t *testing.T) {
	cases := []struct {
		over       bool
		want, miss string
	}{
		{true, "Die Insel ist komplett verloren", "Hälfte der Vorräte"},
		{false, "Hälfte der Vorräte sind verloren", "komplett verloren"},
	}
	for _, c := range cases {
		_, f := twoPlayers(t, 0)
		f.Events = []view.Event{{Stage: 0, Data: sim.Event{"type": "castleFallen"}}}
		if c.over {
			f.Events = append(f.Events, view.Event{Stage: 0, Data: sim.Event{"type": "gameOver"}})
		}
		h := New(nil).(*hud)
		h.Listen(f)
		got := h.seats[0].banner.showing
		if !strings.Contains(got, c.want) || strings.Contains(got, c.miss) || strings.Contains(got, "…") {
			t.Errorf("gameOver %v: %q", c.over, got)
		}
	}
}

// Das Banner liegt mit der echten Schrift ganz in der Zelle (B-813 AC-01): 2 Spieler bei 1280 × 720 und 1920 × 1080,
// dazu ein schmales Fenster, in dem fitPx die Schrift verkleinert.
func TestBurgfallBannerImBild(t *testing.T) {
	font, err := art.LoadFont("../../../public/schrift/k3c.ttf", true)
	if err != nil {
		t.Skipf("Schrift fehlt: %v", err)
	}
	text := fitBanner(tr("de", "ev.castleFallen"), bannerMaxChars)
	for _, win := range [][2]int{{1280, 720}, {1920, 1080}, {640, 720}} {
		area := rect{0, 0, float64(win[0]), float64(win[1]) / 2}
		k := area.H * 2 / refHeight
		px := fontPx(roleBanner, 2) * k
		w, _ := font.Measure(text, px)
		px = fitPx(px, w+2*boxPadX*k, area.W-2*gap*k)
		w, h := font.Measure(text, px)
		var in items
		in[itemBanner].W, in[itemBanner].H = boxSize(w, h, k)
		pl := layout(&in, area, nil, k)
		if !pl.ok[itemBanner] || !pl.at[itemBanner].inside(area) {
			t.Errorf("%v: Banner %+v nicht in der Zelle %+v", win, pl.at[itemBanner], area)
		}
	}
	if fitPx(28, 100, 200) != 28 || fitPx(28, 400, 200) != 14 {
		t.Error("fitPx: passt bleibt, zu breit im Verhältnis kleiner")
	}
}
