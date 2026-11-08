package net

import (
	"encoding/json"
	"path/filepath"
	"reflect"
	"slices"
	"strconv"
	"strings"
	"testing"

	"k3c/engine/room"
	"k3c/engine/sim"
)

// B-731 Stufe 1: Der alte Weg (JSON-Roundtrip) bleibt als Orakel. stateOf muss dieselbe Map liefern (gleiche Schlüssel,
// gleiche Go-Typen), damit Snapshot- und Delta-JSON byte-gleich sind und deltaOf (reflect.DeepEqual) gleich vergleicht.

// stateOfJSON ist stateOf vor B-731: json.Marshal der Welt und der Wirtschaft, zurück in eine Map, dann ergänzt.
func stateOfJSON(w *sim.World, timescale int, paused bool) map[string]any {
	raw, _ := json.Marshal(w)
	var s map[string]any
	_ = json.Unmarshal(raw, &s)
	raw, _ = json.Marshal(sim.EconomyOf(w))
	_ = json.Unmarshal(raw, &s) // ergänzt s um die Felder der Wirtschaft, Werte unverändert aus der Sim
	delete(s, "seed")
	delete(s, "widthUnits")
	for _, p := range s["players"].([]any) {
		delete(p.(map[string]any), "free")
	}
	addActions(w, s["players"].([]any))
	addPayable(w, s)
	filterSites(w, s)
	s["depth"] = w.Biome.Depth
	if timescale > 0 {
		s["devTimescale"] = timescale
		s["devPaused"] = paused
	}
	return s
}

// sameState prüft Map (Typen) und Bytes von snap und delta gegen das Orakel.
func sameState(t *testing.T, where string, want, got, prevWant, prevGot map[string]any) {
	t.Helper()
	if !reflect.DeepEqual(want, got) {
		t.Fatalf("%s: Zustand weicht vom Orakel ab: %s", where, firstDiff(want, got))
	}
	ws, _ := json.Marshal(stateMsg{"snap", 0, 1, 2, want})
	gs, _ := json.Marshal(stateMsg{"snap", 0, 1, 2, got})
	if string(ws) != string(gs) {
		t.Fatalf("%s: snap-Bytes weichen ab", where)
	}
	if prevWant == nil {
		return
	}
	wd, _ := json.Marshal(stateMsg{"delta", 0, 1, 2, deltaOf(prevWant, want)})
	gd, _ := json.Marshal(stateMsg{"delta", 0, 1, 2, deltaOf(prevGot, got)})
	if string(wd) != string(gd) {
		t.Fatalf("%s: delta-Bytes weichen ab:\n%s\n%s", where, wd, gd)
	}
}

// firstDiff nennt den ersten abweichenden Pfad samt Werten und Typen.
func firstDiff(want, got any) string {
	switch w := want.(type) {
	case map[string]any:
		g, ok := got.(map[string]any)
		if !ok {
			return typed(want, got)
		}
		keys := []string{}
		for k := range w {
			keys = append(keys, k)
		}
		for k := range g {
			if _, ok := w[k]; !ok {
				keys = append(keys, k)
			}
		}
		slices.Sort(keys)
		for _, k := range keys {
			if !reflect.DeepEqual(w[k], g[k]) {
				return k + "." + firstDiff(w[k], g[k])
			}
		}
	case []any:
		g, ok := got.([]any)
		if !ok || len(g) != len(w) {
			return typed(want, got)
		}
		for i := range w {
			if !reflect.DeepEqual(w[i], g[i]) {
				return strconv.Itoa(i) + "." + firstDiff(w[i], g[i])
			}
		}
	}
	return typed(want, got)
}

func typed(want, got any) string {
	return " Orakel " + strings.TrimSpace(sprintT(want)) + ", neu " + strings.TrimSpace(sprintT(got))
}

func sprintT(v any) string {
	raw, _ := json.Marshal(v)
	return reflect.TypeOf(v).String() + " " + string(raw)
}

// AC-02: alle Biome (je Tiefe eine Stufe), feste Seeds, 660 Ticks mit zwei Spielern; dazu Zeitraffer und Pause,
// das Ereignis saved (wie pushStatesSaved) und Game Over (Niederlage-Modus „lost“).
func TestStateGleichOrakelAlleBiome(t *testing.T) {
	biomes := 0
	for depth := 0; ; depth++ {
		isl, err := sim.CreateIsland("rs23-"+strconv.Itoa(depth), []int{depth}, 20, sim.CreateOptions{})
		if err != nil {
			break
		}
		biomes++
		isl.Options.Defeat = "lost"
		sim.AddIslandPlayer(isl, 0)
		sim.AddIslandPlayer(isl, 0)
		runOracle(t, isl, "Tiefe "+strconv.Itoa(depth))
		if !isl.Over {
			t.Errorf("Tiefe %d: Game Over nicht erreicht", depth)
		}
	}
	if biomes < 5 {
		t.Fatalf("nur %d Biome geprüft", biomes)
	}
}

// oracleCovered sind Listen, die im Lauf jedes Bioms mindestens einmal nicht leer sein müssen (sonst prüft der
// Vergleich ihre Einträge nicht).
// Pfeile und Händler decken die Golden-Läufe ab (TestStateGleichOrakelGolden).
var oracleCovered = []string{"players", "coins", "troops", "nodes", "sites", "enemies", "events"}

// runOracle rechnet 660 Takte und vergleicht in jedem Takt Zustand und Delta mit dem Orakel. Dev-Aktionen sorgen
// für Bauten, Truppen, Welle, Nacht, Event und Boss, damit alle Listen Einträge haben.
func runOracle(t *testing.T, isl *sim.Island, name string) {
	t.Helper()
	w := isl.Stages[0]
	var prevWant, prevGot map[string]any
	seen := map[string]bool{}
	for tick := range 660 {
		oracleDev(isl, tick)
		sim.StepIsland(isl, oracleCommands(tick), 1.0/30)
		if tick == 600 {
			w.Castle.HP = 0 // fällt im nächsten Takt: Game Over
		}
		timescale, paused := 0, false
		if tick >= 200 && tick < 400 {
			timescale, paused = 4, tick%50 < 25
		}
		saved := tick%150 == 75
		if saved {
			w.Events = append(w.Events, sim.Event{"type": "saved", "stage": 0})
		}
		want, got := stateOfJSON(w, timescale, paused), stateOf(w, timescale, paused)
		if saved {
			w.Events = w.Events[:len(w.Events)-1]
		}
		sameState(t, name+", Takt "+strconv.Itoa(tick), want, got, prevWant, prevGot)
		prevWant, prevGot = want, got
		for _, k := range oracleCovered {
			seen[k] = seen[k] || len(asList(got[k])) > 0
		}
	}
	for _, k := range oracleCovered {
		if !seen[k] {
			t.Errorf("%s: %s war nie gefüllt, der Vergleich deckt es nicht ab", name, k)
		}
	}
}

// oracleDev: Dev-Aktionen zu festen Takten (Gold, Vorrat, Bauten, Welle, Nacht, Blutmond, Boss).
func oracleDev(isl *sim.Island, tick int) {
	switch tick {
	case 0:
		sim.DevFillStock(isl)
		sim.DevDropGold(isl, 0, 30)
		sim.DevBuildAll(isl, 3)
	case 60:
		sim.DevStartWave(isl, 0)
	case 180:
		sim.DevStartEvent(isl, "bloodMoon")
	case 300:
		_ = sim.DevSpawnBoss(isl, 1, "goblinLeader")
	case 420:
		sim.DevStartWave(isl, 1)
	}
}

// oracleCommands: zwei Spieler laufen gegeneinander, zahlen, schlagen und lösen Skills aus.
func oracleCommands(tick int) []sim.PlayerCommand {
	dir := 1.0
	if (tick/120)%2 == 1 {
		dir = -1
	}
	return []sim.PlayerCommand{
		{MoveX: dir, Sprint: tick%90 < 30, Pay: tick%40 < 10, Attack: tick%7 == 0},
		{MoveX: -dir * 0.5, Pay: tick%60 < 20, Attack: tick%11 == 0, Skill: 1 + tick%4},
	}
}

// Bosswelle (Warnkreis, Boss-Felder) gegen das Orakel.
func TestStateGleichOrakelBosswelle(t *testing.T) {
	isl, w := bossWelt(t)
	prevWant, prevGot := stateOfJSON(w, 0, false), stateOf(w, 0, false)
	sameState(t, "Bosswelle Start", prevWant, prevGot, nil, nil)
	for tick := range 300 {
		bossStep(isl, w)
		want, got := stateOfJSON(w, 0, false), stateOf(w, 0, false)
		sameState(t, "Bosswelle, Takt "+strconv.Itoa(tick), want, got, prevWant, prevGot)
		prevWant, prevGot = want, got
	}
}

// Leere Stufe (Campaign ohne Spieler) und die Vorschau nehmen denselben Weg.
func TestStateGleichOrakelLeereStufe(t *testing.T) {
	w := sim.CreateCampaign("leer", "l", 1).CurrentWorld()
	sameState(t, "leere Stufe", stateOfJSON(w, 0, false), stateOf(w, 0, false), nil, nil)
}

// Alle Golden-Läufe (testdata/golden/sim-*.json.gz) gegen das Orakel, jeder fünfte Takt samt Delta dazwischen;
// über alle Läufe müssen Pfeile, Händler und Pickups vorkommen. Stürme und Bodenflächen (Skills, Boss) deckt nur der
// Reflection-Test ab.
func TestStateGleichOrakelGolden(t *testing.T) {
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
	files, _ := filepath.Glob("../../testdata/golden/sim-*.json.gz")
	if len(files) == 0 {
		t.Fatal("keine Golden-Läufe")
	}
	seen := map[string]bool{}
	for _, f := range files {
		var prevWant, prevGot map[string]any
		replayWorld(t, filepath.Base(f), 0, func(tick int, w *sim.World) {
			if tick%5 != 0 {
				return
			}
			want, got := stateOfJSON(w, 0, false), stateOf(w, 0, false)
			sameState(t, filepath.Base(f)+", Takt "+strconv.Itoa(tick), want, got, prevWant, prevGot)
			prevWant, prevGot = want, got
			for _, k := range []string{"projectiles", "merchant", "pickups", "drops", "hazards"} {
				if l, isList := got[k].([]any); got[k] != nil && (!isList || len(l) > 0) {
					seen[k] = true
				}
			}
		})
	}
	for _, k := range []string{"projectiles", "merchant", "pickups"} {
		if !seen[k] {
			t.Errorf("%s kam in keinem Golden-Lauf vor", k)
		}
	}
	t.Logf("abgedeckt: %v", seen)
}

// Die Vorschau (preview.go) baut ihren Zustand über stateOf: gleich dem Orakel für dieselbe Vorschau-Welt.
func TestStateGleichOrakelVorschau(t *testing.T) {
	for _, biome := range []string{"forest", "cave", "mine", "ironhold", "crystal"} {
		o := previewOpts()
		o.Source.Biome = biome
		w, err := room.BuildPreview(o)
		if err != nil {
			t.Fatal(err)
		}
		res, err := Preview(o)
		if err != nil {
			t.Fatal(err)
		}
		sameState(t, "Vorschau "+biome, stateOfJSON(w, 0, false), res.State, nil, nil)
	}
}
