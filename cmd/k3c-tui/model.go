package main

import (
	"context"
	"strings"
	"time"

	tea "charm.land/bubbletea/v2"
)

// Takt der Anzeige: jede Sekunde neu laden, nach einem Fehler alle 2 Sekunden erneut versuchen.
const (
	refreshEvery = time.Second
	retryEvery   = 2 * time.Second
)

type screen int

const (
	scrOverview screen = iota
	scrRoom
	scrLog
)

// Nachrichten. chain ordnet Ergebnisse und Takte einer Ansicht zu: Wer nach einem Wechsel noch eintrifft, plant keinen Takt mehr.
type (
	statusMsg struct {
		st    status
		err   error
		chain int
	}
	roomMsg struct {
		d     roomDetail
		err   error
		chain int
	}
	logMsg struct {
		page  logPage
		err   error
		chain int
	}
	// actionMsg ist das Ergebnis einer Aktion (Gerät trennen, Spielstand sichern); text steht in der Statuszeile.
	actionMsg struct {
		text string
		err  error
	}
	tickMsg struct{ chain int }
)

// model ist der Zustand der TUI (Bubble Tea, Elm-Architektur).
type model struct {
	c             *client
	st            *status
	err           string // Fehler des letzten Abrufs der aktuellen Ansicht
	flash         string // Statuszeile: Ergebnis der letzten Aktion
	width, height int
	refresh       time.Duration
	retry         time.Duration

	screen, back screen
	chain        int
	sel          int // gewählte Raumzeile der Übersicht
	roomCode     string
	detail       *roomDetail
	devSel       int  // gewähltes Gerät der Raumansicht
	confirm      bool // „Gerät trennen? (y/n)“ ist offen
	log          logState
}

func newModel(c *client) model {
	return model{c: c, refresh: refreshEvery, retry: retryEvery}
}

func (m model) Init() tea.Cmd { return m.poll() }

// poll ruft die Daten der aktuellen Ansicht ab.
func (m model) poll() tea.Cmd {
	chain := m.chain
	switch m.screen {
	case scrRoom:
		return m.call(func(ctx context.Context) tea.Msg {
			d, err := m.c.room(ctx, m.roomCode)
			return roomMsg{d, err, chain}
		})
	case scrLog:
		return m.fetchLog()
	}
	return m.call(func(ctx context.Context) tea.Msg {
		st, err := m.c.status(ctx)
		return statusMsg{st, err, chain}
	})
}

// call führt einen Aufruf mit Zeitlimit als Befehl aus.
func (m model) call(f func(context.Context) tea.Msg) tea.Cmd {
	return func() tea.Msg {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		return f(ctx)
	}
}

// next plant den nächsten Abruf der aktuellen Ansicht; nach einem Fehler später.
func (m model) next() tea.Cmd {
	wait, chain := m.refresh, m.chain
	if m.err != "" {
		wait = m.retry
	}
	return tea.Tick(wait, func(time.Time) tea.Msg { return tickMsg{chain} })
}

// enter wechselt die Ansicht, beginnt einen neuen Takt und ruft sofort ab.
func (m model) enter(s screen) (model, tea.Cmd) {
	m.screen, m.chain, m.err, m.confirm = s, m.chain+1, "", false
	return m, m.poll()
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.width, m.height = msg.Width, msg.Height
	case tea.KeyPressMsg:
		return m.key(msg.String())
	case tickMsg:
		if msg.chain == m.chain {
			return m, m.poll()
		}
	case statusMsg:
		return m.onStatus(msg)
	case roomMsg:
		return m.onRoom(msg)
	case logMsg:
		return m.onLog(msg)
	case actionMsg:
		m.flash = msg.text
		if msg.err != nil {
			m.flash = msg.err.Error()
		}
		return m, m.poll()
	}
	return m, nil
}

func (m model) onStatus(msg statusMsg) (tea.Model, tea.Cmd) {
	if msg.err == nil {
		m.st = &msg.st
		m.sel = max(0, min(m.sel, len(msg.st.Rooms)-1))
	}
	if msg.chain != m.chain {
		return m, nil
	}
	m.err = errText(msg.err)
	return m, m.next()
}

func (m model) onRoom(msg roomMsg) (tea.Model, tea.Cmd) {
	if msg.chain != m.chain {
		return m, nil
	}
	m.err = errText(msg.err)
	if msg.err == nil {
		m.detail = &msg.d
		m.devSel = max(0, min(m.devSel, len(msg.d.Devices)-1))
	} else if msg.err.Error() == "Raum nicht gefunden" {
		m.flash = "Raum " + m.roomCode + " ist nicht mehr da"
		return m.enter(scrOverview)
	}
	return m, m.next()
}

func errText(err error) string {
	if err == nil {
		return ""
	}
	return err.Error()
}

func (m model) View() tea.View {
	var body string
	switch m.screen {
	case scrRoom:
		body = renderRoom(roomView{d: m.detail, code: m.roomCode, sel: m.devSel, confirm: m.confirm, err: m.err, width: m.width})
	case scrLog:
		body = renderLog(m.log, m.err, m.width, m.height)
	default:
		body = render(view{addr: m.c.base, st: m.st, err: m.err, width: m.width, sel: m.sel})
	}
	foot := []string{styleDim.Render(m.help())}
	if m.flash != "" {
		foot = append([]string{m.flash}, foot...)
	}
	v := tea.NewView(body + "\n\n" + strings.Join(foot, "\n"))
	v.AltScreen = true
	return v
}

// help ist die Tastenzeile der Ansicht.
func (m model) help() string {
	switch {
	case m.screen == scrRoom && m.confirm:
		return "y trennen · n abbrechen"
	case m.screen == scrRoom:
		return "↑↓ Gerät · d trennen · s Spielstand sichern · l Log · Esc zurück"
	case m.screen == scrLog:
		return "↑↓ blättern · f folgen · Esc zurück"
	}
	return "↑↓ Raum · Enter öffnen · l Log · q beenden"
}
