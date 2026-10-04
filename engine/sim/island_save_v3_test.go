package sim

import (
	"bytes"
	"encoding/json"
	"os"
	"path/filepath"
	"slices"
	"testing"
)

// Spielstand Version 3 (S1.4, B-022): Pool, Verteilung und Slots im Stand, Migration aus 1 und 2, Fehlerfälle.

// fixtureMap liest ein Fixture als JSON-Baum, damit Tests einzelne Felder ändern können.
func fixtureMap(t *testing.T, version int) map[string]any {
	t.Helper()
	var m map[string]any
	if err := json.Unmarshal(readFixture(t, version), &m); err != nil {
		t.Fatal(err)
	}
	return m
}

func mapJSON(t *testing.T, m map[string]any) []byte {
	t.Helper()
	raw, err := json.Marshal(m)
	if err != nil {
		t.Fatal(err)
	}
	return raw
}

// (a) Pool, Verteilung und Slots zweier Spieler bleiben erhalten; Speichern, Laden, Speichern ist gleich.
func TestIslandSaveV3Rundlauf(t *testing.T) {
	isl := mustIsland(t, "stand-v3", []int{0, 1})
	a, b := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 1)
	addPoolPoints(isl.Stages[0], 4)
	mustLearn(t, isl.Stages[0], a, "taunt", "shieldBash", "armorAura")
	mustLearn(t, isl.Stages[1], b, "heal", "fireball")
	b.Slots[0] = "" // freier Slot bleibt frei

	first := saveJSON(t, isl.ToSave("2026-10-04T10:00:00Z"))
	loaded := loadIsland(t, first)
	if second := saveJSON(t, loaded.ToSave("2026-10-04T10:00:00Z")); !bytes.Equal(first, second) {
		t.Fatalf("Speichern, Laden, Speichern ergibt etwas anderes:\n%s\n%s", first, second)
	}
	if loaded.SkillPool != 4 || loaded.Stages[1].SkillPoints != 4 {
		t.Fatalf("Pool %d, Stufe 1 %d", loaded.SkillPool, loaded.Stages[1].SkillPoints)
	}
	for i, p := range isl.Players() {
		q := loaded.Players()[i]
		if !slices.Equal(q.Skills, p.Skills) || !slices.Equal(q.Slots, p.Slots) {
			t.Errorf("Spieler %d: Skills %v/%v, Slots %q/%q", p.Index, q.Skills, p.Skills, q.Slots, p.Slots)
		}
	}
	if q := loaded.Players()[1]; AvailablePoints(loaded.Stages[1], q) != 2 {
		t.Fatalf("frei nach dem Laden: %d", AvailablePoints(loaded.Stages[1], q))
	}
}

// (b) v1 und v2 laden mit leerer Verteilung, der Pool folgt der Migrationsregel (Summe der Zähler je Stufe).
func TestIslandSaveV3Migration(t *testing.T) {
	v2 := fixtureMap(t, 2)
	stages := v2["stages"].([]any)
	third := map[string]any{}
	_ = json.Unmarshal(mapJSON(t, stages[1].(map[string]any)), &third)
	third["depth"] = 2
	v2["stages"] = append(stages, third)
	v2["stageSkillPoints"] = []int{0, 2, 1}
	for name, c := range map[string]struct {
		raw  []byte
		pool int
	}{"v1": {readFixture(t, 1), 0}, "v2": {readFixture(t, 2), 2}, "v2 mit drei Stufen": {mapJSON(t, v2), 3}} {
		s, err := ParseIslandSave(c.raw)
		if err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		isl, err := FromIslandSave(s, islandSpeed)
		if err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		if s.SkillPool != c.pool || isl.SkillPool != c.pool || isl.Stages[0].SkillPoints != c.pool {
			t.Errorf("%s: Pool %d/%d, erwartet %d", name, s.SkillPool, isl.SkillPool, c.pool)
		}
		for _, p := range isl.Players() {
			if len(p.Skills) != 0 || len(p.Slots) != 0 {
				t.Errorf("%s: Spieler %d hat Verteilung %v %v (Startwerte: leer)", name, p.Index, p.Skills, p.Slots)
			}
		}
	}
}

// (c) Das Fixture v3 lädt mit Pool und Verteilung.
func TestIslandSaveV3FixtureLaedt(t *testing.T) {
	isl := loadIsland(t, readFixture(t, 3))
	ps := isl.Players()
	if isl.SkillPool != 5 || len(ps) != 2 || !slices.Equal(ps[0].Skills, []string{"taunt", "shieldBash", "armorAura"}) ||
		!slices.Equal(ps[1].Slots, []string{"fireball", "iceWall"}) {
		t.Fatalf("Pool %d, Spieler %+v", isl.SkillPool, ps)
	}
}

// (d) Ungültige Verteilungen ergeben einen klaren Fehler; die Quelldatei bleibt Byte für Byte gleich.
func TestIslandSaveV3Fehler(t *testing.T) {
	player := func(m map[string]any, i int) map[string]any { return m["players"].([]any)[i].(map[string]any) }
	cases := map[string]func(map[string]any){
		"unbekannter Skill":    func(m map[string]any) { player(m, 0)["skills"] = []any{"taunt", "backstab"} },
		"doppelter Skill":      func(m map[string]any) { player(m, 1)["skills"] = []any{"fireball", "fireball"} },
		"mehr Skills als Pool": func(m map[string]any) { m["skillPool"] = 2 },
		"Gating verletzt":      func(m map[string]any) { player(m, 0)["skills"] = []any{"taunt", "armorAura"} },
		"Slot nicht gelernt":   func(m map[string]any) { player(m, 1)["slots"] = []any{"fireball", "heal"} },
		"Passiv im Slot":       func(m map[string]any) { player(m, 0)["slots"] = []any{"armorAura"} },
		"Pool fehlt":           func(m map[string]any) { delete(m, "skillPool") },
	}
	for name, mutate := range cases {
		m := fixtureMap(t, 3)
		mutate(m)
		path := filepath.Join(t.TempDir(), "kaputt.json")
		raw := mapJSON(t, m)
		if err := os.WriteFile(path, raw, 0o600); err != nil {
			t.Fatal(err)
		}
		src, _ := os.ReadFile(path)
		if _, err := ParseIslandSave(src); err == nil {
			t.Errorf("%s: kein Fehler", name)
		}
		if after, _ := os.ReadFile(path); !bytes.Equal(after, raw) {
			t.Errorf("%s: Quelldatei verändert", name)
		}
	}
}

// (e) Der Truhen-Zähler folgt aus den geöffneten Truhen des Stands: nach zwei Truhen gibt die dritte den Punkt.
func TestIslandSaveV3TruhenZaehler(t *testing.T) {
	s, err := ParseIslandSave(readFixture(t, 3))
	if err != nil {
		t.Fatal(err)
	}
	base := mustIsland(t, s.Seed, []int{0, 1})
	opened := 0
	for i, w := range base.Stages {
		for _, pk := range w.Pickups {
			if pk.Kind == "chest" && opened < 2 {
				s.Stages[i].PickupsTaken = append(s.Stages[i].PickupsTaken, key(pk.Kind, pk.X))
				opened++
			}
		}
	}
	if opened != 2 {
		t.Fatalf("nur %d Truhen in den Stufen", opened)
	}
	isl, err := FromIslandSave(s, islandSpeed)
	if err != nil {
		t.Fatal(err)
	}
	if isl.ChestsOpened != 2 {
		t.Fatalf("Truhen-Zähler %d, erwartet 2", isl.ChestsOpened)
	}
	w, p := isl.Stages[0], isl.Stages[0].Players[0]
	w.Pickups = append(w.Pickups, &Pickup{ID: w.newID(), Kind: "chest", X: p.X})
	StepIsland(isl, []PlayerCommand{{}, {}}, dt)
	if isl.ChestsOpened != 3 || isl.SkillPool != 6 || isl.Stages[1].SkillPoints != 6 {
		t.Fatalf("3. Truhe nach dem Laden: geöffnet %d, Pool %d", isl.ChestsOpened, isl.SkillPool)
	}
}

// (f) Wer nach dem Laden beitritt, hat alle Pool-Punkte für sich (Preset plus frei).
func TestIslandSaveV3SpaeterBeitritt(t *testing.T) {
	isl := loadIsland(t, readFixture(t, 3))
	p := AddIslandPlayer(isl, 0)
	if p.Index != 2 || len(p.Skills)+AvailablePoints(isl.Stages[0], p) != isl.SkillPool {
		t.Fatalf("Spieler %d: Skills %v, frei %d, Pool %d", p.Index, p.Skills, AvailablePoints(isl.Stages[0], p), isl.SkillPool)
	}
}
