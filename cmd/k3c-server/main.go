// Command k3c-server ist der Heimnetz-Server von K3C in Go (SP03, ersetzt server/*.mjs für Auslieferung, Spielstände
// und Berichte). Der Online-Modus (WebSocket) läuft bis SP08 weiter über den Node-Server.
//
// Konfiguration per Umgebung: K3C_HTTP_PORT (8080), K3C_HTTPS_PORT (8443, nur mit <certs>/key.pem und cert.pem),
// K3C_DIST (dist), K3C_SAVES_DIR (saves), K3C_REPORTS_DIR (reports), K3C_CERTS_DIR (certs), K3C_STATUS_TOKEN
// (schützt /api/status; leer = Diagnose aus). `k3c-server -health` fragt /api/health des laufenden Servers ab
// (Docker-HEALTHCHECK, das Image hat kein curl).
package main

import (
	"context"
	"errors"
	"flag"
	"fmt"
	"log/slog"
	stdnet "net"
	"net/http"
	"os"
	"os/signal"
	"path/filepath"
	"syscall"
	"time"

	k3cnet "k3c/engine/net"
	"k3c/engine/room"
	"k3c/engine/store"
)

// version setzt release.yml per -ldflags "-X main.version=<tag>".
var version = "dev"

// config sind die Einstellungen aus der Umgebung.
type config struct {
	httpPort, httpsPort string
	dist, saves         string
	reports, certs      string
	statusToken         string
}

func env(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}

func loadConfig() config {
	return config{
		httpPort: env("K3C_HTTP_PORT", "8080"), httpsPort: env("K3C_HTTPS_PORT", "8443"),
		dist: env("K3C_DIST", "dist"), saves: env("K3C_SAVES_DIR", "saves"),
		reports: env("K3C_REPORTS_DIR", "reports"), certs: env("K3C_CERTS_DIR", "certs"),
		statusToken: os.Getenv("K3C_STATUS_TOKEN"),
	}
}

func main() {
	health := flag.Bool("health", false, "fragt /api/health des laufenden Servers ab (Exit 0 = gesund)")
	flag.Parse()
	if *health {
		os.Exit(checkHealth(loadConfig().httpPort))
	}
	log := slog.New(slog.NewTextHandler(os.Stderr, nil))
	if err := run(loadConfig(), log); err != nil {
		log.Error(err.Error())
		os.Exit(1)
	}
}

func run(cfg config, log *slog.Logger) error {
	if _, err := os.Stat(filepath.Join(cfg.dist, "index.html")); err != nil {
		return fmt.Errorf("%s/index.html fehlt. Erst bauen: npm run build", cfg.dist)
	}
	saves := &store.Saves{Dir: cfg.saves}
	rooms := room.NewManager(saves)
	rooms.Log = log
	handler := k3cnet.NewHandler(k3cnet.Config{Dist: cfg.dist, Log: log, Version: version, StartedAt: time.Now(),
		StatusToken: cfg.statusToken, Saves: saves, Reports: &store.Reports{Dir: cfg.reports}, Rooms: rooms})
	if cfg.statusToken == "" {
		log.Info("diagnose aus: K3C_STATUS_TOKEN ist nicht gesetzt (/api/status antwortet 404)")
	}
	// SIGTERM schicken docker stop und compose down; ohne Handler würde PID 1 im Container es ignorieren.
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	go rooms.Run(ctx) // eine Goroutine je Raum, 30 Hz, Fristen einmal pro Sekunde
	servers := []*http.Server{newServer(":"+cfg.httpPort, handler)}
	errs := make(chan error, 2)
	go func() { errs <- servers[0].ListenAndServe() }()
	logAddresses(log, "http", cfg.httpPort)
	key, cert := filepath.Join(cfg.certs, "key.pem"), filepath.Join(cfg.certs, "cert.pem")
	if exists(key) && exists(cert) {
		tls := newServer(":"+cfg.httpsPort, handler)
		servers = append(servers, tls)
		go func() { errs <- tls.ListenAndServeTLS(cert, key) }()
		logAddresses(log, "https", cfg.httpsPort)
	} else {
		log.Info("kein HTTPS: " + key + " und " + cert + " fehlen (nur nötig, falls die Xbox HTTPS verlangt)")
	}
	select {
	case err := <-errs:
		return err
	case <-ctx.Done():
	}
	rooms.Close() // alle Räume speichern, room_closed an die Geräte, Verbindungen schließen
	return shutdown(servers)
}

func newServer(addr string, h http.Handler) *http.Server {
	return &http.Server{Addr: addr, Handler: h, ReadHeaderTimeout: 10 * time.Second}
}

func shutdown(servers []*http.Server) error {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	var errs []error
	for _, s := range servers {
		errs = append(errs, s.Shutdown(ctx))
	}
	return errors.Join(errs...)
}

// checkHealth ist der Schalter -health: 0, wenn /api/health mit 200 antwortet, sonst 1.
func checkHealth(port string) int {
	client := http.Client{Timeout: 3 * time.Second}
	res, err := client.Get("http://127.0.0.1:" + port + "/api/health")
	if err != nil {
		fmt.Fprintln(os.Stderr, "k3c-server:", err)
		return 1
	}
	_ = res.Body.Close()
	if res.StatusCode != http.StatusOK {
		return 1
	}
	return 0
}

func exists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

// logAddresses nennt localhost und die IPv4-Adressen im Heimnetz, wie server/server.mjs.
func logAddresses(log *slog.Logger, scheme, port string) {
	urls := []string{scheme + "://localhost:" + port + "/"}
	addrs, _ := stdnet.InterfaceAddrs()
	for _, a := range addrs {
		if ip, ok := a.(*stdnet.IPNet); ok && ip.IP.To4() != nil && !ip.IP.IsLoopback() {
			urls = append(urls, scheme+"://"+ip.IP.String()+":"+port+"/")
		}
	}
	log.Info("K3C läuft", "urls", urls)
}
