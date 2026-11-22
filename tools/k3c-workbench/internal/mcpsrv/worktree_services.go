package mcpsrv

import (
	"context"
	"fmt"
	"path/filepath"
	"sync"
	"time"

	"k3c/tools/k3c-workbench/internal/services"
)

const (
	// slotStep ist der Port-Versatz je Worktree: ein Dienst auf Port 7000 läuft im ersten Worktree auf 7010, im zweiten
	// auf 7020 … (die Dienste aus services.json; seit B-887 ist die Liste leer).
	slotStep = 10
	maxSlots = 50
)

// worktreeServices sind die Dienste eines Worktrees: eigener Controller mit eigenen Ports, angelegt beim ersten
// Dienste- oder Server-Tool aus diesem Worktree. Die Repo-Wurzel behält ihren Controller (Config.Services).
type worktreeServices struct {
	ctl   *services.Controller
	off   int
	ports map[string]string // PortEnv → Port, z. B. K3C_X_PORT → 7010
	stop  context.CancelFunc
}

type worktrees struct {
	mu  sync.Mutex
	all map[string]*worktreeServices // je Worktree-Wurzel
}

// controller liefert den Controller des Checkouts oder den Grund, warum es keinen gibt.
func (s *Server) controller(ctx context.Context) (*services.Controller, error) {
	ws := s.ws(ctx)
	if ws.Name != "" {
		w, err := s.worktreeServices(ws)
		if err != nil {
			return nil, err
		}
		return w.ctl, nil
	}
	if s.cfg.Services != nil {
		return s.cfg.Services, nil
	}
	if s.cfg.ServicesErr != nil {
		return nil, fmt.Errorf("keine Dienste: services.json nicht geladen: %w", s.cfg.ServicesErr)
	}
	return nil, fmt.Errorf("keine Dienste konfiguriert")
}

// worktreeServices legt die Dienste eines Worktrees beim ersten Bedarf an: services.json des Worktrees, Ports um
// den ersten freien Versatz verschoben, Konsolen-Quellen mit Präfix <worktree>/.
func (s *Server) worktreeServices(ws workspace) (*worktreeServices, error) {
	s.wt.mu.Lock()
	defer s.wt.mu.Unlock()
	if w, ok := s.wt.all[ws.Root]; ok {
		return w, nil
	}
	list, err := services.Load(filepath.Join(ws.Root, "tools", "k3c-workbench", "services.json"))
	if err != nil {
		return nil, fmt.Errorf("keine Dienste im Worktree %s: %w", ws.Name, err)
	}
	off, err := s.freeOffset(list)
	if err != nil {
		return nil, err
	}
	shifted := services.Shift(list, off)
	ctx, cancel := context.WithCancel(context.Background())
	ctl := services.New(shifted, services.Options{Root: ws.Root, Console: s.console, Prefix: ws.Name + "/",
		Log: s.log.With("worktree", ws.Name)})
	go ctl.Monitor(ctx, 2*time.Second)
	w := &worktreeServices{ctl: ctl, off: off, stop: cancel, ports: map[string]string{}}
	for _, svc := range shifted {
		if svc.PortEnv != "" {
			w.ports[svc.PortEnv] = svc.Env[svc.PortEnv]
		}
	}
	s.wt.all[ws.Root] = w
	s.log.Info("🏰 dienste für worktree "+ws.Name+" angelegt", "ns", "svc", "worktree", ws.Name, "versatz", off)
	return w, nil
}

// freeOffset ist der kleinste Versatz, dessen Ports weder die Repo-Wurzel noch ein anderer Worktree hat und an denen
// niemand lauscht. Gesperrt sind Ports, nicht Versätze: Worktrees auf älteren Branches haben eine andere Basis in ihrer
// services.json (früher Vite 5173 statt 5183, B-444) und dürfen den Port der Wurzel nicht bekommen, auch wenn sie gerade ruht.
// ponytail: Vergabe nur im Speicher; nach einem Neustart von k3c-workbench kann ein Worktree einen anderen Versatz bekommen.
func (s *Server) freeOffset(list []services.Service) (int, error) {
	taken := map[int]bool{}
	if s.cfg.Services != nil {
		for _, st := range s.cfg.Services.Statuses() {
			taken[st.Port] = true
		}
	}
	for _, w := range s.wt.all {
		for _, st := range w.ctl.Statuses() {
			taken[st.Port] = true
		}
	}
	for k := 1; k <= maxSlots; k++ {
		off := k * slotStep
		free := true
		for _, svc := range list {
			if taken[svc.Port+off] || s.portBusy(svc.Port+off) {
				free = false
				break
			}
		}
		if free {
			return off, nil
		}
	}
	return 0, fmt.Errorf("kein freier Port-Versatz für die Dienste (%d Versuche)", maxSlots)
}

func portBusy(port int) bool {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	_, busy := services.PortListener(ctx, port)
	return busy
}

// StopWorktreeServices stoppt die Dienste aller Worktrees (Ende von k3c-workbench).
func (s *Server) StopWorktreeServices(ctx context.Context) {
	s.wt.mu.Lock()
	defer s.wt.mu.Unlock()
	for root, w := range s.wt.all {
		w.ctl.StopAll(ctx)
		w.stop()
		delete(s.wt.all, root)
	}
}
