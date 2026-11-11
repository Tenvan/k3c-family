package audio

import (
	"os"
	"path/filepath"
	"regexp"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// browserSource liest eine Datei des Browser-Tons (src/audio/), Vorbild der Zuordnungen.
func browserSource(t *testing.T, name string) string {
	t.Helper()
	b, err := os.ReadFile(filepath.Join("..", "..", "..", "src", "audio", name))
	if err != nil {
		t.Fatal(err)
	}
	return string(b)
}

// AC-02: Jeder Eintrag aus events.ts › CUES hat nativ denselben Slot und dieselbe Warnung, und nativ gibt es keinen
// zusätzlichen; jeder Slot hat eine Datei im Register.
func TestEventCuesMatchBrowser(t *testing.T) {
	re := regexp.MustCompile(`(\w+): \{ slot: '([^']+)', global: (true|false) \}`)
	found := re.FindAllStringSubmatch(browserSource(t, "events.ts"), -1)
	if len(found) == 0 {
		t.Fatal("keine Einträge in events.ts gefunden")
	}
	reg := realRegister(t)
	for _, m := range found {
		got, ok := eventCues[m[1]]
		if !ok || got.slot != m[2] || got.global != (m[3] == "true") {
			t.Errorf("%s: nativ %+v, Browser %s global %s", m[1], got, m[2], m[3])
		}
		if _, ok := reg.Effects[m[2]]; !ok {
			t.Errorf("%s: Slot %s ohne Datei im Register", m[1], m[2])
		}
	}
	if len(eventCues) != len(found) {
		t.Errorf("nativ %d Einträge, Browser %d", len(eventCues), len(found))
	}
}

// AC-02: Menü-Klänge wie ui.ts › UI_SLOTS (Auswahl bewegt, Bestätigen); alle anderen Cues bleiben still.
func TestCueSlotsMatchBrowser(t *testing.T) {
	src := browserSource(t, "ui.ts")
	re := regexp.MustCompile(`select: '([^']+)', confirm: '([^']+)'`)
	m := re.FindStringSubmatch(src)
	if m == nil {
		t.Fatal("UI_SLOTS in ui.ts nicht gefunden")
	}
	if cueSlot(view.CueMove) != m[1] || cueSlot(view.CueConfirm) != m[2] {
		t.Errorf("Move %q Confirm %q, Browser %q %q", cueSlot(view.CueMove), cueSlot(view.CueConfirm), m[1], m[2])
	}
	for k := view.CueBack; k <= view.CueRebound; k++ {
		if s := cueSlot(k); s != "" {
			t.Errorf("Cue %d klingt %q, im Browser still", k, s)
		}
	}
}

// Skill-Klang wie skills.ts › skillSlot: aktive Skills über die Linie, passive still, unbekannte Linie → gemeinsamer Slot.
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

// Dämpfung wie events.ts › attenuation: im Bild 1, am Rand 0,5, nach einer Breite 0; Warnung und ohne Ort 1.
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
		{sim.Event{"type": "playerDown", "player": 1}, 42, true},
		{sim.Event{"type": "playerDown", "player": 7}, 0, false},
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
