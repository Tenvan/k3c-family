package mcpsrv

import (
	"context"
	"fmt"
	"slices"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/balance"
)

// simTestIn sind die Parameter von sim_test (B-348 › Anforderungen). Geprüft wird über Positivlisten und Grenzen.
type simTestIn struct {
	Action   string   `json:"action" jsonschema:"start, status, stop oder list"`
	ID       string   `json:"id,omitempty" jsonschema:"Lauf-ID für status und stop, z. B. run-1"`
	Mode     string   `json:"mode,omitempty" jsonschema:"offline (Mocks im Prozess, Standard) oder online (Spielserver des Checkouts)"`
	Clients  int      `json:"clients,omitempty" jsonschema:"0 = headless (Standard), 1 bis 4 = laufende Clients mit Bot-Eingabe, nur online"`
	Players  int      `json:"players,omitempty" jsonschema:"Spieler je Raum, 1 bis 4 (online)"`
	Rooms    int      `json:"rooms,omitempty" jsonschema:"Räume, 1 bis 8 (online)"`
	Bots     string   `json:"bots,omitempty" jsonschema:"Bot-Profil des Balancing-Testers, z. B. saver (online)"`
	Seeds    int      `json:"seeds,omitempty" jsonschema:"Seeds, 1 bis 200; offline ohne Angabe alle aus data/balance-targets.json"`
	Duration string   `json:"duration,omitempty" jsonschema:"Dauer online, z. B. 5m, höchstens 2h"`
	Focus    []string `json:"focus,omitempty" jsonschema:"balance, perf, stability; offline nur balance (Standard)"`
}

// simSpec ist ein geprüfter Start.
type simSpec struct {
	mode           string
	clients        int
	players, rooms int
	bots           string
	seeds          int
	duration       time.Duration
	focus          []string
}

var simFocus = []string{"balance", "perf", "stability"}

func (s simSpec) label() string {
	clients := "headless"
	if s.clients > 0 {
		clients = fmt.Sprintf("%d Clients", s.clients)
	}
	return fmt.Sprintf("%s·%s·%s", s.mode, clients, strings.Join(s.focus, ","))
}

// spec prüft die Parameter von start; jeder Fehler ist eine Zeile mit Grund.
func (in simTestIn) spec() (simSpec, error) {
	s := simSpec{mode: in.Mode, clients: in.Clients, players: in.Players, rooms: in.Rooms, bots: in.Bots,
		seeds: in.Seeds, focus: in.Focus}
	if s.mode == "" {
		s.mode = "offline"
	}
	if len(s.focus) == 0 {
		s.focus = []string{"balance"}
	}
	checks := []struct {
		bad bool
		msg string
	}{
		{s.mode != "offline" && s.mode != "online", fmt.Sprintf("mode %q: offline oder online", s.mode)},
		{s.clients < 0 || s.clients > 4, "clients 0 bis 4"},
		{s.players < 0 || s.players > 4, "players 1 bis 4"},
		{s.rooms < 0 || s.rooms > 8, "rooms 1 bis 8"},
		{s.seeds < 0 || s.seeds > 200, "seeds 1 bis 200"},
		{s.bots != "" && !slices.Contains(balance.BotNames(), s.bots), "bots: " + strings.Join(balance.BotNames(), ", ")},
		{slices.ContainsFunc(s.focus, func(f string) bool { return !slices.Contains(simFocus, f) }), "focus: balance, perf, stability"},
		{in.ID != "", "id nur bei status und stop"},
	}
	for _, c := range checks {
		if c.bad {
			return simSpec{}, fmt.Errorf("sim_test start abgelehnt: %s", c.msg)
		}
	}
	if in.Duration != "" {
		d, err := time.ParseDuration(in.Duration)
		if err != nil || d <= 0 || d > 2*time.Hour {
			return simSpec{}, fmt.Errorf("sim_test start abgelehnt: duration %q, z. B. 5m, höchstens 2h", in.Duration)
		}
		s.duration = d
	}
	return s, s.checkMode()
}

// checkMode lehnt Kombinationen ab, die der Modus nicht kann, und füllt die Standards von online.
func (s *simSpec) checkMode() error {
	if s.mode == "online" {
		s.onlineDefaults()
		return nil
	}
	switch {
	case s.clients > 0:
		return fmt.Errorf("sim_test start abgelehnt: offline mit Clients geht nicht, ein Client braucht den Spielserver (mode online)")
	case !slices.Equal(s.focus, []string{"balance"}):
		return fmt.Errorf("sim_test start abgelehnt: offline misst nur balance; perf und stability mit mode online")
	case s.players != 0 || s.rooms != 0 || s.bots != "" || s.duration != 0:
		return fmt.Errorf("sim_test start abgelehnt: offline bewertet die Ziele aus data/balance-targets.json; " +
			"Spieler, Räume, Bots und Dauer legen dort die Ziele fest")
	}
	return nil
}

// simRunner wählt den Runner des Modus; online prüft er vorher, ob der Spielserver des Checkouts antwortet.
func (s *Server) simRunner(ctx context.Context, spec simSpec) (func(context.Context, *simRun), error) {
	if spec.mode == "offline" {
		return func(ctx context.Context, run *simRun) { s.runOffline(ctx, run, spec) }, nil
	}
	if spec.clients > 0 {
		return nil, fmt.Errorf("sim_test start abgelehnt: clients 1–4 folgen mit TR1.3; jetzt clients 0 (headless)")
	}
	c, err := s.serverClient(ctx)
	if err == nil {
		_, err = c.Status(ctx)
	}
	if err != nil {
		return nil, fmt.Errorf("sim_test start abgelehnt: Spielserver des Checkouts antwortet nicht, erst svc_start server (%v)", oneLine(err))
	}
	return func(ctx context.Context, run *simRun) { s.runOnline(ctx, run, spec, c) }, nil
}

// onlineDefaults füllt fehlende Werte für online: 1 Raum, 2 Bots saver, 5 min.
func (s *simSpec) onlineDefaults() {
	if s.rooms == 0 {
		s.rooms = 1
	}
	if s.players == 0 {
		s.players = 2
	}
	if s.bots == "" {
		s.bots = "saver"
	}
	if s.duration == 0 {
		s.duration = 5 * time.Minute
	}
}

func oneLine(err error) string { return strings.ReplaceAll(err.Error(), "\n", " ") }
