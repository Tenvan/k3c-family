package services

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"slices"
	"strings"
)

// Prozess-Dienste (B-1214) wie das Spiel: ohne Port, gesund, solange der Prozess läuft, nur von Hand gestartet; ein Ende
// ohne Fehler (Fenster geschlossen) ist „gestoppt“, kein Ausfall.

// errNotRunning ist die Health-Antwort eines Prozess-Dienstes, der nicht läuft.
var errNotRunning = errors.New("prozess läuft nicht")

// check ist die Health-Prüfung von u: bei health process genügt der laufende Prozess (die Überwachung sieht sein
// Ende über Wait), sonst die Prüfung aus Options.
func (c *Controller) check(ctx context.Context, svc Service) error {
	if svc.Health == healthProcess {
		return nil
	}
	return c.opts.Check(ctx, svc)
}

// processHealth ist svc_health eines Prozess-Dienstes: gesund, solange ein eigener Lauf besteht.
func processHealth(u *unit) error {
	u.mu.Lock()
	defer u.mu.Unlock()
	if u.run == nil {
		return errNotRunning
	}
	return nil
}

// missingNeeds nennt den Grund, wenn die Datei `needs` von u fehlt (mit dem Hinweis des Dienstes), sonst "".
func (c *Controller) missingNeeds(u *unit) string {
	if u.svc.Needs == "" {
		return ""
	}
	if _, err := os.Stat(filepath.Join(c.opts.Root, filepath.FromSlash(u.svc.Needs))); err == nil {
		return ""
	}
	reason := u.svc.Needs + " fehlt"
	if u.svc.NeedsHint != "" {
		reason += ": " + u.svc.NeedsHint
	}
	return reason
}

// ended behandelt das fehlerfreie Ende eines Prozess-Dienstes: gestoppt statt Ausfall, kein Neustart. Ein Befehl, der
// inzwischen gestoppt oder neu gestartet hat, gewinnt.
func (c *Controller) ended(u *unit, r *run) {
	u.cmd.Lock()
	defer u.cmd.Unlock()
	u.mu.Lock()
	current := u.run == r
	if current {
		u.run = nil
	}
	u.mu.Unlock()
	if !current {
		return
	}
	c.opts.Log.Info("🛑 dienst "+u.svc.Name+": von selbst beendet", "ns", "svc")
	c.set(u, func(s *Status) { s.State, s.PID, s.LastError, s.CPU, s.Memory = Stopped, 0, "", 0, 0 })
}

// StopForTask beendet vor dem Task `task` jeden laufenden eigenen Dienst, der ihn unter stopBefore nennt (die
// laufende Paket-EXE blockiert sonst das Kopieren beim Neubau), und liefert ihre Namen. Ein Hinweis steht im Log.
func (c *Controller) StopForTask(ctx context.Context, task string) []string {
	var stopped []string
	for _, u := range c.list() {
		if !slices.Contains(u.svc.StopBefore, task) || u.status().State != Running {
			continue
		}
		if _, err := c.Stop(ctx, u.svc.Name, false); err != nil {
			c.opts.Log.Warn("💥 dienst "+u.svc.Name+" vor Task "+task+" nicht beendet: "+err.Error(), "ns", "svc")
			continue
		}
		c.opts.Log.Info("🛑 dienst "+u.svc.Name+" für Task "+task+" beendet (laufende EXE blockiert sonst den Neubau)",
			"ns", "svc")
		stopped = append(stopped, u.svc.Name)
	}
	return stopped
}

// program ist das zu startende Programm: ein Pfad (mit /) liegt relativ zur Repo-Wurzel root, ein Name (task) kommt
// aus PATH.
func program(name, root string) string {
	if !strings.Contains(name, "/") || filepath.IsAbs(name) {
		return name
	}
	return filepath.Join(root, filepath.FromSlash(name))
}
