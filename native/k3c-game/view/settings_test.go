package view

import (
	"math"
	"testing"
)

func TestSettingsNormalized(t *testing.T) {
	cases := []struct{ in, want Settings }{
		{Settings{}, Settings{Language: DefaultLanguage}},
		{DefaultSettings(), DefaultSettings()},
		{Settings{Window: "kino"}, Settings{Language: DefaultLanguage}}, // unbekannter Anzeige-Modus: Fenster (B-826)
		{Settings{Music: 1.5, Effects: -0.2, Window: WindowFullscreen, Language: "en"}, Settings{Music: 1, Effects: 0, Window: WindowFullscreen, Language: "en"}},
		{Settings{Music: math.NaN(), Effects: 0.4, Language: "fr"}, Settings{Music: 1, Effects: 0.4, Language: "de"}},
		{Settings{FlashesOff: true, GuideOff: true, GuideSeen: -3}, Settings{FlashesOff: true, GuideOff: true, Language: "de"}},
		{Settings{GuideSeen: 5}, Settings{GuideSeen: 5, Language: "de"}},
	}
	for _, c := range cases {
		if got := c.in.Normalized(); got != c.want {
			t.Errorf("%+v.Normalized() = %+v, erwartet %+v", c.in, got, c.want)
		}
	}
}

// River: nur die zwei Fassungen bleiben, alles andere fällt auf "" (Startwert); NextRiver wechselt von Wert oder Start.
func TestSettingsRiver(t *testing.T) {
	for _, v := range []string{"", RiverShader, RiverTiles} {
		if got := (Settings{River: v}).Normalized().River; got != v {
			t.Errorf("River %q -> %q", v, got)
		}
	}
	if got := (Settings{River: "lava"}).Normalized().River; got != "" {
		t.Errorf("unbekannt: %q", got)
	}
	cases := []struct{ river, start, want string }{
		{"", RiverShader, RiverTiles}, {"", RiverTiles, RiverShader},
		{RiverTiles, RiverShader, RiverShader}, {RiverShader, RiverTiles, RiverTiles},
	}
	for _, c := range cases {
		if got := (Settings{River: c.river}).NextRiver(c.start); got != c.want {
			t.Errorf("NextRiver(%q, start %q) = %q, erwartet %q", c.river, c.start, got, c.want)
		}
	}
}
