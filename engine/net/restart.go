package net

// codeRestarting kündigt einen Neustart des Servers an (B-507): Das Studio hat Dateien geändert, der Server startet
// gleich neu. Raum und Verbindung bleiben, bis das Herunterfahren room_closed schickt (docs/protocol.md › Fehler-Codes).
const codeRestarting = "restarting"

// Notices sind Hinweise des Servers an alle Geräte; NewHandler verbindet sie mit seinen Verbindungen
// (wie Rooms.Changed). Der Nullwert und nil sind ohne Wirkung.
type Notices struct {
	restarting func()
}

// Restarting schickt jedem verbundenen Gerät den Hinweis „Server startet neu“ (Fehler-Code restarting).
func (n *Notices) Restarting() {
	if n != nil && n.restarting != nil {
		n.restarting()
	}
}

// broadcastRestarting stellt den Hinweis in die Warteschlange jeder Verbindung, in der Raumliste wie im Raum.
func (s *server) broadcastRestarting() {
	msg := errMsg(codeRestarting)
	s.mu.Lock()
	defer s.mu.Unlock()
	s.log.Info("🔁 Hinweis an alle Geräte: Server startet neu", "ns", "ws", "verbindungen", len(s.conns))
	for c := range s.conns {
		c.enqueue(msg)
	}
}
