//go:build windows

package playtest

import (
	"os"
	"strings"
	"testing"
)

func TestExpandPercentLoestNurEinzelneEintraegeAuf(t *testing.T) {
	t.Setenv("K3C_TEST_DIR", `C:\Tools`)
	expand := func(s string) string { return os.Expand(expandPercent(s), expandVar) }
	if got := expand(`%K3C_TEST_DIR%\ffmpeg\bin`); got != `C:\Tools\ffmpeg\bin` {
		t.Errorf("Variable: %q", got)
	}
	if got := expand(`C:\a%b`); got != `C:\a%b` {
		t.Errorf("einzelnes %%: %q", got)
	}
	if got := expand(`C:\cost$5\bin`); got != `C:\cost$5\bin` {
		t.Errorf("literales $: %q", got)
	}
	// das ; trennt vor dem Auflösen, ein einzelnes % wandert nicht in den Nachbareintrag
	for _, d := range strings.Split(`C:\a%b;C:\c%d`, ";") {
		if expand(d) != d {
			t.Errorf("Eintrag %q verändert: %q", d, expand(d))
		}
	}
}
