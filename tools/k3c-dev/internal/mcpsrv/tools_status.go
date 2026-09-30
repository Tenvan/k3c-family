package mcpsrv

import (
	"context"
	"fmt"
	"time"
)

// workbenchStatus ist das Tool workbench_status: kurz, eine Zeile je Punkt.
func (s *Server) workbenchStatus(context.Context, struct{}) (string, error) {
	st := s.Stats()
	return fmt.Sprintf("k3c-dev · %s · läuft seit %s\nAufrufe %d · Fehler %d · Clients %d (max %d) · parallel %d (max %d)",
		s.URL(), formatUptime(s.stats.uptime()), st.TotalCalls, st.Errors,
		st.Clients, st.PeakClients, st.InFlight, st.PeakInFlight), nil
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
