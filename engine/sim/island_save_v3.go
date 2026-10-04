package sim

import (
	"encoding/json"
	"fmt"
	"slices"
	"strings"
)

// Spielstand Version 3 (S1.4, B-022, Beschluss Q42): Fund-Pool der Insel (`skillPool`) statt eines Zählers je Stufe,
// je Spieler die gelernten Skills und die Slots. Version 2 wird überführt: Pool = Summe der Zähler, Verteilung leer.
// Der Truhen-Zähler steht nicht im Stand, er folgt aus den geöffneten Truhen aller Stufen (`chest@…`).

// islandV2 ist ein Stand der Version 2: statt des Pools ein Skill-Punkt-Zähler je Stufe.
type islandV2 struct {
	IslandSave
	StageSkillPoints []int `json:"stageSkillPoints"`
}

// decodeIsland prüft die Pflichtfelder (dazu poolField) und liest raw nach dst.
func decodeIsland(raw []byte, poolField string, dst any) error {
	var check map[string]json.RawMessage
	if err := json.Unmarshal(raw, &check); err != nil {
		return fmt.Errorf("spielstand: %w", err)
	}
	for _, f := range []string{"campaignId", "seed", "time", "stock", "stages", poolField, "players"} {
		if v, ok := check[f]; !ok || string(v) == "null" {
			return fmt.Errorf("spielstand: Feld %s fehlt", f)
		}
	}
	if err := json.Unmarshal(raw, dst); err != nil {
		return fmt.Errorf("spielstand: %w", err)
	}
	return nil
}

func parseIslandV2(raw []byte) (IslandSave, error) {
	var v islandV2
	v.Options = DefaultOptions() // Felder, die im Stand fehlen, behalten den Standard
	if err := decodeIsland(raw, "stageSkillPoints", &v); err != nil {
		return IslandSave{}, err
	}
	if len(v.StageSkillPoints) != len(v.Stages) {
		return IslandSave{}, fmt.Errorf("spielstand: %d Skill-Punkt-Zähler für %d Stufen", len(v.StageSkillPoints), len(v.Stages))
	}
	s := v.IslandSave
	s.Version = IslandSaveVersion
	for _, n := range v.StageSkillPoints { // Annahme: jeder Zähler zählte eigene Funde
		s.SkillPool += n
	}
	return s, validateIslandSave(s)
}

func parseIslandV3(raw []byte) (IslandSave, error) {
	s := IslandSave{Options: DefaultOptions()}
	if err := decodeIsland(raw, "skillPool", &s); err != nil {
		return IslandSave{}, err
	}
	return s, validateIslandSave(s)
}

// validateSkills prüft Pool und Verteilung: bekannte Skills, jeder einmal, nicht mehr als der Pool, Tier-Gating in
// Lernreihenfolge und Slots nur mit gelernten aktiven Skills.
func validateSkills(s IslandSave) error {
	if s.SkillPool < 0 {
		return fmt.Errorf("spielstand: Pool %d negativ", s.SkillPool)
	}
	for _, p := range s.Players {
		if len(p.Skills) > s.SkillPool {
			return fmt.Errorf("spielstand: Spieler %d hat %d Skills, Pool %d", p.Index, len(p.Skills), s.SkillPool)
		}
		if err := validateLearned(p); err != nil {
			return err
		}
		if err := validateSlots(p); err != nil {
			return err
		}
	}
	return nil
}

func validateLearned(p IslandPlayerSave) error {
	for i, id := range p.Skills {
		sk, ok := skillByID(id)
		switch {
		case !ok:
			return fmt.Errorf("spielstand: Spieler %d: Skill %q unbekannt", p.Index, id)
		case slices.Contains(p.Skills[:i], id):
			return fmt.Errorf("spielstand: Spieler %d: Skill %q doppelt", p.Index, id)
		case lineCount(&Player{Skills: p.Skills[:i]}, sk.Line) < tierNeed(sk.Tier):
			return fmt.Errorf("spielstand: Spieler %d: Skill %q verletzt das Tier-Gating", p.Index, id)
		}
	}
	return nil
}

func validateSlots(p IslandPlayerSave) error {
	if len(p.Slots) > 4 {
		return fmt.Errorf("spielstand: Spieler %d hat %d Slots", p.Index, len(p.Slots))
	}
	for i, id := range p.Slots {
		if id == "" {
			continue
		}
		sk, ok := skillByID(id)
		if !ok || sk.Kind != "active" || !slices.Contains(p.Skills, id) || slices.Contains(p.Slots[:i], id) {
			return fmt.Errorf("spielstand: Spieler %d: Slot %d (%q) passt nicht zu den gelernten aktiven Skills", p.Index, i+1, id)
		}
	}
	return nil
}

// restorePool setzt den Pool der Insel, spiegelt ihn in alle Stufen und leitet den Truhen-Zähler aus den geöffneten
// Truhen ab.
func restorePool(isl *Island, s IslandSave) {
	isl.SkillPool = s.SkillPool
	for _, w := range isl.Stages {
		w.SkillPoints = s.SkillPool
	}
	isl.ChestsOpened = 0
	for _, h := range s.Stages {
		for _, k := range h.PickupsTaken {
			if strings.HasPrefix(k, "chest@") {
				isl.ChestsOpened++
			}
		}
	}
}
