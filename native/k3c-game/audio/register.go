package audio

import (
	"encoding/json"
	"fmt"
	"io/fs"
)

// Register ist das Slot-Register der Klänge (data/sounds.json, wie src/audio/events.ts › sfxUrls und music.ts ›
// musicTracks) mit den Skill-Linien aus data/monarch.json (src/audio/skills.ts). Dateien sind Pfade unter public/.
type Register struct {
	Effects map[string]string // Effekt-Slot (Bus sfx) → erste Präferenz ohne Atlas-Sprite
	Music   map[string]string // Musik-Slot (Bus music) → erste Präferenz
	skills  map[string]skillInfo
}

type skillInfo struct {
	Kind string `json:"kind"`
	Line string `json:"line"`
}

type soundsFile struct {
	Slots []struct {
		ID   string `json:"id"`
		Art  string `json:"art"`
		Bus  string `json:"bus"`
		Wahl []struct {
			Datei  string `json:"datei"`
			Sprite string `json:"sprite"`
		} `json:"wahl"`
	} `json:"slots"`
}

// LoadRegister liest sounds.json und monarch.json aus den Balancing-Daten (data.Files).
func LoadRegister(fsys fs.ReadFileFS) (Register, error) {
	sounds, err := fsys.ReadFile("sounds.json")
	if err != nil {
		return Register{}, fmt.Errorf("❌ Slot-Register data/sounds.json: %w", err)
	}
	monarch, err := fsys.ReadFile("monarch.json")
	if err != nil {
		return Register{}, fmt.Errorf("❌ Skills data/monarch.json: %w", err)
	}
	return ParseRegister(sounds, monarch)
}

// ParseRegister baut das Register aus den Inhalten von sounds.json und monarch.json.
func ParseRegister(sounds, monarch []byte) (Register, error) {
	var sf soundsFile
	if err := json.Unmarshal(sounds, &sf); err != nil {
		return Register{}, fmt.Errorf("❌ data/sounds.json: %w", err)
	}
	var mf struct {
		Skills map[string]skillInfo `json:"skills"`
	}
	if err := json.Unmarshal(monarch, &mf); err != nil {
		return Register{}, fmt.Errorf("❌ data/monarch.json: %w", err)
	}
	r := Register{Effects: map[string]string{}, Music: map[string]string{}, skills: mf.Skills}
	for _, s := range sf.Slots {
		if s.Art != "sound" {
			continue
		}
		switch s.Bus {
		case "sfx":
			for _, w := range s.Wahl {
				if w.Sprite == "" && w.Datei != "" {
					r.Effects[s.ID] = w.Datei
					break
				}
			}
		case "music":
			if len(s.Wahl) > 0 && s.Wahl[0].Datei != "" {
				r.Music[s.ID] = s.Wahl[0].Datei
			}
		}
	}
	return r, nil
}

// skillFallback klingt für aktive Skills ohne eigenen Slot und ohne Slot der Linie (skills.ts › SKILL_FALLBACK).
const skillFallback = "sfx:skill"

// SkillSlot liefert den Slot zum Wirken eines Skills wie skills.ts › skillSlot: eigener Slot, bei aktiven Skills der
// Slot der Linie, sonst der gemeinsame; ein passiver Skill ohne eigenen Slot bleibt still ("").
func (r *Register) SkillSlot(id string) string {
	own := "sfx:skill:" + id
	if _, ok := r.Effects[own]; ok {
		return own
	}
	s := r.skills[id]
	if s.Kind != "active" {
		return ""
	}
	line := "sfx:skill:" + s.Line
	if _, ok := r.Effects[line]; ok {
		return line
	}
	return skillFallback
}
