package sim

import (
	"encoding/json"
	"testing"
)

// Spielstand der Insel (SP12.3): Rundlauf, Stände der Version 1 und Fehlerfälle.

func saveJSON(t *testing.T, s IslandSave) []byte {
	t.Helper()
	b, err := json.Marshal(s)
	if err != nil {
		t.Fatal(err)
	}
	return b
}

func loadIsland(t *testing.T, raw []byte) *Island {
	t.Helper()
	s, err := ParseIslandSave(raw)
	if err != nil {
		t.Fatal(err)
	}
	isl, err := FromIslandSave(s, islandSpeed)
	if err != nil {
		t.Fatal(err)
	}
	return isl
}

func TestIslandSaveRoundTrip(t *testing.T) {
	isl := mustIsland(t, "stand-a", []int{0, 1, 2})
	a, b, c := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 2), AddIslandPlayer(isl, 0)
	a.Gold, b.Gold, c.Gold = 61, 22, 5
	builtSite(t, isl.Stages[0], "wall").HP = 123
	addPoolPoints(isl.Stages[1], 3) // Pool der Insel, alle Stufen spiegeln ihn (S1.4)
	addStock(isl.Stages[0], "wood", 40)
	addStock(isl.Stages[2], "copper", 9)
	runIsland(isl, []PlayerCommand{{MoveX: 1}, {}, {}}, 20)

	first := saveJSON(t, isl.ToSave("2026-10-02T10:00:00Z"))
	loaded := loadIsland(t, first)
	second := saveJSON(t, loaded.ToSave("2026-10-02T10:00:00Z"))
	if string(first) != string(second) {
		t.Fatalf("Speichern, Laden, Speichern ergibt etwas anderes:\n%s\n%s", first, second)
	}
	for i, p := range isl.Players() {
		q := loaded.Players()[i]
		if q.Index != p.Index || q.Gold != p.Gold || loaded.StageOf(q.Index) != isl.StageOf(p.Index) {
			t.Errorf("Spieler %d: Index/Gold/Stufe weichen ab", p.Index)
		}
	}
	if loaded.Stock.Wood != isl.Stock.Wood || loaded.Stock.Copper != 9 ||
		loaded.SkillPool != isl.SkillPool || loaded.SkillPool < 3 || loaded.Stages[0].SkillPoints != loaded.SkillPool {
		t.Errorf("Vorrat oder Pool weichen ab: %+v, Pool %d (%d)", *loaded.Stock, loaded.SkillPool, isl.SkillPool)
	}
	for _, w := range loaded.Stages {
		if w.Stock != loaded.Stock {
			t.Error("geladene Stufen müssen den Vorrat der Insel teilen")
		}
	}
	runIsland(loaded, []PlayerCommand{{}, {}, {}}, 5) // läuft nach dem Laden weiter
	if loaded.Stages[0].Time <= isl.Stages[0].Time-1 {
		t.Errorf("Zeit nach dem Laden zu klein: %v", loaded.Stages[0].Time)
	}
}

// v1Save erzeugt einen Stand der Campaign (Version 1): zwei Spieler, in der Höhle, Vorräte in beiden Hubs.
func v1Save(t *testing.T) []byte {
	t.Helper()
	c := CreateCampaign("alt-a", "alt-1", islandSpeed)
	c.JoinPlayer()
	c.JoinPlayer()
	w := c.CurrentWorld()
	addStock(w, "wood", 30)
	for range 600 {
		Step(w, nil, dt)
	}
	w = c.Travel(1)
	w.Players[0].Gold, w.Players[1].Gold = 41, 17
	addStock(w, "stone", 12)
	old, err := json.Marshal(c.ToSave("2026-10-01T09:00:00Z"))
	if err != nil {
		t.Fatal(err)
	}
	return old
}

func TestParseIslandSaveFromVersion1(t *testing.T) {
	old := v1Save(t)
	if _, err := ParseSave(old); err != nil {
		t.Fatalf("Stand der Version 1 muss für den Raum lesbar bleiben: %v", err)
	}
	isl := loadIsland(t, old)
	if len(isl.Stages) != 3 || isl.Stages[0].Biome.Depth != 0 || isl.Stages[1].Biome.Depth != 1 || isl.Stages[2].Biome.Depth != 2 {
		t.Fatalf("drei Stufen (Tiefe 0, 1, 2) erwartet, bekam %d", len(isl.Stages))
	}
	if isl.ID != "alt-1" || isl.Seed != "alt-a" {
		t.Errorf("ID und Seed bleiben: %q, %q", isl.ID, isl.Seed)
	}
	if isl.Stock.Wood != 30 || isl.Stock.Stone != 12 {
		t.Errorf("Vorräte der Hubs zählen zusammen: %+v", *isl.Stock)
	}
}

func TestVersion1PlayersMoveIntoStoredStage(t *testing.T) {
	isl := loadIsland(t, v1Save(t))
	got := isl.Players()
	if len(got) != 2 || got[0].Gold != 41 || got[1].Gold != 17 {
		t.Errorf("Gold der Spieler stimmt nicht: %+v", got)
	}
	if isl.StageOf(0) != 1 || isl.StageOf(1) != 1 {
		t.Errorf("beide Spieler stehen in der Höhle (Stufe 1): %d, %d", isl.StageOf(0), isl.StageOf(1))
	}
}

func TestParseIslandSaveRejectsBadInput(t *testing.T) {
	good := mustIsland(t, "stand-b", []int{0, 1})
	AddIslandPlayer(good, 0)
	var m map[string]any
	if err := json.Unmarshal(saveJSON(t, good.ToSave("x")), &m); err != nil {
		t.Fatal(err)
	}
	cases := map[string]func(map[string]any){
		"Version":           func(m map[string]any) { m["version"] = 9 },
		"Pflichtfeld":       func(m map[string]any) { delete(m, "stock") },
		"Spieler ohne Stufe": func(m map[string]any) { m["players"] = []any{map[string]any{"index": 0, "gold": 1, "depth": 2}} },
		"doppelter Index":   func(m map[string]any) { m["players"] = []any{map[string]any{"index": 0, "gold": 1, "depth": 0}, map[string]any{"index": 0, "gold": 1, "depth": 0}} },
		"unbekannte Tiefe":  func(m map[string]any) { m["stages"].([]any)[0].(map[string]any)["depth"] = 9 },
	}
	for name, mutate := range cases {
		cp := map[string]any{}
		raw, _ := json.Marshal(m)
		_ = json.Unmarshal(raw, &cp)
		mutate(cp)
		bad, _ := json.Marshal(cp)
		if _, err := ParseIslandSave(bad); err == nil {
			t.Errorf("%s: Fehler erwartet", name)
		}
	}
	if _, err := ParseIslandSave([]byte("kein json")); err == nil {
		t.Error("kein JSON: Fehler erwartet")
	}
}

func TestVersion2IsNotReadByCampaignParser(t *testing.T) {
	isl := mustIsland(t, "stand-c", []int{0})
	if _, err := ParseSave(saveJSON(t, isl.ToSave("x"))); err == nil {
		t.Error("ParseSave (Raum) liest nur Version 1 und muss einen Stand der Version 2 ablehnen")
	}
}
