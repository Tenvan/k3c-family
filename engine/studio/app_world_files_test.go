package studio

import (
	"os"
	"path/filepath"
	"testing"
)

func TestWeltenAnlegenWaehlenEntfernen(t *testing.T) {
	a := islandRepo(t)
	if v := a.PlanWorlds(); v.Active != "standard" || len(v.Worlds) != 1 {
		t.Fatalf("zu Beginn: %+v", v)
	}
	v := a.PlanWorldAdd("Süd Küste")
	if v.Error != "" || v.Active != "sued-kueste" || len(v.Worlds) != 2 || v.Worlds[1].Name != "Süd Küste" {
		t.Fatalf("nach Anlegen: %+v", v)
	}
	file := filepath.Join(a.root, worldsDir, "sued-kueste.json")
	if _, err := os.Stat(file); err != nil {
		t.Fatal(err)
	}
	// Die Planung gilt für die gewählte Welt: Insel anlegen ändert nur ihre Datei.
	standardBefore, _ := os.ReadFile(filepath.Join(a.root, islandsPath))
	if res := a.PlanIslandAdd(stand(t, a)); res.Error != "" {
		t.Fatal(res.Error)
	}
	if got := len(a.PlanLoad().Islands); got != 2 {
		t.Errorf("neue Welt hat %d Inseln, erwartet 2", got)
	}
	if after, _ := os.ReadFile(filepath.Join(a.root, islandsPath)); string(after) != string(standardBefore) {
		t.Error("die Standard-Welt wurde mitgeändert")
	}
	if v := a.PlanWorldSelect("standard"); v.Active != "standard" || len(a.PlanLoad().Islands) != 1 {
		t.Errorf("Wechsel zur Standard-Welt: %+v", v)
	}
	removeAndCheck(t, a, file)
}

func TestWeltenAblehnungen(t *testing.T) {
	a := islandRepo(t)
	a.PlanWorldAdd("Süd Küste")
	if v := a.PlanWorldAdd("Süd Küste"); v.Error == "" {
		t.Error("doppelte Welt angenommen")
	}
	for _, bad := range []string{"", "   ", "!!!"} {
		if v := a.PlanWorldAdd(bad); v.Error == "" {
			t.Errorf("Name %q angenommen", bad)
		}
	}
	if v := a.PlanWorldSelect("../x"); v.Error == "" {
		t.Error("ungültige ID gewählt")
	}
	if v := a.PlanWorldRemove("standard"); v.Error == "" {
		t.Error("Standard-Welt entfernt")
	}
}

func removeAndCheck(t *testing.T, a *App, file string) {
	t.Helper()
	if v := a.PlanWorldRemove("sued-kueste"); v.Error != "" || len(v.Worlds) != 1 || v.Active != "standard" {
		t.Errorf("nach Entfernen: %+v", v)
	}
	if _, err := os.Stat(file); err == nil {
		t.Error("Datei blieb liegen")
	}
}
