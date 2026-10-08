// Package mcpsrv ist der MCP-Server von k3c-dev (B-046): Streamable HTTP nur an 127.0.0.1, ein Tool-Katalog
// (tools.go), Zähler und Aufruf-Log für die Oberfläche (stats.go).
package mcpsrv

import (
	"context"
	_ "embed"
	"errors"
	"fmt"
	"log/slog"
	"net"
	"net/http"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-dev/internal/applog"
	"k3c/tools/k3c-dev/internal/botfeed"
	"k3c/tools/k3c-dev/internal/browser"
	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/services"
	"k3c/tools/k3c-dev/internal/usage"
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
	// stopTimeout begrenzt das Warten auf offene Verbindungen (Streams der Clients) beim Stoppen.
	stopTimeout = 2 * time.Second
	// sessionTimeout schließt Sessions ohne Anfrage (Agent ohne Abmelden beendet); sonst zählten sie für immer als
	// Clients. Ein laufender Aufruf hält die Session offen, der Client meldet sich danach einfach neu an.
	sessionTimeout = 30 * time.Minute
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
	// Services führt die Dienste (B-067); ServicesErr ist der Grund, falls services.json nicht geladen wurde.
	Services    *services.Controller
	ServicesErr error
	Tasks       TaskHost // optional: Tasks-Seite für task_* (Workbench-Spec § 4)
}

// Server hält den MCP-Server und den HTTP-Server, der ihn ausliefert. Der HTTP-Teil lässt sich neu starten,
// ohne dass Katalog und Zähler verloren gehen.
type Server struct {
	cfg     Config
	mcp     *mcp.Server
	stats   *stats
	params  map[string][]string // gültige Parameter je Tool, gefüllt bei der Registrierung
	console *console.Store
	log     *slog.Logger
	checks  *checkRuns
	sims    simRuns      // Testläufe von sim_test (simtest.go)
	feeds   *botfeed.Hub // Bot-Feeds der Läufe mit Clients (simtest_clients.go), Route /bot/
	// findBrowser und launch starten die Clients von sim_test; Test-Naht.
	findBrowser func() (string, error)
	launch      func(ctx context.Context, exe, profile, url string) (stop func(), err error)
	run         func(context.Context, runSpec) runResult // Test-Naht für check_run
	wt          worktrees                                // Dienste der Worktrees (worktree_services.go)
	portBusy    func(int) bool                           // Test-Naht für die Vergabe der Worktree-Ports

	mu   sync.Mutex
	http *http.Server
	addr string
}

// ResolvePort liest den Port aus dem Wert von EnvPort; leer heißt DefaultPort.
func ResolvePort(raw string) (int, error) {
	raw = strings.TrimSpace(raw)
	if raw == "" {
		return DefaultPort, nil
	}
	port, err := strconv.Atoi(raw)
	if err != nil || port < 1 || port > 65535 {
		return 0, fmt.Errorf("%s=%q ist keine gültige Portzahl (1–65535)", EnvPort, raw)
	}
	return port, nil
}

// New baut den Server mit allen Tools aus dem Katalog; gestartet wird er mit Start.
func New(cfg Config) *Server {
	s := &Server{cfg: cfg, stats: newStats(time.Now), params: map[string][]string{},
		console: cfg.Console, log: cfg.Log, checks: newCheckRuns(), run: runProcess,
		wt: worktrees{all: map[string]*worktreeServices{}}, portBusy: portBusy,
		feeds: botfeed.NewHub(), findBrowser: browser.Find, launch: launchBrowser}
	if s.console == nil {
		s.console = console.New(console.DefaultCapacity, nil)
	}
	if s.log == nil {
		s.log = applog.Discard()
	}
	s.stats.onStart, s.stats.onCall = cfg.OnStart, cfg.OnCall
	s.mcp = mcp.NewServer(&mcp.Implementation{Name: "k3c-dev", Version: cfg.Version}, &mcp.ServerOptions{
		Instructions:       instructions,
		InitializedHandler: func(context.Context, *mcp.InitializedRequest) { s.observeClients() },
	})
	register(s)
	s.mcp.AddReceivingMiddleware(s.observe)
	return s
}

// Start öffnet den HTTP-Server an 127.0.0.1 unter /mcp.
func (s *Server) Start() error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.http != nil {
		return errors.New("server läuft bereits")
	}
	ln, err := net.Listen("tcp", net.JoinHostPort("127.0.0.1", strconv.Itoa(s.cfg.Port)))
	if err != nil {
		return fmt.Errorf("port %d nicht verfügbar (%w); einen anderen Port über %s setzen", s.cfg.Port, err, EnvPort)
	}
	mux := http.NewServeMux()
	mux.Handle("/mcp", forceChunked(mcp.NewStreamableHTTPHandler(func(*http.Request) *mcp.Server { return s.mcp },
		&mcp.StreamableHTTPOptions{SessionTimeout: sessionTimeout})))
	mux.Handle(botfeed.Prefix, s.feeds)
	srv := &http.Server{Handler: mux, ReadHeaderTimeout: 10 * time.Second}
	s.http, s.addr = srv, ln.Addr().String()
	go func() { _ = srv.Serve(ln) }()
	return nil
}

// Stop schließt den HTTP-Server (offene Streams bekommen stopTimeout) und alle Sessions, damit sie nach einem
// Neustart nicht weiter als Clients zählen.
func (s *Server) Stop() error {
	s.sims.cancelAll()
	s.mu.Lock()
	srv := s.http
	s.http = nil
	s.mu.Unlock()
	if srv == nil {
		return nil
	}
	ctx, cancel := context.WithTimeout(context.Background(), stopTimeout)
	defer cancel()
	err := srv.Shutdown(ctx)
	if err != nil {
		err = srv.Close()
	}
	for ss := range s.mcp.Sessions() {
		_ = ss.Close()
	}
	return err
}

// Restart startet den HTTP-Teil neu; Clients verbinden sich danach neu.
func (s *Server) Restart() error {
	if err := s.Stop(); err != nil {
		return err
	}
	if err := s.Start(); err != nil {
		s.log.Error("💥 neustart fehlgeschlagen", "ns", "mcp", "error", err.Error())
		return err
	}
	s.log.Info("🔁 server neu gestartet", "ns", "mcp", "url", s.URL())
	return nil
}

// URL ist die Adresse für .mcp.json, leer solange der Server nicht läuft.
func (s *Server) URL() string {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.http == nil {
		return ""
	}
	return "http://" + s.addr + "/mcp"
}

// Stats liefert die Zähler; die Zahl der Clients wird dabei frisch gezählt.
func (s *Server) Stats() Snapshot {
	s.observeClients()
	return s.stats.snapshot()
}

// Calls liefert das Aufruf-Log, laufende Aufrufe eingeschlossen, neueste zuerst.
func (s *Server) Calls() []Call { return s.stats.calls() }

func (s *Server) observeClients() {
	n := 0
	for range s.mcp.Sessions() {
		n++
	}
	s.stats.observeClients(n)
}
