package services

import (
	"context"
	"fmt"
)

// Configs sind die Dienste in Startreihenfolge (Reihenfolge von services.json).
func (c *Controller) Configs() []Service {
	units := c.list()
	out := make([]Service, len(units))
	for i, u := range units {
		out[i] = u.svc
	}
	return out
}

// Health prüft einen Dienst sofort mit seiner Health-Prüfung, ohne zu warten oder seinen Zustand zu ändern.
func (c *Controller) Health(ctx context.Context, name string) (Service, error) {
	u, err := c.unit(name)
	if err != nil {
		return Service{}, err
	}
	return u.svc, c.opts.Check(ctx, u.svc)
}

// PortOwner meldet, wer auf dem Port eines Dienstes lauscht: "frei", "PID n" oder "belegt (PID unbekannt)".
func (c *Controller) PortOwner(ctx context.Context, port int) string {
	pid, busy := c.opts.Listen(ctx, port)
	switch {
	case !busy:
		return "frei"
	case pid > 0:
		return fmt.Sprintf("PID %d", pid)
	default:
		return "belegt (PID unbekannt)"
	}
}
