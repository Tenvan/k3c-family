package sim

import (
	"math"
	"strconv"
	"testing"
)

// W7.1 (B-312, AC-01): Gegner tragen fallengelassene Ausrüstung weg.

func takenEvents(w *World) (n int) {
	for _, e := range w.Events {
		if e["type"] == "equipmentTaken" {
			n++
		}
	}
	return n
}

func TestGegnerNimmtDropMitUndFlieht(t *testing.T) {
	w := quietWorld(t)
	AddPlayer(w)
	AddPlayer(w) // 2 Spieler gleichzeitig
	w.Drops = []*Drop{{ID: w.newID(), Kind: "archer", X: w.HubX - 20}}
	e := spawnEnemy(w, "skeleton", w.HubX-20)
	taken := 0
	for range 90 {
		Step(w, nil, dt)
		taken += takenEvents(w)
	}
	if taken != 1 || len(w.Drops) != 0 || e.Carried == nil || !e.Fleeing {
		t.Fatalf("Ereignisse %d, am Boden %d, getragen %v, flieht %v", taken, len(w.Drops), e.Carried, e.Fleeing)
	}
}

func TestTraegerStirbtDropFaelltZuBoden(t *testing.T) {
	w := quietWorld(t)
	AddPlayer(w)
	w.Drops = []*Drop{{ID: w.newID(), Kind: "warrior", X: w.HubX - 20}}
	e := spawnEnemy(w, "skeleton", w.HubX-20)
	e.HomeX = w.HubX - 40 // sonst verschwindet er beim Greifen sofort im Portal
	Step(w, nil, dt)
	e.X, e.HP = w.HubX-18, 0
	Step(w, nil, dt)
	if len(w.Enemies) != 0 || len(w.Drops) != 1 || w.Drops[0].Kind != "warrior" || math.Abs(w.Drops[0].X-(w.HubX-18)) > 0.5 {
		t.Fatalf("Gegner %d, am Boden %v", len(w.Enemies), w.Drops)
	}
}

// Passiver Spieler (Bot passive, tools/k3c-dev/internal/balance): ohne Eingaben fällt die Burg in mindestens einem Seed.
func TestPassiverSpielerBurgFaellt(t *testing.T) {
	for _, seed := range []string{"1", "2", "3"} {
		isl, err := CreateIsland(seed, []int{0}, 1, CreateOptions{})
		if err != nil {
			t.Fatal(err)
		}
		AddIslandPlayer(isl, 0)
		w, cmds := isl.Stages[0], make([]PlayerCommand, 1)
		for tick := 1; w.Cycle.Day <= 5 && tick < 5*600*30; tick++ {
			StepIsland(isl, cmds, dt)
			for _, e := range w.Events {
				if e["type"] == "castleFallen" {
					t.Logf("Seed %s: Burg fällt in Nacht %d, Tick %d", seed, w.Cycle.Day, tick)
					return
				}
			}
		}
		t.Log("Seed " + seed + " ohne Burgfall, Tag " + strconv.Itoa(w.Cycle.Day))
	}
	t.Fatal("Burg fällt in keinem Seed")
}
