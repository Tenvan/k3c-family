package studio

import (
	"os"
	"path/filepath"
	"testing"
)

func islandRepo(t *testing.T) *App {
	t.Helper()
	root := t.TempDir()
	write(t, filepath.Join(root, islandsPath), `{
  "notes": "x",
  "islands": [
    { "id": "island1", "name": "Insel 1", "depths": [0, 1, 2], "depthScaling": { "hp": 1.5, "damage": 1.3 }, "plannedChunks": {} }
  ]
}
`)
	return &App{root: root, bus: newBus()}
}

func stand(t *testing.T, a *App) string {
	t.Helper()
	raw, err := a.readIslands()
	if err != nil {
		t.Fatal(err)
	}
	return planStand(raw)
}

func TestInselAnlegenUndEntfernen(t *testing.T) {
	a := islandRepo(t)
	before, _ := os.ReadFile(filepath.Join(a.root, islandsPath))
	res := a.PlanIslandAdd(stand(t, a))
	if res.Error != "" {
		t.Fatal(res.Error)
	}
	view := a.PlanLoad()
	if view.Error != "" || len(view.Islands) != 2 || view.Islands[1].ID != "island2" || view.Islands[1].Name != "Insel 2" {
		t.Fatalf("nach Anlegen: %+v", view)
	}
	if res := a.PlanIslandAdd("alt"); !res.Stale {
		t.Errorf("veralteter Stand: %+v", res)
	}
	if res := a.PlanIslandRemove("island1", stand(t, a)); res.Error == "" {
		t.Error("die erste Insel darf nicht entfernt werden")
	}
	if res := a.PlanIslandRemove("island2", stand(t, a)); res.Error != "" {
		t.Fatal(res.Error)
	}
	if after, _ := os.ReadFile(filepath.Join(a.root, islandsPath)); string(after) != string(before) {
		t.Errorf("nach Anlegen und Entfernen nicht byte-gleich: %q", after)
	}
	if len(a.PlanLoad().Islands) != 1 {
		t.Error("eine Insel erwartet")
	}
}
