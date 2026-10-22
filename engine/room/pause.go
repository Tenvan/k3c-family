package room

// Pause eines Geräts (docs/rules/bedienung.md § 1, docs/protocol.md › pause/resume, B-214): Nachrichten `pause` und
// `resume` setzen device.paused. Die Wirkung im Tick folgt in BD3.3.

// Pause hält das Gerät an (on) oder setzt es fort. Ein zweites `pause` und ein `resume` ohne Pause ändern nichts.
func (r *Room) Pause(id string, peer Peer, on bool) error {
	r.mu.Lock()
	defer r.mu.Unlock()
	d := r.own(id, peer)
	if d == nil {
		return ErrBadRequest
	}
	if d.paused == on {
		return nil
	}
	d.paused = on
	r.log().Info("🚦 Gerät pausiert", "device", short(id), "pausiert", on)
	return nil
}
