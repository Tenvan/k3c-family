package sim

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

// Referenz-Zustand der Insel für den Rust-Port (RU10.5, AC-01; Sprint-README RU10 › Offene Fragen): Der Test schreibt den
// Zustand einer Insel nach 1000 Schritten einmalig nach testdata/rust/insel/insel-1000.json. Ohne die Umgebungsvariable
// K3C_RUSTREF=write überspringt er sich, ein normaler Lauf hat keine Nebenwirkung. Die Rust-Seite liest die Datei
// (`k3c-sim`, `island/takt_tests.rs`) und rechnet dieselben 1000 Schritte nach.

const rustrefInselSteps = 1000

// rustrefInsel ist der Inhalt der Datei: die Eingaben des Laufs, die Felder der Insel und der Zustand jeder Stufe.
type rustrefInsel struct {
	Seed       string  `json:"seed"`
	Steps      int     `json:"steps"`
	Dt         float64 `json:"dt"`
	CycleSpeed float64 `json:"cycleSpeed"`
	Depths     []int   `json:"depths"`
	Players    int     `json:"players"`
	Island     struct {
		Stock           Stock    `json:"stock"`
		SkillPool       int      `json:"skillPool"`
		ChestsOpened    int      `json:"chestsOpened"`
		DefeatedBosses  []string `json:"defeatedBosses"`
		EndbossDefeated bool     `json:"endbossDefeated"`
		GoldCollected   int      `json:"goldCollected"`
		Won             bool     `json:"won"`
		Number          int      `json:"number"`
		GateOpen        bool     `json:"gateOpen"`
		SwitchReady     bool     `json:"switchReady"`
		Completed       []string `json:"completed"`
		Chosen          string   `json:"chosen"`
		Over            bool     `json:"over"`
		MerchantVisits  int      `json:"merchantVisits"`
	} `json:"island"`
	Stages []*World `json:"stages"`
}

func TestRustrefInsel(t *testing.T) {
	if os.Getenv("K3C_RUSTREF") != "write" {
		t.Skip("K3C_RUSTREF=write schreibt testdata/rust/insel/insel-1000.json")
	}
	const players = 2
	depths := []int{0, 1}
	isl, err := CreateIsland("rustref", depths, islandSpeed, CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	for range players {
		AddIslandPlayer(isl, 0)
	}
	for range rustrefInselSteps {
		StepIsland(isl, nil, dt)
	}
	out := rustrefInsel{Seed: "rustref", Steps: rustrefInselSteps, Dt: dt, CycleSpeed: islandSpeed, Depths: depths, Players: players, Stages: isl.Stages}
	out.Island.Stock = *isl.Stock
	out.Island.SkillPool, out.Island.ChestsOpened = isl.SkillPool, isl.ChestsOpened
	out.Island.DefeatedBosses, out.Island.EndbossDefeated = isl.DefeatedBosses, isl.EndbossDefeated
	out.Island.GoldCollected, out.Island.Won, out.Island.Number = isl.GoldCollected, isl.Won, isl.Number
	out.Island.GateOpen, out.Island.SwitchReady = isl.GateOpen, isl.SwitchReady
	out.Island.Completed, out.Island.Chosen, out.Island.Over = isl.Completed, isl.Chosen, isl.Over
	out.Island.MerchantVisits = isl.MerchantVisits
	raw, err := json.Marshal(out)
	if err != nil {
		t.Fatal(err)
	}
	dir := filepath.Join("..", "..", "testdata", "rust", "insel")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, "insel-1000.json"), append(raw, '\n'), 0o644); err != nil {
		t.Fatal(err)
	}
	t.Logf("insel-1000.json: %d Bytes", len(raw)+1)
}
