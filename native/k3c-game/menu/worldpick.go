package menu

import (
	"log/slog"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// Weltauswahl (B-938): erster Schritt von „Neues Spiel“. Links/Rechts blättert durch die Welten der Session
// (view.Session.WorldList), A wählt sie und öffnet die Lobby mit dem bestätigenden Gerät als Spieler 1, B geht zurück
// ins Hauptmenü. Die Seite zeigt das große Weltbild mit dem Namen darüber (der Name ist Schrift, nicht Teil des Bildes).

// worldPickScreen ist die Weltauswahl.
type worldPickScreen struct {
	base
	m      *menus
	list   []view.WorldInfo
	listed bool
	sel    int
	pg     page
}

func newWorldPick(m *menus, _ view.DeviceID) *worldPickScreen { return &worldPickScreen{m: m} }

// Update liest die Weltliste beim ersten Mal und bedient die Tasten.
func (sc *worldPickScreen) Update(in view.Input, s view.Session) view.Step {
	if !sc.listed {
		sc.listed = true
		sc.list = s.WorldList()
		if len(sc.list) == 0 {
			sc.list = []view.WorldInfo{{ID: "standard", Name: "Standard"}}
		}
		sc.ver++
	}
	return sc.run(in, s, sc)
}

func (sc *worldPickScreen) handle(d view.DeviceID, k key, _ view.Session) (view.StepKind, view.Screen) {
	switch k {
	case keyLeft, keyRight:
		dir := 1
		if k == keyLeft {
			dir = -1
		}
		if next := cycle(sc.sel, dir, len(sc.list)); next != sc.sel {
			sc.sel = next
			sc.cue(view.CueMove, -1, "")
			sc.ver++
		}
	case keyOK:
		w := sc.list[sc.sel]
		lb := newLobby(sc.m, d)
		lb.world = w.ID
		slog.Info("🚦 Welt gewählt", "welt", w.ID, "device", d.String())
		sc.cue(view.CueConfirm, -1, "")
		return view.StepPush, lb // B in der Lobby führt zurück zur Weltauswahl (Review WG)
	case keyBack, keyHome:
		sc.cue(view.CueBack, -1, "")
		return view.StepPop, nil
	}
	return view.StepStay, nil
}

func (sc *worldPickScreen) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if lang := f.Settings.Language; sc.pg.stale(sc.ver, lang) {
		sc.fill(lang)
	}
	sc.m.paint.page(dst, c, &sc.pg)
}

// fill baut die Seite: Titel, Weltbild mit Name, Hinweis.
func (sc *worldPickScreen) fill(lang string) {
	p := &sc.pg
	p.reset(tr(lang, txtLobbyTitle))
	p.sub = tr(lang, txtWorld)
	if len(sc.list) > 0 {
		w := sc.list[sc.sel]
		name := w.Name
		if name == "" {
			name = w.ID
		}
		p.hero = hero{on: true, name: name, art: sc.m.worldAsset(w.ID, true), left: len(sc.list) > 1, right: len(sc.list) > 1}
	}
	p.note, p.bad = sc.note.text(lang), sc.note.bad
	p.hint = byHint(lang, txtWorldHint, &sc.base)
}
