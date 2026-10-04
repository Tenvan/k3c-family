package sim

import (
	"maps"
	"slices"
	"sort"
)

// Lese-Typen für data/monarch.json (docs/rules/monarch.md §§ 1–3). Wie in data.go ohne Tags (Groß-/Kleinschreibung egal).

// SkillData ist ein Eintrag aus monarch.json › skills; ID ist der Schlüssel. Kind: active oder passive.
type SkillData struct {
	ID, Line, Kind string
	Tier           int
	Cooldown       float64 // Sekunden, nur aktive Skills
	Effect         skillEffect
}

// skillEffect ist die Wirkung eines aktiven Skills; Type wählt die Funktion in skills.go, die übrigen Felder sind
// Parameter je Typ (Units, Sekunden, HP).
type skillEffect struct {
	Type                         string
	Radius, Range, Duration, HP float64
}

// presetData ist eine Startverteilung aus monarch.json › presets (Basiswerte liest der Code nicht, S1.1).
type presetData struct {
	Name         string
	ActiveSkills []string
}

type monarchData struct {
	Base                                           struct{ HP, Damage, Speed, Defense float64 }
	SprintMultiplier, Acceleration, RespawnSeconds float64
	Attack                                         struct{ Damage, Range, Cooldown float64 }
	// TierPoints: Tier n verlangt TierPoints[n-1] gelernte Skills der Linie (Beschluss 🧑 2026-10-04, B-216).
	TierPoints []int
	// SkillPointSources: Punkte je Quelle; ChestEvery = jede n-te Truhe der Insel gibt einen Punkt.
	SkillPointSources struct{ Hidden, Miniboss, Endboss, Milestone, ChestEvery int }
	Lines             map[string]struct{ Name string }
	Skills            map[string]SkillData
	Presets           map[string]presetData
}

// skillCatalog ist monarch.json › skills als nach ID sortierte Liste (keine Map-Iteration im Sim-Code).
var skillCatalog = sortedSkills(monarch.Skills)

func sortedSkills(m map[string]SkillData) []SkillData {
	out := make([]SkillData, 0, len(m))
	for _, id := range slices.Sorted(maps.Keys(m)) {
		s := m[id]
		s.ID = id
		out = append(out, s)
	}
	return out
}

// skillByID sucht einen Skill im Katalog.
func skillByID(id string) (SkillData, bool) {
	i := sort.Search(len(skillCatalog), func(i int) bool { return skillCatalog[i].ID >= id })
	if i < len(skillCatalog) && skillCatalog[i].ID == id {
		return skillCatalog[i], true
	}
	return SkillData{}, false
}
