// Package mcpsrv ist der MCP-Server von k3c-workbench (B-046): ein Tool-Katalog (tools.go) auf dem gemeinsamen Gerüst
// workkit/mcpkit (HTTP an 127.0.0.1, Zähler und Aufruf-Log, Checkout je Aufruf); Dienste, Tasks, Prüfläufe und
// Ressourcen hängt die Workbench über mcpkit.Options ein.
package mcpsrv

import (
	"context"
	_ "embed"
	"log/slog"
	"net/http"
	"time"

	"k3c/tools/k3c-workbench/internal/resmon"
	"k3c/tools/k3c-workbench/internal/services"
	"k3c/tools/workkit/applog"
	"k3c/tools/workkit/console"
	"k3c/tools/workkit/mcpkit"
	"k3c/tools/workkit/usage"
)

// instructions bekommt jeder Client beim Verbinden: welches Tool wofür, statt Shell.
//
//go:embed instructions.md
var instructions string

// Instructions ist der Text, den jeder Client beim Verbinden bekommt (Markdown; die Oberfläche zeigt ihn an).
func Instructions() string { return instructions }

const (
	// DefaultPort ist der Vorgabe-Port, EnvPort überschreibt ihn.
	DefaultPort = 5180
	EnvPort     = "K3C_DEV_PORT"
)

// Config beschreibt einen Server. Port 0 wählt einen freien Port (Tests).
type Config struct {
	Root    string // Repo-Wurzel
	Port    int
	Version string
	OnStart func(Call)       // optional: Aufruf beginnt
	OnCall  func(Call)       // optional: Aufruf beendet
	OnCheck func(CheckState) // optional: ein Prüflauf beginnt oder endet (Quellenleiste der Oberfläche)
	Console *console.Store   // optional: gemeinsamer Konsolenpuffer (mit dem Spiegel des eigenen Logs)
	Log     *slog.Logger     // optional: eigenes Log
	Usage   *usage.Tracker   // optional: Nutzungsstatistik (B-062)
	// Resources misst den Ressourcenverbrauch (resmon): Tool workbench_resources, Messung je Aufruf, pprof unter /debug/pprof/.
	Resources *resmon.Monitor
	// Services führt die Dienste (B-067); ServicesErr ist der Grund, falls services.json nicht geladen wurde.
	Services    *services.Controller
	ServicesErr error
	Tasks       TaskHost     // optional: Tasks-Seite für task_* (Workbench-Spec § 4)
	OnNotify    func(Notice) // optional: Hinweis an den Nutzer in der Oberfläche (notify_ui)
}

// Server ist der MCP-Server der Workbench: das Gerüst mcpkit.Server (kit) mit Katalog, Zählern und HTTP-Teil, dazu
// die Workbench-eigenen Teile (Prüfläufe, Dienste der Worktrees). Testläufe (sim_test) führt seit WZ63.5 k3c-studio.
type Server struct {
	cfg             Config
	kit             *mcpkit.Server
	console         *console.Store
	log             *slog.Logger
	checks          *checkRuns
	run             func(context.Context, runSpec) runResult // Test-Naht für check_run
	taskVersion     func(context.Context) (string, error)    // Test-Naht: `task --version` (B-868, check_taskversion.go)
	taskVersionMemo taskVersionMemo                          // gemerktes Ergebnis von taskVersion, einmal je Workbench-Lauf (B-871)
	checkWait       time.Duration                            // wie lange check_run ohne progressToken wartet (check_wait.go)
	progressEvery   time.Duration                            // Abstand der Fortschritts-Meldungen von check_run
	wt              worktrees                                // Dienste der Worktrees (worktree_services.go)
	portBusy        func(int) bool                           // Test-Naht für die Vergabe der Worktree-Ports
}

// ResolvePort liest den Port aus dem Wert von EnvPort; leer heißt DefaultPort.
func ResolvePort(raw string) (int, error) { return mcpkit.ResolvePort(EnvPort, raw, DefaultPort) }

// New baut den Server mit allen Tools aus dem Katalog; gestartet wird er mit Start.
func New(cfg Config) *Server {
	s := &Server{cfg: cfg, console: cfg.Console, log: cfg.Log, checks: newCheckRuns(), run: runProcess, taskVersion: readTaskVersion,
		checkWait: defaultCheckWait, progressEvery: defaultProgressEvery,
		wt: worktrees{all: map[string]*worktreeServices{}}, portBusy: portBusy}
	if s.console == nil {
		s.console = console.New(console.DefaultCapacity, nil)
	}
	if s.log == nil {
		s.log = applog.Discard()
	}
	s.kit = mcpkit.New(s.kitOptions())
	register(s)
	s.registerGauges()
	return s
}

// kitOptions hängt die Workbench in das Gerüst: Checkout-Regeln, Ziel im Aufruf-Log, Messung, pprof.
func (s *Server) kitOptions() mcpkit.Options {
	o := mcpkit.Options{Name: "k3c-workbench", Version: s.cfg.Version, Instructions: instructions, Root: s.cfg.Root,
		Port: s.cfg.Port, EnvPort: EnvPort, Log: s.log, OnStart: s.cfg.OnStart, OnCall: s.cfg.OnCall, Usage: s.cfg.Usage,
		TakesCheckout: takesCheckout, WriteTool: func(tool string) bool { return writeTools[tool] }, Target: targetOf}
	if r := s.cfg.Resources; r != nil {
		o.Track = r.Track
		o.Mount = func(mux *http.ServeMux) { mountPprof(mux) }
	}
	return o
}

// Start öffnet den HTTP-Server an 127.0.0.1 unter /mcp.
func (s *Server) Start() error { return s.kit.Start() }

// Stop schließt HTTP-Server und Sessions.
func (s *Server) Stop() error { return s.kit.Stop() }

// Restart startet den HTTP-Teil neu; Clients verbinden sich danach neu.
func (s *Server) Restart() error { return s.kit.Restart() }

// URL ist die Adresse für .mcp.json, leer solange der Server nicht läuft.
func (s *Server) URL() string { return s.kit.URL() }

// Stats liefert die Zähler; die Zahl der Clients wird dabei frisch gezählt.
func (s *Server) Stats() Snapshot { return s.kit.Stats() }

// Calls liefert das Aufruf-Log, laufende Aufrufe eingeschlossen, neueste zuerst.
func (s *Server) Calls() []Call { return s.kit.Calls() }
