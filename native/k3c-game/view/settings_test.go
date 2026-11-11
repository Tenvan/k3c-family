package view

import (
	"math"
	"testing"
)

func TestSettingsNormalized(t *testing.T) {
	cases := []struct{ in, want Settings }{
		{Settings{}, Settings{Language: DefaultLanguage}},
		{DefaultSettings(), DefaultSettings()},
		{Settings{Music: 1.5, Effects: -0.2, Fullscreen: true, Language: "en"}, Settings{Music: 1, Effects: 0, Fullscreen: true, Language: "en"}},
		{Settings{Music: math.NaN(), Effects: 0.4, Language: "fr"}, Settings{Music: 1, Effects: 0.4, Language: "de"}},
	}
	for _, c := range cases {
		if got := c.in.Normalized(); got != c.want {
			t.Errorf("%+v.Normalized() = %+v, erwartet %+v", c.in, got, c.want)
		}
	}
}
