// Command k3c-server ist der Server von K3C in Go: Auslieferung, Spielstände, Berichte und der Online-Modus (WebSocket).
//
// Konfiguration per Umgebung: K3C_HTTP_PORT (8080), K3C_HTTPS_PORT (8443, nur mit <certs>/key.pem und cert.pem),
// K3C_DIST (dist), K3C_SAVES_DIR (saves), K3C_REPORTS_DIR (reports), K3C_CERTS_DIR (certs), K3C_STATUS_TOKEN
// (schützt /api/status; leer = Diagnose aus), K3C_LOG_DIR (JSON-Log nach <Ordner>/k3c-server.jsonl; Standard: ein vorhandener
// Ordner logs/, sonst nur Text auf stderr), K3C_DEV (Dev-Mode: leer oder 1 = an, 0 = aus; Grad dev wählbar und Standard). `k3c-server -health` fragt /api/health des laufenden Servers ab
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
	"sync"
	"syscall"
	"time"

	k3cnet "k3c/engine/net"
	"k3c/engine/room"
	"k3c/engine/store"
)

// version setzt release.yml per -ldflags "-X main.version=<tag>".
var version = "dev"

// testSaveMaxAge: Test-Spielstände (Präfix test-) älter als das räumt der Server beim Start auf.
const testSaveMaxAge = 24 * time.Hour

// config sind die Einstellungen aus der Umgebung.
type config struct {
	httpPort, httpsPort string
	dist, saves         string
	reports, certs      string
	statusToken         string
	dev                 bool
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
		statusToken: os.Getenv("K3C_STATUS_TOKEN"), dev: os.Getenv("K3C_DEV") != "0",
	}
}

func main() {
	health := flag.Bool("health", false, "fragt /api/health des laufenden Servers ab (Exit 0 = gesund)")
	flag.Parse()
	if *health {
		os.Exit(checkHealth(loadConfig().httpPort))
	}
	log, closeLog := newLogger(logDir(), os.Stderr)
	err := run(loadConfig(), log)
	if err != nil {
		log.Error(err.Error())
	}
	closeLog()
	if err != nil {
		os.Exit(1)
	}
}

func run(cfg config, log *slog.Logger) error {
	if _, err := os.Stat(filepath.Join(cfg.dist, "index.html")); err != nil {
		return fmt.Errorf("%s/index.html fehlt. Erst bauen: task build", cfg.dist)
	}
	saves := &store.Saves{Dir: cfg.saves}
	rooms := room.NewManager(saves)
	rooms.Log = log
	rooms.Dev = cfg.dev
	// Testläufe (Spielstände mit Präfix test-, B-086) räumen sich beim Aufräumen des Raums auf; Reste alter Läufe hier.
	if n, err := saves.Purge(room.TestPrefix, time.Now().Add(-testSaveMaxAge)); err != nil {
		log.Error("Test-Spielstände nicht aufgeräumt", "err", err)
	} else if n > 0 {
		log.Info("Test-Spielstände aufgeräumt", "anzahl", n)
	}
	conns := &sync.WaitGroup{}
	handler := k3cnet.NewHandler(k3cnet.Config{Dist: cfg.dist, Log: log, Version: version, StartedAt: time.Now(),
		StatusToken: cfg.statusToken, LogDir: logDir(), Saves: saves, Reports: &store.Reports{Dir: cfg.reports}, Rooms: rooms,
		Conns: conns})
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
	waitConns(conns, 2*time.Second)
	return shutdown(servers)
}

// waitConns wartet kurz auf die WebSocket-Handler, damit room_closed noch ankommt. http.Server.Shutdown wartet nicht
// auf gekaperte Verbindungen; Geräte in der Raumliste bleiben bis zum Ende der Frist offen.
func waitConns(conns *sync.WaitGroup, limit time.Duration) {
	done := make(chan struct{})
	go func() { conns.Wait(); close(done) }()
	select {
	case <-done:
	case <-time.After(limit):
	}
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
