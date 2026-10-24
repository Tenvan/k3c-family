package room

// CloseRoom schließt den offenen Raum mit dem Spielstand name auf Wunsch eines Geräts (B-581, BD9.1). Ablauf wie das
// Aufräumen eines leeren Raums (lockedSweep): speichern, Report schreiben, schließen, Test-Stand löschen; verbundene
// Geräte bekommen `room_closed` und gehen zurück zur Raumliste, die Verbindung bleibt. Danach ist der Code frei und
// DeleteSave möglich. Ungültiger Name → ErrBadRequest, kein Raum mit diesem Stand → ErrRoomNotFound.
func (m *Manager) CloseRoom(name string) error {
	if !saveName.MatchString(name) {
		return ErrBadRequest
	}
	err := ErrRoomNotFound
	defer func() { m.notify(err == nil) }()
	m.mu.Lock()
	defer m.mu.Unlock()
	for code, r := range m.rooms {
		if r.Name != name {
			continue
		}
		r.closeByDevice()
		delete(m.rooms, code)
		err = nil
		return err
	}
	m.log().Info("🚫 Raum schließen abgelehnt, kein Raum offen", "ns", "room", "save", name)
	return err
}

func (r *Room) closeByDevice() {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.log().Info("🛑 Raum schließt", "grund", "vom Gerät geschlossen", "geraete", r.connected())
	r.saveNow()
	r.writeReport()
	r.closed = true
	r.dropTestSave()
	for _, d := range r.devices {
		if d.connected {
			d.peer.Closed(false)
		}
	}
}
