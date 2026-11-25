package mcpsrv

import (
	"context"
	"fmt"
	"slices"
	"strings"

	"k3c/tools/k3c-studio/internal/balance"
	"k3c/tools/k3c-studio/internal/simrun"
)

// simTestIn sind die Parameter von sim_test (B-348 › Anforderungen). Geprüft wird über Positivlisten und Grenzen.
// Seit dem Rückbau des Spielservers (B-887) läuft sim_test nur offline (Spiel-EXE des Checkouts, RU40.3).
type simTestIn struct {
	Action       string   `json:"action" jsonschema:"start, status, wait, stop, list oder baseline"`
	Wait         bool     `json:"wait,omitempty" jsonschema:"bei start: wartet wie check_run auf das Ende des Laufs (mit progressToken bis zum Ende, sonst höchstens 4 min, dann Lauf-ID)"`
	ID           string   `json:"id,omitempty" jsonschema:"Lauf-ID für status und stop, z. B. run-1"`
	Mode         string   `json:"mode,omitempty" jsonschema:"offline (Spiel-EXE des Checkouts, Standard und einziger Modus)"`
	Head         bool     `json:"head,omitempty" jsonschema:"bei start: Spiel mit Fenster öffnen und die Läufe zeigen (nacheinander); ohne Angabe headless"`
	Speed        int      `json:"speed,omitempty" jsonschema:"Zeitraffer 1 bis 64 (Standard 1 = Echtzeit), nur mit head"`
	Players      int      `json:"players,omitempty" jsonschema:"Spieleranzahl aller Ziele, 1 bis 4, z. B. 1 für den Single-Player-Meilenstein"`
	Bots         string   `json:"bots,omitempty" jsonschema:"Bot-Profil des Balancing-Testers, z. B. coop2 oder saver; ersetzt den Bot aller Ziele"`
	Seeds        int      `json:"seeds,omitempty" jsonschema:"Seeds, 1 bis 200; ohne Angabe nach seedMode aus data/balance-targets.json (dev: nur k3c-dev)"`
	World        string   `json:"world,omitempty" jsonschema:"Welt aus data/worlds, z. B. standard; der Lauf spielt die ganze erste Insel (Bots steigen ab und kämpfen), Meilenstein-Lauf mit bots coop2, days 40"`
	Days         int      `json:"days,omitempty" jsonschema:"Tage je Lauf, 1 bis 60; ersetzt die Tage aller Ziele"`
	Vary         string   `json:"vary,omitempty" jsonschema:"Werte-Varianten für diesen Lauf, mit Semikolon, z. B. economy.json:purse.startGold=500;economy.json:purse.maxGold=500; data/ bleibt unverändert"`
	SeedList     string   `json:"seedList,omitempty" jsonschema:"genau diese Seeds mit Komma, z. B. k3c-dev oder k3c-dev,7; statt seeds"`
	Focus        []string `json:"focus,omitempty" jsonschema:"nur balance (Standard)"`
	Baseline     string   `json:"baseline,omitempty" jsonschema:"bei start: Name einer gesicherten Baseline (z. B. M1) als Vergleich; der Bericht zeigt den Delta-Vergleich je Werte-Gruppe"`
	SaveBaseline string   `json:"saveBaseline,omitempty" jsonschema:"bei action baseline mit id: sichert den durchgelaufenen Lauf unter diesem Namen (z. B. M1) in reports/baseline/<name>/; ohne Namen listet baseline die gesicherten"`
	OhnePass     bool     `json:"ohnePass,omitempty" jsonschema:"bei action baseline mit saveBaseline: sichert auch einen Lauf, der sein Ziel nicht erreicht hat (offener Meilenstein, z. B. Endboss nie besiegt); der Lauf muss gültig durchgelaufen sein"`
}

// simSpec ist ein geprüfter Start.
type simSpec struct {
	mode     string
	players  int
	bots     string
	world    string   // Welt der Insel-Läufe (BS11)
	vary     []string // Werte-Varianten (datei.json:pfad=wert)
	baseline string   // benannte Baseline als Vergleich (B-640)
	days     int      // Tage je Lauf
	seeds    int
	seedList []string // genau diese Seeds (B-634)
	focus    []string
	head     bool // mit Fenster (RU40.3)
	speed    int
}

func (s simSpec) label() string {
	return fmt.Sprintf("%s·%s·%s", s.mode, map[bool]string{true: "head", false: "headless"}[s.head], strings.Join(s.focus, ","))
}

// spec prüft die Parameter von start; jeder Fehler ist eine Zeile mit Grund.
func (in simTestIn) spec() (simSpec, error) {
	s := simSpec{mode: in.Mode, players: in.Players, bots: in.Bots, world: in.World, days: in.Days, vary: splitSemi(in.Vary), baseline: in.Baseline,
		seeds: in.Seeds, seedList: splitList(in.SeedList), focus: in.Focus, head: in.Head, speed: in.Speed}
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
		{s.mode != "offline", fmt.Sprintf("mode %q: nur offline (der Spielserver ist entfernt, B-887)", s.mode)},
		{s.players < 0 || s.players > 4, "players 1 bis 4"},
		{s.seeds < 0 || s.seeds > 200, "seeds 1 bis 200"},
		{s.days < 0 || s.days > 60, "days 1 bis 60"},
		{s.bots != "" && !slices.Contains(balance.BotNames(), s.bots), "bots: " + strings.Join(balance.BotNames(), ", ")},
		{!slices.Equal(s.focus, []string{"balance"}), "focus: nur balance"},
		{in.ID != "", "id nur bei status, wait, stop und baseline"},
		{in.SaveBaseline != "", "saveBaseline nur bei action baseline"},
		{in.OhnePass, "ohnePass nur bei action baseline"},
		{emptyList(in.SeedList, s.seedList), "seedList: Seeds mit Komma, z. B. k3c-dev,7"},
	}
	for _, c := range checks {
		if c.bad {
			return simSpec{}, fmt.Errorf("sim_test start abgelehnt: %s", c.msg)
		}
	}
	return s, s.offline().Check()
}

// simRunner liefert den Offline-Runner.
func (s *Server) simRunner(_ context.Context, spec simSpec) (simrun.Runner, error) {
	return s.sims.Offline(*spec.offline()), nil
}

// offline ist die Spec des Offline-Runners (internal/simrun).
func (s simSpec) offline() *simrun.Spec {
	return &simrun.Spec{Seeds: s.seeds, SeedList: s.seedList, Bot: s.bots, World: s.world, Days: s.days, Players: s.players, Vary: s.vary, Baseline: s.baseline,
		Head: s.head, Speed: s.speed}
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
