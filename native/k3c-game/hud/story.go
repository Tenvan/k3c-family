package hud

// Geschichtszeile (B-974, docs/rules/welten.md § 6.2): An den Wendepunkten der Kampagne blenden 1 bis 2 Zeilen der
// Geschichte in jeder Spielerhälfte ein und wieder aus, ohne Eingabe und ohne Pause. Welche Zeilen gelten, sagt die
// Engine (sim.WorldDef.Story…); hier steht nur, welches Ereignis welche Funktion fragt, und der Ablauf der Einblendung.
// Die Zeit kommt nur aus Frame.Clock; in der Pause, hinter der Ladeanzeige und während der Schiffsfahrt (die Uhr steht)
// wartet die Einblendung. Menüs und Seekarte verdecken sie, sie läuft dahinter ab.

import (
	"slices"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Ablauf einer Einblendung in Sekunden (B-974/R4: Startwerte, vorläufig bis zur Abstimmung am TV).
const (
	storyFadeIn  = 0.5
	storyHold    = 4.0
	storyFadeOut = 0.5
	storySeconds = storyFadeIn + storyHold + storyFadeOut
)

// storyEntry ist ein Eintrag der Geschichte: 1 bis 2 Zeilen (b leer = eine Zeile).
type storyEntry struct{ a, b string }

// storyQueue ist die Geschichtszeile eines Spielers: der laufende Eintrag mit seinem Alter, dahinter die wartenden.
type storyQueue struct {
	showing storyEntry
	age     float64 // Sekunden seit Beginn des laufenden Eintrags
	queue   []storyEntry
}

// add reiht einen Eintrag ein; derselbe Eintrag steht nur einmal (laufend oder wartend), ein leerer gar nicht.
func (q *storyQueue) add(e storyEntry) {
	if e.a == "" || e == q.showing || slices.Contains(q.queue, e) {
		return
	}
	q.queue = append(q.queue, e)
}

// step lässt dt Sekunden vergehen: Ist der laufende Eintrag abgelaufen, kommt der nächste (oder keiner).
func (q *storyQueue) step(dt float64) {
	if q.showing.a != "" {
		q.age += dt
		if q.age < storySeconds {
			return
		}
		q.showing = storyEntry{}
	}
	if len(q.queue) == 0 {
		return
	}
	q.showing, q.age = q.queue[0], 0
	q.queue = q.queue[:copy(q.queue, q.queue[1:])]
}

// alpha ist die Deckkraft des laufenden Eintrags: einblenden, stehen, ausblenden (0 ohne Eintrag).
func (q *storyQueue) alpha() float64 {
	switch {
	case q.showing.a == "":
		return 0
	case q.age < storyFadeIn:
		return q.age / storyFadeIn
	case q.age > storyFadeIn+storyHold:
		return max(0, (storySeconds-q.age)/storyFadeOut)
	}
	return 1
}

// reset verwirft den laufenden und die wartenden Einträge (der Puffer bleibt).
func (q *storyQueue) reset() { q.showing, q.age, q.queue = storyEntry{}, 0, q.queue[:0] }

// taleState ist der Zustand der Geschichte über alle Spieler: zu welchem Spiel die Einträge gehören, die Uhr des
// letzten Frames und ob der Sieg schon gemeldet ist.
type taleState struct {
	run   uint64
	clock float64
	won   bool
}

// story schreibt die Geschichtszeilen aller Spieler fort (nach den Plätzen in Listen): anderes Spiel oder kein Spiel
// verwirft alles, Wendepunkte dieses Frames reihen ein, dann läuft die Zeit.
func (h *hud) story(f *view.Frame) {
	t := &h.tale
	if f.Campaign.Run != t.run || len(f.Stages) == 0 {
		for p := range h.seats {
			h.seats[p].story.reset()
		}
		t.run, t.won = f.Campaign.Run, false
	}
	dt := f.Clock - t.clock
	t.clock = f.Clock
	if dt < 0 || f.Paused || f.Loading {
		dt = 0
	}
	if f.Campaign.World != nil {
		h.storyPoints(f)
	}
	for p := range h.seats {
		if q := &h.seats[p].story; h.seats[p].active {
			q.step(dt)
		} else {
			q.reset() // wer später dazukommt, sieht erst den nächsten Wendepunkt
		}
	}
}

// storyPoints fragt die Engine zu den Wendepunkten dieses Frames und reiht ihre Zeilen ein: neue Kampagne, Ankunft
// (islandSwitch, wenn der Raum die Insel getauscht hat), Endboss ausgelöst und besiegt, Sieg der Insel. Die Ereignisse
// aller Stufen zählen, nicht nur die der eigenen (der Wendepunkt gilt für die Insel). Miniboss: keine Zeile.
func (h *hud) storyPoints(f *view.Frame) {
	c := &f.Campaign
	w := c.World
	if c.Fresh {
		h.tell(w.StoryStart(), len(w.Story.Start))
	}
	for i := range f.Events {
		e := &f.Events[i]
		switch e.Type() {
		case "islandSwitch":
			if to := strOf(e.Data, "target"); to != "" && to == c.Island {
				h.tell(w.StoryArrival(to), 0)
			}
		case "bossSpawned":
			if isEndBoss(strOf(e.Data, "boss")) {
				h.tell(w.StoryBoss(c.Island), 0)
			}
		case "bossDefeated":
			if isEndBoss(strOf(e.Data, "boss")) {
				h.tell(w.StoryDefeated(c.Island, c.Completed), storyDoneLines(w, c.Island))
			}
		}
	}
	// Sieg der Insel (auch ohne Endboss, z. B. Tage überleben): Das Ereignis victory liegt in Stufe 0 und erreicht das
	// Spiel nicht immer, der Ausgang schon. Nach einem Sieg über den Endboss nennt die Engine dieselben Zeilen wie
	// bossDefeated; add reiht sie nicht doppelt ein.
	if won := f.Outcome == view.OutcomeWon; won != h.tale.won {
		h.tale.won = won
		if won {
			h.tell(w.StoryVictory(c.Island, c.Completed), storyDoneLines(w, c.Island))
		}
	}
}

// EndHolding (view.EndHolder): Am Spielende läuft oder wartet noch ein Eintrag der Geschichte; game zeigt das
// Hauptmenü erst danach (die Zeilen zum Sieg wären sonst nach view.EndDelay abgeschnitten).
func (h *hud) EndHolding() bool {
	for p := range h.seats {
		if q := &h.seats[p].story; h.seats[p].active && (q.showing.a != "" || len(q.queue) > 0) {
			return true
		}
	}
	return false
}

// isEndBoss meldet, ob die Boss-ID ein Endboss ist (bosses.json › endBosses).
func isEndBoss(id string) bool {
	_, ok := endBossPhases[id]
	return ok
}

// storyDoneLines ist die Zahl der Zeilen der Abschlusszeile `done` der Insel: Dort trennt sich der erste Eintrag vom
// folgenden (Etappe oder Ende), denn die Engine nennt die Zeilen eines Wendepunkts hintereinander.
func storyDoneLines(w *sim.WorldDef, island string) int {
	for i := range w.Islands {
		if w.Islands[i].ID == island {
			return len(w.Islands[i].Story.Done)
		}
	}
	return 0
}

// tell reiht die Zeilen eines Wendepunkts bei allen aktiven Spielern ein, in Einträgen zu 1 bis 2 Zeilen: zuerst die
// ersten first Zeilen (der erste Eintrag des Wendepunkts; 0 = keiner), dann der Rest, beides je höchstens zwei Zeilen
// (die Zeilenregel erlaubt je Eintrag 1 bis 2, welten.md § 2.2); über die Grenze first hinweg mischt kein Eintrag.
func (h *hud) tell(lines []string, first int) {
	first = min(max(first, 0), len(lines))
	for len(lines) > 0 {
		n := min(2, len(lines))
		if first > 0 {
			n = min(n, first)
		}
		first -= min(first, n)
		e := storyEntry{a: lines[0]}
		if n == 2 {
			e.b = lines[1]
		}
		lines = lines[n:]
		for p := range h.seats {
			if h.seats[p].active {
				h.seats[p].story.add(e)
			}
		}
	}
}
