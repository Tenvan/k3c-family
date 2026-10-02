package main

import (
	"fmt"
	"strings"

	"charm.land/lipgloss/v2"
)

// Stile der Anzeige; -once druckt ohne Farbe (siehe render, plain).
var (
	styleHead  = lipgloss.NewStyle().Bold(true)
	styleDim   = lipgloss.NewStyle().Foreground(lipgloss.Color("8"))
	styleError = lipgloss.NewStyle().Foreground(lipgloss.Color("9")).Bold(true)
)

// view hält, was render braucht; plain schaltet Farben und Stile ab.
type view struct {
	addr  string
	st    *status
	err   string // Fehlermeldung des letzten Versuchs; sie steht über dem letzten bekannten Zustand
	width int
	sel   int  // gewählte Raumzeile (Übersicht)
	plain bool // -once: ohne Farbe und ohne Auswahlpfeil
}

func (v view) style(s lipgloss.Style, text string) string {
	if v.plain {
		return text
	}
	return s.Render(text)
}

// render ist die Übersicht als Text: Kopfzeile, Räume, Abstürze. Keine Zeile ist breiter als width (0 = unbegrenzt).
func render(v view) string {
	var lines []string
	if v.err != "" {
		lines = append(lines, v.style(styleError, v.err)+v.style(styleDim, " · neuer Versuch in 2 s"))
	}
	if v.st == nil {
		lines = append(lines, v.style(styleDim, "Warte auf "+v.addr+" …"))
		return clip(lines, v.width)
	}
	st := v.st
	lines = append(lines,
		v.style(styleHead, fmt.Sprintf("K3C %s", st.Version))+fmt.Sprintf(" · %s · läuft %s", v.addr, uptime(st.UptimeS)),
		fmt.Sprintf("Speicher %.1f MB Heap / %.1f MB System · %d Spielstände · %d Berichte", st.Memory.HeapMB, st.Memory.SysMB, st.Saves, st.Reports),
		"")
	lines = append(lines, roomTable(v, st.Rooms)...)
	lines = append(lines, failureLines(v, st.Failures)...)
	return clip(lines, v.width)
}

func roomTable(v view, rooms []room) []string {
	if len(rooms) == 0 {
		return []string{v.style(styleDim, "Keine Räume")}
	}
	lines := []string{v.style(styleHead, fmt.Sprintf("%s%-9s %-14s %5s %7s %-6s %8s %-18s %s", "  ", "Raum", "Name", "Tiefe", "Geräte", "Plätze", "Tick", "Tick-Dauer (p99)", "Spieler je Stufe"))}
	for i, r := range rooms {
		mark := "  "
		if !v.plain && i == v.sel {
			mark = "▶ "
		}
		lines = append(lines, fmt.Sprintf("%s%-9s %-14s %5d %7d %-6s %8d %-18s %s",
			mark, r.Code, truncate(r.Name, 14), r.Depth, r.Devices, seats(r.Monarchs), r.Tick,
			fmt.Sprintf("%.2f ms (%.2f)", r.TickMs.Last, r.TickMs.P99), playersPerStage(r.Stages)))
	}
	return lines
}

func failureLines(v view, fails []failure) []string {
	if len(fails) == 0 {
		return nil
	}
	lines := []string{"", v.style(styleError, "Abstürze")}
	for _, f := range fails {
		lines = append(lines, fmt.Sprintf("%s (%s) %s: %s", f.Code, f.Name, f.At, f.Error))
	}
	return lines
}

// playersPerStage schreibt die Spielerzahl je Stufe, durch / getrennt („2/0/1“); ohne Stufen „–“.
func playersPerStage(stages []stage) string {
	if len(stages) == 0 {
		return "–"
	}
	parts := make([]string, len(stages))
	for i, s := range stages {
		parts[i] = fmt.Sprint(len(s.Players))
	}
	return strings.Join(parts, "/")
}

// seats schreibt die Plätze eines Raums als ein Zeichen je Platz: T besetzt, W wartet, F frei.
func seats(monarchs []string) string {
	var b strings.Builder
	for _, m := range monarchs {
		switch m {
		case "taken":
			b.WriteByte('T')
		case "waiting":
			b.WriteByte('W')
		default:
			b.WriteByte('F')
		}
	}
	return b.String()
}

func uptime(seconds int64) string {
	switch {
	case seconds < 60:
		return "< 1 min"
	case seconds < 3600:
		return fmt.Sprintf("%d min", seconds/60)
	default:
		return fmt.Sprintf("%d h %d min", seconds/3600, seconds%3600/60)
	}
}

func truncate(s string, n int) string {
	r := []rune(s)
	if len(r) <= n {
		return s
	}
	return string(r[:n-1]) + "…"
}

// clip kürzt jede Zeile auf width sichtbare Zeichen (ANSI-Stile zählen nicht mit), damit nichts umbricht.
func clip(lines []string, width int) string {
	if width > 0 {
		for i, l := range lines {
			if lipgloss.Width(l) > width {
				lines[i] = lipgloss.NewStyle().MaxWidth(width).Render(l)
			}
		}
	}
	return strings.Join(lines, "\n")
}
