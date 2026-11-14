package menu

import (
	"log/slog"
	"math"
	"slices"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// Testing (B-896): Stresstest einstellen und starten. Zeilen: Vorgabe, Spieler, Stufe, Seed, Gegner je Welle, Anstieg,
// Faktor, Ausbau, Truppen, Zeitraffer, Wellen, Dauer, Starten, Zurück (rollt wie die Spielstände, maxRows sichtbar).
// Links/Rechts ändert am Rand begrenzt, A im Kreis; eine geänderte Zeile macht aus der Vorgabe „eigene Werte“.
// Starten merkt die Werte in den Einstellungen (Settings.Stress) und startet über view.StressStarter (game).

const (
	tstPreset = iota
	tstPlayers
	tstDepth
	tstSeed
	tstEnemies
	tstGrowth
	tstFactor
	tstBuild
	tstTroops
	tstTimescale
	tstWaves
	tstMinutes
	tstStart
	tstBack
	tstRows
)

var tstLabels = [tstRows]textKey{txtTstPreset, txtTstPlayers, txtTstDepth, txtSeed, txtTstEnemies, txtTstGrowth,
	txtTstFactor, txtTstBuild, txtTstTroops, txtTstTimescale, txtTstWaves, txtTstMinutes, txtTstStart, txtBack}

// tstSteps: Schritt und Höchstwert der Zahl-Zeilen (Minimum 0, Spieler 1).
var tstSteps = map[int][2]int{
	tstPlayers: {1, view.MaxPlayers}, tstDepth: {1, view.StressMaxDepth}, tstSeed: {1, view.StressMaxSeed},
	tstEnemies: {5, view.StressMaxEnemies}, tstBuild: {1, view.StressMaxBuild}, tstTroops: {10, view.StressMaxTroops},
	tstWaves: {1, view.StressMaxWaves}, tstMinutes: {5, view.StressMaxMinutes},
}

// testingScreen ist die Seite Testing; cur sind die eingestellten Werte, preset der Index der Vorgabe (-1 = eigene).
type testingScreen struct {
	base
	m       *menus
	cur     view.StressTest
	presets []stressPreset
	preset  int
	loaded  bool
	sel     int
	pg      page
}

func newTesting(m *menus) *testingScreen { return &testingScreen{m: m, preset: -1} }

func (sc *testingScreen) Update(in view.Input, s view.Session) view.Step {
	if !sc.loaded {
		sc.load(s)
	}
	return sc.run(in, s, sc)
}

// load liest die gemerkten Werte und die Vorgaben (ohne Vorgaben: Meldung, die Seite bleibt bedienbar).
func (sc *testingScreen) load(s view.Session) {
	sc.loaded, sc.cur = true, s.Settings().Normalized().Stress
	ps, err := loadStressPresets()
	if err != nil {
		sc.note = note{key: txtTstNoPresets, bad: true, set: true}
	}
	sc.presets = ps
	sc.preset = slices.IndexFunc(ps, func(p stressPreset) bool { return p.Preset == sc.cur.Preset })
	sc.ver++
}

func (sc *testingScreen) handle(d view.DeviceID, k key, s view.Session) (view.StepKind, view.Screen) {
	switch k {
	case keyUp, keyDown:
		dir := 1
		if k == keyUp {
			dir = -1
		}
		sc.sel = move(sc.sel, dir, tstRows, true, nil)
		sc.cue(view.CueMove, -1, "")
		sc.ver++
	case keyBack, keyHome:
		sc.cue(view.CueBack, -1, "")
		return view.StepPop, nil
	case keyLeft, keyRight, keyOK:
		switch {
		case sc.sel == tstStart && k == keyOK:
			return sc.start(d, s)
		case sc.sel == tstBack && k == keyOK:
			sc.cue(view.CueBack, -1, "")
			return view.StepPop, nil
		case sc.sel < tstStart:
			sc.change(sc.sel, k)
		}
	}
	return view.StepStay, nil
}

// change ändert die Zeile r mit Taste k: links/rechts begrenzt, A im Kreis.
func (sc *testingScreen) change(r int, k key) {
	dir := 1
	if k == keyLeft {
		dir = -1
	}
	round := k == keyOK
	if r == tstPreset {
		sc.choosePreset(cycleOrClamp(sc.preset+1, dir, len(sc.presets)+1, round) - 1)
	} else {
		sc.cur = changeStress(sc.cur, r, dir, round)
		sc.cur.Preset, sc.preset = "", -1
	}
	sc.cue(view.CueMove, -1, "")
	sc.ver++
}

// choosePreset übernimmt die Werte der Vorgabe i; -1 (eigene) behält die Werte.
func (sc *testingScreen) choosePreset(i int) {
	sc.preset = i
	if i < 0 {
		sc.cur.Preset = ""
		return
	}
	sc.cur = sc.presets[i].StressTest
}

// cycleOrClamp verschiebt i um dir unter n Werten: im Kreis (round) oder am Rand begrenzt.
func cycleOrClamp(i, dir, n int, round bool) int {
	if round {
		return cycle(i, dir, n)
	}
	return max(0, min(n-1, i+dir))
}

// changeStress ändert den Wert der Zeile r (rein): Zahlen in Schritten, Anstieg kippt, Faktor in 1 (linear) bzw.
// 0,1 (exponentiell), Zeitraffer durch view.Timescales.
func changeStress(t view.StressTest, r, dir int, round bool) view.StressTest {
	switch r {
	case tstGrowth:
		if t.Growth == view.GrowthExp {
			t.Growth = view.GrowthLinear
		} else {
			t.Growth = view.GrowthExp
		}
	case tstFactor:
		step := 1.0
		if t.Growth == view.GrowthExp {
			step = 0.1
		}
		t.Factor = math.Round((t.Factor+float64(dir)*step)*10) / 10
		if round && t.Factor > view.StressMaxFactor {
			t.Factor = 0
		}
	case tstTimescale:
		i := max(0, slices.Index(view.Timescales[:], t.Timescale))
		t.Timescale = view.Timescales[cycleOrClamp(i, dir, len(view.Timescales), round)]
	default:
		p := stressField(&t, r)
		st := tstSteps[r]
		lo := 0
		if r == tstPlayers {
			lo = 1
		}
		*p = stepInt(*p, dir*st[0], lo, st[1], round)
	}
	return t.Normalized()
}

// stepInt verschiebt v um d in lo … hi: am Rand begrenzt oder (round) von hi nach lo.
func stepInt(v, d, lo, hi int, round bool) int {
	n := v + d
	switch {
	case round && n > hi:
		return lo
	case round && n < lo:
		return hi
	}
	return max(lo, min(hi, n))
}

// stressField ist das Zahl-Feld der Zeile r.
func stressField(t *view.StressTest, r int) *int {
	switch r {
	case tstPlayers:
		return &t.Players
	case tstDepth:
		return &t.Depth
	case tstSeed:
		return &t.Seed
	case tstEnemies:
		return &t.Enemies
	case tstBuild:
		return &t.Build
	case tstTroops:
		return &t.Troops
	case tstWaves:
		return &t.Waves
	}
	return &t.Minutes
}

// start merkt die Werte und startet den Test über view.StressStarter; ohne die Rolle oder bei Fehler bleibt die Seite.
func (sc *testingScreen) start(d view.DeviceID, s view.Session) (view.StepKind, view.Screen) {
	set := s.Settings().Normalized()
	set.Stress = sc.cur
	if err := s.SetSettings(set); err != nil {
		slog.Warn("💥 Einstellungen nicht gespeichert", "err", err)
	}
	st, ok := s.(view.StressStarter)
	if !ok {
		sc.note = note{key: txtTstNoRunner, bad: true, set: true}
		sc.ver++
		return view.StepStay, nil
	}
	if err := st.StartStress(sc.cur, d); err != nil {
		sc.fail(err)
		sc.ver++
		return view.StepStay, nil
	}
	slog.Info("🚀 Stresstest aus dem Menü", "vorgabe", sc.cur.Preset, "spieler", sc.cur.Players, "gegner", sc.cur.Enemies,
		"anstieg", sc.cur.Growth, "faktor", sc.cur.Factor, "wellen", sc.cur.Waves)
	sc.cue(view.CueConfirm, -1, "")
	return view.StepPlay, nil
}

func (sc *testingScreen) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if lang := f.Settings.Language; sc.pg.stale(sc.ver, lang) {
		sc.fill(lang)
	}
	sc.m.paint.page(dst, c, &sc.pg)
}

// fill baut die sichtbaren Zeilen (Fenster von maxRows um die Auswahl) in der Sprache lang.
func (sc *testingScreen) fill(lang string) {
	p := &sc.pg
	p.reset(tr(lang, txtTesting))
	p.sub = tr(lang, txtTestingSub)
	from := window(sc.sel, tstRows, maxRows)
	for r := from; r < min(tstRows, from+maxRows); r++ {
		rw := row{label: tr(lang, tstLabels[r]), mat: matBlue}
		switch r {
		case tstStart:
			rw.mat = matGreen
		case tstBack:
			rw.mat = matCoral
		default:
			rw.value = "< " + sc.valueText(lang, r) + " >"
		}
		p.add(rw)
	}
	p.sel = sc.sel - from
	p.note, p.bad = sc.note.text(lang), sc.note.bad
	p.hint = byHint(lang, txtTestingHint, &sc.base)
}
