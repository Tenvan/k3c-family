package room

import "k3c/engine/sim"

// Inselwechsel im Raum (B-345, docs/rules/stufen.md § 1): Meldet die Sim SwitchReady, ersetzt der Raum die Insel im
// selben Takt durch die nächste. Monarchen und Slots bleiben (Spieler-Index = Monarch), jedes Gerät bekommt Level und
// vollen Zustand jeder Stufe neu (kein Delta über den Wechsel), der Spielstand enthält die neue Insel.

// nextDef liefert die Daten der nächsten Insel; ein Test ersetzt es, weil die Daten nur eine Insel kennen.
var nextDef = (*sim.Island).NextDef

// switchIsland tauscht die Insel, wenn der Wechsel bereit ist und es eine nächste gibt (unter der Raum-Sperre).
// true: Die Insel wurde getauscht.
func (r *Room) switchIsland() bool {
	if !r.isl.SwitchReady {
		return false
	}
	def, ok := nextDef(r.isl)
	if !ok {
		return false
	}
	next, err := sim.NextIsland(r.isl, def)
	if err != nil {
		r.log().Error("💥 Inselwechsel fehlgeschlagen", "err", err)
		return false
	}
	r.log().Info("🪜 Inselwechsel", "von", r.isl.Number, "nach", next.Number, "tick", r.tick)
	r.isl, r.start = next, 0
	for _, d := range r.devices {
		clear(d.stages)
	}
	r.broadcastSeats()
	return true
}
