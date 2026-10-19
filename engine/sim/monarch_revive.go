package sim

import "math"

// Wiederbeleben (docs/rules/monarch.md § 5, B-120): Ein lebender Monarch derselben Stufe hält A in Reichweite eines
// Gefallenen, ohne Zahlziel in der Nähe; nach monarch.Revive.Seconds steht der Gefallene am Ort mit MaxHP ×
// HPFraction (Ereignis revived). Mehrere Helfer beschleunigen nicht, keiner lässt eine Münze fallen; hält kein Helfer
// mehr gültig (losgelassen, außer Reichweite, gefallen, getroffen), beginnt die Zeit von vorn (Q33). Ein getrennter
// Monarch (Free) ist nicht wiederbelebbar. Ohne Hilfe läuft RespawnIn weiter (Respawn an der Burg, stepPlayers).

// stepRevive läuft vor dem Zahlen in stepPlayers: markiert die Helfer (reviving) und treibt den Fortschritt.
func stepRevive(w *World, commands []PlayerCommand, dt float64) {
	for _, p := range w.Players {
		p.reviving = false
	}
	for _, down := range w.Players {
		if isAlive(down) || down.Free {
			continue
		}
		helped := false
		for _, h := range w.Players {
			if helpsRevive(w, h, down, commands) {
				h.reviving, helped = true, true
			}
		}
		if !helped {
			down.ReviveProgress = 0
			continue
		}
		down.ReviveProgress += dt
		if down.ReviveProgress >= monarch.Revive.Seconds {
			down.RespawnIn, down.HP, down.VX, down.ReviveProgress = 0, down.MaxHP*monarch.Revive.HPFraction, 0, 0
			emit(w, "revived", Event{"player": down.Index, "x": unitX(down.X)})
		}
	}
	for _, p := range w.Players {
		p.hit = false
	}
}

// helpsRevive: h lebt, wurde nicht getroffen, hält A in Reichweite von down und hat kein Zahlziel in der Nähe.
func helpsRevive(w *World, h, down *Player, commands []PlayerCommand) bool {
	if h == down || !isAlive(h) || h.hit || h.Index >= len(commands) || !commands[h.Index].Pay {
		return false
	}
	return math.Abs(h.X-down.X) <= monarch.Revive.RangeUnits && findPayTarget(w, h) == nil
}
