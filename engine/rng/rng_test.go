package rng

import (
	"encoding/json"
	"os"
	"testing"
)

type golden struct {
	Params struct {
		Int     [2]int   `json:"int"`
		Weights [][2]any `json:"weights"`
		Pick    []string `json:"pick"`
		Shuffle int      `json:"shuffle"`
	} `json:"params"`
	Seeds []struct {
		Seed     string    `json:"seed"`
		Hash     uint32    `json:"hash"`
		Next     []float64 `json:"next"`
		Int      []int     `json:"int"`
		Weighted []string  `json:"weighted"`
		Shuffle  []int     `json:"shuffle"`
		Pick     []string  `json:"pick"`
	} `json:"seeds"`
}

func load(t *testing.T) golden {
	t.Helper()
	raw, err := os.ReadFile("../../testdata/golden/rng.json")
	if err != nil {
		t.Fatal(err)
	}
	var g golden
	if err := json.Unmarshal(raw, &g); err != nil {
		t.Fatal(err)
	}
	if len(g.Seeds) == 0 {
		t.Fatal("rng.json enthält keine Seeds")
	}
	return g
}

// same meldet die erste Abweichung mit Seed, Funktion und Position.
func same[T comparable](t *testing.T, seed, fn string, got, want []T) {
	t.Helper()
	if len(got) != len(want) {
		t.Errorf("Seed %q: %s liefert %d Werte, erwartet %d", seed, fn, len(got), len(want))
		return
	}
	for i := range want {
		if got[i] != want[i] {
			t.Errorf("Seed %q: %s[%d] = %v, erwartet %v", seed, fn, i, got[i], want[i])
			return
		}
	}
}

func times[T any](n int, f func() T) []T {
	out := make([]T, n)
	for i := range out {
		out[i] = f()
	}
	return out
}

func TestGolden(t *testing.T) {
	g := load(t)
	weights := make([]Weight, len(g.Params.Weights))
	for i, w := range g.Params.Weights {
		weights[i] = Weight{Key: w[0].(string), W: w[1].(float64)}
	}
	for _, e := range g.Seeds {
		if h := HashSeed(e.Seed); h != e.Hash {
			t.Errorf("Seed %q: hash = %d, erwartet %d", e.Seed, h, e.Hash)
		}
		r := New(e.Seed)
		same(t, e.Seed, "next", times(len(e.Next), r.Next), e.Next)
		r = New(e.Seed)
		same(t, e.Seed, "int", times(len(e.Int), func() int { return r.Int(g.Params.Int[0], g.Params.Int[1]) }), e.Int)
		r = New(e.Seed)
		same(t, e.Seed, "weighted", times(len(e.Weighted), func() string { return r.Weighted(weights) }), e.Weighted)
		items := make([]int, g.Params.Shuffle)
		for i := range items {
			items[i] = i
		}
		same(t, e.Seed, "shuffle", Shuffle(New(e.Seed), items), e.Shuffle)
		r = New(e.Seed)
		same(t, e.Seed, "pick", times(len(e.Pick), func() string { return Pick(r, g.Params.Pick) }), e.Pick)
	}
}

func TestUmlauteUndEmoji(t *testing.T) {
	seeds := map[string]bool{}
	for _, e := range load(t).Seeds {
		seeds[e.Seed] = true
	}
	for _, s := range []string{"Käse🧀", "Ärger über Öl", "👨‍👩‍👧‍👦"} {
		if !seeds[s] {
			t.Errorf("rng.json deckt Seed %q nicht ab (B-043/AC-01)", s)
		}
	}
}

func TestFehler(t *testing.T) {
	for name, f := range map[string]func(){
		"Pick leer":        func() { Pick(New("x"), []int{}) },
		"Weighted ohne >0": func() { New("x").Weighted([]Weight{{"a", 0}}) },
	} {
		func() {
			defer func() {
				if recover() == nil {
					t.Errorf("%s: kein Fehler", name)
				}
			}()
			f()
		}()
	}
}
