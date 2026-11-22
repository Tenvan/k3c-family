// Package mcpsrv ist der MCP-Server von k3c-studio (WZ63): die Tools des Weltdesigns (world_*, assets_*, hub_*,
// level_generate) auf dem gemeinsamen Gerüst workkit/mcpkit (HTTP an 127.0.0.1, Zähler und Aufruf-Log, Checkout je
// Aufruf). Dienste, Tasks, Prüf- und Testläufe bleiben in k3c-workbench.
package mcpsrv

import (
	_ "embed"
	"log/slog"

	"k3c/tools/workkit/applog"
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
	// Name ist der Name des Servers für die Clients (.mcp.json, Tool-Präfix mcp__k3c-studio__).
	Name = "k3c-studio"
	// DefaultPort ist der Vorgabe-Port (nächster freier nach der Workbench, 5180), EnvPort überschreibt ihn.
	DefaultPort = 5181
	EnvPort     = "K3C_STUDIO_PORT"
)

// Config beschreibt einen Server. Port 0 wählt einen freien Port (Tests).
type Config struct {
	Root    string // Repo-Wurzel
	Port    int
	Version string
	OnStart func(Call)     // optional: Aufruf beginnt
	OnCall  func(Call)     // optional: Aufruf beendet
	Log     *slog.Logger   // optional: eigenes Log
	Usage   *usage.Tracker // optional: Nutzungsstatistik
}

// Server ist der MCP-Server des Studios: das Gerüst mcpkit.Server (kit) mit dem Katalog aus tools.go.
type Server struct {
	cfg Config
	kit *mcpkit.Server
	log *slog.Logger
}

// ResolvePort liest den Port aus dem Wert von EnvPort; leer heißt DefaultPort.
func ResolvePort(raw string) (int, error) { return mcpkit.ResolvePort(EnvPort, raw, DefaultPort) }

// New baut den Server mit allen Tools aus dem Katalog; gestartet wird er mit Start.
func New(cfg Config) *Server {
	s := &Server{cfg: cfg, log: cfg.Log}
	if s.log == nil {
		s.log = applog.Discard()
	}
	s.kit = mcpkit.New(mcpkit.Options{Name: Name, Version: cfg.Version, Instructions: instructions, Root: cfg.Root,
		Port: cfg.Port, EnvPort: EnvPort, Log: s.log, OnStart: cfg.OnStart, OnCall: cfg.OnCall, Usage: cfg.Usage,
		TakesCheckout: takesCheckout, WriteTool: func(tool string) bool { return writeTools[tool] }})
	register(s)
	return s
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
