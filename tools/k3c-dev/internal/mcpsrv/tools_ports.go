package mcpsrv

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/services"
)

// maxWaitSeconds begrenzt waitSeconds bei svc_start und svc_restart (wie das Start-Timeout der Dienste).
const maxWaitSeconds = 60

type startIn struct {
	Service     string `json:"service" jsonschema:"Name aus svc_status, z. B. Vite"`
	WaitSeconds int    `json:"waitSeconds,omitempty" jsonschema:"höchstens so lange auf Health warten (1–60, Standard 60)"`
}

type restartIn struct {
	Service     string `json:"service" jsonschema:"Name aus svc_status, z. B. Vite"`
	WaitSeconds int    `json:"waitSeconds,omitempty" jsonschema:"höchstens so lange auf Health warten (1–60, Standard 60)"`
	Confirm     bool   `json:"confirm,omitempty" jsonschema:"muss true sein: der Dienst ist während des Neustarts weg"`
}

type urlsIn struct {
	Target string `json:"target,omitempty" jsonschema:"Dienst aus svc_status oder mcp; leer: alle"`
}

// waitCtx begrenzt das Warten auf Health; 0 heißt Vorgabe des Controllers.
func waitCtx(ctx context.Context, seconds int) (context.Context, context.CancelFunc, error) {
	if seconds == 0 {
		return ctx, func() {}, nil
	}
	if seconds < 1 || seconds > maxWaitSeconds {
		return nil, nil, fmt.Errorf("waitSeconds %d ungültig: erlaubt 1–%d", seconds, maxWaitSeconds)
	}
	c, cancel := context.WithTimeout(ctx, time.Duration(seconds)*time.Second)
	return c, cancel, nil
}

// svcHealth ist das Tool svc_health: prüft sofort, ohne den Zustand zu ändern.
func (s *Server) svcHealth(ctx context.Context, in serviceIn) (string, error) {
	ctl, err := s.controller(ctx)
	if err != nil {
		return "", err
	}
	svc, err := ctl.Health(ctx, in.Service)
	if svc.Name == "" {
		return "", err
	}
	if err != nil {
		return fmt.Sprintf("%s · ungesund · %s · %v · Port: %s", svc.Name, svc.HealthURL(), err, ctl.PortOwner(ctx, svc.Port)), nil
	}
	return fmt.Sprintf("%s · gesund · %s", svc.Name, svc.HealthURL()), nil
}

// svcStartAll ist das Tool svc_start_all: alle Dienste, die nicht laufen, parallel bis gesund.
func (s *Server) svcStartAll(ctx context.Context, _ struct{}) (string, error) {
	ctl, err := s.controller(ctx)
	if err != nil {
		return "", err
	}
	startErr := ctl.StartAll(ctx)
	text, _ := s.svcStatus(ctx, struct{}{})
	if startErr != nil {
		return "", errors.New(startErr.Error() + "\n" + text)
	}
	return text, nil
}

// svcStopAll ist das Tool svc_stop_all: eigene Dienste rückwärts stoppen, übernommene bleiben.
func (s *Server) svcStopAll(ctx context.Context, _ struct{}) (string, error) {
	ctl, err := s.controller(ctx)
	if err != nil {
		return "", err
	}
	ctl.StopAll(ctx)
	return s.svcStatus(ctx, struct{}{})
}

// portsStatus ist das Tool ports_status: Dev-Ports der Dienste und von k3c-dev, auch fremd belegte.
func (s *Server) portsStatus(ctx context.Context, _ struct{}) (string, error) {
	ctl, err := s.controller(ctx)
	if err != nil {
		return "", err
	}
	lines := []string{}
	for _, svc := range ctl.Configs() {
		lines = append(lines, fmt.Sprintf("%d · %s · %s", svc.Port, svc.Name, ctl.PortOwner(ctx, svc.Port)))
	}
	if url := s.URL(); url != "" {
		lines = append(lines, strings.TrimSuffix(strings.TrimPrefix(url, "http://"), "/mcp")+" · MCP k3c-dev · eigener Prozess")
	}
	return strings.Join(lines, "\n"), nil
}

// getURLs ist das Tool get_urls: Adressen je Dienst für HTTP-Aufrufe und Browserprüfungen.
func (s *Server) getURLs(ctx context.Context, in urlsIn) (string, error) {
	ctl, err := s.controller(ctx)
	if err != nil {
		return "", err
	}
	lines := []string{}
	for _, svc := range ctl.Configs() {
		if in.Target == "" || strings.EqualFold(in.Target, svc.Name) {
			lines = append(lines, serviceURLs(svc))
		}
	}
	if url := s.URL(); url != "" && (in.Target == "" || strings.EqualFold(in.Target, "mcp")) {
		lines = append(lines, "MCP · "+url)
	}
	if len(lines) == 0 {
		return "", fmt.Errorf("unbekanntes Ziel %q (gültig: %s, mcp)", in.Target, strings.Join(ctl.Names(), ", "))
	}
	return strings.Join(lines, "\n"), nil
}

func serviceURLs(svc services.Service) string {
	return fmt.Sprintf("%s · http://localhost:%d/ · Health %s", svc.Name, svc.Port, svc.HealthURL())
}
