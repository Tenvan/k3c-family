package hud

// Kampf-Anzeige (Spiegel von src/scenes/kampfAnzeige.ts, K5.1, B-132): Boss-Leiste und laufende Nacht-Events aus dem
// Zustand der Stufe. Der Warnkreis vor dem Flächenschlag liegt in der Welt (Paket fx).

import (
	"math"
	"strings"

	"k3c/engine/sim"
)

// bossView ist die Leiste eines Bosses: Anteil HP/Maximum, Name und Phase („Phase 2 von 3“, beim Miniboss leer).
type bossView struct {
	ok          bool
	id          int
	share       float64
	name, phase string
}

// bossBar wählt den Endboss, sonst den ersten Boss; ohne Boss ok false.
func bossBar(lang string, enemies []*sim.Enemy) bossView {
	var boss *sim.Enemy
	for _, e := range enemies {
		if e == nil || !e.Boss {
			continue
		}
		if _, end := endBossPhases[e.Kind]; end {
			boss = e
			break
		}
		if boss == nil {
			boss = e
		}
	}
	if boss == nil {
		return bossView{}
	}
	share := 0.0
	if boss.MaxHP > 0 {
		share = min(1, max(0, boss.HP/boss.MaxHP))
	}
	return bossView{true, boss.ID, share, nameOf(lang, "boss", boss.Kind, boss.Kind), phaseText(lang, boss.Kind, boss.Phase)}
}

// phaseText ist „Phase n von m“ eines Endbosses; Miniboss oder ohne Phase "".
func phaseText(lang, kind string, phase int) string {
	of, ok := endBossPhases[kind]
	if !ok || phase <= 0 {
		return ""
	}
	return tf(lang, "boss.phase", "n", phase, "of", of)
}

// bossTitle ist die Zeile über der Leiste: Name und Phase.
func bossTitle(b bossView) string {
	if b.phase == "" {
		return b.name
	}
	return b.name + sep + b.phase
}

// eventsKey: laufende Events mit Restzeit in ganzen Sekunden (Zeitraffer eingerechnet) und Händler-Überfall.
type eventsKey struct {
	lang string
	ids  [3]string
	left [3]int
	n    int
	raid bool
}

// newEventsKey liest höchstens drei laufende Events.
func newEventsKey(lang string, events []sim.ActiveEvent, speed float64, m *sim.Merchant) eventsKey {
	if speed <= 0 {
		speed = 1
	}
	k := eventsKey{lang: lang, raid: m != nil && m.Raid}
	for _, e := range events {
		if k.n == len(k.ids) {
			break
		}
		k.ids[k.n], k.left[k.n] = e.ID, int(max(0, math.Ceil(e.SecondsLeft/speed)))
		k.n++
	}
	return k
}

// eventBanner ist eine Zeile je Event mit Restzeit, der Händler-Überfall ohne Restzeit; ohne Event "".
func eventBanner(k eventsKey) string {
	lines := make([]string, 0, k.n+1)
	for i := range k.n {
		lines = append(lines, tf(k.lang, "event.timed", "name", nameOf(k.lang, "event", k.ids[i], k.ids[i]),
			"left", formatTime(float64(k.left[i]))))
	}
	if k.raid {
		lines = append(lines, tr(k.lang, "event.merchantRaid"))
	}
	return strings.Join(lines, "\n")
}
