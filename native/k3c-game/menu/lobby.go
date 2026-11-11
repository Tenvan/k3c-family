package menu

import (
	"log/slog"
	"slices"
	"strconv"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Lobby (Vorbild src/scenes/lobbyOptions.ts, localSlots.ts): Wer „Neues Spiel“ gewählt hat, ist Spieler 1. Ein
// weiteres Gerät tritt mit A bei (höchstens view.MaxPlayers), Spieler 2 geht mit B, B von Spieler 1 bricht ab.
// Bedienen dürfen nur beigetretene Geräte. Grad, Ziel, Niederlage, Welt und Seed wie der Anlegen-Dialog des Browsers;
// ein neuer Grad bringt seine Standard-Niederlage mit, solange sie niemand selbst gewählt hat.

// Zeilen der Lobby von oben nach unten.
const (
	lobbyGrade = iota
	lobbyGoal
	lobbyDefeat
	lobbyWorld
	lobbySeed
	lobbyStart
	lobbyCancel
	lobbyRows
)

// maxSeed ist der höchste wählbare Seed; 0 heißt „Name des Spielstands“ (NewGame.Seed leer).
const maxSeed = 999

var (
	grades       = []string{"easy", "normal", "hard", "ultra"} // ohne dev (nur Dev-Mode, B-800 Nicht-Ziel)
	gradeTexts   = []textKey{txtGradeEasy, txtGradeNormal, txtGradeHard, txtGradeUltra}
	goals        = []string{"endboss", "gold", "days", "mineAll", "buildAll"}
	goalTexts    = []textKey{txtGoalEndboss, txtGoalGold, txtGoalDays, txtGoalMineAll, txtGoalBuildAll}
	defeats      = []string{"resources", "stage", "lost"}
	defeatTexts  = []textKey{txtDefeatResources, txtDefeatStage, txtDefeatLost}
	worlds       = []string{sim.StandardWorldID} // weitere Welten nur im Dev-Mode (docs/rules/welten.md § 6)
	lobbyLabels  = [lobbyRows]textKey{txtGrade, txtGoal, txtDefeat, txtWorld, txtSeed, txtStart, txtCancel}
	lobbyChoices = [lobbySeed]int{len(grades), len(goals), len(defeats), len(worlds)}
)

// lobbyScreen ist die Lobby eines neuen Spiels.
type lobbyScreen struct {
	base
	m         *menus
	devs      [view.MaxPlayers]view.DeviceID
	n         int
	row       int
	pick      [lobbySeed]int // gewählter Index je Wahl-Zeile (Grad, Ziel, Niederlage, Welt)
	seed      int
	defeatSet bool
	pg        page
}

// newLobby öffnet die Lobby mit dem Gerät d als Spieler 1 und den Standards (Normal, Endboss, Standard-Welt).
func newLobby(m *menus, d view.DeviceID) *lobbyScreen {
	sc := &lobbyScreen{m: m, n: 1}
	sc.devs[0] = d
	sc.pick[lobbyGrade] = slices.Index(grades, "normal")
	sc.pick[lobbyDefeat] = defaultDefeat(grades[sc.pick[lobbyGrade]])
	return sc
}

// defaultDefeat ist der Index der Standard-Niederlage des Grads (data/difficulty.json über sim).
func defaultDefeat(grade string) int {
	return max(0, slices.Index(defeats, sim.DefaultOptionsFor(grade).Defeat))
}

func (sc *lobbyScreen) Update(in view.Input, s view.Session) view.Step { return sc.run(in, s, sc) }

// slot liefert den Spieler des Geräts d (-1 = nicht beigetreten).
func (sc *lobbyScreen) slot(d view.DeviceID) int { return slices.Index(sc.devs[:sc.n], d) }

func (sc *lobbyScreen) handle(d view.DeviceID, k key, s view.Session) (view.StepKind, view.Screen) {
	p := sc.slot(d)
	switch {
	case p < 0 && k == keyOK:
		sc.join(d)
	case p < 0:
		return view.StepStay, nil // nicht beigetretene Geräte bedienen nichts
	case k == keyBack && p > 0:
		sc.leave(p)
	case k == keyBack || k == keyHome:
		sc.cue(view.CueBack, -1, "")
		return view.StepPop, nil
	case k == keyUp || k == keyDown:
		sc.moveRow(k)
	case k == keyLeft || k == keyRight || k == keyOK:
		return sc.change(k, s)
	}
	return view.StepStay, nil
}

// join nimmt das Gerät d als nächsten Spieler auf.
func (sc *lobbyScreen) join(d view.DeviceID) {
	if sc.n >= view.MaxPlayers {
		sc.deny(txtTooMany, "")
		return
	}
	sc.devs[sc.n] = d
	sc.n++
	sc.cue(view.CueJoined, sc.n-1, "")
	sc.ver++
	slog.Info("👑 Spieler tritt der Lobby bei", "player", sc.n-1, "device", d.String())
}

// leave lässt Spieler p gehen; die Späteren rücken auf.
func (sc *lobbyScreen) leave(p int) {
	copy(sc.devs[p:], sc.devs[p+1:sc.n])
	sc.n--
	sc.cue(view.CueLeft, p, "")
	sc.ver++
	slog.Info("👋 Spieler verlässt die Lobby", "player", p)
}

// moveRow wählt die Zeile darüber bzw. darunter (am Rand bleibt sie).
func (sc *lobbyScreen) moveRow(k key) {
	dir := 1
	if k == keyUp {
		dir = -1
	}
	if next := move(sc.row, dir, lobbyRows, false, nil); next != sc.row {
		sc.row = next
		sc.cue(view.CueMove, -1, "")
	}
	sc.ver++
}

// change ändert den Wert der Zeile (links/rechts begrenzt, A im Kreis) oder löst Start bzw. Abbrechen aus.
func (sc *lobbyScreen) change(k key, s view.Session) (view.StepKind, view.Screen) {
	dir := 1
	if k == keyLeft {
		dir = -1
	}
	switch sc.row {
	case lobbyStart:
		if k == keyOK {
			return sc.start(s)
		}
		return view.StepStay, nil
	case lobbyCancel:
		if k == keyOK {
			sc.cue(view.CueBack, -1, "")
			return view.StepPop, nil
		}
		return view.StepStay, nil
	case lobbySeed:
		sc.seed = stepValue(sc.seed, dir, maxSeed+1, k == keyOK)
	default:
		sc.choose(sc.row, stepValue(sc.pick[sc.row], dir, lobbyChoices[sc.row], k == keyOK))
	}
	sc.cue(view.CueMove, -1, "")
	sc.ver++
	return view.StepStay, nil
}

// stepValue verschiebt i um dir unter n Werten: im Kreis (A) oder am Rand begrenzt (links/rechts).
func stepValue(i, dir, n int, round bool) int {
	if round {
		return cycle(i, dir, n)
	}
	return max(0, min(n-1, i+dir))
}

// choose setzt die Wahl-Zeile r auf Index i; ein neuer Grad bringt seine Standard-Niederlage mit.
func (sc *lobbyScreen) choose(r, i int) {
	sc.pick[r] = i
	switch r {
	case lobbyGrade:
		if !sc.defeatSet {
			sc.pick[lobbyDefeat] = defaultDefeat(grades[i])
		}
	case lobbyDefeat:
		sc.defeatSet = true
	}
}

// newGame sind die Angaben des neuen Spiels (Name leer: local wählt spiel-N; Seed leer: Name als Seed).
func (sc *lobbyScreen) newGame() view.NewGame {
	g := view.NewGame{
		Grade:   grades[sc.pick[lobbyGrade]],
		Goal:    goals[sc.pick[lobbyGoal]],
		Defeat:  defeats[sc.pick[lobbyDefeat]],
		World:   worlds[sc.pick[lobbyWorld]],
		Devices: slices.Clone(sc.devs[:sc.n]),
	}
	if sc.seed > 0 {
		g.Seed = strconv.Itoa(sc.seed)
	}
	return g
}

// start legt das Spiel an; bei Fehler bleibt die Lobby mit Meldung.
func (sc *lobbyScreen) start(s view.Session) (view.StepKind, view.Screen) {
	g := sc.newGame()
	if err := s.New(g); err != nil {
		sc.fail(err)
		return view.StepStay, nil
	}
	slog.Info("🚀 Neues Spiel aus der Lobby", "grad", g.Grade, "ziel", g.Goal, "welt", g.World, "seed", g.Seed, "spieler", len(g.Devices))
	sc.cue(view.CueConfirm, -1, "")
	return view.StepPlay, nil
}

func (sc *lobbyScreen) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if lang := f.Settings.Language; sc.pg.stale(sc.ver, lang) {
		sc.fill(lang)
	}
	sc.m.paint.page(dst, c, &sc.pg)
}

// fill baut die Texte der Lobby: Spieler-Zeile oben, Wahl-Zeilen mit „< Wert >“, Start und Abbrechen.
func (sc *lobbyScreen) fill(lang string) {
	p := &sc.pg
	p.reset(tr(lang, txtLobbyTitle))
	p.sub = sc.players(lang)
	for r := range lobbyRows {
		rw := row{label: tr(lang, lobbyLabels[r]), mat: matBlue}
		switch r {
		case lobbyStart:
			rw.mat = matGreen
		case lobbyCancel:
			rw.mat = matCoral
		default:
			rw.value = "< " + sc.valueText(lang, r) + " >"
		}
		p.add(rw)
	}
	p.sel = sc.row
	p.note, p.bad = sc.note.text(lang), sc.note.bad
	p.hint = tr(lang, txtLobbyHint)
}

// players ist die Zeile der Plätze: „Spieler 1: Pad 1 · Spieler 2: frei: A drücken“.
func (sc *lobbyScreen) players(lang string) string {
	out := ""
	for i := range view.MaxPlayers {
		if i > 0 {
			out += "   ·   "
		}
		who := tr(lang, txtPlayerFree)
		if i < sc.n {
			who = deviceName(lang, sc.devs[i])
		}
		out += tr1(lang, txtPlayer, strconv.Itoa(i+1)) + ": " + who
	}
	return out
}

// valueText ist der angezeigte Wert der Wahl-Zeile r.
func (sc *lobbyScreen) valueText(lang string, r int) string {
	switch r {
	case lobbyGrade:
		return tr(lang, gradeTexts[sc.pick[r]])
	case lobbyGoal:
		return tr(lang, goalTexts[sc.pick[r]])
	case lobbyDefeat:
		return tr(lang, defeatTexts[sc.pick[r]])
	case lobbyWorld:
		return worlds[sc.pick[r]]
	}
	if sc.seed == 0 {
		return tr(lang, txtSeedAuto)
	}
	return strconv.Itoa(sc.seed)
}
