package room

// DevAction ist eine Dev-Aktion eines Geräts (Nachricht dev, B-178): gold und material brauchen slot und amount,
// material zusätzlich resource, timescale braucht factor. Die Wirkung bauen DBG1.2 (gold, material) und DBG1.3
// (timescale).
type DevAction struct {
	Action   string
	Slot     *int
	Amount   int
	Resource string
	Factor   int
}

// Dev führt eine Dev-Aktion aus. Ohne Dev-Mode (Manager.Dev) ist sie verboten, noch vor jeder Feldprüfung, damit ein
// Server ohne Dev-Mode nichts über Felder verrät; die Ablehnung steht als Warnung im Log.
func (r *Room) Dev(id string, peer Peer, a DevAction) error {
	if !r.m.Dev {
		r.log().Warn("Dev-Aktion abgelehnt", "device", short(id), "aktion", a.Action)
		return ErrForbidden
	}
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.own(id, peer) == nil {
		return ErrBadRequest
	}
	switch a.Action {
	default:
		return ErrBadRequest
	}
}
