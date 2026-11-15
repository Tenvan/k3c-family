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
	Mix     Mixing            // Mischpegel aus sounds.json › mischung; Null-Wert = defaultMixing
	skills  map[string]skillInfo
}

// Mixing sind die Mischpegel (B-935, Klangbild docs/rules/klang.md): Dämpfung je Bus vor den Einstellungen und die
// Form des Duckings.
type Mixing struct {
	Effects, Music float64
	Duck           duckShape
}

// defaultMixing gilt ohne Abschnitt mischung: Busse ungedämpft, Ducking wie vor B-935.
var defaultMixing = Mixing{Effects: 1, Music: 1, Duck: defaultDuck}

// mixing liefert die Mischpegel; ein Register ohne Pegel (Null-Wert, z. B. in Tests) nimmt defaultMixing.
func (r *Register) mixing() Mixing {
	if r.Mix == (Mixing{}) {
		return defaultMixing
	}
	return r.Mix
}

// mixingFile ist der Abschnitt mischung in sounds.json.
type mixingFile struct {
	Effects     float64 `json:"effekte"`
	Music       float64 `json:"musik"`
	DuckLevel   float64 `json:"duckPegel"`
	DuckAttack  float64 `json:"duckAnstieg"`
	DuckHold    float64 `json:"duckHalten"`
	DuckRelease float64 `json:"duckAbklingen"`
}

// parse prüft die Werte: Pegel in (0, 1], Ducking-Pegel in (0, 1), Rampen > 0, Halten ≥ 0.
func (m mixingFile) parse() (Mixing, error) {
	unit := func(v float64) bool { return v > 0 && v <= 1 }
	switch {
	case !unit(m.Effects) || !unit(m.Music):
		return Mixing{}, fmt.Errorf("❌ data/sounds.json › mischung: effekte %v / musik %v nicht in (0, 1]", m.Effects, m.Music)
	case m.DuckLevel <= 0 || m.DuckLevel >= 1:
		return Mixing{}, fmt.Errorf("❌ data/sounds.json › mischung: duckPegel %v nicht in (0, 1)", m.DuckLevel)
	case m.DuckAttack <= 0 || m.DuckRelease <= 0 || m.DuckHold < 0:
		return Mixing{}, fmt.Errorf("❌ data/sounds.json › mischung: Ducking-Zeiten %v / %v / %v ungültig",
			m.DuckAttack, m.DuckHold, m.DuckRelease)
	}
	return Mixing{Effects: m.Effects, Music: m.Music,
		Duck: duckShape{level: m.DuckLevel, attack: m.DuckAttack, hold: m.DuckHold, release: m.DuckRelease}}, nil
}

type skillInfo struct {
	Kind string `json:"kind"`
	Line string `json:"line"`
}

type soundsFile struct {
	Mix   *mixingFile `json:"mischung"`
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
	r := Register{Effects: map[string]string{}, Music: map[string]string{}, Mix: defaultMixing, skills: mf.Skills}
	if sf.Mix != nil {
		mix, err := sf.Mix.parse()
		if err != nil {
			return Register{}, err
		}
		r.Mix = mix
	}
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
