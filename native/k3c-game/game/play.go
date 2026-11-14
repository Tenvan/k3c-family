package game

import (
	"log/slog"
	"slices"

	"k3c/native/k3c-game/view"
)

// play bedient das laufende Spiel (kein Screen offen): Overlays (Skill-Menü) zuerst, dann Pause bei ActionMenu bzw.
// ActionHome, verwaiste Plätze nach der Zuordnungs-Regel, Befehle der Geräte an ihre Spieler und Beitritt neuer Geräte
// mit Bestätigen. Verbrauchte Spieler bekommen einen leeren Befehl, ihre Aktionen zählen nicht. B ist im Spiel unbelegt.
// Endet das Spiel (Absturz oder Spielende), liegt danach das Hauptmenü oben und dieses Update tut sonst nichts.
func (g *Game) play(in view.Input) {
	if !g.sim.Active() || g.ending() {
		g.endIn = 0
		g.stack.Push(g.parts.Menus.Main())
		g.mode = ModeMenu
		return
	}
	consumed := g.overlays(in)
	if g.wantsPause(in, consumed) {
		g.sim.Pause(true)
		g.stack.Push(g.parts.Menus.Pause())
		g.mode = ModeMenu
		return
	}
	g.rebind(in)
	for _, s := range g.frame.Seats {
		var c view.Command
		if di, ok := in.Of(s.Device); ok && !isConsumed(consumed, s.Player) {
			c = di.Command
		}
		if err := g.sim.Command(s.Player, c); err != nil {
			slog.Warn("🚫 Befehl abgelehnt", "player", s.Player, "err", err)
		}
	}
	g.join(in)
}

// ending zählt die Takte im Spiel, seit der Frame einen Ausgang trägt (Frame.Outcome), und schließt das Spiel nach
// view.EndDelay (Session.Close speichert; nach einer Niederlage speichert der Raum nicht mehr). Ein Frame ohne Stufe
// (Menü, beendetes Spiel) setzt den Zähler zurück, damit ein späteres Spiel neu zählt. true: Spiel geschlossen.
func (g *Game) ending() bool {
	if g.frame.Outcome == view.OutcomeNone || len(g.frame.Stages) == 0 {
		g.endIn = 0
		return false
	}
	if g.endIn++; g.endIn < view.EndDelay {
		return false
	}
	if err := g.sim.Close(); err != nil {
		slog.Warn("💥 Spiel nach Spielende nicht sauber beendet", "err", err)
	}
	slog.Info("🛑 Spielende, zurück ins Hauptmenü", "ausgang", g.frame.Outcome)
	return true
}

// overlays ruft alle Overlays und vereinigt die verbrauchten Spieler; ihre Cues kommen in den nächsten Frame.
func (g *Game) overlays(in view.Input) [view.MaxPlayers]bool {
	var consumed [view.MaxPlayers]bool
	for _, o := range g.parts.Overlays {
		st := o.Update(in, &g.frame, g.sim)
		g.cues = append(g.cues, st.Cues...)
		for p, c := range st.Consumed {
			consumed[p] = consumed[p] || c
		}
	}
	return consumed
}

func isConsumed(consumed [view.MaxPlayers]bool, p int) bool {
	return p >= 0 && p < view.MaxPlayers && consumed[p]
}

// wantsPause meldet ActionMenu oder ActionHome eines Geräts, dessen Spieler nicht im Overlay ist.
func (g *Game) wantsPause(in view.Input, consumed [view.MaxPlayers]bool) bool {
	for _, di := range in.Devices {
		if isConsumed(consumed, seatOf(g.frame.Seats, di.Device)) {
			continue
		}
		if slices.Contains(di.Actions, view.ActionMenu) || slices.Contains(di.Actions, view.ActionHome) {
			return true
		}
	}
	return false
}

// rebind gibt verwaisten Plätzen ein freies Gerät (Rebinds) über die Session.
func (g *Game) rebind(in view.Input) {
	g.devs.rebinds = Rebinds(g.devs.rebinds[:0], g.frame.Seats, in, g.devs.fresh)
	for _, r := range g.devs.rebinds {
		if err := g.sim.Rebind(r.Player, r.Device); err != nil {
			g.cues = append(g.cues, view.Cue{Kind: view.CueDenied, Player: r.Player, Text: err.Error()})
		}
	}
}

// join nimmt freie Geräte, die Bestätigen drücken und nicht gerade einen verwaisten Platz übernommen haben, als
// weitere Spieler auf.
func (g *Game) join(in view.Input) {
	for _, di := range in.Devices {
		if seatOf(g.frame.Seats, di.Device) >= 0 || !slices.Contains(di.Actions, view.ActionConfirm) || g.rebound(di.Device) {
			continue
		}
		if p, err := g.sim.Join(di.Device); err != nil {
			g.cues = append(g.cues, view.Cue{Kind: view.CueDenied, Player: -1, Text: err.Error()})
		} else {
			slog.Info("👑 Gerät spielt mit", "device", di.Device.String(), "player", p)
		}
	}
}

// rebound meldet, ob das Gerät in diesem Update einen verwaisten Platz bekommen hat.
func (g *Game) rebound(d view.DeviceID) bool {
	for _, r := range g.devs.rebinds {
		if r.Device == d {
			return true
		}
	}
	return false
}
