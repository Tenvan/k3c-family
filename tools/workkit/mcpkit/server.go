// Package mcpkit ist das gemeinsame Gerüst der MCP-Server von k3c-workbench und k3c-studio (B-046, WZ63): Streamable
// HTTP nur an 127.0.0.1, Tool-Registrierung mit Parameter-Hinweis (tool.go), Zähler und Aufruf-Log (stats.go),
// Checkout je Aufruf aus Header X-K3C-Root oder Argument checkout (workspace.go, checkout.go) und Fortschritt
// (progress.go). Was nur ein Werkzeug braucht (Dienste, Tasks, Testläufe, Ressourcen), hängt es über Options ein.
package mcpkit

import (
	"context"
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

	"k3c/tools/workkit/applog"
	"k3c/tools/workkit/usage"
)

const (
	// stopTimeout begrenzt das Warten auf offene Verbindungen (Streams der Clients) beim Stoppen.
	stopTimeout = 2 * time.Second
	// sessionTimeout schließt Sessions ohne Anfrage (Agent ohne Abmelden beendet); sonst zählten sie für immer als
	// Clients. Ein laufender Aufruf hält die Session offen, der Client meldet sich danach einfach neu an.
	sessionTimeout = 30 * time.Minute
)

// Options beschreibt einen Server. Port 0 wählt einen freien Port (Tests). Alle Funktionen sind optional.
type Options struct {
	Name         string // Name des Servers für die Clients, z. B. k3c-workbench
	Version      string
	Instructions string // Markdown, das jeder Client beim Verbinden bekommt
	Root         string // Repo-Wurzel
	Port         int
	EnvPort      string       // Umgebungsvariable des Ports, für die Meldung bei belegtem Port
	Log          *slog.Logger // eigenes Log; ohne verworfen
	OnStart      func(Call)   // Aufruf beginnt
	OnCall       func(Call)   // Aufruf beendet
	Usage        *usage.Tracker
	// TakesCheckout nennt die Tools mit dem Argument checkout im Schema (B-388).
	TakesCheckout func(tool string) bool
	// WriteTool nennt die schreibenden Tools; ihre Antwort endet mit der Zeile "Checkout: …" (B-275).
	WriteTool func(tool string) bool
	// Target liest das Ziel eines Aufrufs aus den rohen Argumenten ins Aufruf-Log (Call.Target).
	Target func(tool, args string) string
	// Track misst einen Aufruf (Name "tool:<name>"); die zurückgegebene Funktion beendet die Messung.
	Track func(name string) func()
	// Mount hängt weitere Handler an den HTTP-Server (z. B. pprof); /mcp ist belegt.
	Mount func(mux *http.ServeMux)
	// BeforeStop läuft in Stop vor dem Schließen des HTTP-Servers (z. B. laufende Läufe abbrechen).
	BeforeStop func()
}

// Server hält den MCP-Server und den HTTP-Server, der ihn ausliefert. Der HTTP-Teil lässt sich neu starten,
// ohne dass Katalog und Zähler verloren gehen.
type Server struct {
	opts   Options
	mcp    *mcp.Server
	stats  *Stats
	params map[string][]string // gültige Parameter je Tool, gefüllt bei der Registrierung
	log    *slog.Logger

	mu   sync.Mutex
	http *http.Server
	addr string
}

// ResolvePort liest den Port aus dem Wert der Umgebungsvariable env; leer heißt def.
func ResolvePort(env, raw string, def int) (int, error) {
	raw = strings.TrimSpace(raw)
	if raw == "" {
		return def, nil
	}
	port, err := strconv.Atoi(raw)
	if err != nil || port < 1 || port > 65535 {
		return 0, fmt.Errorf("%s=%q ist keine gültige Portzahl (1–65535)", env, raw)
	}
	return port, nil
}

// New baut den Server ohne Tools; Tools kommen mit Add und AddImage hinzu, gestartet wird er mit Start.
func New(o Options) *Server {
	s := &Server{opts: o, stats: NewStats(time.Now, o.OnStart, o.OnCall, o.Target), params: map[string][]string{}, log: o.Log}
	if s.log == nil {
		s.log = applog.Discard()
	}
	s.mcp = mcp.NewServer(&mcp.Implementation{Name: o.Name, Version: o.Version}, &mcp.ServerOptions{
		Instructions:       o.Instructions,
		InitializedHandler: func(context.Context, *mcp.InitializedRequest) { s.observeClients() },
	})
	s.mcp.AddReceivingMiddleware(s.observe)
	return s
}

// MCP ist der MCP-Server des SDK (Prompts, Sessions, In-Memory-Transport in Tests).
func (s *Server) MCP() *mcp.Server { return s.mcp }

// Root ist die Repo-Wurzel des Servers.
func (s *Server) Root() string { return s.opts.Root }

// Start öffnet den HTTP-Server an 127.0.0.1 unter /mcp.
func (s *Server) Start() error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.http != nil {
		return errors.New("server läuft bereits")
	}
	ln, err := net.Listen("tcp", net.JoinHostPort("127.0.0.1", strconv.Itoa(s.opts.Port)))
	if err != nil {
		return fmt.Errorf("port %d nicht verfügbar (%w); einen anderen Port über %s setzen", s.opts.Port, err, s.opts.EnvPort)
	}
	mux := http.NewServeMux()
	mux.Handle("/mcp", ForceChunked(mcp.NewStreamableHTTPHandler(func(*http.Request) *mcp.Server { return s.mcp },
		&mcp.StreamableHTTPOptions{SessionTimeout: sessionTimeout})))
	if s.opts.Mount != nil {
		s.opts.Mount(mux)
	}
	srv := &http.Server{Handler: mux, ReadHeaderTimeout: 10 * time.Second}
	s.http, s.addr = srv, ln.Addr().String()
	go func() { _ = srv.Serve(ln) }()
	return nil
}

// Stop schließt den HTTP-Server (offene Streams bekommen stopTimeout) und alle Sessions, damit sie nach einem
// Neustart nicht weiter als Clients zählen.
func (s *Server) Stop() error {
	if s.opts.BeforeStop != nil {
		s.opts.BeforeStop()
	}
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
	return s.stats.Snapshot()
}

// Calls liefert das Aufruf-Log, laufende Aufrufe eingeschlossen, neueste zuerst.
func (s *Server) Calls() []Call { return s.stats.Calls() }

// Uptime ist die Laufzeit seit New.
func (s *Server) Uptime() time.Duration { return s.stats.Uptime() }

// Clients zählt die offenen Sessions.
func (s *Server) Clients() int {
	n := 0
	for range s.mcp.Sessions() {
		n++
	}
	return n
}

func (s *Server) observeClients() { s.stats.ObserveClients(s.Clients()) }
