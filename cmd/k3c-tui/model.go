package main

import (
	"context"
	"time"

	tea "charm.land/bubbletea/v2"
)

// Takt der Anzeige: jede Sekunde neu laden, nach einem Fehler alle 2 Sekunden erneut versuchen.
const (
	refreshEvery = time.Second
	retryEvery   = 2 * time.Second
)

type (
	// statusMsg ist die Antwort auf einen Abruf; err gesetzt heißt: der Abruf ist gescheitert.
	statusMsg struct {
		st  status
		err error
	}
	// tickMsg löst den nächsten Abruf aus.
	tickMsg struct{}
)

// model ist der Zustand der Übersicht (Bubble Tea, Elm-Architektur).
type model struct {
	c       *client
	st      *status
	err     string
	width   int
	refresh time.Duration
	retry   time.Duration
}

func newModel(c *client) model {
	return model{c: c, refresh: refreshEvery, retry: retryEvery}
}

// fetch ruft den Status ab und meldet das Ergebnis als statusMsg.
func (m model) fetch() tea.Cmd {
	return func() tea.Msg {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		st, err := m.c.status(ctx)
		return statusMsg{st, err}
	}
}

func (m model) Init() tea.Cmd { return m.fetch() }

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.width = msg.Width
	case tea.KeyPressMsg:
		if k := msg.String(); k == "q" || k == "ctrl+c" {
			return m, tea.Quit
		}
	case tickMsg:
		return m, m.fetch()
	case statusMsg:
		wait := m.refresh
		if msg.err != nil {
			m.err = msg.err.Error()
			wait = m.retry
		} else {
			m.st, m.err = &msg.st, ""
		}
		return m, tea.Tick(wait, func(time.Time) tea.Msg { return tickMsg{} })
	}
	return m, nil
}

func (m model) View() tea.View {
	text := render(view{addr: m.c.base, st: m.st, err: m.err, width: m.width}) + "\n\n" + styleDim.Render("q beenden")
	v := tea.NewView(text)
	v.AltScreen = true
	return v
}
