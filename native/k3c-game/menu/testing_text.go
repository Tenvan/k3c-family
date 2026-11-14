package menu

import (
	"strconv"

	"k3c/native/k3c-game/view"
)

// valueText ist der angezeigte Wert der Zeile r der Seite Testing.
func (sc *testingScreen) valueText(lang string, r int) string {
	t := sc.cur
	switch r {
	case tstPreset:
		if sc.preset < 0 || sc.preset >= len(sc.presets) {
			return tr(lang, txtTstOwn)
		}
		return sc.presets[sc.preset].name(lang)
	case tstSeed:
		if t.Seed == 0 {
			return tr(lang, txtSeedAuto)
		}
	case tstGrowth:
		if t.Growth == view.GrowthExp {
			return tr(lang, txtTstExp)
		}
		return tr(lang, txtTstLinear)
	case tstFactor:
		f := strconv.FormatFloat(t.Factor, 'f', -1, 64)
		if t.Growth == view.GrowthExp {
			return "× " + f
		}
		return "+ " + f
	case tstTimescale:
		return strconv.Itoa(t.Timescale) + "×"
	case tstWaves, tstMinutes, tstBuild, tstTroops:
		if *stressField(&t, r) == 0 {
			return tr(lang, txtTstNone)
		}
	}
	return strconv.Itoa(*stressField(&t, r))
}
