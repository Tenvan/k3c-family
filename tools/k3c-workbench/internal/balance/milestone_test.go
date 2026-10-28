//go:build fullgame

package balance

import (
	"encoding/json"
	"fmt"
	"os"
	"strconv"
	"strings"
	"testing"

	"k3c/engine/sim"
	"k3c/tools/k3c-workbench/internal/enginetools"
)

// Meilenstein-Lauf coop2 (BS3.6) als Go-Test: dasselbe wie `sim_test {bots: coop2, world: standard, days: 40}`, ohne die
// Workbench neu zu bauen. Werte-Varianten wie bei sim_test über die Umgebung K3C_VARY (Semikolon, `datei.json:a.b=wert`);
// K3C_SEEDS (Komma) ersetzt den Seed k3c-dev. Der Test meldet nur, er schlägt nie fehl (offener Meilenstein, wie ohne Hilfen).
// Aufruf: `task balance:milestone` (Taskfile.yml).
func TestMeilensteinCoop2(t *testing.T) {
	seeds := []string{"k3c-dev"}
	if s := os.Getenv("K3C_SEEDS"); s != "" {
		seeds = strings.Split(s, ",")
	}
	var specs []string
	if v := os.Getenv("K3C_VARY"); v != "" {
		specs = strings.Split(v, ";")
	}
	m := Matrix{Seeds: seeds, Players: []int{2}, Bots: []string{"coop2"}, Depths: []int{0}, Days: fullgameDays, World: "standard"}
	var rep Report
	err := WithVariants(specs, func() (e error) {
		rep, e = Run(m)
		return e
	})
	if err != nil {
		t.Fatal(err)
	}
	for _, res := range rep.Results {
		if !res.Valid || res.Metrics == nil {
			t.Errorf("❌ Lauf %s ungültig: %s", res.Seed, res.Error)
			continue
		}
		b, _ := json.Marshal(res.Metrics.IslandMetrics)
		t.Logf("Seed %s Endboss-Tag: %v | %s", res.Seed, res.Metrics.EndbossDay, b)
		if p := progressOf(rep, res.Scenario, seeds); p != nil {
			t.Log("Blocker: " + p.Blocker)
			for _, r := range p.Ladder {
				day := "–"
				if r.MedianDay != nil {
					day = strconv.Itoa(*r.MedianDay)
				}
				t.Logf("  %-40s %d/%d Tag %s", r.Label, r.Reached, r.Runs, day)
			}
		}
	}
}

// TestDiagnoseCoop2 spielt den Meilenstein-Lauf wie runOne und schreibt zu jedem Tagesbeginn den Stand (logFullgameDay),
// zusätzlich bei Gegnern mit Boss alle 30 s Spielzeit die Bosse der Stufe, in der ein Monarch steht (Tag-für-Tag-Diagnose, B-645).
func TestDiagnoseCoop2(t *testing.T) {
	if os.Getenv("K3C_DIAG") == "" {
		t.Skip("nur mit K3C_DIAG=1")
	}
	sc := Scenario{Seed: "k3c-dev", Players: 2, Bot: "coop2", Days: fullgameDays, World: "standard"}
	isl, err := newIsland(sc)
	if err != nil {
		t.Fatal(err)
	}
	bot, _ := BotByName(sc.Bot)
	bot = withBossFight(withDescent(bot, isl, deepestOf(isl)), isl, deepestOf(isl))
	cmds := make([]sim.PlayerCommand, sc.Players)
	last := 0
	for tick := 1; tick <= sc.Days*maxTicksPerDay; tick++ {
		for _, w := range isl.Stages {
			for _, p := range w.Players {
				cmds[p.Index] = bot(w, p)
			}
		}
		sim.StepIsland(isl, cmds, 1.0/enginetools.TickHz)
		afterStep(isl, sc, tick, enginetools.TickHz)
		if d := isl.Stages[0].Cycle.Day; d != last {
			last = d
			logFullgameDay(t, isl, d)
		}
		if d := isl.Stages[0].Cycle.Day; tick%(20*enginetools.TickHz) == 0 && d >= 22 && d <= 31 {
			logTitan(t, isl, tick)
		}
		if isl.Stages[0].Cycle.Day > sc.Days {
			break
		}
	}
}

// logTitan schreibt die Lage der beiden tiefsten Stufen: Gegner (Boss mit HP), Kämpfer, Monarchen mit Stufe, Gold, Ziel.
func logTitan(t *testing.T, isl *sim.Island, tick int) {
	t.Helper()
	line := "t" + strconv.Itoa(tick/enginetools.TickHz) + "s"
	for i := len(isl.Stages) - 2; i < len(isl.Stages); i++ {
		w := isl.Stages[i]
		kinds := map[string]int{}
		for _, en := range w.Enemies {
			kinds[en.Kind]++
		}
		line += " | S" + strconv.Itoa(i) + " Tag " + strconv.Itoa(w.Cycle.Day) + " K" + strconv.Itoa(sim.EconomyOf(w).Fighters) + " Gegner " + strconv.Itoa(len(w.Enemies)) + fmt.Sprint(kinds)
		for _, en := range w.Enemies {
			if en.Boss {
				line += " [" + en.Kind + " " + strconv.Itoa(int(en.HP)) + "/" + strconv.Itoa(int(en.MaxHP)) + " x" + strconv.Itoa(int(en.X)) + "]"
			}
		}
		for _, p := range w.Players {
			line += " M" + strconv.Itoa(p.Index) + " HP" + strconv.Itoa(int(p.HP)) + " G" + strconv.Itoa(p.Gold) + " x" + strconv.Itoa(int(p.X))
		}
	}
	t.Log(line)
}
