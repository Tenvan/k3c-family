package sim

import (
	"encoding/json"
	"math"
	"testing"
)

// Raum-Optionen, Grade und Wellenfaktor der Insel (SP13.1, B-101).

// islandWith baut eine Insel mit n Spielern und Grad.
func islandWith(t *testing.T, seed, grade string, n int) *Island {
	t.Helper()
	isl := mustIsland(t, seed, []int{0})
	if err := SetGrade(isl, grade, true); err != nil {
		t.Fatal(err)
	}
	for range n {
		AddIslandPlayer(isl, 0)
	}
	return isl
}

// waveCount startet eine Welle (vorherige Welle = wave-1) und liefert die Gegnerzahl des wave-Ereignisses.
func waveCount(isl *Island, wave int) int {
	w := isl.Stages[0]
	w.Wave = wave - 1
	w.Events = nil
	startWave(w)
	for _, e := range w.Events {
		if e["type"] == "wave" {
			return e["count"].(int)
		}
	}
	return -1
}

func TestIslandOptionsWaveSizeByPlayers(t *testing.T) {
	base := waveCount(islandWith(t, "opt-a", "normal", 1), 1)
	if base <= 0 {
		t.Fatalf("Basiswelle leer: %d", base)
	}
	for n := 1; n <= 4; n++ {
		want := int(math.Round(float64(base) * (1 + 0.5*float64(n-1))))
		if got := waveCount(islandWith(t, "opt-a", "normal", n), 1); got != want {
			t.Errorf("%d Spieler, Welle 1: %d Gegner, erwartet %d", n, got, want)
		}
	}
	b6 := waveCount(islandWith(t, "opt-a", "normal", 1), 6)
	if got := waveCount(islandWith(t, "opt-a", "normal", 4), 6); got < b6*2 {
		t.Errorf("Welle 6 mit 4 Spielern: %d, erwartet mindestens %d", got, b6*2)
	}
}

func TestIslandOptionsGradeFactors(t *testing.T) {
	ref := islandWith(t, "opt-b", "normal", 1)
	refCount := waveCount(ref, 1)
	refEnemy := spawnEnemy(ref.Stages[0], "goblin", 0)
	for _, g := range gradeNames {
		isl := islandWith(t, "opt-b", g, 1)
		f := difficulty[g]
		if got, want := waveCount(isl, 1), int(math.Round(float64(refCount)*f.WaveSize)); got != want {
			t.Errorf("%s: %d Gegner, erwartet %d", g, got, want)
		}
		w := isl.Stages[0]
		w.Enemies = nil
		w.Time = 1e6 // alle wartenden Gegner sind fällig
		stepSpawns(w)
		e := w.Enemies[0]
		base := enemyData[e.Kind]
		if want := math.Round(base.HP * f.EnemyHP); e.MaxHP != want {
			t.Errorf("%s: HP %v, erwartet %v", g, e.MaxHP, want)
		}
		if want := math.Round(base.Damage * f.EnemyDamage); e.Damage != want {
			t.Errorf("%s: Schaden %v, erwartet %v", g, e.Damage, want)
		}
	}
	if refEnemy.MaxHP != enemyData["goblin"].HP || refEnemy.Damage != enemyData["goblin"].Damage {
		t.Error("spawnEnemy ohne Grad muss die Basiswerte liefern")
	}
}

func TestIslandOptionsGradeChangeAppliesNextWave(t *testing.T) {
	isl := islandWith(t, "opt-c", "hard", 1)
	w := isl.Stages[0]
	waveCount(isl, 1)
	queued := len(w.SpawnQueue)
	if err := SetGrade(isl, "easy", false); err != nil {
		t.Fatal(err)
	}
	w.Time = 1e6
	stepSpawns(w)
	if len(w.Enemies) != queued {
		t.Fatalf("%d Gegner, erwartet %d", len(w.Enemies), queued)
	}
	hard := difficulty["hard"]
	if e := w.Enemies[0]; e.MaxHP != math.Round(enemyData[e.Kind].HP*hard.EnemyHP) {
		t.Errorf("wartender Gegner hat nach dem Gradwechsel andere HP: %v", e.MaxHP)
	}
	w.Enemies = nil
	waveCount(isl, 2)
	w.Time = 2e6
	stepSpawns(w)
	if e := w.Enemies[0]; e.MaxHP != math.Round(enemyData[e.Kind].HP*difficulty["easy"].EnemyHP) {
		t.Errorf("nächste Welle nutzt nicht den neuen Grad: %v", e.MaxHP)
	}
}

func TestIslandOptionsSetOptions(t *testing.T) {
	isl := mustIsland(t, "opt-d", []int{0})
	if isl.Options != DefaultOptions() || isl.Options.Defeat != "stage" {
		t.Fatalf("Standard falsch: %+v", isl.Options)
	}
	for _, bad := range []IslandOptions{{"x", "endboss", "stage"}, {"normal", "x", "stage"}, {"normal", "endboss", "x"}} {
		if SetOptions(isl, bad, true) == nil {
			t.Errorf("%+v sollte abgelehnt werden", bad)
		}
	}
	if SetGrade(isl, "dev", false) == nil {
		t.Error("dev ohne Dev-Mode muss abgelehnt werden")
	}
	if err := SetGrade(isl, "ultra", false); err != nil || isl.Options.Defeat != "stage" {
		t.Errorf("Gradwechsel ändert den Niederlage-Modus nicht: %v %+v", err, isl.Options)
	}
	if o := DefaultOptionsFor("ultra"); o.Defeat != "lost" || DefaultOptionsFor("easy").Defeat != "resources" {
		t.Errorf("Standards je Grad falsch: %+v", o)
	}
}

func TestIslandOptionsSaveRoundTrip(t *testing.T) {
	isl := mustIsland(t, "opt-e", []int{0, 1})
	if err := SetOptions(isl, IslandOptions{"hard", "gold", "lost"}, false); err != nil {
		t.Fatal(err)
	}
	got := loadIsland(t, saveJSON(t, isl.ToSave("t")))
	if got.Options != isl.Options {
		t.Errorf("Optionen nach dem Laden: %+v", got.Options)
	}
	// Stand ohne Optionen (Feld fehlt) lädt mit Standard
	raw := saveJSON(t, isl.ToSave("t"))
	var m map[string]any
	if err := json.Unmarshal(raw, &m); err != nil {
		t.Fatal(err)
	}
	delete(m, "options")
	noOpts, err := json.Marshal(m)
	if err != nil {
		t.Fatal(err)
	}
	if got := loadIsland(t, noOpts); got.Options != DefaultOptions() {
		t.Errorf("Stand ohne Optionen: %+v", got.Options)
	}
	// Stand der Version 1 (Campaign) ebenfalls
	if got := loadIsland(t, v1Save(t)); got.Options != DefaultOptions() {
		t.Errorf("v1-Stand: %+v", got.Options)
	}
}

func TestIslandOptionsDeterministic(t *testing.T) {
	run := func() []string {
		isl := islandWith(t, "opt-f", "ultra", 3)
		runIsland(isl, []PlayerCommand{{}, {}, {}}, 60)
		return islandJSON(t, isl)
	}
	a, b := run(), run()
	if a[0] != b[0] {
		t.Error("gleiche Eingaben, gleicher Seed: Zustand weicht ab")
	}
}

func TestIslandOptionsWorldWithoutIslandUnchanged(t *testing.T) {
	w, err := CreateWorld(biomeForDepth(0), "opt-g", Options{CycleSpeed: islandSpeed})
	if err != nil {
		t.Fatal(err)
	}
	startWave(w)
	for _, s := range w.SpawnQueue {
		if s.hpFactor != 1 || s.damageFactor != 1 {
			t.Fatalf("Welt ohne Insel: Faktor %v/%v", s.hpFactor, s.damageFactor)
		}
	}
}
