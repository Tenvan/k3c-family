// Command k3c-server ist der Server von K3C in Go: Auslieferung, Spielstände, Berichte und der Online-Modus (WebSocket).
//
// Konfiguration per Umgebung: K3C_HTTP_PORT (8080), K3C_HTTPS_PORT (8443, nur mit <certs>/key.pem und cert.pem),
// K3C_DIST (dist), K3C_DATA_DIR (data, Daten von der Platte; fehlt der Ordner: eingebettet), K3C_PUBLIC_DIR (public, Grafiken und Sound,
// nur beobachtet), K3C_SAVES_DIR (saves), K3C_WORLDS_DIR (data/worlds, Welt-Dateien, nur im Dev-Mode gelesen), K3C_REPORTS_DIR (reports), K3C_CERTS_DIR (certs), K3C_STATUS_TOKEN
// (schützt /api/status; leer = Diagnose aus), K3C_LOG_LEVEL (Konsole: debug, info, warn, error; Standard info; die JSON-Dateien schreiben immer ab debug),
// K3C_LOG_DIR (JSON-Log nach <Ordner>/k3c-server.jsonl, Browser-Meldungen nach k3c-client.jsonl; Standard: ein vorhandener
// Ordner logs/, sonst nur Text auf stderr), K3C_DEV (Dev-Mode: leer oder 1 = an, 0 = aus; Grad dev wählbar und Standard). `k3c-server -health` fragt /api/health des laufenden Servers ab
// (Docker-HEALTHCHECK, das Image hat kein curl). Der Prozess startet sich selbst als Kind und startet es neu, sobald
// sich beobachtete Dateien ändern (supervise.go, restart.go, B-507).
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
	"runtime"
	"sync"
	"syscall"
	"time"

	"k3c/data"
	k3cnet "k3c/engine/net"
	"k3c/engine/room"
	"k3c/engine/store"
)

// version setzt release.yml per -ldflags "-X main.version=<tag>".
var version = "dev"

// built setzt das Dockerfile per -ldflags "-X main.built=<UTC-Zeit>"; leer = Änderungszeit der EXE (buildTime).
var built = ""

// buildTime ist die Buildzeit für /api/health (RFC 3339, UTC); ohne ldflags die Änderungszeit der EXE, sonst leer.
func buildTime() string {
	if built != "" {
		return built
	}
	exe, err := os.Executable()
	if err != nil {
		return ""
	}
	info, err := os.Stat(exe)
	if err != nil {
		return ""
	}
	return info.ModTime().UTC().Format(time.RFC3339)
}

// testSaveMaxAge: Test-Spielstände (Präfix test-) älter als das räumt der Server beim Start auf.
const testSaveMaxAge = 24 * time.Hour

// config sind die Einstellungen aus der Umgebung.
type config struct {
	httpPort, httpsPort string
	dist, saves, worlds string
	data, public        string
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
		dist: env("K3C_DIST", "dist"), saves: env("K3C_SAVES_DIR", "saves"), worlds: env("K3C_WORLDS_DIR", "data/worlds"),
		data: env(data.DirEnv, "data"), public: env("K3C_PUBLIC_DIR", "public"),
		reports: env("K3C_REPORTS_DIR", "reports"), certs: env("K3C_CERTS_DIR", "certs"),
		statusToken: os.Getenv("K3C_STATUS_TOKEN"), dev: os.Getenv("K3C_DEV") != "0",
	}
}

func main() {
	health := flag.Bool("health", false, "fragt /api/health des laufenden Servers ab (Exit 0 = gesund)")
	check := flag.Bool("check-data", false, "prüft Daten (K3C_DATA_DIR) und Welten wie beim Start (Exit 0 = gültig)")
	flag.Parse()
	switch {
	case *health:
		os.Exit(checkHealth(loadConfig().httpPort))
	case *check:
		os.Exit(checkData(loadConfig(), os.Stderr))
	case os.Getenv(childEnv) == "": // Supervisor
		dir, log := dataDir(), consoleLogger()
		sigs := make(chan os.Signal, 2) // für die ganze Laufzeit: kein Signal beendet den Supervisor vor dem Kind
		signal.Notify(sigs, os.Interrupt, syscall.SIGTERM)
		os.Exit(supervise(startChild(dir, log), sigs, clock{time.Now, time.After}, dir == "", log))
	}
	log, closeLog := newLogger(logDir(), os.Stderr)
	err := run(loadConfig(), log)
	code := 0
	switch {
	case errors.Is(err, errRestart):
		code = restartCode
	case err != nil:
		log.Error("💥 " + err.Error())
		code = 1
	}
	closeLog()
	os.Exit(code)
}

func run(cfg config, log *slog.Logger) error {
	if _, err := os.Stat(filepath.Join(cfg.dist, "index.html")); err != nil {
		return fmt.Errorf("%s/index.html fehlt. Erst bauen: task build", cfg.dist)
	}
	log.Info("🚀 Server startet", "ns", "main", "version", version, "pid", os.Getpid(), "go", runtime.Version(), "http", cfg.httpPort,
		"dist", cfg.dist, "daten", dataSource(), "saves", cfg.saves, "worlds", cfg.worlds, "reports", cfg.reports, "dev", cfg.dev, "logDir", logDir(), "konsolenLevel", consoleLevel().String())
	saves := &store.Saves{Dir: cfg.saves}
	reports := &store.Reports{Dir: cfg.reports}
	rooms := newRooms(cfg, log, saves, reports)
	conns := &sync.WaitGroup{}
	clientLog, closeClientLog := newNamedLogger(logDir(), clientLogFile, os.Stderr)
	defer closeClientLog()
	notices := &k3cnet.Notices{}
	handler := k3cnet.NewHandler(k3cnet.Config{Dist: cfg.dist, Log: log, ClientLog: clientLog, Version: version, Built: buildTime(), StartedAt: time.Now(),
		StatusToken: cfg.statusToken, LogDir: logDir(), Saves: saves, Reports: reports, Rooms: rooms,
		Conns: conns, Notices: notices})
	if cfg.statusToken == "" {
		log.Info("🔒 diagnose aus: K3C_STATUS_TOKEN ist nicht gesetzt (/api/status antwortet 404)")
	}
	// SIGTERM schicken docker stop und compose down; ohne Handler würde PID 1 im Container es ignorieren.
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	go rooms.Run(ctx) // eine Goroutine je Raum, 30 Hz, Fristen einmal pro Sekunde
	restart := make(chan struct{}, 1)
	startWatch(ctx, watchRoots(cfg, []string{cfg.saves, cfg.reports, logDir()}, log), restarter{log: log, check: checkChild(cfg),
		announce: notices.Restarting, wait: time.Sleep, restart: func() {
			select {
			case restart <- struct{}{}:
			default:
			}
		}})
	servers, errs := listen(cfg, handler, log)
	var reason error
	select {
	case err := <-errs:
		return err
	case <-ctx.Done():
		log.Info("🛑 Server fährt herunter (Signal)", "ns", "main", "raeume", len(rooms.Rooms()))
	case <-restart:
		log.Info("🔁 Server fährt für den Neustart herunter", "ns", "watch", "raeume", len(rooms.Rooms()))
		reason = errRestart
	}
	rooms.Close() // alle Räume speichern, room_closed an die Geräte, Verbindungen schließen
	waitConns(conns, 2*time.Second)
	err := shutdown(servers)
	if reason != nil && err != nil {
		log.Warn("⏳ Herunterfahren vor dem Neustart unvollständig", "ns", "watch", "err", err)
	}
	return errors.Join(err, reason)
}

// dataSource nennt die Herkunft der Daten fürs Log: Ordner von der Platte oder „eingebettet“.
func dataSource() string {
	if dir := os.Getenv(data.DirEnv); dir != "" {
		return dir
	}
	return "eingebettet"
}

// newRooms baut die Räume und räumt Reste alter Testläufe auf.
func newRooms(cfg config, log *slog.Logger, saves *store.Saves, reports *store.Reports) *room.Manager {
	rooms := room.NewManager(saves)
	rooms.Log = log
	rooms.Sessions = reports // Spielmetrik-Report je Raumlauf (B-150)
	rooms.Dev = cfg.dev
	if cfg.dev { // weitere Welten nur im Dev-Mode, frisch von der Platte (docs/rules/welten.md § 6)
		rooms.Worlds = os.DirFS(cfg.worlds)
	}
	// Testläufe (Spielstände mit Präfix test-, B-086) räumen sich beim Aufräumen des Raums auf; Reste alter Läufe hier.
	if n, err := saves.Purge(room.TestPrefix, time.Now().Add(-testSaveMaxAge)); err != nil {
		log.Error("💥 Test-Spielstände nicht aufgeräumt", "err", err)
	} else if n > 0 {
		log.Info("🧹 Test-Spielstände aufgeräumt", "anzahl", n)
	}
	return rooms
}

// listen startet HTTP und, wenn Schlüssel und Zertifikat da sind, HTTPS; errs meldet das Ende eines Servers.
func listen(cfg config, handler http.Handler, log *slog.Logger) ([]*http.Server, <-chan error) {
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
		log.Info("🔒 kein HTTPS: " + key + " und " + cert + " fehlen (nur nötig, falls die Xbox HTTPS verlangt)")
	}
	return servers, errs
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
	log.Info("✅ K3C läuft", "urls", urls)
}
