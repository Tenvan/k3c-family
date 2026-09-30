package mcpsrv

import (
	"context"
	"fmt"
	"strconv"
	"strings"
	"time"
)

// workbenchStatus ist das Tool workbench_status: kurz, eine Zeile je Punkt.
func (s *Server) workbenchStatus(context.Context, struct{}) (string, error) {
	st := s.Stats()
	lines := []string{
		fmt.Sprintf("k3c-dev · %s · läuft seit %s", s.URL(), formatUptime(s.stats.uptime())),
		fmt.Sprintf("Aufrufe %d · Fehler %d · Clients %d (max %d) · parallel %d (max %d)",
			st.TotalCalls, st.Errors, st.Clients, st.PeakClients, st.InFlight, st.PeakInFlight),
	}
	if s.cfg.Usage != nil {
		u := s.cfg.Usage.Snapshot().Session
		lines = append(lines, fmt.Sprintf("Sitzung: p95 %s · Ausreißer %d", formatMs(u.P95Ms), u.Outliers))
	}
	lines = append(lines, "Log-Quellen: "+strings.Join(s.logSources(), ", "))
	return strings.Join(append(lines, s.runLines()...), "\n"), nil
}

// runLines nennt den letzten Lauf je check_run-Ziel in Katalog-Reihenfolge.
func (s *Server) runLines() []string {
	var out []string
	for _, t := range checkTargets {
		r, ok := s.checks.lastRun(t.name)
		if !ok {
			continue
		}
		state := "exit " + strconv.Itoa(r.exit)
		if r.timedOut {
			state = "Zeitlimit"
		}
		out = append(out, fmt.Sprintf("Lauf %s · %s · %s · %s", t.name, state, formatMs(r.ms()), r.at.Format("15:04:05")))
	}
	if len(out) == 0 {
		return []string{"Läufe: noch keine"}
	}
	return out
}

// formatUptime schreibt eine Laufzeit grob: "< 1 min", "13 min", "2 h 13 min", "3 T 4 h".
func formatUptime(d time.Duration) string {
	minutes := int(d / time.Minute)
	switch {
	case minutes < 1:
		return "< 1 min"
	case minutes < 60:
		return fmt.Sprintf("%d min", minutes)
	case minutes < 24*60:
		return fmt.Sprintf("%d h %d min", minutes/60, minutes%60)
	default:
		return fmt.Sprintf("%d T %d h", minutes/(24*60), minutes%(24*60)/60)
	}
}

// formatMs schreibt eine Dauer deutsch: "468 ms", "12,4 s", "2 min 5 s", "1 h 2 min".
func formatMs(ms float64) string {
	switch {
	case ms < 1000:
		return fmt.Sprintf("%.0f ms", ms)
	case ms < 60_000:
		return strings.Replace(fmt.Sprintf("%.1f s", ms/1000), ".", ",", 1)
	case ms < 3_600_000:
		sec := int(ms / 1000)
		return fmt.Sprintf("%d min %d s", sec/60, sec%60)
	default:
		minutes := int(ms / 60_000)
		return fmt.Sprintf("%d h %d min", minutes/60, minutes%60)
	}
}
