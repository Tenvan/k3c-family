package sim

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"reflect"
	"strings"
	"testing"

	"k3c/data"
)

// Tests zu B-680 (docs/rules/welten.md § 1): Welt-Werte erben und übersteuern.

// testWorlds liefert lookup über die Welten ws (nur ID, Extends, Overrides zählen für die Auflösung).
func testWorlds(ws ...WorldDef) func(string) (WorldDef, error) {
	return func(id string) (WorldDef, error) {
		for _, w := range ws {
			if w.ID == id {
				return w, nil
			}
		}
		return WorldDef{}, fmt.Errorf("%s: Datei fehlt", worldPath(id))
	}
}

// ov baut Overrides aus JSON je Datei.
func ov(files map[string]string) map[string]json.RawMessage {
	out := map[string]json.RawMessage{}
	for k, v := range files {
		out[k] = json.RawMessage(v)
	}
	return out
}

func baseValues() WorldValues {
	return WorldValues{Buildings: buildings, Troops: troops, Economy: economyData(economy), Hub: hub, Monarch: monarch,
		Enemies: enemyData, Waves: waves, Goals: goals}
}

func mustMarshal(t *testing.T, v any) string {
	t.Helper()
	b, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	return string(b)
}

// B-680/AC-02: Ohne Overrides gleich der Basis (auch über extends), byte-gleich als JSON.
func TestWorldValuesOhneOverrides(t *testing.T) {
	lookup := testWorlds(StandardWorld(), WorldDef{ID: "kind", Extends: StandardWorldID, Overrides: ov(map[string]string{})})
	for _, id := range []string{StandardWorldID, "kind"} {
		v, err := ResolveWorldValues(data.Files, lookup, id)
		if err != nil {
			t.Fatalf("%s: %v", id, err)
		}
		if !reflect.DeepEqual(v, baseValues()) {
			t.Errorf("%s: Wertesatz weicht von der Basis ab", id)
		}
		if got, want := mustMarshal(t, v), mustMarshal(t, baseValues()); got != want {
			t.Errorf("%s: JSON nicht byte-gleich zur Basis", id)
		}
	}
}

// B-680/AC-02: Ein Override ändert genau das genannte Feld; Einträge in Listen je Index bzw. je id.
func TestWorldValuesOverrideAendertNurFeld(t *testing.T) {
	w := WorldDef{ID: "nordmeer", Overrides: ov(map[string]string{
		"buildings": `{"wall": {"cost": {"wood": 7}, "levels": {"1": {"hp": 650}}}}`,
		"ship":      `{"stages": {"hull": {"cost": {"gold": 9}}}}`,
		"waves":     `{"perExtraPlayer": 0.37}`,
		"enemies":   `{"wolf": {"gold": [4, 6]}}`,
	})}
	v, err := ResolveWorldValues(data.Files, testWorlds(w), "nordmeer")
	if err != nil {
		t.Fatal(err)
	}
	want := baseValues()
	want.Buildings = cloneBuildings(t)
	wall := want.Buildings["wall"]
	wall.Cost.Wood = 7
	wall.Levels = append([]LevelData(nil), wall.Levels...)
	wall.Levels[1].HP = 650
	want.Buildings["wall"] = wall
	ship := want.Buildings[shipKind]
	ship.Levels = append([]LevelData(nil), ship.Levels...)
	ship.Levels[1].Cost.Gold = 9
	want.Buildings[shipKind] = ship
	want.Waves.PerExtraPlayer = 0.37
	want.Enemies = map[string]EnemyData{}
	for k, e := range enemyData {
		want.Enemies[k] = e
	}
	wolf := want.Enemies["wolf"]
	wolf.Gold = [2]int{4, 6}
	want.Enemies["wolf"] = wolf
	if got, w := mustMarshal(t, v), mustMarshal(t, want); got != w {
		t.Errorf("Wertesatz falsch:\n got %s\nwant %s", got, w)
	}
	if buildings["wall"].Cost.Wood == 7 || waves.PerExtraPlayer == 0.37 {
		t.Error("Auflösung hat die Basis verändert")
	}
}

func cloneBuildings(t *testing.T) map[string]BuildingData {
	t.Helper()
	var out map[string]BuildingData
	if err := json.Unmarshal([]byte(mustMarshal(t, buildings)), &out); err != nil {
		t.Fatal(err)
	}
	return out
}

// B-680/AC-03: Ungültige Overrides werden mit Welt, Datei und Pfad abgelehnt.
func TestWorldValuesUngueltig(t *testing.T) {
	cases := []struct{ file, json, want string }{
		{"biomes", `{}`, `overrides: unbekannte Datei "biomes"`},
		{"buildings", `{"wall": {"size": 3}}`, "overrides › data/buildings.json › wall.size: unbekanntes Feld"},
		{"buildings", `{"palast": {"hp": 3}}`, "overrides › data/buildings.json › palast: unbekannter Eintrag"},
		{"buildings", `{"wall": {"cost": {"wood": "viel"}}}`, "overrides › data/buildings.json › wall.cost.wood: Ganzzahl erwartet, ist Text"},
		{"buildings", `{"wall": {"cost": {"wood": 2.5}}}`, "wall.cost.wood: Ganzzahl erwartet, ist Zahl"},
		{"buildings", `{"wall": {"levels": [{"hp": 1}]}}`, "wall.levels: Liste von Objekten je Eintrag übersteuern"},
		{"buildings", `{"wall": {"levels": {"9": {"hp": 1}}}}`, "wall.levels.9: unbekannter Eintrag"},
		{"buildings", `{"wall": {"HP": 1}}`, `wall.HP: Schreibweise wie in der Basis: "hp"`},
		{"ship", `{"stages": {"mast": {"hp": 1}}}`, "overrides › data/ship.json › stages.mast: unbekannter Eintrag"},
		{"enemies", `{"wolf": {"gold": [1]}}`, "wolf.gold: genau 2 Werte erwartet, sind 1"},
		{"enemies", `{"wolf": {"traits": ["teleports"]}}`, `overrides: data/enemies.json: wolf: unbekanntes Trait "teleports"`},
		{"enemies", `{"caveTroll": {"tier": "standard"}}`, "overrides: data/biomes/cave.json: Portal-Pool braucht Standard- und Elite-Gegner"},
		{"waves", `{"perExtraPlayer": -1}`, "overrides: data/waves.json: perExtraPlayer und bossHpPerExtraPlayer dürfen nicht negativ sein"},
		{"waves", `{"bossHpPerExtraPlayer": null}`, "bossHpPerExtraPlayer: Zahl erwartet, ist null"},
		{"monarch", `{"presets": {"tank": {"activeSkills": ["fliegen"]}}}`, "overrides: data/monarch.json: preset tank nennt unbekannten Skill fliegen"},
		{"goals", `7`, "overrides › data/goals.json › Objekt erwartet, ist Zahl"},
	}
	for _, c := range cases {
		w := WorldDef{ID: "nordmeer", Overrides: ov(map[string]string{c.file: c.json})}
		_, err := ResolveWorldValues(data.Files, testWorlds(w), "nordmeer")
		if err == nil || !strings.HasPrefix(err.Error(), "data/worlds/nordmeer.json: ") || !strings.Contains(err.Error(), c.want) {
			t.Errorf("%s %s: Fehler %v, erwartet …%s…", c.file, c.json, err, c.want)
		}
	}
}

// B-680/AC-05: Kette Basis → A → B; das Kind gewinnt, Geerbtes bleibt, der Fehler nennt die auslösende Welt.
func TestWorldValuesKette(t *testing.T) {
	a := WorldDef{ID: "a", Overrides: ov(map[string]string{"goals": `{"gold": 111, "days": 5}`})}
	b := WorldDef{ID: "b", Extends: "a", Overrides: ov(map[string]string{"goals": `{"days": 9}`})}
	v, err := ResolveWorldValues(data.Files, testWorlds(a, b), "b")
	if err != nil {
		t.Fatal(err)
	}
	if v.Goals != (Goals{Gold: 111, Days: 9}) {
		t.Errorf("Goals = %+v, erwartet Gold 111 aus a, Days 9 aus b", v.Goals)
	}
	bad := WorldDef{ID: "a", Overrides: ov(map[string]string{"waves": `{"perExtraPlayer": -2}`})}
	if _, err := ResolveWorldValues(data.Files, testWorlds(bad, b), "b"); err == nil || !strings.HasPrefix(err.Error(), "data/worlds/a.json: ") {
		t.Errorf("Fehler der Eltern-Welt: %v", err)
	}
}

// B-680/AC-05: Zyklus, unbekannte Eltern-Welt und zu tiefe Kette werden abgelehnt.
func TestWorldValuesKetteAbgelehnt(t *testing.T) {
	chain := func(ids ...string) []WorldDef {
		var ws []WorldDef
		for i, id := range ids {
			w := WorldDef{ID: id}
			if i+1 < len(ids) {
				w.Extends = ids[i+1]
			}
			ws = append(ws, w)
		}
		return ws
	}
	cases := []struct {
		name   string
		worlds []WorldDef
		want   string
	}{
		{"Zyklus", []WorldDef{{ID: "b", Extends: "a"}, {ID: "a", Extends: "b"}}, "data/worlds/b.json: extends: Zyklus b → a → b"},
		{"unbekannt", []WorldDef{{ID: "b", Extends: "fehlt"}}, `data/worlds/b.json: extends: Eltern-Welt "fehlt" unbekannt`},
		{"zu tief", chain("e", "d", "c", "b", "a"), "data/worlds/e.json: extends: Kette zu lang (e → d → c → b → a, höchstens 4 Welten)"},
	}
	for _, c := range cases {
		lookup := testWorlds(c.worlds...)
		_, err := ResolveWorldValues(data.Files, lookup, c.worlds[0].ID)
		if err == nil || !strings.Contains(err.Error(), c.want) {
			t.Errorf("%s: Fehler %v, erwartet …%s…", c.name, err, c.want)
		}
	}
	if _, err := ResolveWorldValues(data.Files, testWorlds(chain("d", "c", "b", "a")...), "d"); err != nil {
		t.Errorf("Kette aus 4 Welten abgelehnt: %v", err)
	}
}

// B-680/AC-05: Löschen oder Umbenennen einer Welt mit Kind-Welten wird verweigert, die Meldung nennt die Kinder.
func TestWorldRemovable(t *testing.T) {
	all := []WorldDef{{ID: "a"}, {ID: "c", Extends: "a"}, {ID: "b", Extends: "a"}, {ID: "d", Extends: "b"}}
	err := CheckWorldRemovable(all, "a")
	if err == nil || err.Error() != "data/worlds/a.json: Welt hat Kind-Welten (b, c), Löschen und Umbenennen abgelehnt" {
		t.Errorf("a: %v", err)
	}
	if err := CheckWorldRemovable(all, "d"); err != nil {
		t.Errorf("d ohne Kinder: %v", err)
	}
}

// Die Welt-Datei liest extends und overrides streng (welten.md § 2).
func TestParseWorldExtendsOverrides(t *testing.T) {
	raw, err := fs.ReadFile(data.Files, "worlds/standard.json")
	if err != nil {
		t.Fatal(err)
	}
	with := func(extra string) []byte {
		var m map[string]json.RawMessage
		if err := json.Unmarshal(raw, &m); err != nil {
			t.Fatal(err)
		}
		var add map[string]json.RawMessage
		if err := json.Unmarshal([]byte(extra), &add); err != nil {
			t.Fatal(err)
		}
		for k, v := range add {
			m[k] = v
		}
		m["id"] = json.RawMessage(`"kind"`)
		b, _ := json.Marshal(m)
		return b
	}
	w, err := ParseWorld(with(`{"extends": "standard", "overrides": {"goals": {"days": 3}}}`), "kind")
	if err != nil || w.Extends != StandardWorldID || string(w.Overrides["goals"]) != `{"days":3}` {
		t.Fatalf("ParseWorld: %+v, %v", w.Overrides, err)
	}
	for extra, want := range map[string]string{
		`{"extends": "kind"}`:           "extends: Welt erbt von sich selbst",
		`{"extends": "Nord Meer"}`:      `extends: Welt-ID "Nord Meer" ungültig`,
		`{"overrides": {"biomes": {}}}`: `overrides: unbekannte Datei "biomes"`,
		`{"overrides": [1]}`:            "kein gültiges JSON",
		`{"extends": 3}`:                "kein gültiges JSON",
	} {
		if _, err := ParseWorld(with(extra), "kind"); err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%s: Fehler %v, erwartet …%s…", extra, err, want)
		}
	}
	if _, err := ParseWorld(raw, StandardWorldID); err != nil {
		t.Errorf("Standard-Welt: %v", err)
	}
}
