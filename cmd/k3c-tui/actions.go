package main

import (
	"context"

	tea "charm.land/bubbletea/v2"
)

// disconnectCmd trennt das Gerät (Kennung aus der Raumansicht) und meldet das Ergebnis in der Statuszeile.
func (m model) disconnectCmd(room, device string) tea.Cmd {
	return m.call(func(ctx context.Context) tea.Msg {
		if err := m.c.disconnect(ctx, room, device); err != nil {
			return actionMsg{err: err}
		}
		return actionMsg{text: "Gerät " + device + " getrennt"}
	})
}

// saveCmd sichert den Spielstand des Raums sofort.
func (m model) saveCmd(room string) tea.Cmd {
	return m.call(func(ctx context.Context) tea.Msg {
		if err := m.c.save(ctx, room); err != nil {
			return actionMsg{err: err}
		}
		return actionMsg{text: "Spielstand von " + room + " gesichert"}
	})
}
