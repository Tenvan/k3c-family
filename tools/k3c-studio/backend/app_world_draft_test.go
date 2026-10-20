package backend

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/engine/sim"
)

// WZ18.6 (B-488/AC-05): Welten Editor, Entwurf einer Welt mit ihren Inseln. Alle Tests schreiben nur in Kopien
// (t.TempDir()), nie in data/worlds/ des Repos.

const worldFixture = `{
  "id": "standard",
  "name": "Standard",
  "notes": "Fixture",
  "islands": [
    { "id": "island1", "name": "Insel 1", "depths": [0, 1, 2, 3, 4], "depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 }, "plannedChunks": { "0": [{"slot": -3, "kind": "clearing"}] }, "chunkCounts": { "0": 24 } },
    { "id": "island2", "name": "Insel 2", "depths": [0, 4], "depthScaling": { "hp": 1.6, "damage": 1.4, "speed": 1.15 } }
  ]
}
`

// island2Line ist die Zeile der zweiten Insel im Fixture (ohne Komma).
const island2Line = `{ "id": "island2", "name": "Insel 2", "depths": [0, 4], "depthScaling": { "hp": 1.6, "damage": 1.4, "speed": 1.15 } }`

func worldApp(t *testing.T) (*App, string) {
	t.Helper()
	a, path := planApp(t, worldFixture)
	a.bus = newBus()
	return a, path
}

// draftOf ist der Entwurf zum geladenen Stand, wie die Oberfläche ihn anlegt.
func draftOf(t *testing.T, a *App) (WorldDraft, string) {
	t.Helper()
	v := a.PlanLoad()
	if v.Error != "" {
		t.Fatal(v.Error)
	}
	d := WorldDraft{Name: v.Name, Notes: v.Notes, Islands: []IslandDraft{}}
	for _, i := range v.Islands {
		d.Islands = append(d.Islands, IslandDraft{From: i.ID, ID: i.ID, Name: i.Name, Depths: i.Depths, Scaling: i.Scaling})
	}
	return d, v.Stand
}

func saveDraft(t *testing.T, a *App, d WorldDraft, stand string) PlanSaved {
	t.Helper()
	res := a.PlanWorldSave(d, stand)
	if res.Error != "" {
		t.Fatal(res.Error)
	}
	if res.Hint != planRegHint {
		t.Errorf("REG-Hinweis fehlt: %+v", res)
	}
	return res
}

func TestWeltEntwurfUnveraendertIstByteGleich(t *testing.T) {
	a, path := worldApp(t)
	d, stand := draftOf(t, a)
	if c := a.PlanWorldCheck(d); !c.OK || c.Preview != worldFixture {
		t.Fatalf("Prüfung %+v", c)
	}
	if res := saveDraft(t, a, d, stand); res.Stand != stand || readFile(t, path) != worldFixture {
		t.Errorf("nicht byte-gleich: %+v\n%s", res, readFile(t, path))
	}
}

func TestWeltUmbenennenUndNotizenAendernNurDieseFelder(t *testing.T) {
	a, path := worldApp(t)
	d, stand := draftOf(t, a)
	d.Name, d.Notes = "Nordmeer", `Entwurf "kalt"`
	saveDraft(t, a, d, stand)
	want := strings.Replace(strings.Replace(worldFixture, `"name": "Standard"`, `"name": "Nordmeer"`, 1), `"notes": "Fixture"`, `"notes": "Entwurf \"kalt\""`, 1)
	if got := readFile(t, path); got != want {
		t.Errorf("Datei\n%s\nerwartet\n%s", got, want)
	}
	if v := a.PlanLoad(); v.Name != "Nordmeer" {
		t.Errorf("Name %q, die id bleibt standard", v.Name)
	}
}

func TestInselAnlegenUndReihenfolgeAendern(t *testing.T) {
	a, path := worldApp(t)
	d, stand := draftOf(t, a)
	extra := d.Islands[1]
	extra.From, extra.ID, extra.Name = "", "eisinsel", "Eisinsel"
	d.Islands = []IslandDraft{d.Islands[1], d.Islands[0], extra}
	saveDraft(t, a, d, stand)
	got := readFile(t, path)
	w, err := sim.ParseWorld([]byte(got), "standard")
	if err != nil {
		t.Fatal(err)
	}
	if ids := []string{w.Islands[0].ID, w.Islands[1].ID, w.Islands[2].ID}; strings.Join(ids, ",") != "island2,island1,eisinsel" {
		t.Errorf("Reihenfolge %v", ids)
	}
	if !strings.Contains(got, "\n    "+island2Line+",\n    { \"id\": \"island1\"") {
		t.Errorf("Inseln nicht unverändert übernommen:\n%s", got)
	}
	v := a.PlanLoad()
	if one := v.Islands[1]; len(one.Planned["0"]) != 1 || one.ChunkCounts["0"] != 24 {
		t.Errorf("Planung und Chunk-Anzahl der Insel 1 verloren: %+v", one)
	}
	if v.Islands[2].Scaling != (DraftScaling{HP: 1.6, Damage: 1.4, Speed: 1.15}) {
		t.Errorf("Skalierung der neuen Insel %+v", v.Islands[2].Scaling)
	}
}

func TestInselFelderAendernFormatStabil(t *testing.T) {
	a, path := worldApp(t)
	d, stand := draftOf(t, a)
	d.Islands[1] = IslandDraft{From: "island2", ID: "eis", Name: "Eis", Depths: []int{0, 1, 4}, Scaling: DraftScaling{HP: 2, Damage: 1.4, Speed: 1.15}}
	saveDraft(t, a, d, stand)
	line := `{ "id": "eis", "name": "Eis", "depths": [0, 1, 4], "depthScaling": { "hp": 2, "damage": 1.4, "speed": 1.15 } }`
	if want := strings.Replace(worldFixture, island2Line, line, 1); readFile(t, path) != want {
		t.Errorf("Datei\n%s", readFile(t, path))
	}
	if entries, _ := os.ReadDir(filepath.Dir(path)); len(entries) != 1 {
		t.Errorf("Temp-Datei blieb liegen: %v", entries)
	}
}

func TestInselEntfernenBehaeltDieAnderen(t *testing.T) {
	a, path := worldApp(t)
	d, stand := draftOf(t, a)
	d.Islands = d.Islands[:1]
	saveDraft(t, a, d, stand)
	if want := strings.Replace(worldFixture, ",\n    "+island2Line, "", 1); readFile(t, path) != want {
		t.Errorf("Datei\n%s", readFile(t, path))
	}
}

// Ungültige Entwürfe: abgelehnt mit Feld (Welt = -1, sonst Platz der Insel im Entwurf), die Datei bleibt.
func TestWeltEntwurfAbgelehntMitFeld(t *testing.T) {
	cases := []struct {
		name   string
		edit   func(*WorldDraft)
		island int
		field  string
		reason string
	}{
		{"doppelte id", func(d *WorldDraft) { d.Islands[1].ID = "island1" }, 1, "id", "Insel island1: doppelt"},
		{"id ungültig", func(d *WorldDraft) { d.Islands[1].ID = "Insel 2" }, 1, "id", `Insel "Insel 2": id ungültig`},
		{"unbekannte Tiefe", func(d *WorldDraft) { d.Islands[1].Depths = []int{0, 4, 5} }, 1, "depths", "unbekannte Tiefe 5"},
		{"ohne Endboss", func(d *WorldDraft) { d.Islands[0].Depths = []int{0, 1} }, 0, "depths", "kein Endboss"},
		{"Faktor 0", func(d *WorldDraft) { d.Islands[1].Scaling.HP = 0 }, 1, "depthScaling", "Faktoren > 0"},
		{"Planung ohne Tiefe", func(d *WorldDraft) { d.Islands[0].Depths = []int{1, 2, 3, 4} }, 0, "plan", "keine Tiefe der Insel"},
		{"Name leer", func(d *WorldDraft) { d.Name = "" }, -1, "name", "name ist leer"},
		{"Inselname leer", func(d *WorldDraft) { d.Islands[0].Name = "" }, 0, "name", "Insel island1"},
		{"keine Insel", func(d *WorldDraft) { d.Islands = []IslandDraft{} }, -1, "islands", "keine Insel"},
	}
	for _, c := range cases {
		a, path := worldApp(t)
		d, stand := draftOf(t, a)
		c.edit(&d)
		check := a.PlanWorldCheck(d)
		if check.OK || check.Island != c.island || check.Field != c.field || !strings.Contains(check.Error, c.reason) {
			t.Errorf("%s: Prüfung %+v", c.name, check)
		}
		if res := a.PlanWorldSave(d, stand); res.Error == "" || res.Stale {
			t.Errorf("%s: gespeichert %+v", c.name, res)
		}
		if readFile(t, path) != worldFixture {
			t.Errorf("%s: Datei geändert", c.name)
		}
	}
}

func TestWeltSpeichernAufVeraltetemStand(t *testing.T) {
	a, path := worldApp(t)
	d, stand := draftOf(t, a)
	external := strings.Replace(worldFixture, "Fixture", "von außen", 1)
	write(t, path, external)
	d.Name = "Nordmeer"
	res := a.PlanWorldSave(d, stand)
	if !res.Stale || res.Error != "data/worlds/standard.json geändert, bitte neu laden" || readFile(t, path) != external {
		t.Errorf("Stale erwartet: %+v", res)
	}
}

func TestWeltEntwurfMitUnbekannterHerkunft(t *testing.T) {
	a, _ := worldApp(t)
	d, _ := draftOf(t, a)
	d.Islands[1].From = "island1"
	if c := a.PlanWorldCheck(d); c.OK || !strings.Contains(c.Error, "zweimal") {
		t.Errorf("doppelte Herkunft: %+v", c)
	}
	d.Islands[1].From = "fehlt"
	if c := a.PlanWorldCheck(d); c.OK || !strings.Contains(c.Error, `"fehlt"`) {
		t.Errorf("unbekannte Herkunft: %+v", c)
	}
}

// Eine Welt ohne Notizen mit CRLF am Ende (wie standard.json unter Windows): Notizen werden in der Welt angelegt, eine
// zweite Insel bekommt das Zeilenende der Datei.
func TestNotizenAnlegenUndZeilenendeDerDatei(t *testing.T) {
	crlf := strings.ReplaceAll(strings.Replace(worldFixture, "  \"notes\": \"Fixture\",\n", "", 1), ",\n    "+island2Line, "")
	crlf = strings.ReplaceAll(crlf, "\n", "\r\n")
	a, path := planApp(t, crlf)
	d, stand := draftOf(t, a)
	d.Notes = "neu"
	d.Islands = append(d.Islands, IslandDraft{ID: "island2", Name: "Insel 2", Depths: []int{0, 4}, Scaling: d.Islands[0].Scaling})
	saveDraft(t, a, d, stand)
	got := readFile(t, path)
	if _, err := sim.ParseWorld([]byte(got), "standard"); err != nil {
		t.Fatal(err)
	}
	if strings.Count(got, "\n") != strings.Count(got, "\r\n") || !strings.HasSuffix(got, "}\r\n") {
		t.Errorf("Zeilenenden gemischt:\n%q", got)
	}
}
