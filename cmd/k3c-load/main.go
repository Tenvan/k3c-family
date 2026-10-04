// Command k3c-load ist das Lasttest-Werkzeug (B-175, Sprint LT1): Es legt n Test-Räume mit je m Bots an, die wie ein Client
// über /ws spielen (Protokoll und Version aus engine/net), deterministisch aus einem Seed (engine/rng). Die Räume heißen
// test-…, der Server räumt sie nach seiner Leer-Frist auf. Am Ende trennen sich alle Bots, auch bei Strg+C.
//
// Aufruf: k3c-load -url http://127.0.0.1:8080 -token … -rooms 2 -players 3 -duration 15m -seed load
// Das Token kommt aus -token oder K3C_STATUS_TOKEN und steht nie in der Ausgabe.
// -duration night misst bis zur ersten Nacht aller Räume (höchstens -max-duration). Alle -interval wird /api/status gelesen
// (Tick-Dauer, Phase, CPU); der Bericht liegt als <-out>.json und <-out>.md (Standard reports/load-<Zeit>), auch nach Strg+C.
// Exit-Code: 0 erreicht oder knapp (Ziel -target-p99); 1 verfehlt; 2 Server nicht erreichbar, Token falsch, Raum abgelehnt,
// Bots noch verbunden oder keine Messwerte.
package main

import (
	"context"
	"flag"
	"fmt"
	"io"
	"os"
	"os/signal"
	"strconv"
	"strings"
	"syscall"
	"time"
)

const envToken = "K3C_STATUS_TOKEN"

func main() {
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	code := run(ctx, os.Args[1:], os.Getenv, os.Stdout, os.Stderr)
	stop()
	os.Exit(code)
}

// config sind die Flags eines Laufs.
type config struct {
	url, token, seed string
	rooms, players   int
	duration         time.Duration
	night            bool                            // -duration night
	maxDuration      time.Duration                   // Obergrenze der Nachtmessung
	interval         time.Duration                   // Abstand der Status-Abfragen
	target           float64                         // Ziel für p99 der Tick-Dauer in ms
	out              string                          // Basis der Berichtsdateien (ohne Endung)
	tag              string                          // Lauf-Kennung in Raumnamen und Geräte-IDs (Wanduhr, nicht Teil der Eingaben)
	sent             func(device string, msg []byte) // Test-Naht: jede gesendete Nachricht; nil = keine
}

func parse(args []string, getenv func(string) string, stderr io.Writer) (config, error) {
	fs := flag.NewFlagSet("k3c-load", flag.ContinueOnError)
	fs.SetOutput(stderr)
	var c config
	fs.StringVar(&c.url, "url", "http://127.0.0.1:8080", "Adresse des Servers")
	fs.StringVar(&c.token, "token", "", "Diagnose-Token des Servers (Standard: Umgebung "+envToken+")")
	fs.IntVar(&c.rooms, "rooms", 2, "Anzahl Test-Räume")
	fs.IntVar(&c.players, "players", 3, "Bots je Raum")
	dur := fs.String("duration", "15m", "Dauer des Laufs (z. B. 5m, 1h) oder night (bis zum Ende der ersten Nacht)")
	fs.DurationVar(&c.maxDuration, "max-duration", 60*time.Minute, "Obergrenze für -duration night")
	fs.DurationVar(&c.interval, "interval", 5*time.Second, "Abstand der Status-Abfragen")
	fs.Float64Var(&c.target, "target-p99", 10, "Ziel für p99 der Tick-Dauer in ms")
	fs.StringVar(&c.out, "out", "", "Bericht ohne Endung (Standard reports/load-<Zeit>), es entstehen .json und .md")
	fs.StringVar(&c.seed, "seed", "load", "Seed der Bot-Eingaben")
	if err := fs.Parse(args); err != nil {
		return c, err
	}
	if c.token == "" {
		c.token = getenv(envToken)
	}
	if *dur == "night" {
		c.night, c.duration = true, c.maxDuration
	} else if d, err := time.ParseDuration(*dur); err == nil {
		c.duration = d
	} else {
		return c, failf("-duration: %q ist weder eine Dauer noch night", *dur)
	}
	switch {
	case c.token == "":
		return c, failf("Token fehlt (-token oder %s)", envToken)
	case c.rooms < 1 || c.players < 1 || c.duration <= 0 || c.interval <= 0 || c.target <= 0:
		return c, failf("-rooms, -players, -duration, -interval und -target-p99 müssen größer 0 sein")
	}
	c.url = strings.TrimRight(c.url, "/")
	if c.out == "" {
		c.out = "reports/load-" + time.Now().Format("20060102-150405")
	}
	c.tag = strconv.FormatInt(time.Now().UnixMilli()%(36*36*36*36*36*36), 36)
	return c, nil
}

func run(ctx context.Context, args []string, getenv func(string) string, stdout, stderr io.Writer) int {
	c, err := parse(args, getenv, stderr)
	if err != nil {
		_, _ = fmt.Fprintln(stderr, "k3c-load:", err)
		return 2
	}
	_, _ = fmt.Fprintf(stdout, "k3c-load: %d Räume × %d Bots, %s gegen %s, Seed %q\n", c.rooms, c.players, c.duration, c.url, c.seed)
	start := time.Now()
	runs, err := load(ctx, c)
	for _, r := range runs {
		_, _ = fmt.Fprintln(stdout, r)
	}
	if err != nil {
		_, _ = fmt.Fprintln(stderr, "k3c-load:", err)
		return 2
	}
	_, _ = fmt.Fprintln(stdout, "Alle Bots getrennt; der Server räumt die Test-Räume nach seiner Leer-Frist auf.")
	rep := buildReport(c, start, runs)
	paths, err := rep.write(c.out)
	if err != nil {
		_, _ = fmt.Fprintln(stderr, "k3c-load: Bericht nicht geschrieben:", err)
		return 2
	}
	_, _ = fmt.Fprintf(stdout, "Bericht: %s\n%s", strings.Join(paths, ", "), rep.markdown())
	if rep.Verdict == ohneDaten {
		_, _ = fmt.Fprintln(stderr, "k3c-load: keine Messwerte (Lauf kürzer als -interval oder Räume nicht getickt)")
	}
	return exitCode(rep.Verdict)
}

// load prüft Server und Token, startet die Räume, spielt bis Dauer oder Signal und trennt alle Bots.
func load(ctx context.Context, c config) ([]*roomRun, error) {
	a := newAPI(c.url, c.token)
	if _, err := a.rooms(ctx); err != nil {
		return nil, err // vor dem ersten Raum: keine Räume bei falschem Token oder fehlendem Server
	}
	play, cancel := context.WithTimeout(ctx, c.duration)
	defer cancel()
	runs, bots, err := startRooms(play, c)
	if err == nil {
		go newPoller(a, runs, c.night).run(play, c.interval, cancel)
		<-play.Done()
	}
	for _, b := range bots {
		b.stop()
	}
	if err != nil {
		return runs, err
	}
	return runs, a.waitDisconnected(context.WithoutCancel(ctx), roomPrefix(c))
}

// roomRun ist ein Test-Raum mit der Tick-Reihe, die sein erster Bot empfangen hat (Grundlage für den Bericht).
type roomRun struct {
	Name, Code string
	Ticks      []tickPoint
	Errors     int      // Fehler-Nachrichten des Servers während des Spiels
	Samples    []sample // Proben aus /api/status (Tick-Dauer, Phase, CPU), alle -interval
}

type tickPoint struct {
	At   time.Duration // seit Start des Raums
	Tick int
}

func (r *roomRun) String() string {
	if len(r.Ticks) == 0 {
		return fmt.Sprintf("%s (%s): keine Ticks, %d Fehler", r.Name, r.Code, r.Errors)
	}
	first, last := r.Ticks[0], r.Ticks[len(r.Ticks)-1]
	return fmt.Sprintf("%s (%s): %d Zustände, Tick %d bis %d in %s, %d Fehler", r.Name, r.Code, len(r.Ticks),
		first.Tick, last.Tick, (last.At - first.At).Round(time.Millisecond), r.Errors)
}

func roomPrefix(c config) string { return "test-load-" + c.tag + "-" }

// meldung ist ein Fehlertext für Menschen (großgeschrieben, deutsch), wie in cmd/k3c-tui.
type meldung string

func (m meldung) Error() string { return string(m) }

func failf(format string, args ...any) error { return meldung(fmt.Sprintf(format, args...)) }
