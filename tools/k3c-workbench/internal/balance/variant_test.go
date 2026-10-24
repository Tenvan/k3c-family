package balance

import (
	"crypto/sha256"
	"io/fs"
	"os"
	"strings"
	"testing"

	"k3c/engine/sim"
)

// dataSum: Prüfsumme aller Dateien unter data/ auf der Platte.
func dataSum(t *testing.T) [32]byte {
	t.Helper()
	h := sha256.New()
	err := fs.WalkDir(os.DirFS("../../../../data"), ".", func(p string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		b, err := os.ReadFile("../../../../data/" + p)
		h.Write([]byte(p))
		h.Write(b)
		return err
	})
	if err != nil {
		t.Fatal(err)
	}
	return [32]byte(h.Sum(nil))
}

// WZ28.4/AC-04: Eine Variante setzt den Wert im Lauf, data/ bleibt unverändert, danach gilt wieder der alte Wert.
func TestVarianteAendertWertNichtData(t *testing.T) {
	before := dataSum(t)
	startGold := func() int {
		isl, err := sim.CreateIsland("1", []int{0}, 1, sim.CreateOptions{})
		if err != nil {
			t.Fatal(err)
		}
		return sim.AddIslandPlayer(isl, 0).Gold
	}
	normal := startGold()
	var varied int
	if err := WithVariants([]string{"economy.json:purse.startGold=37"}, func() error {
		varied = startGold()
		return nil
	}); err != nil {
		t.Fatal(err)
	}
	if varied != 37 || startGold() != normal {
		t.Errorf("Startgold mit Variante %d (erwartet 37), danach %d (erwartet %d)", varied, startGold(), normal)
	}
	if dataSum(t) != before {
		t.Error("data/ wurde geändert")
	}
}

// Ungültige Varianten sind Fehler mit der Variante.
func TestVarianteUngueltig(t *testing.T) {
	for _, s := range []string{"economy.json:purse.gibtsnicht=1", "economy.json:purse=1", "fehlt.json:a=1",
		"economy.json:purse.startGold=viel", "economy.json:purse.startGold"} {
		err := WithVariants([]string{s}, func() error { return nil })
		if err == nil || !strings.Contains(err.Error(), s) {
			t.Errorf("%s: Fehler mit Variante erwartet, war %v", s, err)
		}
	}
}

// Matrix-Einschränkung: Das Szenario der Zielläufe folgt Scope; Fortschritt kommt nach jedem Lauf.
func TestScopeUndFortschritt(t *testing.T) {
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	days, players := 1, 1
	r, err := ts.Restrict(Scope{Days: &days, Players: &players, Bot: "passive", Grade: "easy"})
	if err != nil {
		t.Fatal(err)
	}
	var lines []string
	rep, err := r.RunTargetsProgress(2, func(done, total int) {
		lines = append(lines, strings.Repeat("x", done)+"/"+strings.Repeat("x", total))
	})
	if err != nil {
		t.Fatal(err)
	}
	if len(rep.Results) != 2 || len(lines) != 2 || lines[1] != "xx/xx" {
		t.Fatalf("%d Läufe, Fortschritt %v", len(rep.Results), lines)
	}
	for _, res := range rep.Results {
		if res.Days != 1 || res.Players != 1 || res.Bot != "passive" || res.Grade != "easy" || !res.Valid {
			t.Errorf("Lauf %+v (%s)", res.Scenario, res.Error)
		}
	}
	if len(ts.Targets) > 0 && ts.Targets[0].Days == 1 {
		t.Error("Restrict hat die Ziele selbst geändert")
	}
}

// Ein ungültiges Szenario ist ein Fehler.
func TestScopeUngueltig(t *testing.T) {
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	for _, s := range []Scope{{Bot: "gibtsnicht"}, {Grade: "gibtsnicht"}, {Players: &[]int{5}[0]}} {
		if _, err := ts.Restrict(s); err == nil {
			t.Errorf("%+v: Fehler erwartet", s)
		}
	}
}
