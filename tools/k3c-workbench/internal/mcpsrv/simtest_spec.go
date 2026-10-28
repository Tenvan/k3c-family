package mcpsrv

import (
	"context"
	"fmt"
	"regexp"
	"slices"
	"strings"
	"time"

	"k3c/tools/k3c-workbench/internal/balance"
	"k3c/tools/k3c-workbench/internal/simrun"
)

// simTestIn sind die Parameter von sim_test (B-348 › Anforderungen). Geprüft wird über Positivlisten und Grenzen.
type simTestIn struct {
	Action       string   `json:"action" jsonschema:"start, status, wait, stop, list oder baseline"`
	Wait         bool     `json:"wait,omitempty" jsonschema:"bei start: wartet wie check_run auf das Ende des Laufs (mit progressToken bis zum Ende, sonst höchstens 4 min, dann Lauf-ID)"`
	ID           string   `json:"id,omitempty" jsonschema:"Lauf-ID für status und stop, z. B. run-1"`
	Mode         string   `json:"mode,omitempty" jsonschema:"offline (Mocks im Prozess, Standard) oder online (Spielserver des Checkouts)"`
	Clients      int      `json:"clients,omitempty" jsonschema:"0 = headless (Standard), 1 bis 4 = laufende Clients mit Bot-Eingabe, nur online"`
	Players      int      `json:"players,omitempty" jsonschema:"Spieler je Raum, 1 bis 4 (online); offline die Spieleranzahl aller Ziele, z. B. 1 für den Single-Player-Meilenstein"`
	Rooms        int      `json:"rooms,omitempty" jsonschema:"Räume, 1 bis 8 (online)"`
	Bots         string   `json:"bots,omitempty" jsonschema:"Bot-Profil des Balancing-Testers, z. B. coop2 oder saver; offline ersetzt es den Bot aller Ziele"`
	Seeds        int      `json:"seeds,omitempty" jsonschema:"Seeds, 1 bis 200; offline ohne Angabe nach seedMode aus data/balance-targets.json (dev: nur k3c-dev)"`
	World        string   `json:"world,omitempty" jsonschema:"nur offline: Welt aus data/worlds, z. B. standard; der Lauf spielt die ganze erste Insel (Bots steigen ab und kämpfen), Meilenstein-Lauf mit bots coop2, days 40"`
	Days         int      `json:"days,omitempty" jsonschema:"nur offline: Tage je Lauf, 1 bis 60; ersetzt die Tage aller Ziele"`
	Vary         string   `json:"vary,omitempty" jsonschema:"nur offline: Werte-Varianten für diesen Lauf, mit Semikolon, z. B. economy.json:purse.startGold=500;economy.json:purse.maxGold=500; data/ bleibt unverändert"`
	SeedList     string   `json:"seedList,omitempty" jsonschema:"nur offline: genau diese Seeds mit Komma, z. B. k3c-dev oder k3c-dev,7; statt seeds"`
	Duration     string   `json:"duration,omitempty" jsonschema:"Dauer online, z. B. 5m, höchstens 2h"`
	Focus        []string `json:"focus,omitempty" jsonschema:"balance, perf, stability; offline nur balance (Standard)"`
	Baseline     string   `json:"baseline,omitempty" jsonschema:"nur offline, bei start: Name einer gesicherten Baseline (z. B. M1) als Vergleich; der Bericht zeigt den Delta-Vergleich je Werte-Gruppe"`
	SaveBaseline string   `json:"saveBaseline,omitempty" jsonschema:"bei action baseline mit id: sichert den durchgelaufenen Lauf unter diesem Namen (z. B. M1) in reports/baseline/<name>/; ohne Namen listet baseline die gesicherten"`
	Attach       string   `json:"attach,omitempty" jsonschema:"Raumcode laufender Clients (online, clients 1): kein Browser-Start, Clients mit der Feed-Adresse aus dem Status öffnen"`
}

// simSpec ist ein geprüfter Start.
type simSpec struct {
	mode           string
	clients        int
	players, rooms int
	bots           string
	world          string   // offline: Welt der Insel-Läufe (BS11)
	vary           []string // offline: Werte-Varianten (datei.json:pfad=wert)
	baseline       string   // offline: benannte Baseline als Vergleich (B-640)
	days           int      // offline: Tage je Lauf
	attach         string   // Raumcode laufender Clients, leer = Browser starten
	seeds          int
	seedList       []string // offline: genau diese Seeds (B-634)
	duration       time.Duration
	focus          []string
}

var (
	simFocus = []string{"balance", "perf", "stability"}
	roomCode = regexp.MustCompile(`^[A-Z]{4}$`)
)

func (s simSpec) label() string {
	clients := "headless"
	if s.clients > 0 {
		clients = fmt.Sprintf("%d Clients", s.clients)
	}
	return fmt.Sprintf("%s·%s·%s", s.mode, clients, strings.Join(s.focus, ","))
}

// spec prüft die Parameter von start; jeder Fehler ist eine Zeile mit Grund.
func (in simTestIn) spec() (simSpec, error) {
	s := simSpec{mode: in.Mode, clients: in.Clients, players: in.Players, rooms: in.Rooms, bots: in.Bots, world: in.World, days: in.Days, vary: splitSemi(in.Vary), baseline: in.Baseline,
		seeds: in.Seeds, seedList: splitList(in.SeedList), focus: in.Focus, attach: strings.ToUpper(in.Attach)}
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
		{in.ID != "", "id nur bei status, wait, stop und baseline"},
		{in.SaveBaseline != "", "saveBaseline nur bei action baseline"},
		{emptyList(in.SeedList, s.seedList), "seedList: Seeds mit Komma, z. B. k3c-dev,7"},
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

// checkAttach: attach hängt sich an genau einen laufenden Raum (online, ein Client).
func (s simSpec) checkAttach() error {
	switch {
	case s.attach == "":
		return nil
	case s.mode != "online" || s.clients != 1:
		return fmt.Errorf("sim_test start abgelehnt: attach nur mit mode online und clients 1")
	case !roomCode.MatchString(s.attach):
		return fmt.Errorf("sim_test start abgelehnt: attach: Raumcode aus 4 Buchstaben")
	}
	return nil
}

// checkMode lehnt Kombinationen ab, die der Modus nicht kann, und füllt die Standards von online.
func (s *simSpec) checkMode() error {
	if s.days < 0 || s.days > 60 {
		return fmt.Errorf("sim_test start abgelehnt: days 1 bis 60")
	}
	if err := s.checkAttach(); err != nil {
		return err
	}
	if s.mode == "online" {
		if s.world != "" || s.days != 0 || len(s.vary) > 0 || s.baseline != "" {
			return fmt.Errorf("sim_test start abgelehnt: world, days, vary und baseline nur offline")
		}
		if len(s.seedList) > 0 {
			return fmt.Errorf("sim_test start abgelehnt: seedList nur offline")
		}
		return s.onlineCheck()
	}
	switch {
	case s.clients > 0:
		return fmt.Errorf("sim_test start abgelehnt: offline mit Clients geht nicht, ein Client braucht den Spielserver (mode online)")
	case !slices.Equal(s.focus, []string{"balance"}):
		return fmt.Errorf("sim_test start abgelehnt: offline misst nur balance; perf und stability mit mode online")
	case s.rooms != 0 || s.duration != 0:
		return fmt.Errorf("sim_test start abgelehnt: offline bewertet die Ziele aus data/balance-targets.json; " +
			"Räume und Dauer legen dort die Ziele fest (bots ersetzt den Bot aller Ziele, players die Spieleranzahl)")
	}
	return s.offline().Check()
}

// simRunner wählt den Runner des Modus; online prüft er vorher, ob der Spielserver des Checkouts antwortet.
func (s *Server) simRunner(ctx context.Context, spec simSpec) (simrun.Runner, error) {
	if spec.mode == "offline" {
		return s.sims.Offline(*spec.offline()), nil
	}
	c, err := s.serverClient(ctx)
	if err == nil {
		_, err = c.Status(ctx)
	}
	if err != nil {
		return nil, fmt.Errorf("sim_test start abgelehnt: Spielserver des Checkouts antwortet nicht, erst svc_start server (%v)", oneLine(err))
	}
	if spec.clients > 0 {
		return s.clientsRunner(ctx, spec, c)
	}
	return func(ctx context.Context, run *simRun) { s.runOnline(ctx, run, spec, c) }, nil
}

// onlineCheck prüft die Kombinationen mit Clients und füllt die Standards. Mit Clients hat jeder Client seinen Raum;
// das Beobachter-Gerät der Workbench belegt dort keinen Platz (simtest_clients.go, B-352), players geht bis 4.
func (s *simSpec) onlineCheck() error {
	if s.clients > 0 && s.rooms != 0 {
		return fmt.Errorf("sim_test start abgelehnt: rooms ergibt sich aus clients (ein Raum je Client)")
	}
	s.onlineDefaults()
	if s.clients > 0 {
		s.rooms = s.clients
	}
	return nil
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

// offline ist die Spec des Offline-Runners (internal/simrun); online nil.
func (s simSpec) offline() *simrun.Spec {
	if s.mode != "offline" {
		return nil
	}
	return &simrun.Spec{Seeds: s.seeds, SeedList: s.seedList, Bot: s.bots, World: s.world, Days: s.days, Players: s.players, Vary: s.vary, Baseline: s.baseline}
}

// emptyList: Die Eingabe ist gesetzt, enthält aber keinen Eintrag (z. B. „,“).
func emptyList(raw string, list []string) bool { return strings.TrimSpace(raw) != "" && len(list) == 0 }

// splitSemi teilt eine Liste mit Semikolon und lässt leere Einträge weg.
func splitSemi(v string) []string {
	var out []string
	for _, p := range strings.Split(v, ";") {
		if p = strings.TrimSpace(p); p != "" {
			out = append(out, p)
		}
	}
	return out
}

// splitList teilt eine Komma-Liste und lässt leere Einträge weg.
func splitList(v string) []string {
	var out []string
	for _, p := range strings.Split(v, ",") {
		if p = strings.TrimSpace(p); p != "" {
			out = append(out, p)
		}
	}
	return out
}

func oneLine(err error) string { return strings.ReplaceAll(err.Error(), "\n", " ") }
