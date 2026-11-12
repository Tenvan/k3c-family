package menu

import (
	"log/slog"
	"strconv"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// Einträge des Hauptmenüs von oben nach unten (B-800: Weiter, Neues Spiel, Laden, Optionen, Beenden; B-808: Lizenzen).
const (
	mainContinue = iota
	mainNew
	mainLoad
	mainOptions
	mainLicenses
	mainQuit
	mainCount
)

var mainLabels = [mainCount]textKey{txtContinue, txtNew, txtLoad, txtOptions, txtLicenses, txtQuit}
var mainSubs = [mainCount]textKey{txtContinueSub, txtNewSub, txtLoadSub, txtOptionsSub, txtLicensesSub, txtQuitSub}
var mainMats = [mainCount]material{matGold, matGreen, matBlue, matPurple, matRed, matCoral}

// mainScreen ist das Hauptmenü. „Weiter“ ist ausgegraut, solange es keinen ladbaren Spielstand gibt; die Liste liest
// es beim ersten Update und nach jeder Rückkehr aus einem Untermenü (Laden kann löschen). Bis dahin zeigt „Weiter“
// „lädt ...“, nie „Kein Spielstand vorhanden“ (Draw vor dem ersten Update, B-813). Unter dem Titel steht die Version.
type mainScreen struct {
	base
	m      *menus
	sel    int
	listed bool
	latest view.SaveInfo // neuester ladbarer Spielstand (Name leer: keiner)
	ended  view.Outcome  // Ausgang des letzten Spiels, für den die Texte gelten (Frame.Outcome)
	pg     page
}

func newMain(m *menus) *mainScreen { return &mainScreen{m: m} }

// Update liest bei Bedarf die Spielstände und bedient die Tasten.
func (sc *mainScreen) Update(in view.Input, s view.Session) view.Step {
	if !sc.listed {
		sc.refresh(s)
	}
	return sc.run(in, s, sc)
}

// refresh sucht den neuesten ladbaren Spielstand und wählt „Weiter“, wenn es ihn gibt, sonst „Neues Spiel“.
func (sc *mainScreen) refresh(s view.Session) {
	sc.listed, sc.latest = true, view.SaveInfo{}
	list, err := s.List()
	if err != nil {
		sc.note = note{raw: err.Error(), bad: true, set: true}
	}
	for _, e := range list {
		if e.Loadable {
			sc.latest = e
			break
		}
	}
	if sc.sel == mainContinue && sc.off(mainContinue) {
		sc.sel = mainNew
	}
	sc.ver++
}

// off meldet ausgegraute Einträge (Weiter ohne Spielstand; vor dem Lesen der Liste noch nicht).
func (sc *mainScreen) off(i int) bool { return i == mainContinue && sc.listed && sc.latest.Name == "" }

func (sc *mainScreen) handle(d view.DeviceID, k key, s view.Session) (view.StepKind, view.Screen) {
	switch k {
	case keyUp, keyDown:
		dir := 1
		if k == keyUp {
			dir = -1
		}
		if next := move(sc.sel, dir, mainCount, false, sc.off); next != sc.sel {
			sc.sel = next
			sc.cue(view.CueMove, -1, "")
		}
		sc.ver++
	case keyHome:
		sc.sel = first(mainCount, sc.off)
		sc.ver++
	case keyOK:
		return sc.confirm(d, s)
	}
	return view.StepStay, nil
}

// confirm löst den gewählten Eintrag aus. Untermenüs kommen über StepPush; danach ist die Spielstand-Liste neu zu lesen.
func (sc *mainScreen) confirm(d view.DeviceID, s view.Session) (view.StepKind, view.Screen) {
	sc.note = note{}
	switch sc.sel {
	case mainContinue:
		if err := s.Continue([]view.DeviceID{d}); err != nil {
			sc.fail(err)
			sc.listed = false
			return view.StepStay, nil
		}
		slog.Info("📂 Weiter mit dem neuesten Spielstand", "device", d.String())
		sc.cue(view.CueConfirm, -1, "")
		return view.StepPlay, nil
	case mainNew:
		sc.cue(view.CueConfirm, -1, "")
		return view.StepPush, newLobby(sc.m, d)
	case mainLoad:
		sc.listed = false
		sc.cue(view.CueConfirm, -1, "")
		return view.StepPush, newSaves(sc.m)
	case mainOptions:
		sc.cue(view.CueConfirm, -1, "")
		return view.StepPush, newOptions(sc.m, false)
	case mainLicenses:
		sc.cue(view.CueConfirm, -1, "")
		return view.StepPush, newLicenses(sc.m)
	}
	slog.Info("🛑 Beenden aus dem Hauptmenü")
	sc.cue(view.CueConfirm, -1, "")
	return view.StepQuit, nil
}

// Draw zeichnet das Hauptmenü; die Texte entstehen nur bei neuer Version oder Sprache.
func (sc *mainScreen) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if lang := f.Settings.Language; sc.pg.stale(sc.ver, lang) || sc.ended != f.Outcome {
		sc.ended = f.Outcome
		sc.fill(lang)
	}
	sc.m.paint.page(dst, c, &sc.pg)
}

// fill baut die Texte der Seite in der Sprache lang.
func (sc *mainScreen) fill(lang string) {
	p := &sc.pg
	p.reset(tr(lang, txtTitle))
	p.sub = versionText(lang)
	for i := range mainCount {
		r := row{label: tr(lang, mainLabels[i]), value: tr(lang, mainSubs[i]), mat: mainMats[i], off: sc.off(i)}
		if i == mainContinue {
			r.value = tr(lang, txtContinueNone)
			if !sc.listed {
				r.value = tr(lang, txtContinueLoading)
			}
			if sc.latest.Name != "" {
				r.value = tr1(lang, txtContinueSub, sc.latest.Name) + " · " + tr1(lang, txtSavesLine, strconv.Itoa(sc.latest.Day))
			}
		}
		p.add(r)
	}
	p.sel = sc.sel
	n := sc.note
	if !n.set {
		n = outcomeNote(sc.ended)
	}
	p.note, p.bad = n.text(lang), n.bad
	p.hint = byHint(lang, txtMainHint, &sc.base)
}

// outcomeNote ist die Meldung zum Ausgang des letzten Spiels (leer, solange keiner feststeht).
func outcomeNote(o view.Outcome) note {
	switch o {
	case view.OutcomeLost:
		return note{key: txtLost, set: true, bad: true}
	case view.OutcomeWon:
		return note{key: txtWon, set: true}
	}
	return note{}
}

// byHint ist die Hinweiszeile mit dem bedienenden Gerät (bei zwei Spielern sichtbar, wer das Menü führt).
func byHint(lang string, k textKey, b *base) string {
	h := tr(lang, k)
	if b.byOK {
		h += " · " + tr1(lang, txtBy, deviceName(lang, b.by))
	}
	return h
}
