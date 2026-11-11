package menu

import (
	"log/slog"
	"strconv"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// Spielstand-Liste (Vorbild src/scenes/saveMenu.ts): neueste zuerst wie Session.List. A lädt, View (Tab) fragt nach
// dem Löschen; die Rückfrage hat „Abbrechen“ vorgewählt, nur „Löschen“ + A löscht, B bricht ab. Nicht ladbare Stände
// stehen markiert mit Grund in der Liste: Laden ist gesperrt, Löschen geht.

// savesScreen ist die Spielstand-Liste.
type savesScreen struct {
	base
	m      *menus
	list   []view.SaveInfo
	listed bool
	sel    int
	asking bool
	yes    bool // in der Rückfrage ist „Löschen“ gewählt
	pg     page
}

func newSaves(m *menus) *savesScreen { return &savesScreen{m: m} }

// Update liest die Liste beim ersten Mal und bedient die Tasten.
func (sc *savesScreen) Update(in view.Input, s view.Session) view.Step {
	if !sc.listed {
		sc.refresh(s)
	}
	return sc.run(in, s, sc)
}

// refresh liest die Spielstände neu; die Auswahl bleibt in der Liste.
func (sc *savesScreen) refresh(s view.Session) {
	sc.listed = true
	list, err := s.List()
	if err != nil {
		sc.note = note{raw: err.Error(), bad: true, set: true}
	}
	sc.list = list
	sc.sel = max(0, min(sc.sel, len(sc.list)-1))
	sc.ver++
}

func (sc *savesScreen) handle(d view.DeviceID, k key, s view.Session) (view.StepKind, view.Screen) {
	if sc.asking {
		return sc.answer(k, s)
	}
	switch k {
	case keyUp, keyDown:
		dir := 1
		if k == keyUp {
			dir = -1
		}
		if next := move(sc.sel, dir, len(sc.list), false, nil); next != sc.sel {
			sc.sel = next
			sc.cue(view.CueMove, -1, "")
		}
		sc.ver++
	case keyOK:
		return sc.load(d, s)
	case keyDelete:
		if len(sc.list) > 0 {
			sc.asking, sc.yes = true, false
			sc.ver++
		}
	case keyBack, keyHome:
		sc.cue(view.CueBack, -1, "")
		return view.StepPop, nil
	}
	return view.StepStay, nil
}

// load lädt den gewählten Stand mit dem bestätigenden Gerät als Spieler 1 (weitere treten im Spiel mit A bei).
func (sc *savesScreen) load(d view.DeviceID, s view.Session) (view.StepKind, view.Screen) {
	if len(sc.list) == 0 {
		return view.StepStay, nil
	}
	e := sc.list[sc.sel]
	if !e.Loadable {
		sc.note = note{raw: e.Problem, bad: true, set: true}
		sc.cue(view.CueDenied, -1, e.Problem)
		sc.ver++
		return view.StepStay, nil
	}
	if err := s.Load(e.Name, []view.DeviceID{d}); err != nil {
		sc.fail(err)
		return view.StepStay, nil
	}
	slog.Info("📂 Spielstand aus dem Menü geladen", "save", e.Name, "device", d.String())
	sc.cue(view.CueConfirm, -1, "")
	return view.StepPlay, nil
}

// answer bedient die Rückfrage vor dem Löschen.
func (sc *savesScreen) answer(k key, s view.Session) (view.StepKind, view.Screen) {
	switch k {
	case keyUp, keyDown, keyLeft, keyRight:
		sc.yes = !sc.yes
		sc.cue(view.CueMove, -1, "")
	case keyOK:
		sc.asking = false
		if sc.yes {
			sc.remove(s)
		} else {
			sc.cue(view.CueBack, -1, "")
		}
	case keyBack, keyDelete:
		sc.asking = false
		sc.cue(view.CueBack, -1, "")
	case keyHome:
		sc.asking = false
		sc.cue(view.CueBack, -1, "")
		return view.StepPop, nil
	}
	sc.ver++
	return view.StepStay, nil
}

// remove löscht den gewählten Stand und liest die Liste neu.
func (sc *savesScreen) remove(s view.Session) {
	name := sc.list[sc.sel].Name
	if err := s.Delete(name); err != nil {
		sc.fail(err)
		return
	}
	slog.Info("🧹 Spielstand aus dem Menü gelöscht", "save", name)
	sc.cue(view.CueConfirm, -1, "")
	sc.refresh(s)
	sc.inform(txtDeleted, name)
}

func (sc *savesScreen) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if lang := f.Settings.Language; sc.pg.stale(sc.ver, lang) {
		sc.fill(lang)
	}
	sc.m.paint.page(dst, c, &sc.pg)
}

// fill baut die sichtbaren Zeilen (rollend um die Auswahl) und die Rückfrage.
func (sc *savesScreen) fill(lang string) {
	p := &sc.pg
	p.reset(tr(lang, txtSavesTitle))
	from := window(sc.sel, len(sc.list), maxRows)
	for i := from; i < min(len(sc.list), from+maxRows); i++ {
		p.add(saveRow(lang, sc.list[i]))
	}
	p.sel = sc.sel - from
	if len(sc.list) == 0 {
		p.sub = tr(lang, txtSavesEmpty)
	}
	p.note, p.bad = sc.note.text(lang), sc.note.bad
	p.hint = byHint(lang, txtSavesHint, &sc.base)
	if sc.asking {
		p.ask = tr1(lang, txtDeleteAsk, sc.list[sc.sel].Name)
		p.askRows = [2]string{tr(lang, txtDeleteNo), tr(lang, txtDeleteYes)}
		p.askSel = 0
		if sc.yes {
			p.askSel = 1
		}
		p.hint = tr(lang, txtAskHint)
	}
}

// saveRow ist die Karte eines Spielstands: Name links; rechts Tag, Stufen und Zeit bzw. der Grund, warum er nicht
// ladbar ist.
func saveRow(lang string, e view.SaveInfo) row {
	r := row{label: e.Name, mat: matBlue}
	if !e.Loadable {
		r.value, r.off = e.Problem, true
		return r
	}
	r.value = tr1(lang, txtSavesLine, strconv.Itoa(e.Day))
	for i, d := range e.Depths {
		sep := " · " + tr(lang, txtDepth) + " "
		if i > 0 {
			sep = ","
		}
		r.value += sep + strconv.Itoa(d)
	}
	if !e.SavedAt.IsZero() {
		r.value += " · " + e.SavedAt.Local().Format("02.01. 15:04")
	}
	return r
}
