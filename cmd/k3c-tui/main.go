// Command k3c-tui ist die Diagnose-Oberfläche für den laufenden k3c-server (B-002): Räume, Geräte, Tick-Dauer, Speicher und
// Abstürze live. Sie spricht nur mit /api/status… des Servers, mit Token.
//
// Umgebung: K3C_STATUS_TOKEN (Pflicht), K3C_SERVER_URL oder K3C_HTTP_PORT (Standard 8080). `k3c-tui -once` druckt den Zustand
// einmal als Text und endet (Skripte, CI, `docker exec` ohne Terminal).
package main

import (
	"context"
	"flag"
	"fmt"
	"io"
	"os"
	"time"

	tea "charm.land/bubbletea/v2"
)

func main() { os.Exit(run(os.Args[1:], os.Getenv, os.Stdout, os.Stderr)) }

func run(args []string, getenv func(string) string, stdout, stderr io.Writer) int {
	fs := flag.NewFlagSet("k3c-tui", flag.ContinueOnError)
	fs.SetOutput(stderr)
	once := fs.Bool("once", false, "druckt den Zustand einmal als Text und endet")
	if err := fs.Parse(args); err != nil {
		return 2
	}
	c := clientFromEnv(getenv)
	if *once {
		return printOnce(c, stdout, stderr)
	}
	if _, err := tea.NewProgram(newModel(c)).Run(); err != nil {
		_, _ = fmt.Fprintln(stderr, "k3c-tui:", err)
		return 1
	}
	return 0
}

// printOnce holt den Status einmal und druckt ihn ohne Farbe; Fehler gehen als Meldung nach stderr (Exit 1).
func printOnce(c *client, stdout, stderr io.Writer) int {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	st, err := c.status(ctx)
	if err != nil {
		_, _ = fmt.Fprintln(stderr, err)
		return 1
	}
	_, _ = fmt.Fprintln(stdout, render(view{addr: c.base, st: &st, plain: true}))
	return 0
}
