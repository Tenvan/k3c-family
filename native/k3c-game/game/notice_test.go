package game

import (
	"slices"
	"strings"
	"testing"
)

func TestAsciiText(t *testing.T) {
	cases := map[string]string{
		"Lädt …":                        "Laedt",
		"💥 Größe prüfen":                "Groesse pruefen",
		"fehlende Grafik: public/grafik": "fehlende Grafik: public/grafik",
		"ÄÖÜ ß":                          "AeOeUe ss",
	}
	for in, want := range cases {
		if got := asciiText(in); got != want {
			t.Errorf("asciiText(%q) = %q, erwartet %q", in, got, want)
		}
	}
}

func TestWrapText(t *testing.T) {
	cases := []struct {
		in   string
		cols int
		want []string
	}{
		{"", 10, []string{""}},
		{"ein kurzer Satz", 20, []string{"ein kurzer Satz"}},
		{"ein kurzer Satz", 8, []string{"ein", "kurzer", "Satz"}},
		{"aaaa bbbb cccc", 9, []string{"aaaa bbbb", "cccc"}},
		{"abcdefghijkl x", 5, []string{"abcde", "fghij", "kl x"}},
	}
	for _, c := range cases {
		got := wrapText(c.in, c.cols)
		if !slices.Equal(got, c.want) {
			t.Errorf("wrapText(%q, %d) = %q, erwartet %q", c.in, c.cols, got, c.want)
		}
		for _, l := range got {
			if len(l) > c.cols {
				t.Errorf("Zeile %q zu lang", l)
			}
		}
	}
}

// Ein langer Pfad bleibt vollständig lesbar: nach dem Umbruch ergeben die Teile wieder den Pfad.
func TestNoticeLinesKeepPath(t *testing.T) {
	path := "C:/Users/spieler/k3c/public/grafik/gebaeude/burg/sheet-idle-0123456789abcdef0123456789abcdef.png"
	lines := noticeLines([]string{"Grafik fehlt:", path})
	if lines[0] != "Grafik fehlt:" || strings.Join(lines[1:], "") != path {
		t.Errorf("Pfad zerstört: %q", lines)
	}
	for _, l := range lines {
		if len(l) > noticeCols {
			t.Errorf("Zeile %q breiter als %d", l, noticeCols)
		}
	}
}

func TestNoticeScale(t *testing.T) {
	cases := []struct{ w, h, iw, ih, want int }{
		{1280, 720, 352, 150, 3},
		{1920, 1080, 352, 150, 4},
		{3840, 2160, 352, 150, 6}, // Deckel
		{320, 180, 352, 150, 1},   // nie unter 1
		{1280, 720, 0, 0, 1},
		{1280, 200, 352, 150, 1}, // die Höhe begrenzt
	}
	for _, c := range cases {
		if got := noticeScale(c.w, c.h, c.iw, c.ih); got != c.want {
			t.Errorf("noticeScale(%d,%d,%d,%d) = %d, erwartet %d", c.w, c.h, c.iw, c.ih, got, c.want)
		}
	}
}
