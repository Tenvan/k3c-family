package audio

import (
	"sort"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// effectSlots sind die Effekt-Slots, die das Spiel aus den nativen Tabellen abspielt: Ereignisse und Menü-Klänge.
func effectSlots() map[string]string {
	slots := map[string]string{"cue:move": cueSlot(view.CueMove), "cue:confirm": cueSlot(view.CueConfirm)}
	for name, c := range eventCues {
		slots[name] = c.slot
	}
	return slots
}

// missingFiles nennt sortiert jeden Eintrag aus slots (Name → Slot), dessen Slot leer ist oder in files keine Datei hat.
func missingFiles(slots, files map[string]string) []string {
	var res []string
	for name, slot := range slots {
		if slot == "" || files[slot] == "" {
			res = append(res, name+" → "+slot)
		}
	}
	sort.Strings(res)
	return res
}

// AC-01: Jeder Slot aus eventCues hat eine Datei im Register; global (Warnung) sind genau die vier Warn-Ereignisse.
func TestEventCuesHaveFiles(t *testing.T) {
	if len(eventCues) == 0 {
		t.Fatal("eventCues ist leer")
	}
	for _, m := range missingFiles(effectSlots(), realRegister(t).Effects) {
		t.Errorf("Slot ohne Datei im Register: %s", m)
	}
	warn := map[string]bool{"bossSpawned": true, "dusk": true, "wave": true, "castleFallen": true}
	for name, c := range eventCues {
		if c.global != warn[name] {
			t.Errorf("%s: global %v, erwartet %v", name, c.global, warn[name])
		}
	}
}

// AC-02: Fehlt einem Slot die Datei im Register, nennt die Prüfung genau diesen Eintrag (Schutz gegen stumme Slots).
func TestMissingFilesNamesSlot(t *testing.T) {
	files := map[string]string{}
	for k, v := range realRegister(t).Effects {
		files[k] = v
	}
	delete(files, eventCues["hit"].slot)
	got := missingFiles(effectSlots(), files)
	if len(got) != 1 || got[0] != "hit → sfx:hit" {
		t.Errorf("fehlende Slots %v, erwartet [hit → sfx:hit]", got)
	}
	if got := missingFiles(map[string]string{"leer": ""}, files); len(got) != 1 {
		t.Errorf("leerer Slot nicht gemeldet: %v", got)
	}
}

// Menü-Klänge: nur Auswahl bewegt und Bestätigen klingen, je mit eigenem Slot; alle anderen Cues bleiben still.
func TestCueSlots(t *testing.T) {
	if cueSlot(view.CueMove) != slotUISelect || cueSlot(view.CueConfirm) != slotUIConfirm {
		t.Errorf("Move %q Confirm %q", cueSlot(view.CueMove), cueSlot(view.CueConfirm))
	}
	if slotUISelect == slotUIConfirm {
		t.Error("Bewegen und Bestätigen teilen einen Slot")
	}
	for k := view.CueBack; k <= view.CueRebound; k++ {
		if s := cueSlot(k); s != "" {
			t.Errorf("Cue %d klingt %q, erwartet still", k, s)
		}
	}
}

// Skill-Klang: aktive Skills über die Linie, passive still, unbekannte Linie → gemeinsamer Slot.
func TestSkillSlot(t *testing.T) {
	reg := realRegister(t)
	cases := map[string]string{
		"fireball": "sfx:skill:mage", "taunt": "sfx:skill:tank", "heal": "sfx:skill:healer",
		"backstab": "sfx:skill:thief", "armorAura": "", "gibtsnicht": "",
	}
	for id, want := range cases {
		if got := reg.SkillSlot(id); got != want {
			t.Errorf("%s: %q, erwartet %q", id, got, want)
		}
	}
	r := Register{Effects: map[string]string{"sfx:skill:x": "f"}, skills: map[string]skillInfo{
		"a": {Kind: "active", Line: "neu"}, "x": {Kind: "passive"},
	}}
	if r.SkillSlot("a") != skillFallback || r.SkillSlot("x") != "sfx:skill:x" {
		t.Errorf("Rückfall %q, eigener Slot %q", r.SkillSlot("a"), r.SkillSlot("x"))
	}
}

// Dämpfung: im Bild 1, am Rand 0,5, nach einer Breite 0; Warnung und ohne Ort 1.
func TestAttenuation(t *testing.T) {
	ears := []ear{{center: 0, span: 40}}
	cases := []struct {
		x      float64
		hasX   bool
		global bool
		want   float64
	}{
		{10, true, false, 1},
		{20.4, true, false, edgeGain * (1 - 0.4/40)},
		{60, true, false, edgeGain * (1 - 40.0/40)},
		{100, true, false, 0},
		{100, true, true, 1},
		{100, false, false, 1},
	}
	for _, c := range cases {
		if got := attenuation(c.x, c.hasX, ears, c.global); got-c.want > 1e-9 || c.want-got > 1e-9 {
			t.Errorf("x=%v: %v, erwartet %v", c.x, got, c.want)
		}
	}
	two := []ear{{center: 0, span: 40}, {center: 200, span: 40}}
	if attenuation(200, true, two, false) != 1 {
		t.Error("das lauteste Ohr gilt")
	}
	if attenuation(500, true, nil, false) != 1 {
		t.Error("ohne Ohr keine Dämpfung")
	}
}

func TestEventXAndNightGain(t *testing.T) {
	w := &sim.World{Players: []*sim.Player{nil, {Index: 1, X: 42}}}
	cases := []struct {
		data sim.Event
		x    float64
		ok   bool
	}{
		{sim.Event{"type": "hit", "x": 3.5}, 3.5, true},
		{sim.Event{"type": "crownFallen", "player": 1}, 42, true},
		{sim.Event{"type": "crownFallen", "player": 7}, 0, false},
		{sim.Event{"type": "dusk"}, 0, false},
	}
	for _, c := range cases {
		if x, ok := eventX(view.Event{Data: c.data}, w); x != c.x || ok != c.ok {
			t.Errorf("%v: %v %v", c.data, x, ok)
		}
	}
	if nightGain("night") != nightGainVal || nightGain("dawn") != 1 {
		t.Error("Nacht leise, sonst voll")
	}
}
