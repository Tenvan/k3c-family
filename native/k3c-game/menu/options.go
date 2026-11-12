package menu

import (
	"log/slog"
	"math"
	"strconv"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// Optionen (Entscheidung 🧑 2026-11-11, B-800, dazu Blitze und Anleitung, B-809; Vorbild src/scenes/optionsLogic.ts):
// Lautstärke Musik und Effekte in Schritten von 10 %, Vollbild, Sprache, Blitze, Anleitung. Links/Rechts ändert, A legt
// Schalter und Sprache um, Hoch/Runter springt am Ende zum Anfang. Jede Änderung geht sofort über Session.SetSettings
// (local speichert sie); ein Fehler betrifft nur das Speichern und erscheint als Meldung. Grundlage jeder Änderung
// ist der aktuelle Stand der Session, nicht der beim Öffnen: F2/F3 (Settings.Stats, River) gehen nicht verloren.
// Anleitung wieder an setzt „Anleitung gesehen“ zurück (alle Hinweise erneut, wie OptionsScene.ts).

const (
	optMusic = iota
	optEffects
	optFullscreen
	optLanguage
	optFlashes
	optGuide
	optBack
	optRows
)

// volumeStep ist der Schritt der Lautstärke-Regler (0 … 1).
const volumeStep = 0.1

var optLabels = [optRows]textKey{txtMusic, txtEffects, txtFullscreen, txtLanguage, txtFlashes, txtGuide, txtBack}

// optionsScreen bearbeitet die Einstellungen. cur ist der Stand, den das Menü kennt und schreibt.
type optionsScreen struct {
	base
	m      *menus
	cur    view.Settings
	loaded bool
	sel    int
	dim    bool // aus dem Pause-Menü: halbdunkel über dem stehenden Spiel
	pg     page
}

func newOptions(m *menus, dim bool) *optionsScreen { return &optionsScreen{m: m, dim: dim} }

func (sc *optionsScreen) Update(in view.Input, s view.Session) view.Step {
	if !sc.loaded {
		sc.cur, sc.loaded = s.Settings().Normalized(), true
		sc.ver++
	}
	return sc.run(in, s, sc)
}

func (sc *optionsScreen) handle(_ view.DeviceID, k key, s view.Session) (view.StepKind, view.Screen) {
	switch k {
	case keyUp, keyDown:
		dir := 1
		if k == keyUp {
			dir = -1
		}
		sc.sel = move(sc.sel, dir, optRows, true, nil)
		sc.cue(view.CueMove, -1, "")
		sc.ver++
	case keyLeft, keyRight, keyOK:
		if sc.sel == optBack {
			if k != keyOK {
				return view.StepStay, nil
			}
			sc.cue(view.CueBack, -1, "")
			return view.StepPop, nil
		}
		sc.cur = s.Settings().Normalized()
		sc.apply(s, applyOption(sc.cur, sc.sel, k))
	case keyBack, keyHome:
		sc.cue(view.CueBack, -1, "")
		return view.StepPop, nil
	}
	return view.StepStay, nil
}

// applyOption ändert die Einstellung der Zeile opt mit Taste k (rein): Regler um ±10 % (A ändert nichts), Schalter und
// Sprache kippen bei jeder Taste.
func applyOption(s view.Settings, opt int, k key) view.Settings {
	dir := volumeStep
	if k == keyLeft {
		dir = -volumeStep
	}
	switch opt {
	case optMusic:
		if k != keyOK {
			s.Music = stepVolume(s.Music, dir)
		}
	case optEffects:
		if k != keyOK {
			s.Effects = stepVolume(s.Effects, dir)
		}
	case optFullscreen:
		s.Fullscreen = !s.Fullscreen
	case optLanguage:
		s.Language = otherLanguage(s.Language)
	case optFlashes:
		s.FlashesOff = !s.FlashesOff
	case optGuide:
		s.GuideOff = !s.GuideOff
		if !s.GuideOff {
			s.GuideSeen = 0
		}
	}
	return s
}

// stepVolume verschiebt v um d und rundet auf ganze 10 % (0 … 1).
func stepVolume(v, d float64) float64 {
	return math.Max(0, math.Min(1, math.Round((v+d)*10)/10))
}

// otherLanguage wechselt zwischen Deutsch und Englisch.
func otherLanguage(lang string) string {
	if lang == view.LanguageEnglish {
		return view.DefaultLanguage
	}
	return view.LanguageEnglish
}

// apply übernimmt neue Einstellungen über die Session; die Werte gelten auch, wenn das Speichern scheitert.
func (sc *optionsScreen) apply(s view.Session, next view.Settings) {
	next = next.Normalized()
	if next == sc.cur {
		return
	}
	sc.cur = next
	sc.note = note{}
	sc.cue(view.CueMove, -1, "")
	sc.ver++
	if err := s.SetSettings(next); err != nil {
		slog.Warn("💥 Einstellungen nicht gespeichert", "err", err)
		sc.note = note{raw: err.Error(), bad: true, set: true}
	}
}

func (sc *optionsScreen) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if lang := f.Settings.Language; sc.pg.stale(sc.ver, lang) {
		sc.fill(lang)
	}
	sc.m.paint.page(dst, c, &sc.pg)
}

// fill baut die Zeilen „Musik  < 70 % >“, „Vollbild  an“ … in der Sprache lang.
func (sc *optionsScreen) fill(lang string) {
	p := &sc.pg
	p.reset(tr(lang, txtOptions))
	for i := range optRows {
		r := row{label: tr(lang, optLabels[i]), value: optionValue(lang, sc.cur, i), mat: matPurple}
		if i == optBack {
			r.mat = matCoral
		}
		p.add(r)
	}
	p.sel = sc.sel
	p.dim = sc.dim
	p.note, p.bad = sc.note.text(lang), sc.note.bad
	p.hint = byHint(lang, txtOptionsHint, &sc.base)
}

// optionValue ist der angezeigte Wert der Zeile opt.
func optionValue(lang string, s view.Settings, opt int) string {
	onOff := func(v bool) string {
		if v {
			return tr(lang, txtOn)
		}
		return tr(lang, txtOff)
	}
	switch opt {
	case optMusic:
		return "< " + strconv.Itoa(int(math.Round(s.Music*100))) + " % >"
	case optEffects:
		return "< " + strconv.Itoa(int(math.Round(s.Effects*100))) + " % >"
	case optFullscreen:
		return onOff(s.Fullscreen)
	case optLanguage:
		return "< " + tr(s.Language, txtLangName) + " >"
	case optFlashes:
		return onOff(!s.FlashesOff)
	case optGuide:
		return onOff(!s.GuideOff)
	}
	return ""
}
