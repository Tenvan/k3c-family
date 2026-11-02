package sim

import (
	"encoding/json"
	"io/fs"
	"maps"
	"math"
	"strings"
	"testing"
	"testing/fstest"

	"k3c/data"
	"k3c/engine/level"
)

// Nachträge aus Review BAL8.2 (B-692): Skill-Katalog, Spielstand-Prüfung, Ruinen-HP und gesperrte Felder je Welt,
// UseData lädt frisch (samt goals.json und Skill-Katalog), ResolveWorldFiles prüft auch die Basis, Test-Gerüst ohne
// Allokation je Aufruf.

// overrideWorld erbt von der Standard-Welt und übersteuert mit ov (Schlüssel = Basis-Datei).
func overrideWorld(t *testing.T, id string, ov map[string]string) WorldDef {
	t.Helper()
	w := StandardWorld()
	w.ID, w.Name, w.Extends, w.Overrides = id, id, StandardWorldID, map[string]json.RawMessage{}
	for k, v := range ov {
		w.Overrides[k] = json.RawMessage(v)
	}
	return need(ResolveWorld(w, standardLookup))(t)
}

// Punkt 1: Overrides auf `monarch.skills.*` wirken im Skill-Katalog der Welt (CanLearn), die Basis bleibt.
func TestB692SkillOverrideWirktJeWelt(t *testing.T) {
	world := overrideWorld(t, "skill", map[string]string{"monarch": `{"skills": {"ironWall": {"tier": 1}}}`})
	isl := need(CreateIslandIn(world, "skill", world.Islands[0].Depths, 1, CreateOptions{}))(t)
	w := isl.Stages[0]
	p := AddIslandPlayer(isl, 0)
	addPoolPoints(w, 1)
	if s, _ := w.values().skillByID("ironWall"); s.Tier != 1 {
		t.Fatalf("Welt: ironWall Tier %d, erwartet 1", s.Tier)
	}
	if err := CanLearn(w, p, "ironWall"); err != nil {
		t.Errorf("Welt mit Tier 1: %v", err)
	}
	if s, _ := defaultValues().skillByID("ironWall"); s.Tier != 3 {
		t.Errorf("Basis: ironWall Tier %d, erwartet 3", s.Tier)
	}
	base := quietWorld(t)
	bp := AddPlayer(base)
	addPoolPoints(base, 1)
	if CanLearn(base, bp, "ironWall") == nil {
		t.Error("Basis: ironWall ohne Tier-Gating lernbar")
	}
}

// Punkt 2: Der Spielstand wird gegen den Wertesatz seiner Welt geprüft (Tier-Gating aus monarch.tierPoints).
func TestB692SpielstandPruefungJeWelt(t *testing.T) {
	world := overrideWorld(t, "leicht", map[string]string{"monarch": `{"tierPoints": [0, 0, 0, 0]}`})
	isl := need(CreateIslandIn(world, "leicht", world.Islands[0].Depths, 1, CreateOptions{}))(t)
	p := AddIslandPlayer(isl, 0)
	addPoolPoints(isl.Stages[0], 1)
	if err := LearnSkill(isl.Stages[0], p, "ironWall"); err != nil {
		t.Fatal(err)
	}
	s := need(ParseIslandSave(need(json.Marshal(isl.ToSave("")))(t)))(t) // Parsen prüft ohne Welt
	if _, err := FromIslandSaveIn(s, 1, world); err != nil {
		t.Errorf("Welt mit tierPoints 0: %v", err)
	}
	if _, err := FromIslandSave(s, 1); err == nil || !strings.Contains(err.Error(), "Tier-Gating") {
		t.Errorf("Standard-Welt: Tier-Gating erwartet, war %v", err)
	}
}

// Punkt 2: validateEconomy prüft die Rüstungsstufen gegen den übergebenen Wertesatz, nicht gegen die Basis.
func TestB692RuestungsstufeJeWertesatz(t *testing.T) {
	v := *defaultValues()
	v.Buildings = maps.Clone(v.Buildings)
	armory := v.Buildings["armory"]
	armory.Armor = armory.Armor[:1]
	v.Buildings["armory"] = armory
	h := HubSave{ArmorLevel: 2}
	if validateEconomy(&v, h) == nil {
		t.Error("Wertesatz mit 1 Rüstungsstufe: Stufe 2 angenommen")
	}
	if err := validateEconomy(defaultValues(), h); err != nil {
		t.Errorf("Basis mit 2 Rüstungsstufen: %v", err)
	}
}

// Punkt 3: Die Start-HP einer Ruine folgen den HP der Art im Wertesatz der Welt.
func TestB692RuinenHPJeWelt(t *testing.T) {
	world := overrideWorld(t, "ruine", map[string]string{"buildings": `{"smithy": {"hp": 400}}`})
	isl := need(CreateIslandIn(world, "ruine", world.Islands[0].Depths, 1, CreateOptions{}))(t)
	want := math.Ceil(level.DefaultRuinRules().HPShare * 400)
	if got := ruinSite(isl.Stages[0], "smithy", 10).HP; got != want {
		t.Errorf("Ruine smithy: HP %v, erwartet %v", got, want)
	}
	if got := ruinSite(quietWorld(t), "smithy", 10).HP; got != level.RuinHP(buildings["smithy"].HP) {
		t.Errorf("Basis: HP %v", got)
	}
}

// Punkt 3: Felder, die der Level-Generator aus der Basis liest, sind nicht übersteuerbar (welten.md § 1.3).
func TestB692GesperrteFelder(t *testing.T) {
	cases := map[string][2]string{
		"economy.payRangeUnits":   {"economy", `{"payRangeUnits": 3}`},
		"hub.sitePlacement":       {"hub", `{"sitePlacement": {"zone1WallUnits": 5}}`},
		"hub.sites":               {"hub", `{"sites": {"0": {"offsetUnits": 4}}}`},
		"nursery.plantation":      {"buildings", `{"nursery": {"plantation": {"stripUnits": 5}}}`},
		"buildings.nursery.hp ok": {"buildings", `{"nursery": {"hp": 120, "plantation": {"slots": 4}}}`},
	}
	for name, c := range cases {
		w := StandardWorld()
		w.ID, w.Extends, w.Overrides = "gesperrt", StandardWorldID, map[string]json.RawMessage{c[0]: json.RawMessage(c[1])}
		_, err := ResolveWorldValues(data.Files, testWorlds(StandardWorld(), w), "gesperrt")
		if strings.HasSuffix(name, " ok") {
			if err != nil {
				t.Errorf("%s: %v", name, err)
			}
			continue
		}
		want := "nicht übersteuerbar"
		if err == nil || !strings.Contains(err.Error(), want) || !strings.Contains(err.Error(), "data/worlds/gesperrt.json") {
			t.Errorf("%s: Ablehnung mit Pfad erwartet, war %v", name, err)
		}
	}
}

// memData ist eine Kopie der eingebetteten Daten zum Ändern.
func memData(t *testing.T) fstest.MapFS {
	t.Helper()
	mem := fstest.MapFS{}
	if err := fs.WalkDir(data.Files, ".", func(p string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		raw, err := data.Files.ReadFile(p)
		mem[p] = &fstest.MapFile{Data: raw}
		return err
	}); err != nil {
		t.Fatal(err)
	}
	return mem
}

// replaceIn ersetzt old in der Datei name genau einmal.
func replaceIn(t *testing.T, mem fstest.MapFS, name, old, repl string) {
	t.Helper()
	s := string(mem[name].Data)
	if !strings.Contains(s, old) {
		t.Fatalf("%s: %q fehlt", name, old)
	}
	mem[name].Data = []byte(strings.Replace(s, old, repl, 1))
}

// Punkt 4: UseData lädt in frische Werte (ein vorher geholter Wertesatz bleibt), samt goals.json und Skill-Katalog.
func TestB692UseDataFrischMitGoalsUndSkills(t *testing.T) {
	t.Cleanup(func() {
		if err := UseData(data.Files); err != nil {
			t.Fatal(err)
		}
	})
	before := defaultValues()
	wallHP, gold := before.Buildings["wall"].HP, before.Goals.Gold
	mem := memData(t)
	replaceIn(t, mem, "goals.json", `"gold": 66`, `"gold": 77`)
	replaceIn(t, mem, "monarch.json", `"ironWall":         { "line": "tank",   "tier": 3`, `"ironWall":         { "line": "tank",   "tier": 1`)
	replaceIn(t, mem, "buildings.json", `"wall":`, `"wallX": {"hp": 1}, "wall":`)
	if err := UseData(mem); err != nil {
		t.Fatal(err)
	}
	now := defaultValues()
	if now.Goals.Gold != 77 || gold != 66 {
		t.Errorf("goals.json: Gold %d (vorher %d), erwartet 77", now.Goals.Gold, gold)
	}
	if s, _ := now.skillByID("ironWall"); s.Tier != 1 {
		t.Errorf("Skill-Katalog nicht neu geladen: Tier %d", s.Tier)
	}
	if _, ok := before.Buildings["wallX"]; ok || before.Buildings["wall"].HP != wallHP {
		t.Error("UseData hat die Maps eines vorher geholten Wertesatzes geändert")
	}
	if s, _ := before.skillByID("ironWall"); s.Tier != 3 {
		t.Errorf("Skill-Katalog des alten Wertesatzes geändert: Tier %d", s.Tier)
	}
}

// Punkt 5: ResolveWorldFiles mit leerer ID prüft die Basis wie ResolveWorldValues.
func TestB692ResolveWorldFilesPrueftBasis(t *testing.T) {
	if _, err := ResolveWorldFiles(data.Files, testWorlds(StandardWorld()), ""); err != nil {
		t.Fatal(err)
	}
	mem := memData(t)
	replaceIn(t, mem, "waves.json", `"perExtraPlayer":`, `"perExtraPlayer": -1, "x":`)
	if _, err := ResolveWorldFiles(mem, testWorlds(StandardWorld()), ""); err == nil {
		t.Error("kaputte Basis (perExtraPlayer < 0) ohne Fehler")
	}
}

// Punkt 6: Das Test-Gerüst `&World{}` merkt sich den Basis-Wertesatz, keine Allokation je Aufruf.
func TestB692WertesatzOhneCreateWorldGemerkt(t *testing.T) {
	w := &World{}
	if first := w.values(); first != w.values() {
		t.Error("values() liefert je Aufruf einen neuen Wertesatz")
	}
	if n := testing.AllocsPerRun(10, func() { _ = w.values() }); n != 0 {
		t.Errorf("values(): %v Allokationen je Aufruf", n)
	}
}
