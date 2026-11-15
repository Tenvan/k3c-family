package devtools

// Wechsel und Zusammenfassungen der Chronik je Stufe, Spieler und Takt (chronicle.go).

import (
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// stageChanges loggt Tageszeit und Welle beim Wechsel und wertet die Ereignisse der Stufe aus.
func (c *chronicle) stageChanges(f *view.Frame, st *view.Stage) {
	w := st.World
	if w == nil {
		return
	}
	if changed(c.phase, st.Index, w.Cycle.Phase) {
		c.log.Info("🚦 Tageszeit", "stufe", st.Index, "tag", w.Cycle.Day, "phase", w.Cycle.Phase)
	}
	if changed(c.wave, st.Index, w.Wave) {
		c.log.Info("🚦 Welle", "stufe", st.Index, "welle", w.Wave, "gegner", len(w.Enemies))
	}
	for e := range f.EventsOf(st.Index) {
		c.event(st.Index, w, e.Data)
	}
}

// event wertet ein Ereignis aus: Bau fertig und Burgfall sofort, Kampf in die Nacht der Stufe.
func (c *chronicle) event(stage int, w *sim.World, e sim.Event) {
	switch typ, _ := e["type"].(string); typ {
	case "built":
		c.log.Info("🚦 Bau fertig", "stufe", stage, "art", e["kind"], "x", e["x"])
	case "castleFallen":
		c.log.Info("🚦 Burgfall", "stufe", stage, "tag", w.Cycle.Day)
	case "night":
		c.nights[stage] = &nightStats{day: w.Cycle.Day, troops: len(w.Troops)}
	case "dawn":
		c.endNight(stage, w)
	default:
		if n := c.nights[stage]; n != nil {
			n.add(typ, e)
		}
	}
}

// add zählt ein Kampf-Ereignis der Nacht.
func (n *nightStats) add(typ string, e sim.Event) {
	switch typ {
	case "wave":
		n.spawned += int(num(e["count"]))
	case "kill":
		n.killed++
	case "crownFallen", "destroyed":
		n.losses++
	case "hit":
		if e["target"] == "enemy" {
			n.dmgEnemy += num(e["damage"])
		} else {
			n.dmgOwn += num(e["damage"])
		}
	}
}

// endNight loggt die Kampf-Zusammenfassung der Nacht der Stufe (🏹) und vergisst sie.
func (c *chronicle) endNight(stage int, w *sim.World) {
	n := c.nights[stage]
	if n == nil {
		return
	}
	delete(c.nights, stage)
	lost := n.losses + max(0, n.troops-len(w.Troops))
	c.log.Info("🏹 Kampf der Nacht", "stufe", stage, "tag", n.day, "gespawnt", n.spawned, "getötet", n.killed,
		"verluste", lost, "schaden_an_gegnern", round1(n.dmgEnemy), "schaden_eigene_seite", round1(n.dmgOwn))
}

// seats loggt Stufenwechsel je Spieler, zählt Einnahmen und Ausgaben (Gold-Änderungen) und fasst sie je Tag zusammen.
func (c *chronicle) seats(f *view.Frame) {
	for _, s := range f.Seats {
		p := s.Player
		if p < 0 || p >= view.MaxPlayers {
			continue
		}
		if c.stage[p] >= 0 && c.stage[p] != s.Stage {
			c.log.Info("🚦 Stufenwechsel", "spieler", p+1, "von", c.stage[p], "nach", s.Stage)
		}
		c.stage[p] = s.Stage
		m := f.Monarch(p)
		if m == nil {
			continue
		}
		if d := m.Gold - c.gold[p]; c.seen[p] && d > 0 {
			c.income[p] += d
		} else if c.seen[p] && d < 0 {
			c.spending[p] -= d
		}
		c.gold[p], c.seen[p] = m.Gold, true
	}
	if w := f.Stages[0].World; w != nil && w.Cycle.Day > c.day {
		c.economy(f)
		c.day = w.Cycle.Day
	}
}

// economy loggt Einnahmen und Ausgaben je Spieler des abgelaufenen Tags (🪙) und setzt sie zurück.
func (c *chronicle) economy(f *view.Frame) {
	for _, s := range f.Seats {
		p := s.Player
		if p < 0 || p >= view.MaxPlayers {
			continue
		}
		c.log.Info("🪙 Wirtschaft des Tags", "tag", c.day, "spieler", p+1, "einnahmen", c.income[p], "ausgaben", c.spending[p],
			"gold", c.gold[p])
		c.income[p], c.spending[p] = 0, 0
	}
}

// budget loggt einen Takt über Budget (🐢) mit der Objektanzahl, höchstens alle slowLogEverySeconds; die Zahl der langsamen
// Takte seither steht dabei.
func (c *chronicle) budget(f *view.Frame) {
	if f.StepMs <= c.vals.StepBudgetMs {
		return
	}
	c.slowN++
	every := uint64(c.vals.SlowLogEverySeconds * 30)
	if c.slowAt != 0 && f.Tick < c.slowAt+every {
		return
	}
	troops, enemies, coins := 0, 0, 0
	for _, st := range f.Stages {
		if w := st.World; w != nil {
			troops, enemies, coins = troops+len(w.Troops), enemies+len(w.Enemies), coins+len(w.Coins)
		}
	}
	c.log.Warn("🐢 Takt über Budget", "ms", round1(f.StepMs), "budget_ms", c.vals.StepBudgetMs, "langsame_takte", c.slowN,
		"takt", f.Tick, "truppen", troops, "gegner", enemies, "muenzen", coins)
	c.slowAt, c.slowN = max(f.Tick, 1), 0
}

// num liest eine Zahl aus einem Ereignis-Feld (0 sonst).
func num(v any) float64 {
	switch x := v.(type) {
	case float64:
		return x
	case int:
		return float64(x)
	}
	return 0
}

func round1(v float64) float64 { return float64(int(v*10+0.5)) / 10 }
