package mcpsrv

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/services"
)

// consoleOnError ist die Zahl der Konsolenzeilen, die eine gescheiterte Aktion mitliefert.
const consoleOnError = 10

type serviceIn struct {
	Service string `json:"service" jsonschema:"Name aus svc_status, z. B. Vite"`
}

type stopIn struct {
	Service string `json:"service" jsonschema:"Name aus svc_status, z. B. Vite"`
	Force   bool   `json:"force,omitempty" jsonschema:"nötig für übernommene Dienste (vor k3c-dev gestartet): beendet deren Prozessbaum"`
}

// svcStatus ist das Tool svc_status: eine Zeile je Dienst.
func (s *Server) svcStatus(ctx context.Context, _ struct{}) (string, error) {
	ctl, err := s.controller(ctx)
	if err != nil {
		return "", err
	}
	lines := make([]string, 0, len(ctl.Names()))
	for _, st := range ctl.Statuses() {
		lines = append(lines, s.serviceLine(ctl, st))
	}
	return strings.Join(lines, "\n"), nil
}

// svcStart ist das Tool svc_start; waitSeconds begrenzt das Warten auf Health.
func (s *Server) svcStart(ctx context.Context, in startIn) (string, error) {
	wctx, cancel, err := waitCtx(ctx, in.WaitSeconds)
	if err != nil {
		return "", err
	}
	defer cancel()
	return s.serviceAction(ctx, in.Service, func(ctl *services.Controller) (services.Status, error) {
		return ctl.Start(wctx, in.Service)
	})
}

// svcStop ist das Tool svc_stop.
func (s *Server) svcStop(ctx context.Context, in stopIn) (string, error) {
	return s.serviceAction(ctx, in.Service, func(ctl *services.Controller) (services.Status, error) {
		return ctl.Stop(ctx, in.Service, in.Force)
	})
}

// svcRestart ist das Tool svc_restart; ohne confirm=true lehnt es ab.
func (s *Server) svcRestart(ctx context.Context, in restartIn) (string, error) {
	if !in.Confirm {
		return "", errors.New("svc_restart braucht confirm=true (der Dienst ist während des Neustarts nicht erreichbar)")
	}
	wctx, cancel, err := waitCtx(ctx, in.WaitSeconds)
	if err != nil {
		return "", err
	}
	defer cancel()
	return s.serviceAction(ctx, in.Service, func(ctl *services.Controller) (services.Status, error) {
		return ctl.Restart(wctx, in.Service)
	})
}

// serviceAction führt einen Befehl aus und antwortet mit dem neuen Zustand und der Dauer; bei einem Fehler mit dem
// Grund und den letzten Konsolenzeilen des Dienstes.
func (s *Server) serviceAction(ctx context.Context, name string, act func(*services.Controller) (services.Status, error)) (string, error) {
	ctl, err := s.controller(ctx)
	if err != nil {
		return "", err
	}
	began := time.Now()
	st, err := act(ctl)
	if err != nil {
		return "", errors.New(err.Error() + s.consoleExcerpt(s.ws(ctx).source(name)))
	}
	return s.serviceLine(ctl, st) + " · " + formatMs(float64(time.Since(began).Milliseconds())), nil
}

func (s *Server) consoleExcerpt(name string) string {
	lines, ok := s.console.Tail(name, consoleOnError)
	if !ok || len(lines) == 0 {
		return ""
	}
	out := make([]string, len(lines))
	for i, l := range lines {
		out[i] = l.Text
	}
	return "\nletzte Zeilen:\n" + strings.Join(out, "\n")
}

// serviceLine ist ein Dienst in einer Zeile, z. B. "Vite · läuft · Port 5173 · PID 41232 · CPU 3,1 % · 480,0 MB ·
// 2 h 13 min · Neustarts 0".
func (s *Server) serviceLine(ctl *services.Controller, st services.Status) string {
	parts := []string{st.Name, string(st.State), fmt.Sprintf("Port %d", st.Port)}
	if st.State == services.Running || st.State == services.Adopted {
		parts = append(parts, fmt.Sprintf("PID %d", st.PID))
		if st.Memory > 0 { // die erste Messung kommt bis zu 2 s nach dem Start
			parts = append(parts, "CPU "+strings.Replace(fmt.Sprintf("%.1f %%", st.CPU), ".", ",", 1), formatBytes(int64(st.Memory)))
		}
		parts = append(parts, formatUptime(time.Since(st.StartedAt)))
	}
	if st.Restarts > 0 {
		parts = append(parts, fmt.Sprintf("Neustarts %d", st.Restarts))
	}
	if counts, ok, err := ctl.LogLevels(st.Name); ok && err == nil {
		parts = append(parts, "Log 60 min: "+levelLine(counts))
	}
	if st.LastError != "" {
		parts = append(parts, "letzter Fehler: "+st.LastError)
	}
	return strings.Join(parts, " · ")
}

// levelLine nennt WARN und ERROR immer, die übrigen Level nur, wenn es Einträge gibt.
func levelLine(c services.LevelCounts) string {
	parts := []string{}
	for _, level := range []string{"DEBUG", "INFO", "WARN", "ERROR"} {
		if n := c.Counts[level]; n > 0 || level == "WARN" || level == "ERROR" {
			parts = append(parts, fmt.Sprintf("%s %d", level, n))
		}
	}
	if c.BudgetHit {
		parts = append(parts, "(Budget erreicht)")
	}
	return strings.Join(parts, " ")
}

// serviceSummary ist die Zeile für workbench_status.
func (s *Server) serviceSummary(ctx context.Context) string {
	ctl, err := s.controller(ctx)
	if err != nil {
		return "Dienste: " + err.Error()
	}
	parts := make([]string, 0, len(ctl.Names()))
	for _, st := range ctl.Statuses() {
		parts = append(parts, fmt.Sprintf("%s %s (%d)", st.Name, st.State, st.Port))
	}
	return "Dienste: " + strings.Join(parts, " · ")
}
