package room

// DeleteSave führt del aus (Löschen des Spielstands name, B-506), solange kein Raum ihn offen hat. Ist er in einem Raum
// offen, auch ohne verbundenes Gerät, ErrSaveInUse: Der Raum würde ihn sonst beim nächsten Speichern neu schreiben.
// del läuft unter der Sperre des Managers, damit kein Raum den Stand währenddessen öffnet.
func (m *Manager) DeleteSave(name string, del func() error) error {
	if !saveName.MatchString(name) {
		return ErrBadRequest
	}
	m.mu.Lock()
	defer m.mu.Unlock()
	for _, r := range m.rooms {
		if r.Name == name {
			m.log().Info("🚫 Spielstand löschen abgelehnt, Raum offen", "ns", "room", "save", name, "room", r.Code)
			return ErrSaveInUse
		}
	}
	return del()
}
