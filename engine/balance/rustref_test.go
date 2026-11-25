package balance

import (
	"flag"
	"os"
	"path/filepath"
	"testing"
)

// write schreibt die Vergleichsdateien für den Rust-Port neu (RU28.1, Muster engine/internal/detvec).
var write = flag.Bool("write", false, "Vergleichsdateien unter testdata/rust/balance neu schreiben")

const rustRefDir = "../../testdata/rust/balance"

// rustRefMatrices sind die festen kurzen Läufe, die der Rust-Port (`k3c-balance`, `rustref_tests.rs`) nachrechnet:
// ein Tag, kein Balancing-Lauf (B-1014).
var rustRefMatrices = map[string]Matrix{
	"matrix-saver-1.json":      {Seeds: []string{"1"}, Players: []int{1}, Bots: []string{"saver"}, Depths: []int{0}, Days: 1},
	"matrix-wirtschaft-2.json": {Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"wirtschaft"}, Depths: []int{0}, Days: 1},
}

// TestRustRef schreibt mit -write die Berichte der festen Matrizen; ohne -write vergleicht er einen neuen Lauf mit den Dateien.
func TestRustRef(t *testing.T) {
	for name, m := range rustRefMatrices {
		t.Run(name, func(t *testing.T) {
			got, err := mustRun(t, m).JSON()
			if err != nil {
				t.Fatal(err)
			}
			path := filepath.Join(rustRefDir, name)
			if *write {
				if err := os.WriteFile(path, got, 0o644); err != nil {
					t.Fatal(err)
				}
				return
			}
			want, err := os.ReadFile(path)
			if err != nil {
				t.Fatalf("%v (schreiben mit -write)", err)
			}
			if string(want) != string(got) {
				t.Fatalf("%s weicht von einem neuen Lauf ab (neu schreiben mit -write, nur bei gewollter Änderung)", name)
			}
		})
	}
}
