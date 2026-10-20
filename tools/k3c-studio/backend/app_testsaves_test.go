package backend

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/engine/room"
)

// TS1.2 (B-507/AC-02): Testspielstände des Studios anlegen, laden (im Spiel öffnen) und löschen, nur im Testordner.

func TestTestspielstaendeAnlegenLadenLoeschen(t *testing.T) {
	a, root := engineApp(t)
	if v := a.WorldTestSaves(); v.Error != "" || len(v.Saves) != 0 {
		t.Fatalf("zu Beginn: %+v", v)
	}
	v := a.WorldTestSaveCreate("abc")
	if v.Error != "" || len(v.Saves) != 1 || v.Saves[0].Name != "voll-abc" {
		t.Fatalf("angelegt: %+v", v)
	}
	res := a.WorldTestSaveOpen("http://127.0.0.1:8091", "voll-abc")
	if want := "http://127.0.0.1:8091/index.html#game.html%3Fautostart%3D1%26save%3Dvoll-abc"; res.Error != "" || res.URL != want {
		t.Errorf("laden: %+v, erwartet %s", res, want)
	}
	if v := a.WorldTestSaveDelete("voll-abc"); v.Error != "" || len(v.Saves) != 0 {
		t.Errorf("gelöscht: %+v", v)
	}
	if _, err := os.Stat(filepath.Join(root, filepath.FromSlash(testSavesDir), "voll-abc.json")); !os.IsNotExist(err) {
		t.Errorf("Datei noch da: %v", err)
	}
	noServerSaves(t, root)
}

func TestTestspielstaendeAblehnungen(t *testing.T) {
	a, _ := engineApp(t)
	for _, seed := range []string{"", "Abc", "a b", "../x"} {
		if v := a.WorldTestSaveCreate(seed); v.Error == "" {
			t.Errorf("Seed %q angenommen", seed)
		}
	}
	for _, name := range []string{"", "../saves/x", "A"} {
		if v := a.WorldTestSaveDelete(name); v.Error == "" {
			t.Errorf("Löschen von %q angenommen", name)
		}
	}
	if res := a.WorldTestSaveOpen("http://127.0.0.1:8091", "fehlt"); res.Error == "" {
		t.Errorf("unbekannter Spielstand geöffnet: %+v", res)
	}
	for _, name := range []string{"", "../saves/x", "A"} {
		if res := a.WorldTestSaveOpen("http://127.0.0.1:8091", name); !strings.Contains(res.Error, "ungültig") {
			t.Errorf("Name %q: %+v", name, res)
		}
	}
	if res := a.WorldTestSaveOpen("http://127.0.0.1:8091", "fehlt"); !strings.Contains(res.Error, "nicht gefunden") {
		t.Errorf("fehlender Stand: %+v", res)
	}
	if res := a.WorldTestSaveOpen("http://example.com", "fehlt"); res.Error == "" {
		t.Errorf("fremde Adresse: %+v", res)
	}
	if v := (&App{}).WorldTestSaves(); v.Error == "" {
		t.Errorf("ohne Engine: %+v", v)
	}
}

// Ein Testspielstand in einem Raum mit verbundenem Gerät wird nicht gelöscht (der Raum schriebe ihn sonst neu).
func TestTestspielstandImOffenenRaumBleibt(t *testing.T) {
	a, _ := engineApp(t)
	a.WorldTestSaveCreate("abc")
	if _, err := a.engine.Rooms.Create("geraet", nopPeer{}, "voll-abc", false, 0, []int{0}, room.Options{}); err != nil {
		t.Fatal(err)
	}
	if v := a.WorldTestSaveDelete("voll-abc"); v.Error == "" || len(v.Saves) != 1 {
		t.Errorf("offener Spielstand gelöscht: %+v", v)
	}
}
