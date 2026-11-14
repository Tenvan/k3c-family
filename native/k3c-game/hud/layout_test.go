package hud

import (
	"fmt"
	"image"
	"testing"

	"k3c/native/k3c-game/view"
)

// sizesAt: Größen wie im Spiel (Browser-Test hudLayout.test.ts, 28 px fett, Innenabstand 8/4 px) beim Maßstab k.
func sizesAt(k, cellW float64) items {
	var in items
	set := func(id itemID, w, h float64) { in[id].W, in[id].H = w*k, h*k }
	set(itemPlayer, 704, 42)
	set(itemSkills, 832, 42)
	set(itemBanner, 640, 42)
	set(itemShared, 416, 42)
	set(itemClock, 352, 42)
	set(itemFight, 288, 42)
	set(itemCitizens, 448, 42)
	set(itemPlace, 448, 38)
	set(itemTravel, 512, 56)
	set(itemJoin, 480, 40)
	set(itemBoss, 672, 80)
	set(itemEvents, 320, 42)
	in[itemRadar].W, in[itemRadar].H = radarSize(cellW, k)
	return in
}

// cellRect wie game/layout.go › CellRect: eine Zelle füllt das Fenster, zwei liegen übereinander.
func cellRect(win image.Rectangle, n, i int) image.Rectangle {
	if n <= 1 {
		return win
	}
	mid := win.Min.Y + win.Dy()/2
	if i == 0 {
		return image.Rect(win.Min.X, win.Min.Y, win.Max.X, mid)
	}
	return image.Rect(win.Min.X, mid, win.Max.X, win.Max.Y)
}

func tickFree(area rect) rect {
	return rectOf(view.StatsRect(image.Rect(int(area.X), int(area.Y), int(area.X+area.W), int(area.Y+area.H))))
}

func TestLayoutOhneUeberschneidungPflichtDa(t *testing.T) {
	for _, win := range []image.Rectangle{image.Rect(0, 0, 1280, 720), image.Rect(0, 0, 1920, 1080)} {
		for _, n := range []int{1, 2} {
			for i := range n {
				name := fmt.Sprintf("%dx%d, %d Spieler, Zelle %d", win.Dx(), win.Dy(), n, i)
				t.Run(name, func(t *testing.T) { checkCell(t, win, n, i) })
			}
		}
	}
}

func checkCell(t *testing.T, win image.Rectangle, n, i int) {
	area := rectOf(cellRect(win, n, i))
	k := float64(win.Dy()) / refHeight
	var free []rect
	if i == 0 {
		free = []rect{tickFree(area)}
	}
	in := sizesAt(k, area.W)
	out := layout(&in, area, free, k)
	for id := range itemCount {
		if !out.ok[id] {
			if anchors[id].rank == rankMust {
				t.Errorf("Pflicht %d fehlt", id)
			}
			continue
		}
		r := out.at[id]
		if !r.inside(area) {
			t.Errorf("Element %d %v nicht in der Zelle %v", id, r, area)
		}
		for _, f := range free {
			if r.overlaps(f) {
				t.Errorf("Element %d verdeckt die Takt-Anzeige", id)
			}
		}
		for o := id + 1; o < itemCount; o++ {
			if out.ok[o] && r.overlaps(out.at[o]) {
				t.Errorf("Element %d schneidet %d", id, o)
			}
		}
	}
}

func TestLayoutOhneFreiflaecheAmAnker(t *testing.T) {
	area, k := rect{0, 0, 1920, 1080}, 1.0
	in := sizesAt(k, area.W)
	out := layout(&in, area, nil, k)
	for _, id := range []itemID{itemPlayer, itemSkills, itemClock} {
		if want := anchorRect(id, area, in[id].W, in[id].H, k); out.at[id] != want {
			t.Errorf("%d: %v, erwartet Anker %v", id, out.at[id], want)
		}
	}
	if out.at[itemPlayer].X != 24 || out.at[itemFight].X+out.at[itemFight].W != 1920-24 {
		t.Errorf("Spielerkarte links, Kampf rechts mit 24 px Rand: %v %v", out.at[itemPlayer], out.at[itemFight])
	}
	if out.at[itemShared].Y < out.at[itemPlayer].Y+out.at[itemPlayer].H {
		t.Error("Vorrat steht unter der Spielerkarte")
	}
}

func TestLayoutTaktAnzeigeVerschiebtKampf(t *testing.T) {
	area, k := rect{0, 0, 1280, 360}, 720.0/refHeight
	in := sizesAt(k, area.W)
	free := tickFree(area)
	out := layout(&in, area, []rect{free}, k)
	if !out.ok[itemFight] || out.at[itemFight].overlaps(free) {
		t.Fatalf("Kampf-Zeile weicht der Takt-Anzeige aus: %v", out.at[itemFight])
	}
	if out.at[itemFight] == anchorRect(itemFight, area, in[itemFight].W, in[itemFight].H, k) {
		t.Error("Kampf-Zeile steht noch an ihrem Anker")
	}
}

func TestLayoutZuKleinFaelltNachRang(t *testing.T) {
	area, k := rect{0, 0, 400, 110}, 1.0
	var in items
	in[itemPlayer] = struct{ W, H float64 }{400, 46}
	in[itemClock] = struct{ W, H float64 }{400, 46}
	in[itemRadar] = struct{ W, H float64 }{400, 46}
	out := layout(&in, area, nil, k)
	if !out.ok[itemPlayer] || !out.ok[itemClock] || out.ok[itemRadar] {
		t.Errorf("Pflicht und Rang 1 bleiben, Radar fällt weg: %v", out.ok)
	}
}

func TestLayoutNichtGezeigtBleibtLeer(t *testing.T) {
	var in items
	in[itemPlayer] = struct{ W, H float64 }{100, 20}
	out := layout(&in, rect{0, 0, 800, 600}, nil, 1)
	for id := range itemCount {
		if id != itemPlayer && out.ok[id] {
			t.Errorf("Element %d ohne Größe gezeigt", id)
		}
	}
}

func TestScaleOf(t *testing.T) {
	c := view.Cell{Rect: image.Rect(0, 360, 1280, 720)}
	if k := scaleOf(c, 2); k != 720.0/1080 {
		t.Errorf("zwei Zellen à 360 px: Maßstab %v", k)
	}
	if k := scaleOf(view.Cell{Rect: image.Rect(0, 0, 1920, 1080)}, 1); k != 1 {
		t.Errorf("1080 px: Maßstab %v", k)
	}
}
