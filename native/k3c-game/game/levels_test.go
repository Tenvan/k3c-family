package game

import (
	"errors"
	"maps"
	"slices"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func stages(levels ...string) []view.Stage {
	out := make([]view.Stage, len(levels))
	for i, lv := range levels {
		out[i] = view.Stage{Index: i, Level: lv, World: &sim.World{}}
	}
	return out
}

func TestSameLevels(t *testing.T) {
	cases := []struct {
		have   []string
		stages []view.Stage
		want   bool
	}{
		{nil, nil, true},
		{nil, stages("wald"), false},
		{[]string{"wald"}, stages("wald", "wald"), true},
		{[]string{"wald"}, stages("wald", "mine"), false},
		{[]string{"mine", "wald"}, stages("wald"), false},
		{[]string{"mine", "wald"}, stages("wald", "mine"), true},
	}
	for i, c := range cases {
		if got := SameLevels(c.have, c.stages); got != c.want {
			t.Errorf("Fall %d: %v", i, got)
		}
	}
}

// assetsOf ist eine Asset-Quelle: je Level feste Assets.
type assetsOf map[string][]string

func (a assetsOf) Assets(level string, _ *sim.World) []string { return a[level] }

// Die Vereinigung aller Quellen je Level, sortiert und ohne Doppelte; zwei Spieler in zwei Leveln laden beide.
func TestWantedAssets(t *testing.T) {
	srcs := []view.AssetSource{
		assetsOf{"wald": {"natur/baum", "figuren/zwerg"}, "mine": {"natur/stein"}},
		assetsOf{"wald": {"figuren/zwerg", "gebaeude/burg"}},
	}
	got := WantedAssets(stages("wald", "mine", "wald"), srcs)
	want := map[string][]string{"wald": {"figuren/zwerg", "gebaeude/burg", "natur/baum"}, "mine": {"natur/stein"}}
	if !maps.EqualFunc(got, want, slices.Equal[[]string]) {
		t.Errorf("%v", got)
	}
}

// loader merkt sich die Aufrufe von SetLevels; fail lässt ihn scheitern.
type loader struct {
	calls []map[string][]string
	fail  error
}

func (l *loader) SetLevels(want map[string][]string) error {
	l.calls = append(l.calls, want)
	return l.fail
}

// Die Schleife lädt nur, wenn sich die Level ändern; ein Fehler beendet das Programm.
func TestLoadLevels(t *testing.T) {
	ld := &loader{}
	g := &Game{parts: Parts{Art: ld, Assets: []view.AssetSource{assetsOf{"wald": {"a/b"}}}}}
	g.frame.Stages = stages("wald")
	g.loadLevels()
	g.loadLevels()
	if len(ld.calls) != 1 || !slices.Equal(g.levels, []string{"wald"}) {
		t.Fatalf("Aufrufe %v, Level %v", ld.calls, g.levels)
	}
	g.frame.Stages = nil
	if g.loadLevels(); len(ld.calls) != 2 || len(ld.calls[1]) != 0 || len(g.levels) != 0 {
		t.Errorf("ins Menü: Aufrufe %v", ld.calls)
	}
	ld.fail = errors.New("fehlende Grafik")
	g.frame.Stages = stages("mine")
	if g.loadLevels(); g.err == nil || len(g.levels) != 0 {
		t.Errorf("Fehler nicht gemeldet: %v %v", g.err, g.levels)
	}
}
