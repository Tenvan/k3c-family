package main

import (
	"context"
	"encoding/json"
	"fmt"
	"maps"
	"slices"
	"strings"
	"time"

	tea "charm.land/bubbletea/v2"
	"charm.land/lipgloss/v2"
)

// Grenzen der Log-Ansicht: höchstens 500 Zeilen im Speicher, beim Öffnen höchstens 20 Seiten à 500 Zeilen nachholen.
const (
	logKeep     = 500
	logPageSize = 500
	logCatchUp  = 20
)

// logState ist der Zustand der Log-Ansicht: rohe JSON-Zeilen, Cursor des Servers, Folgen und Blätterstand.
type logState struct {
	lines  []string
	cursor int64
	follow bool
	off    int // Zeilen oberhalb des Endes (0 = Ende)
	note   string
	pages  int
}

func newLogState() logState { return logState{follow: true} }

// apply übernimmt eine Seite des Servers. truncated heißt: die Datei wurde kleiner, das Log beginnt neu.
func (l *logState) apply(p logPage) {
	if p.Truncated {
		l.lines, l.note = nil, "Log neu begonnen"
	}
	l.lines = append(l.lines, p.Lines...)
	if len(l.lines) > logKeep {
		l.lines = l.lines[len(l.lines)-logKeep:]
	}
	l.cursor = p.Cursor
	l.pages++
}

// fetchLog holt die Seite ab dem aktuellen Cursor.
func (m model) fetchLog() tea.Cmd {
	chain, since := m.chain, m.log.cursor
	return m.call(func(ctx context.Context) tea.Msg {
		p, err := m.c.log(ctx, since, logPageSize)
		return logMsg{p, err, chain}
	})
}

// onLog übernimmt eine Seite. Beim Öffnen wird das Log von vorn nachgeholt (ohne Wartezeit), danach kommt jede Sekunde Neues.
func (m model) onLog(msg logMsg) (tea.Model, tea.Cmd) {
	if msg.chain != m.chain {
		return m, nil
	}
	m.err = errText(msg.err)
	if msg.err != nil {
		return m, m.next()
	}
	m.log.apply(msg.page)
	if len(msg.page.Lines) >= logPageSize && m.log.pages < logCatchUp {
		return m, m.fetchLog()
	}
	return m, m.next()
}

// renderLog zeigt die letzten Zeilen (oder, beim Blättern, ein Fenster darüber) in der Höhe des Terminals.
func renderLog(l logState, err string, width, height int) string {
	var lines []string
	if err != "" {
		lines = append(lines, styleError.Render(err)+styleDim.Render(" · neuer Versuch in 2 s"))
	}
	if l.note != "" {
		lines = append(lines, styleDim.Render(l.note))
	}
	rows := 20
	if height > 8 {
		rows = height - 8
	}
	end := len(l.lines) - l.off
	start := max(0, end-rows)
	for _, raw := range l.lines[start:max(start, end)] {
		lines = append(lines, formatLogLine(raw))
	}
	if len(l.lines) == 0 && err == "" {
		lines = append(lines, styleDim.Render("Noch keine Zeilen"))
	}
	state := "folgt"
	if !l.follow {
		state = fmt.Sprintf("angehalten (%d Zeilen oberhalb des Endes)", l.off)
	}
	lines = append([]string{styleHead.Render("Log des Servers") + styleDim.Render(" · "+state)}, lines...)
	return clip(lines, width)
}

// formatLogLine macht aus einer slog-JSON-Zeile „HH:MM:SS LEVEL ns Meldung k=v …“; was kein JSON ist, bleibt unverändert (grau).
func formatLogLine(raw string) string {
	var e map[string]any
	if json.Unmarshal([]byte(raw), &e) != nil {
		return styleDim.Render(raw)
	}
	clock, _ := e["time"].(string)
	if t, err := time.Parse(time.RFC3339Nano, clock); err == nil {
		clock = t.Format("15:04:05")
	}
	level, _ := e["level"].(string)
	msg, _ := e["msg"].(string)
	ns, _ := e["ns"].(string)
	var extra []string
	for _, k := range slices.Sorted(maps.Keys(e)) {
		if k != "time" && k != "level" && k != "msg" && k != "ns" {
			extra = append(extra, fmt.Sprintf("%s=%v", k, e[k]))
		}
	}
	parts := []string{styleDim.Render(clock), levelStyle(level).Render(fmt.Sprintf("%-5s", level))}
	if ns != "" {
		parts = append(parts, styleDim.Render(ns))
	}
	parts = append(parts, msg)
	if len(extra) > 0 {
		parts = append(parts, styleDim.Render(strings.Join(extra, " ")))
	}
	return strings.Join(parts, " ")
}

func levelStyle(level string) lipgloss.Style {
	switch level {
	case "ERROR":
		return styleError
	case "WARN":
		return lipgloss.NewStyle().Foreground(lipgloss.Color("3")).Bold(true)
	case "DEBUG":
		return styleDim
	}
	return lipgloss.NewStyle()
}
