package room

import (
	"slices"
	"testing"

	"k3c/engine/sim"
)

// S2.1 (B-123): Schlag, Skill-Slot, Lernen und Respec über den Raum, zwei Spieler an einem Gerät.

// (a) attack und skill kommen als PlayerCommand beim Monarchen des Slots an, der andere Slot bleibt leer.
func TestSkillsEingabeKommtAn(t *testing.T) {
	r, p, _ := devRoom(t)
	ok(t, r.Input("a", p, map[int]sim.PlayerCommand{1: {Attack: true, Skill: 2}}))
	d := r.devices["a"]
	if got := r.monarchs[d.slots[1]].input; !got.Attack || got.Skill != 2 {
		t.Fatalf("Slot 1: %+v", got)
	}
	if got := r.monarchs[d.slots[0]].input; got.Attack || got.Skill != 0 {
		t.Fatalf("Slot 0 verändert: %+v", got)
	}
}

// (c, e) Lernen kostet einen Pool-Punkt und betrifft nur den Monarchen des Slots; Fehler sind bad_request ohne Änderung.
func TestSkillsLernen(t *testing.T) {
	r, p, _ := devRoom(t)
	d := r.devices["a"]
	m0, m1 := islPlayer(t, r, d.slots[0]), islPlayer(t, r, d.slots[1])
	r.isl.SkillPool = len(m1.Skills) + 1 // genau ein freier Punkt für Slot 1
	w := r.isl.Stages[r.isl.StageOf(m1.Index)]
	before0 := slices.Clone(m0.Skills)
	ok(t, r.Learn("a", p, 1, "fireball"))
	if !slices.Contains(m1.Skills, "fireball") || sim.AvailablePoints(w, m1) != 0 {
		t.Fatalf("Slot 1: %v, Punkte %d", m1.Skills, sim.AvailablePoints(w, m1))
	}
	if !slices.Equal(m0.Skills, before0) {
		t.Fatalf("Slot 0 verändert: %v", m0.Skills)
	}
	r.isl.SkillPool++
	for name, err := range map[string]error{
		"unbekannt":     r.Learn("a", p, 1, "gibtsnicht"),
		"schon gelernt": r.Learn("a", p, 1, "fireball"),
		"fremdes Gerät": r.Learn("b", p, 0, "taunt"),
		"fremder Slot":  r.Learn("a", p, 3, "taunt"),
		"fremde Verb.":  r.Learn("a", &peer{}, 0, "taunt"),
	} {
		if err != ErrBadRequest {
			t.Errorf("%s: %v", name, err)
		}
	}
	r.isl.SkillPool--
	if err := r.Learn("a", p, 1, "iceWall"); err != ErrBadRequest {
		t.Fatalf("ohne Punkte: %v", err)
	}
	if !slices.Equal(m0.Skills, before0) || len(m1.Skills) != r.isl.SkillPool {
		t.Fatalf("Ablehnung hat geändert: %v / %v", m0.Skills, m1.Skills)
	}
}

// (d) Respec am Tag an der Burg leert die Skills; in der Nacht bad_request, die Skills bleiben.
func TestSkillsRespec(t *testing.T) {
	r, p, _ := devRoom(t)
	m0 := islPlayer(t, r, r.devices["a"].slots[0])
	w := r.isl.Stages[r.isl.StageOf(m0.Index)]
	r.isl.SkillPool = len(m0.Skills) + 1
	ok(t, r.Learn("a", p, 0, "taunt"))
	w.Cycle.Phase = "night"
	if err := r.Respec("a", p, 0); err != ErrBadRequest || !slices.Contains(m0.Skills, "taunt") {
		t.Fatalf("Nacht: %v, %v", err, m0.Skills)
	}
	w.Cycle.Phase = "day"
	m0.X = w.Castle.X
	ok(t, r.Respec("a", p, 0))
	if len(m0.Skills) != 0 || len(m0.Slots) != 0 {
		t.Fatalf("nach Respec: %v %v", m0.Skills, m0.Slots)
	}
}
