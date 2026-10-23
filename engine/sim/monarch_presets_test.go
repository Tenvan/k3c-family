package sim

import (
	"strings"
	"testing"
)

// Jedes Preset nennt nur Skills aus monarch.json › skills; sonst scheitert das Laden mit Preset und Skill (B-504/AC-01).
func TestPresetsNennenNurVorhandeneSkills(t *testing.T) {
	if err := checkPresets(monarch); err != nil {
		t.Fatalf("eingebettete Daten: %v", err)
	}
	bad := monarch
	bad.Presets = map[string]presetData{"thief": {Name: "Dieb", ActiveSkills: []string{"taunt", "backstab"}}}
	err := checkPresets(bad)
	if err == nil || !strings.Contains(err.Error(), "thief") || !strings.Contains(err.Error(), "backstab") {
		t.Fatalf("unbekannter Skill: %v", err)
	}
}
