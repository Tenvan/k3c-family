package balance

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"slices"
	"strings"

	"k3c/data"
)

// Namen, die die Workbench vor dem Start eines Laufs prüft (RS2.2, B-726): Bot-Profile, Hilfen und Welten. Die Bots
// selbst rechnet bin/k3c-sim (engine/balance); die Liste hier hält names_test.go gleich mit engine/balance/bots.go
// und helps.go (Quelltext-Vergleich, ohne Import).

// botNames sind die Profile aus engine/balance/bots.go › bots, sortiert.
var botNames = []string{"coop2", "coop4", "economy", "passive", "saver", "walls", "wirtschaft"}

// minPlayers ist die Mindest-Spieleranzahl eines Profils (fehlt der Name: 1), wie engine/balance.
var minPlayers = map[string]int{"coop2": 2, "coop4": 4}

// MixSep trennt die Profile einer Mischung je Spieler (B-611): "saver+passive" = Spieler 1 saver, Spieler 2 passive.
const MixSep = "+"

// Hilfen (BS3.4) wie engine/balance/helps.go; leer = keine, nur mit World.
const (
	HelpsAids = "hilfen"
	HelpsFull = "voll"
	HelpsBoss = "boss"
)

// BotNames liefert die Profilnamen sortiert.
func BotNames() []string { return slices.Clone(botNames) }

// KnownBot sagt, ob name ein Profil oder eine Mischung je Spieler ist ("saver+passive"); coop2/coop4 gehören in keine
// Mischung.
func KnownBot(name string) bool {
	parts := strings.Split(name, MixSep)
	if len(parts) == 1 {
		return slices.Contains(botNames, name)
	}
	for _, n := range parts {
		if !slices.Contains(botNames, n) || minPlayers[n] > 0 {
			return false
		}
	}
	return true
}

// checkPlayers meldet einen Fehler, wenn das Profil mehr Spieler braucht, als das Szenario hat, oder eine Mischung
// nicht genau ein Profil je Spieler nennt.
func checkPlayers(bot string, players int) error {
	if parts := strings.Split(bot, MixSep); len(parts) > 1 && len(parts) != players {
		return fmt.Errorf("mischung %s nennt %d Profile, Szenario hat %d Spieler", bot, len(parts), players)
	}
	if n := minPlayers[bot]; players < n {
		return fmt.Errorf("profil %s braucht %d Spieler, Szenario hat %d", bot, n, players)
	}
	return nil
}

// HelpsNames sind die gültigen Werte von Scenario.Helps (ohne den leeren).
func HelpsNames() []string { return []string{HelpsAids, HelpsFull, HelpsBoss} }

// checkHelps prüft Wert und Zusammenhang mit der Welt.
func checkHelps(helps, world string) error {
	switch helps {
	case "":
		return nil
	case HelpsAids, HelpsFull, HelpsBoss:
		if world == "" {
			return fmt.Errorf("hilfen %q brauchen eine Welt (ganze Insel)", helps)
		}
		return nil
	}
	return fmt.Errorf("unbekannte hilfen %q (bekannt: %v)", helps, HelpsNames())
}

// Worlds liefert die IDs der Welt-Dateien in data/worlds (sortiert), die ein Lauf als World nehmen darf.
func Worlds() ([]string, error) {
	entries, err := fs.ReadDir(data.Files, "worlds")
	if err != nil {
		return nil, err
	}
	var ids []string
	for _, e := range entries {
		if id, ok := strings.CutSuffix(e.Name(), ".json"); ok && !e.IsDir() {
			ids = append(ids, id)
		}
	}
	return ids, nil
}

// worldDef ist der Teil einer Welt-Datei, den Prüfung und Meilenstein-Leiter brauchen; streng prüft bin/k3c-sim.
type worldDef struct {
	Islands []struct {
		Depths []int `json:"depths"`
	} `json:"islands"`
}

// loadWorld liest data/worlds/<id>.json.
func loadWorld(id string) (worldDef, error) {
	if !slices.Contains(idsOrNil(Worlds()), id) {
		return worldDef{}, fmt.Errorf("data/worlds/%s.json fehlt", id)
	}
	raw, err := fs.ReadFile(data.Files, "worlds/"+id+".json")
	if err != nil {
		return worldDef{}, err
	}
	var w worldDef
	if err := json.Unmarshal(raw, &w); err != nil {
		return worldDef{}, fmt.Errorf("data/worlds/%s.json: %w", id, err)
	}
	return w, nil
}

func idsOrNil(ids []string, err error) []string {
	if err != nil {
		return nil
	}
	return ids
}
