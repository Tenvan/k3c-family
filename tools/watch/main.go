// Command k3c-watch baut ein Programm bei jeder Quelländerung neu und startet es neu (Watcher für EXE-Projekte ohne
// eigenen Hot-Reload, z. B. k3c-server und k3c-starter; die Wails-Fenster k3c-dev und k3c-studio nutzen `wails dev`).
//
//	k3c-watch --name server --paths cmd/k3c-server,engine,data --build "task k3c-server:build" -- bin/k3c-server.exe
//
// Aufruf über die Tasks `<projekt>:dev` (Taskfile.yml). Die EXE hinter `--` behält ihren festen Pfad (Firewall-Freigabe);
// ein fehlgeschlagener Build lässt die laufende Version stehen.
package main

import (
	"context"
	"errors"
	"flag"
	"fmt"
	"io"
	"log/slog"
	"os"
	"os/signal"
	"strings"
	"syscall"
	"time"

	"k3c/engine/conlog"
)

// config sind die Aufrufparameter.
type config struct {
	Name  string
	Paths []string      // Ordner oder Dateien relativ zum Arbeitsverzeichnis
	Exts  []string      // Endungen mit Punkt; leer = alle
	Skip  []string      // Dateien oder Ordner, die nie zählen (z. B. was die laufende Anwendung selbst schreibt)
	Build []string      // Befehl ohne Shell (Anführungszeichen werden nicht ausgewertet); leer = nicht bauen
	Cmd   []string      // EXE und Argumente; Cmd[0] ist der Pfad, den der Build schreibt
	Poll  time.Duration // Abstand der Prüfungen
	Grace time.Duration // so lange darf das Programm nach der Bitte um ein Ende speichern, bevor es hart beendet wird
	Quiet time.Duration // so lange muss Ruhe sein, bevor ein Neustart beginnt
}

func main() {
	cfg, err := parseArgs(os.Args[1:], os.Stderr)
	if err != nil {
		if !errors.Is(err, flag.ErrHelp) {
			fmt.Fprintln(os.Stderr, "k3c-watch:", err)
		}
		os.Exit(2)
	}
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	log := slog.New(conlog.New(os.Stderr, slog.LevelInfo, conlog.Color())).With("ns", "watch", "name", cfg.Name)
	if err := newSupervisor(cfg, log).run(ctx); err != nil {
		log.Error("💥 Watcher beendet", "err", err)
		os.Exit(1)
	}
}

// parseArgs liest Flags und den Befehl hinter `--`.
func parseArgs(args []string, errOut io.Writer) (config, error) {
	fs := flag.NewFlagSet("k3c-watch", flag.ContinueOnError)
	fs.SetOutput(errOut)
	name := fs.String("name", "", "Name für die Log-Zeilen (Standard: Dateiname der EXE)")
	paths := fs.String("paths", "", "beobachtete Ordner oder Dateien, mit Komma getrennt (Pflicht)")
	exts := fs.String("ext", ".go,.mod,.sum,.json", "beobachtete Endungen, mit Komma getrennt; leer = alle (Testdateien zählen nie)")
	skip := fs.String("ignore", "", "Dateien oder Ordner, die nicht zählen, mit Komma getrennt (relativ wie --paths)")
	build := fs.String("build", "", "Build-Befehl, der die EXE neu schreibt (ohne Shell, Leerzeichen trennen)")
	poll := fs.Duration("poll", 500*time.Millisecond, "Abstand der Prüfungen")
	grace := fs.Duration("grace", 10*time.Second, "Frist für ein sauberes Ende (Strg+Pause/SIGINT) des Programms vor dem harten Beenden")
	quiet := fs.Duration("quiet", 400*time.Millisecond, "Ruhezeit nach der letzten Änderung vor dem Neustart")
	if err := fs.Parse(args); err != nil {
		return config{}, err
	}
	cfg := config{Name: *name, Paths: splitList(*paths), Exts: splitList(*exts), Skip: splitList(*skip), Build: strings.Fields(*build),
		Cmd: fs.Args(), Poll: *poll, Grace: *grace, Quiet: *quiet}
	switch {
	case len(cfg.Paths) == 0:
		return config{}, errors.New("--paths fehlt")
	case len(cfg.Cmd) == 0:
		return config{}, errors.New("die EXE fehlt: k3c-watch [Flags] -- <exe> [Argumente]")
	case cfg.Poll <= 0 || cfg.Quiet < 0 || cfg.Grace < 0:
		return config{}, errors.New("--poll muss positiv sein, --quiet und --grace dürfen nicht negativ sein")
	}
	if cfg.Name == "" {
		cfg.Name = cfg.Cmd[0]
	}
	return cfg, nil
}

// splitList teilt "a, b,c" in ["a" "b" "c"] und lässt Leeres weg.
func splitList(s string) []string {
	var out []string
	for _, p := range strings.Split(s, ",") {
		if p = strings.TrimSpace(p); p != "" {
			out = append(out, p)
		}
	}
	return out
}
