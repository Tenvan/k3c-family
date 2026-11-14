package hud

// Werte aus data/ für die Anzeige: Skill-Katalog in der Reihenfolge der Datei (monarch.json › skills; wie
// Object.entries im Browser), Namen der Linien, Gold-Maximum, Bogen-Regal, Namen der Gebäude und Phasenzahl der
// Endbosse. Einmal beim Laden gelesen; ein Lesefehler lässt die Werte leer (die Anzeige zeigt dann weniger, nie falsch).

import (
	"bytes"
	"encoding/json"
	"log/slog"

	"k3c/data"
)

// skillInfo ist ein Eintrag aus monarch.json › skills.
type skillInfo struct {
	ID, Line, Kind string
	Tier           int
}

// lineInfo ist eine Linie aus monarch.json › lines (Reihenfolge der Datei).
type lineInfo struct{ ID, Name string }

type monarchFile struct {
	Lines  orderedJSON
	Skills orderedJSON
}

var (
	skillLines, skillCatalog = readMonarch()
	purseMaxGold             = readJSON[struct{ Purse struct{ MaxGold int } }]("economy.json").Purse.MaxGold
	buildingInfo             = readJSON[map[string]struct {
		Name    string
		BowRack int
	}]("buildings.json")
	endBossPhases = readBossPhases()
)

// readJSON liest eine Datei aus data/; Fehler landen im Log, das Ergebnis bleibt dann leer.
func readJSON[T any](name string) T {
	var out T
	raw, err := data.Files.ReadFile(name)
	if err == nil {
		err = json.Unmarshal(raw, &out)
	}
	if err != nil {
		slog.Error("💥 HUD-Werte nicht lesbar", "datei", name, "err", err)
	}
	return out
}

// orderedJSON ist ein JSON-Objekt mit seinen Schlüsseln in Datei-Reihenfolge.
type orderedJSON struct {
	keys []string
	vals []json.RawMessage
}

// UnmarshalJSON liest die Schlüssel in ihrer Reihenfolge.
func (o *orderedJSON) UnmarshalJSON(b []byte) error {
	dec := json.NewDecoder(bytes.NewReader(b))
	if _, err := dec.Token(); err != nil { // {
		return err
	}
	for dec.More() {
		t, err := dec.Token()
		if err != nil {
			return err
		}
		var v json.RawMessage
		if err := dec.Decode(&v); err != nil {
			return err
		}
		k, _ := t.(string)
		o.keys, o.vals = append(o.keys, k), append(o.vals, v)
	}
	return nil
}

// readMonarch liest Linien und Skills aus monarch.json.
func readMonarch() ([]lineInfo, []skillInfo) {
	m := readJSON[monarchFile]("monarch.json")
	lines := make([]lineInfo, 0, len(m.Lines.keys))
	for i, id := range m.Lines.keys {
		var l struct{ Name string }
		_ = json.Unmarshal(m.Lines.vals[i], &l)
		lines = append(lines, lineInfo{ID: id, Name: l.Name})
	}
	skills := make([]skillInfo, 0, len(m.Skills.keys))
	for i, id := range m.Skills.keys {
		var s skillInfo
		_ = json.Unmarshal(m.Skills.vals[i], &s)
		s.ID = id
		skills = append(skills, s)
	}
	return lines, skills
}

// readBossPhases liest die Phasenzahl je Endboss aus bosses.json.
func readBossPhases() map[string]int {
	b := readJSON[struct {
		EndBosses []struct {
			ID     string
			Phases []json.RawMessage
		}
	}]("bosses.json")
	out := map[string]int{}
	for _, e := range b.EndBosses {
		out[e.ID] = len(e.Phases)
	}
	return out
}

// lineIndex ist die Stelle der Linie in monarch.json › lines (unbekannt: hinter allen).
func lineIndex(line string) int {
	for i, l := range skillLines {
		if l.ID == line {
			return i
		}
	}
	return len(skillLines)
}

// lineName ist der Name der Linie aus den Daten, sonst ihre ID.
func lineName(line string) string {
	for _, l := range skillLines {
		if l.ID == line && l.Name != "" {
			return l.Name
		}
	}
	return line
}
