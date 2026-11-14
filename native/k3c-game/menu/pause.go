package menu

import (
	"log/slog"
	"strconv"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// Pause-Menü (Vorbild src/scenes/pauseHub.ts) auf dem Papier-Hintergrund: Fortsetzen, Optionen, Tasten (Belegung,
// B-807), Spieler abmelden, Spiel verlassen. B und Menu setzen das Spiel fort (game hebt die Pause auf, sobald der Stapel leer ist). „Spieler
// abmelden“ meldet den Spieler des bedienenden Geräts ab und ist ausgegraut, solange nur einer spielt (der letzte
// Spieler bleibt, Session.Leave). „Spiel verlassen“ speichert und beendet (Session.Close) und führt ins Hauptmenü.

const (
	pauseResume = iota
	pauseOptions
	pauseKeys
	pauseRemove
	pauseLeave
	pauseRows
)

var (
	pauseLabels = [pauseRows]textKey{txtResume, txtOptions, txtKeys, txtRemove, txtLeave}
	pauseMats   = [pauseRows]material{matGold, matPurple, matGreen, matBlue, matCoral}
)

// pauseScreen ist das Pause-Menü über dem stehenden Spiel.
type pauseScreen struct {
	base
	m   *menus
	sel int
	pg  page
}

func newPause(m *menus) *pauseScreen { return &pauseScreen{m: m} }

// Update übernimmt die Plätze aus der Session (Abmelden) und bedient die Tasten.
func (sc *pauseScreen) Update(in view.Input, s view.Session) view.Step {
	n := sc.m.nSeats
	if sc.m.sync(s); n != sc.m.nSeats {
		sc.ver++
	}
	return sc.run(in, s, sc)
}

// off meldet ausgegraute Einträge: Abmelden braucht zwei Spieler.
func (sc *pauseScreen) off(i int) bool { return i == pauseRemove && sc.m.nSeats < 2 }

func (sc *pauseScreen) handle(d view.DeviceID, k key, s view.Session) (view.StepKind, view.Screen) {
	switch k {
	case keyUp, keyDown:
		dir := 1
		if k == keyUp {
			dir = -1
		}
		if next := move(sc.sel, dir, pauseRows, false, sc.off); next != sc.sel {
			sc.sel = next
			sc.cue(view.CueMove, -1, "")
		}
		sc.ver++
	case keyHome:
		sc.sel = pauseResume
		sc.ver++
	case keyBack:
		sc.cue(view.CueResume, -1, "")
		return view.StepPop, nil
	case keyOK:
		return sc.confirm(d, s)
	}
	return view.StepStay, nil
}

// confirm löst den gewählten Eintrag aus.
func (sc *pauseScreen) confirm(d view.DeviceID, s view.Session) (view.StepKind, view.Screen) {
	switch sc.sel {
	case pauseResume:
		sc.cue(view.CueResume, -1, "")
		return view.StepPop, nil
	case pauseOptions:
		sc.cue(view.CueConfirm, -1, "")
		return view.StepPush, newOptions(sc.m, true)
	case pauseKeys:
		sc.cue(view.CueConfirm, -1, "")
		return view.StepPush, newKeys(sc.m)
	case pauseRemove:
		return sc.remove(d, s)
	}
	return sc.leave(s)
}

// remove meldet den Spieler des Geräts d ab und setzt das Spiel fort.
func (sc *pauseScreen) remove(d view.DeviceID, s view.Session) (view.StepKind, view.Screen) {
	p := sc.m.seatOf(d)
	if p < 0 {
		sc.deny(txtNoSeat, "")
		return view.StepStay, nil
	}
	if err := s.Leave(p); err != nil {
		sc.fail(err)
		return view.StepStay, nil
	}
	slog.Info("👋 Spieler im Pause-Menü abgemeldet", "player", p, "device", d.String())
	sc.cue(view.CueResume, -1, "")
	return view.StepPop, nil
}

// leave speichert und beendet das Spiel und ersetzt die Pause durch das Hauptmenü mit schon gelesener Spielstand-Liste.
// Ein Fehler beim Speichern erscheint dort; das Spiel ist trotzdem beendet (Session.Close).
func (sc *pauseScreen) leave(s view.Session) (view.StepKind, view.Screen) {
	if err := s.Close(); err != nil {
		slog.Warn("💥 Spiel beim Verlassen nicht sauber beendet", "err", err)
		sc.m.notice = note{raw: err.Error(), bad: true, set: true}
	}
	slog.Info("🛑 Spiel aus dem Pause-Menü verlassen")
	sc.cue(view.CueConfirm, -1, "")
	next := sc.m.Main()
	if ms, ok := next.(*mainScreen); ok {
		ms.refresh(s) // das erste Bild zeigt schon den Spielstand (B-813)
	}
	return view.StepReplace, next
}

func (sc *pauseScreen) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if lang := f.Settings.Language; sc.pg.stale(sc.ver, lang) {
		sc.fill(lang)
	}
	sc.m.paint.page(dst, c, &sc.pg)
}

// fill baut die Texte des Pause-Menüs.
func (sc *pauseScreen) fill(lang string) {
	p := &sc.pg
	p.reset(tr(lang, txtPauseTitle))
	for i := range pauseRows {
		p.add(row{label: tr(lang, pauseLabels[i]), mat: pauseMats[i], off: sc.off(i), art: pauseArt[i]})
	}
	if pl := sc.m.seatOf(sc.by); sc.byOK && pl >= 0 {
		p.rows[pauseRemove].value = tr1(lang, txtPlayer, strconv.Itoa(pl+1)) + " · " + deviceName(lang, sc.by)
	}
	p.sel = sc.sel
	p.note, p.bad = sc.note.text(lang), sc.note.bad
	p.hint = byHint(lang, txtPauseHint, &sc.base)
}
