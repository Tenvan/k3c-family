//go:build fullgame

package balance

import (
	"encoding/json"
	"os"
	"strconv"
	"strings"
	"testing"
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
