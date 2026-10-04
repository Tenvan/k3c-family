package room

import "k3c/engine/sim"

// Skills über das Protokoll (docs/protocol.md › Skills und Aktionen, B-123): Lernen und Respec für einen Slot des
// Geräts. Die Regeln prüft die Sim (sim.LearnSkill, sim.Respec); jeder Fehler ist bad_request.

// Learn lernt für den Monarchen im Slot `slot` des Geräts den Skill `skill` (ID aus monarch.json › skills).
func (r *Room) Learn(id string, peer Peer, slot int, skill string) error {
	return r.withPlayer(id, peer, slot, func(w *sim.World, p *sim.Player) error { return sim.LearnSkill(w, p, skill) })
}

// Respec setzt die Skills des Monarchen im Slot `slot` des Geräts zurück (nur am Tag an der Burg, prüft die Sim).
func (r *Room) Respec(id string, peer Peer, slot int) error {
	return r.withPlayer(id, peer, slot, sim.Respec)
}

// withPlayer ruft f unter der Raum-Sperre mit Spieler und Welt seiner Stufe auf; fremder Slot oder Fehler: ErrBadRequest.
func (r *Room) withPlayer(id string, peer Peer, slot int, f func(*sim.World, *sim.Player) error) error {
	r.mu.Lock()
	defer r.mu.Unlock()
	d := r.own(id, peer)
	if d == nil {
		return ErrBadRequest
	}
	idx, ok := d.slots[slot]
	if !ok {
		return ErrBadRequest
	}
	stage := r.isl.StageOf(idx)
	if stage < 0 {
		return ErrBadRequest
	}
	w := r.isl.Stages[stage]
	for _, p := range w.Players {
		if p.Index == idx {
			if err := f(w, p); err != nil {
				r.log().Debug("🚫 Skill-Aktion abgelehnt", "slot", slot, "err", err)
				return ErrBadRequest
			}
			return nil
		}
	}
	return ErrBadRequest
}
