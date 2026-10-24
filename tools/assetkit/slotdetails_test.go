package assets

import (
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
)

const detailsRegister = `{"_comment":"","slots":[
 {"id":"greed","art":"grafik","kategorie":"figuren","beschreibung":"Gegner: greed","wahl":[{"datei":"sprites/goblin/idle.png","sheet":"goblin"}]},
 {"id":"nosheet","art":"grafik","kategorie":"figuren","beschreibung":"x","wahl":[{"datei":"sprites/goblin/idle.png","sheet":"ghost"}]},
 {"id":"leer","art":"grafik","kategorie":"objekte","beschreibung":"y","wahl":[]},
 {"id":"klang","art":"sound","kategorie":"sfx","beschreibung":"z","wahl":[]}
]}`

const detailsSprites = `{"sheets":{"goblin":{"label":"Goblin","frameWidth":150,"frameHeight":150,"originX":0.5,"originY":0.7,
 "height":36,"scale":2.25,"pack":"Monsters","source":"https://example.test/m",
 "anims":{"run":{"frames":2,"fps":12},"idle":{"frames":2,"fps":8}}}},
 "enemies":{"greed":{"sheet":"goblin","tint":"#c77dff"}}}`

const detailsAtlas = `{"textures":[{"image":"atlas-0.png","frames":[
 {"filename":"goblin-idle/0","frame":{"x":10,"y":20,"w":150,"h":150}},
 {"filename":"goblin-idle/1","frame":{"x":160,"y":20,"w":150,"h":150}}]}]}`

const detailsPacks = `# Zuordnung

## Figuren-Packs

| Pack (Ordner-ID) | Quelle/Urheber | Lizenz |
|---|---|---|
| goblin, skeleton | Monsters, LuizMelo | CC0 1.0 |
`

const detailsObjekte = `## Gegner

| Objekt | Pack | Lizenz |
|---|---|---|
| ` + "`greed`" + ` | sprites/goblin | CC0 |
| wolf | x | y |
`

// detailsRepo baut ein Temp-Repo mit Register, sprites.json, Atlas, Zuordnung und Verwendung; mit atlas=false fehlt der Atlas.
func detailsRepo(t *testing.T, atlas bool) string {
	t.Helper()
	root := t.TempDir()
	writeFile(t, filepath.Join(root, RegisterPath), detailsRegister)
	writePNG(t, filepath.Join(root, "public", "sprites", "goblin", "idle.png"), 4, 4)
	writeFile(t, filepath.Join(root, "data", "sprites.json"), detailsSprites)
	if atlas {
		writeFile(t, filepath.Join(root, "public", "atlas", "atlas.json"), detailsAtlas)
	}
	writeFile(t, filepath.Join(root, "docs", "assets", "zuordnung.md"), detailsPacks)
	writeFile(t, filepath.Join(root, "docs", "assets", "zuordnung-objekte.md"), detailsObjekte)
	writeFile(t, filepath.Join(root, "data", "biomes.json"), `{"forest":{"enemies":["greed","wolf"],"greed":1}}`)
	writeFile(t, filepath.Join(root, "data", "enemies.json"), `{"greed":{"name":"Greed"}}`)
	return root
}

func TestSlotDetailsVollstaendig(t *testing.T) {
	d, err := SlotDetailsFor(detailsRepo(t, true), "greed")
	if err != nil {
		t.Fatal(err)
	}
	if d.Status != StatusAssigned || d.Choice == nil || d.Choice.Sheet != "goblin" {
		t.Fatalf("Status/Wahl: %+v", d)
	}
	if d.Sheet == nil || d.Sheet.Tint != "#c77dff" || d.Sheet.FrameW != 150 || d.Sheet.Label != "Goblin" {
		t.Fatalf("Sheet: %+v", d.Sheet)
	}
	if len(d.Sheet.Anims) != 2 || d.Sheet.Anims[0].Name != "run" || d.Sheet.Anims[1].Name != "idle" {
		t.Errorf("Reihenfolge der Animationen: %+v", d.Sheet.Anims)
	}
	if d.Atlas == nil || len(d.Atlas.Anims) != 1 || d.Atlas.Anims[0] != (AtlasAni{Name: "idle", Frames: 2, Page: "atlas-0.png", X: 10, Y: 20, W: 150, H: 150}) {
		t.Errorf("Atlas: %+v", d.Atlas)
	}
}

func TestSlotDetailsHerkunft(t *testing.T) {
	d, err := SlotDetailsFor(detailsRepo(t, true), "greed")
	if err != nil {
		t.Fatal(err)
	}
	if !slices.Equal(d.Hints, []string{"Atlas ohne Frames für goblin-run"}) {
		t.Errorf("Hinweise: %v", d.Hints)
	}
	if d.Pack == nil || d.Pack.ID != "goblin" || d.Pack.Source != "Monsters, LuizMelo" || d.Pack.License != "CC0 1.0" {
		t.Errorf("Pack: %+v", d.Pack)
	}
	if len(d.Assignment) != 1 || d.Assignment[0].Section != "Gegner" || d.Assignment[0].Cells["Lizenz"] != "CC0" {
		t.Errorf("Zuordnung: %+v", d.Assignment)
	}
}

func TestSlotDetailsOhneZuordnung(t *testing.T) {
	root := detailsRepo(t, true)
	for id, want := range map[string]string{"leer": "keine Grafik", "klang": "kein Ton"} {
		d, err := SlotDetailsFor(root, id)
		if err != nil {
			t.Fatal(err)
		}
		if !slices.Equal(d.Hints, []string{want}) || d.Sheet != nil || d.Choice != nil || d.Status != StatusGap {
			t.Errorf("%s: %+v", id, d)
		}
	}
}

func TestSlotDetailsAtlasFehlt(t *testing.T) {
	d, err := SlotDetailsFor(detailsRepo(t, false), "greed")
	if err != nil {
		t.Fatal(err)
	}
	if d.Atlas != nil || !slices.Equal(d.Hints, []string{"Atlas fehlt: task gen:atlas"}) {
		t.Errorf("Atlas %+v Hinweise %v", d.Atlas, d.Hints)
	}
	if d.Sheet == nil {
		t.Error("Sheet soll trotzdem da sein")
	}
}

func TestSlotDetailsSheetFehlt(t *testing.T) {
	d, err := SlotDetailsFor(detailsRepo(t, true), "nosheet")
	if err != nil {
		t.Fatal(err)
	}
	if d.Sheet != nil || !slices.Equal(d.Hints, []string{"Sheet ghost fehlt in data/sprites.json"}) {
		t.Errorf("Sheet %+v Hinweise %v", d.Sheet, d.Hints)
	}
}

func TestSlotDetailsVerwendung(t *testing.T) {
	d, err := SlotDetailsFor(detailsRepo(t, true), "greed")
	if err != nil {
		t.Fatal(err)
	}
	want := []string{"data/biomes.json › forest.enemies", "data/biomes.json › forest.greed", "data/enemies.json › greed"}
	if !slices.Equal(d.Usage, want) {
		t.Errorf("Verwendung: %v", d.Usage)
	}
}

func TestSlotDetailsUnbekannt(t *testing.T) {
	if _, err := SlotDetailsFor(detailsRepo(t, true), "nix"); err == nil {
		t.Error("unbekannter Slot soll ein Fehler sein")
	}
}

// Das echte Repo: greed zeigt Sheet, Atlas, Pack und Zuordnung (nur wenn data/assets.json erreichbar ist).
func TestSlotDetailsEchtesRepo(t *testing.T) {
	root := repoRoot(t)
	d, err := SlotDetailsFor(root, "greed")
	if err != nil {
		t.Fatal(err)
	}
	if d.Sheet == nil || d.Sheet.Name != "goblin" || d.Sheet.Tint != "#c77dff" || len(d.Assignment) == 0 {
		t.Fatalf("greed: %+v", d)
	}
	if _, err := os.Stat(filepath.Join(root, atlasPath)); err == nil && d.Atlas == nil {
		t.Error("Atlas vorhanden, aber nicht gelesen")
	}
	if d.Pack == nil || !strings.Contains(d.Pack.License, "CC0") {
		t.Errorf("Pack: %+v", d.Pack)
	}
}
