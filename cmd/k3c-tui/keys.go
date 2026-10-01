package main

import tea "charm.land/bubbletea/v2"

// key verteilt eine Taste auf die Ansicht. Strg+C beendet überall.
func (m model) key(k string) (tea.Model, tea.Cmd) {
	if k == "ctrl+c" {
		return m, tea.Quit
	}
	switch m.screen {
	case scrRoom:
		return m.keyRoom(k)
	case scrLog:
		return m.keyLog(k)
	}
	return m.keyOverview(k)
}

func (m model) keyOverview(k string) (tea.Model, tea.Cmd) {
	switch k {
	case "q":
		return m, tea.Quit
	case "up", "k":
		m.sel = max(0, m.sel-1)
	case "down", "j":
		if m.st != nil {
			m.sel = max(0, min(m.sel+1, len(m.st.Rooms)-1))
		}
	case "enter":
		if m.st != nil && m.sel < len(m.st.Rooms) {
			m.roomCode, m.detail, m.devSel, m.flash = m.st.Rooms[m.sel].Code, nil, 0, ""
			return m.enter(scrRoom)
		}
	case "l":
		return m.openLog(scrOverview)
	}
	return m, nil
}

func (m model) keyRoom(k string) (tea.Model, tea.Cmd) {
	if m.confirm {
		switch k {
		case "y":
			m.confirm = false
			return m, m.disconnectCmd(m.roomCode, m.detail.Devices[m.devSel].ID)
		case "n", "esc", "q":
			m.confirm = false
		}
		return m, nil
	}
	switch k {
	case "esc", "q":
		m.flash = ""
		return m.enter(scrOverview)
	case "up", "k":
		m.devSel = max(0, m.devSel-1)
	case "down", "j":
		if m.detail != nil {
			m.devSel = max(0, min(m.devSel+1, len(m.detail.Devices)-1))
		}
	case "d":
		m.askDisconnect()
	case "s":
		return m, m.saveCmd(m.roomCode)
	case "l":
		return m.openLog(scrRoom)
	}
	return m, nil
}

// askDisconnect fragt nach, bevor ein verbundenes Gerät getrennt wird.
func (m *model) askDisconnect() {
	if m.detail == nil || m.devSel >= len(m.detail.Devices) || !m.detail.Devices[m.devSel].Connected {
		m.flash = "Kein verbundenes Gerät gewählt"
		return
	}
	m.confirm, m.flash = true, ""
}

func (m model) keyLog(k string) (tea.Model, tea.Cmd) {
	switch k {
	case "esc", "q":
		return m.enter(m.back)
	case "f":
		m.log.follow, m.log.off = !m.log.follow, 0
	case "up", "k":
		m.log.follow = false
		m.log.off = min(m.log.off+1, max(0, len(m.log.lines)-1))
	case "down", "j":
		m.log.off = max(0, m.log.off-1)
		m.log.follow = m.log.off == 0
	}
	return m, nil
}

func (m model) openLog(from screen) (tea.Model, tea.Cmd) {
	m.back, m.log = from, newLogState()
	return m.enter(scrLog)
}
