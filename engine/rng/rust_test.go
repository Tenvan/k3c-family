package rng

import (
	"math"
	"testing"

	"k3c/engine/core"
)

// TestRustBitgleich: Der Rust-Kern (über die Brücke ohne cgo) liefert Hash und Zahlenfolge aller Golden-Seeds bitgleich.
func TestRustBitgleich(t *testing.T) {
	lib, err := core.Find()
	if err != nil {
		t.Skip("Rust-Kern nicht gebaut (task rust:build): ", err)
	}
	for _, e := range load(t).Seeds {
		h, err := lib.RngHash(e.Seed)
		if err != nil {
			t.Fatal(err)
		}
		if h != e.Hash {
			t.Errorf("Seed %q: Rust-Hash %d, erwartet %d", e.Seed, h, e.Hash)
		}
		got, err := lib.RngNext(e.Seed, len(e.Next))
		if err != nil {
			t.Fatal(err)
		}
		for i := range got {
			if math.Float64bits(got[i]) != math.Float64bits(e.Next[i]) {
				t.Errorf("Seed %q: next[%d] = %v, erwartet %v", e.Seed, i, got[i], e.Next[i])
				break
			}
		}
	}
}
