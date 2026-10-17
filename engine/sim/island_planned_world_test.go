package sim

import (
	"math"
	"testing"

	"k3c/engine/level"
)

// fixedPlan setzt im Wald je ein festes Objekt: Baum (Platz +2), Truhe (+1), Camp (−1, Chunk-Mitte), Portal (+6).
func fixedPlan() map[int][]level.PlannedChunk {
	return map[int][]level.PlannedChunk{0: {
		{Slot: 2, Kind: "forest", Objects: []level.PlannedObject{{Kind: "tree", At: 10}}},
		{Slot: 1, Kind: "forest", Objects: []level.PlannedObject{{Kind: "chest", At: 20}}},
		{Slot: -1, Kind: "recruitCamp"},
		{Slot: 6, Kind: "portal"},
	}}
}

func plannedIsland(t *testing.T) (*Island, *World) {
	t.Helper()
	isl, err := CreateIsland("fest", []int{0}, islandSpeed, CreateOptions{Planned: fixedPlan()})
	if err != nil {
		t.Fatal(err)
	}
	return isl, isl.Stages[0]
}

func hasX(xs []float64, want float64) bool {
	for _, x := range xs {
		if math.Abs(x-want) < 0.01 {
			return true
		}
	}
	return false
}

// B-461/AC-07: Feste Objekte stehen an der geplanten Stelle und wirken in der Sim wie gewürfelte, mit 2 Spielern.
func TestFesteObjekteStehenWieGeplant(t *testing.T) {
	_, w := plannedIsland(t)
	var trees, camps []float64
	for _, n := range w.Nodes {
		if n.Kind == "tree" {
			trees = append(trees, n.X)
		}
	}
	for _, c := range w.Camps {
		camps = append(camps, c.X)
	}
	var chests []float64
	for _, p := range w.Pickups {
		if p.Kind == "chest" {
			chests = append(chests, p.X)
		}
	}
	if !hasX(trees, w.HubX+110) || !hasX(chests, w.HubX+70) || !hasX(camps, w.HubX-75) || !hasX(w.Portals, w.HubX+325) {
		t.Fatalf("Baum %v, Truhe %v, Camp %v, Portale %v; Hub %v", trees, chests, camps, w.Portals, w.HubX)
	}
	if n := vagrantsAt(w, w.HubX-75); n == 0 {
		t.Error("das feste Camp liefert keine Landstreicher")
	}
}

func TestFesteTruheIstAufhebbar(t *testing.T) {
	isl, w := plannedIsland(t)
	p0, _ := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 0)
	p0.X = w.HubX + 70
	StepIsland(isl, []PlayerCommand{{}, {}}, dt)
	if isl.ChestsOpened != 1 {
		t.Fatalf("geöffnete Truhen %d, erwartet 1", isl.ChestsOpened)
	}
}

func TestFesterBaumLaesstSichMarkieren(t *testing.T) {
	isl, w := plannedIsland(t)
	AddIslandPlayer(isl, 0)
	p1 := AddIslandPlayer(isl, 0)
	p1.X = w.HubX + 110
	cmds := []PlayerCommand{{}, {Pay: true}}
	for tm := 0.0; tm < 3; tm += dt {
		p1.X, p1.VX = w.HubX+110, 0
		StepIsland(isl, cmds, dt)
	}
	for _, n := range w.Nodes {
		if n.Kind == "tree" && math.Abs(n.X-(w.HubX+110)) < 0.01 {
			if !n.Marked {
				t.Error("der feste Baum ist nach dem Bezahlen nicht markiert")
			}
			return
		}
	}
	t.Fatal("fester Baum fehlt")
}

func TestWelleKommtAusDemFestenPortal(t *testing.T) {
	isl, w := plannedIsland(t)
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	if !DevSetPhase(isl, "night") {
		t.Fatal("Nacht lässt sich nicht setzen")
	}
	for tm := 0.0; tm < 60; tm += dt {
		StepIsland(isl, []PlayerCommand{{}, {}}, dt)
		if len(w.Enemies) > 0 {
			for _, e := range w.Enemies {
				if hasNear(w.Portals, e.X, 3) {
					return
				}
			}
			t.Fatalf("erster Gegner bei %v, Portale %v", w.Enemies[0].X, w.Portals)
		}
	}
	t.Fatal("keine Welle in 60 s")
}

func hasNear(xs []float64, x, tol float64) bool {
	for _, v := range xs {
		if math.Abs(v-x) <= tol {
			return true
		}
	}
	return false
}
