package mcpsrv

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"sync"
	"time"

	"k3c/tools/k3c-dev/internal/serverapi"
	"k3c/tools/k3c-dev/internal/services"
)

const (
	// slotStep ist der Port-Versatz je Worktree: Vite 5173 → 5183, 5193 …; Spielserver 8080 → 8090, 8100 …
	slotStep = 10
	maxSlots = 50
)

// worktreeServices sind die Dienste eines Worktrees: eigener Controller mit eigenen Ports, angelegt beim ersten
// Dienste- oder Server-Tool aus diesem Worktree. Die Repo-Wurzel behält ihren Controller (Config.Services).
type worktreeServices struct {
	ctl   *services.Controller
	off   int
	ports map[string]string // PortEnv → Port, z. B. K3C_HTTP_PORT → 8090
	stop  context.CancelFunc
}

type worktrees struct {
	mu  sync.Mutex
	all map[string]*worktreeServices // je Worktree-Wurzel
}

// controller liefert den Controller des Checkouts oder den Grund, warum es keinen gibt.
func (s *Server) controller(ctx context.Context) (*services.Controller, error) {
	ws := s.ws(ctx)
	if ws.name != "" {
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
	if w, ok := s.wt.all[ws.root]; ok {
		return w, nil
	}
	list, err := services.Load(filepath.Join(ws.root, "tools", "k3c-dev", "services.json"))
	if err != nil {
		return nil, fmt.Errorf("keine Dienste im Worktree %s: %w", ws.name, err)
	}
	off, err := s.freeOffset(list)
	if err != nil {
		return nil, err
	}
	shifted := services.Shift(list, off)
	ctx, cancel := context.WithCancel(context.Background())
	ctl := services.New(shifted, services.Options{Root: ws.root, Console: s.console, Prefix: ws.name + "/",
		Log: s.log.With("worktree", ws.name)})
	go ctl.Monitor(ctx, 2*time.Second)
	w := &worktreeServices{ctl: ctl, off: off, stop: cancel, ports: map[string]string{}}
	for _, svc := range shifted {
		if svc.PortEnv != "" {
			w.ports[svc.PortEnv] = svc.Env[svc.PortEnv]
		}
	}
	s.wt.all[ws.root] = w
	s.log.Info("🏰 dienste für worktree "+ws.name+" angelegt", "ns", "svc", "worktree", ws.name, "versatz", off)
	return w, nil
}

// freeOffset ist der kleinste Versatz, den kein anderer Worktree hat und an dessen Ports niemand lauscht.
// ponytail: Vergabe nur im Speicher; nach einem Neustart von k3c-dev kann ein Worktree einen anderen Versatz bekommen.
func (s *Server) freeOffset(list []services.Service) (int, error) {
	taken := map[int]bool{}
	for _, w := range s.wt.all {
		taken[w.off] = true
	}
	for k := 1; k <= maxSlots; k++ {
		off := k * slotStep
		if taken[off] {
			continue
		}
		free := true
		for _, svc := range list {
			if s.portBusy(svc.Port + off) {
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

// StopWorktreeServices stoppt die Dienste aller Worktrees (Ende von k3c-dev).
func (s *Server) StopWorktreeServices(ctx context.Context) {
	s.wt.mu.Lock()
	defer s.wt.mu.Unlock()
	for root, w := range s.wt.all {
		w.ctl.StopAll(ctx)
		w.stop()
		delete(s.wt.all, root)
	}
}

// serverClient liest Adresse und Token bei jedem Aufruf aus der Umgebung (wie der Server: K3C_STATUS_TOKEN,
// K3C_SERVER_URL, K3C_HTTP_PORT). Aus einem Worktree gilt immer dessen eigener Spielserver. Fehlt das Token in der
// Umgebung, gilt das aus der env des Dienstes Spielserver (B-362).
func (s *Server) serverClient(ctx context.Context) (*serverapi.Client, error) {
	ws := s.ws(ctx)
	if ws.name == "" {
		return serverapi.FromEnv(withServiceToken(s.cfg.Services, os.Getenv)), nil
	}
	w, err := s.worktreeServices(ws)
	if err != nil {
		return nil, err
	}
	return serverapi.FromEnv(withServiceToken(w.ctl, func(key string) string {
		if key == serverapi.EnvURL {
			return ""
		}
		if v, ok := w.ports[key]; ok {
			return v
		}
		return os.Getenv(key)
	})), nil
}

// withServiceToken ergänzt get um das Token aus der env des Dienstes Spielserver, wenn get keins liefert.
func withServiceToken(ctl *services.Controller, get func(string) string) func(string) string {
	return func(key string) string {
		v := get(key)
		if v != "" || key != serverapi.EnvToken || ctl == nil {
			return v
		}
		for _, svc := range ctl.Configs() {
			if svc.Name == "Spielserver" {
				return svc.Env[key]
			}
		}
		return ""
	}
}
