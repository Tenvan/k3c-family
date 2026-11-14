package local

import (
	"testing"

	"k3c/native/k3c-game/view"
)

// B-896: Entwicklermodus und die zuletzt eingestellten Stresstest-Werte überleben Speichern und Neustart.
func TestSettingsStressPersist(t *testing.T) {
	dir := t.TempDir()
	want := view.DefaultSettings()
	want.DevMode = true
	want.Stress = view.StressTest{Preset: "volleBurg", Players: 2, Depth: 1, Seed: 7, Enemies: 20, Growth: view.GrowthExp,
		Factor: 1.5, Build: 5, Troops: 120, Timescale: 2, Waves: 5, Minutes: 10, Enemy: "skeleton", Troop: "warrior"}
	if err := Open(Config{Dir: dir}).SetSettings(want); err != nil {
		t.Fatal(err)
	}
	if got := Open(Config{Dir: dir}).Settings(); got != want {
		t.Errorf("nach Neustart %+v / %v, erwartet %+v / %v", got.Stress, got.DevMode, want.Stress, want.DevMode)
	}
}
