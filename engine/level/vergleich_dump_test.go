package level

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"testing"

	"k3c/engine/internal/golden"
)

// TestVergleichDump schreibt je Biom und Seed den Golden-Baum eines Levels als JSON, damit der Rust-Test `vergleich_go`
// (rust/crates/k3c-level) dieselben Level gegen Rust prüft (RU45.2, B-1080). Ohne K3C_LEVEL_DUMP übersprungen;
// K3C_LEVEL_SEEDS setzt die Zahl der Seeds je Biom (Standard 1000).
func TestVergleichDump(t *testing.T) {
	dir := os.Getenv("K3C_LEVEL_DUMP")
	if dir == "" {
		t.Skip("K3C_LEVEL_DUMP nicht gesetzt")
	}
	seeds := 1000
	if s := os.Getenv("K3C_LEVEL_SEEDS"); s != "" {
		n, err := strconv.Atoi(s)
		if err != nil || n < 1 {
			t.Fatalf("K3C_LEVEL_SEEDS %q unbrauchbar", s)
		}
		seeds = n
	}
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		out := filepath.Join(dir, id)
		if err := os.MkdirAll(out, 0o755); err != nil {
			t.Fatal(err)
		}
		for i := range seeds {
			seed := fmt.Sprintf("vergleich-%04d", i)
			got, err := Generate(b, seed)
			if err != nil {
				t.Errorf("Biom %s, Seed %q: %v", id, seed, err)
				continue
			}
			raw, err := json.Marshal(golden.Tree(t, got))
			if err != nil {
				t.Fatal(err)
			}
			if err := os.WriteFile(filepath.Join(out, seed+".json"), raw, 0o644); err != nil {
				t.Fatal(err)
			}
		}
	}
}
