package menu

import (
	"log/slog"
	"strconv"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// Optionen › Steuerung (B-893): eine Zeile je Spieler mit seinem Eingabe-Profil (Tastatur WASD, Tastatur Pfeile,
// Gamepad). Links/Rechts bzw. A blättert; Tastatur-Profile überspringt das Blättern, solange ein anderer Spieler eins
// hat (view.Settings.NextProfile). Jede Änderung geht sofort über Session.SetSettings; game stellt die Tastatur im
// nächsten Update um. Grundlage jeder Änderung ist der aktuelle Stand der Session.

// ctlBack ist die Zeile Zurück unter den Spielern.
const ctlBack = view.MaxPlayers

// controlsScreen bearbeitet die Eingabe-Profile der Spieler.
type controlsScreen struct {
	base
	m      *menus
	cur    view.Settings
	loaded bool
	sel    int
	dim    bool
	pg     page
}

func newControls(m *menus, dim bool) *controlsScreen { return &controlsScreen{m: m, dim: dim} }

func (sc *controlsScreen) Update(in view.Input, s view.Session) view.Step {
	if !sc.loaded {
		sc.cur, sc.loaded = s.Settings().Normalized(), true
		sc.ver++
	}
	return sc.run(in, s, sc)
}

func (sc *controlsScreen) handle(_ view.DeviceID, k key, s view.Session) (view.StepKind, view.Screen) {
	switch k {
	case keyUp, keyDown:
		dir := 1
		if k == keyUp {
			dir = -1
		}
		sc.sel = move(sc.sel, dir, ctlBack+1, true, nil)
		sc.cue(view.CueMove, -1, "")
		sc.ver++
	case keyLeft, keyRight, keyOK:
		if sc.sel == ctlBack {
			if k != keyOK {
				return view.StepStay, nil
			}
			sc.cue(view.CueBack, -1, "")
			return view.StepPop, nil
		}
		dir := 1
		if k == keyLeft {
			dir = -1
		}
		sc.cur = s.Settings().Normalized()
		next := sc.cur
		next.Controls[sc.sel] = sc.cur.NextProfile(sc.sel, dir)
		sc.apply(s, next)
	case keyBack, keyHome:
		sc.cue(view.CueBack, -1, "")
		return view.StepPop, nil
	}
	return view.StepStay, nil
}

// apply übernimmt die neuen Profile über die Session; sie gelten auch, wenn das Speichern scheitert.
func (sc *controlsScreen) apply(s view.Session, next view.Settings) {
	next = next.Normalized()
	if next == sc.cur {
		return
	}
	sc.cur = next
	sc.note = note{}
	sc.cue(view.CueMove, -1, "")
	sc.ver++
	slog.Info("🚦 Eingabe-Profil gewechselt", "player", sc.sel, "profile", next.Controls[sc.sel])
	if err := s.SetSettings(next); err != nil {
		slog.Warn("💥 Einstellungen nicht gespeichert", "err", err)
		sc.note = note{raw: err.Error(), bad: true, set: true}
	}
}

func (sc *controlsScreen) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if lang := f.Settings.Language; sc.pg.stale(sc.ver, lang) {
		sc.fill(lang)
	}
	sc.m.paint.page(dst, c, &sc.pg)
}

// fill baut die Zeilen „Spieler 1  < Tastatur WASD >“ … und Zurück in der Sprache lang.
func (sc *controlsScreen) fill(lang string) {
	p := &sc.pg
	p.reset(tr(lang, txtControls))
	for i := range view.MaxPlayers {
		p.add(row{label: tr1(lang, txtPlayer, strconv.Itoa(i+1)), value: "< " + profileText(lang, sc.cur.Controls[i]) + " >",
			mat: matPurple})
	}
	p.add(row{label: tr(lang, txtBack), mat: matCoral})
	p.sel = sc.sel
	p.dim = sc.dim
	p.note, p.bad = sc.note.text(lang), sc.note.bad
	p.hint = byHint(lang, txtControlsHint, &sc.base)
}

// profileText ist der Name des Eingabe-Profils p.
func profileText(lang, p string) string {
	switch p {
	case view.ProfileWASD:
		return tr(lang, txtKeysWASD)
	case view.ProfileArrows:
		return tr(lang, txtKeysArrows)
	}
	return tr(lang, txtGamepad)
}
