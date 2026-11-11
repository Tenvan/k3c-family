package fx

// Eigener Effekt je Skill (Spiegel von src/scenes/effects/skillLooks.ts, B-578): Farbe (Palette DB32), Größe, Dauer,
// Partikelzahl und Bewegung. Ein Skill ohne Eintrag zeigt den Ring seiner Linie (lineColors). Nur Optik.

import (
	"encoding/json"
	"log/slog"
	"math"

	"k3c/data"
)

// lineColors: Farbe des Skill-Rings je Linie (effects.ts › LINE_COLORS).
var lineColors = map[string]uint32{"tank": 0x6fa8ff, "mage": 0xc070ff, "healer": 0x70f0a0, "thief": 0x9a9a9a}

var skillLooks = map[string]look{
	"taunt": {0xdf7126, 40, 0.7, 6, motionWave}, "shieldBash": {0xfbf236, 24, 0.4, 5, motionBurst},
	"ironWall": {0x9badb7, 44, 0.9, 6, motionRise}, "lastStand": {0xac3232, 46, 0.9, 8, motionRise},
	"armorAura": {0x639bff, 36, 0.6, 4, motionWave}, "thickSkin": {0x8f563b, 30, 0.5, 4, motionBurst},
	"regeneration": {0x6abe30, 28, 0.7, 5, motionRise}, "guardian": {0x5b6ee1, 40, 0.7, 6, motionWave},
	"fortified": {0x847e87, 32, 0.5, 4, motionFall},
	"fireball":  {0xdf7126, 34, 0.45, 8, motionBurst}, "iceWall": {0x5fcde4, 38, 0.7, 6, motionWave},
	"lightningStorm": {0xfbf236, 36, 0.5, 6, motionFall}, "meteor": {0xac3232, 54, 0.8, 10, motionFall},
	"arcanePower": {0x76428a, 32, 0.6, 6, motionRise}, "spellEcho": {0xd77bba, 30, 0.6, 6, motionBurst},
	"frostArmor": {0xcbdbfc, 32, 0.6, 5, motionFall}, "elementalMastery": {0x306082, 34, 0.6, 6, motionBurst},
	"heal": {0x99e550, 30, 0.6, 5, motionRise}, "groupHeal": {0x6abe30, 44, 0.7, 8, motionWave},
	"divineShield": {0xeec39a, 42, 0.8, 6, motionRise}, "resurrection": {0xffffff, 50, 0.9, 8, motionRise},
	"healingAura": {0x37946e, 36, 0.7, 5, motionWave}, "blessings": {0xd9a066, 32, 0.7, 6, motionFall},
	"holyGround": {0x8f974a, 40, 0.7, 6, motionWave},
	"backstab":   {0xd95763, 22, 0.3, 4, motionBurst}, "smokeBomb": {0x696a6a, 48, 0.9, 8, motionRise},
}

// skillData ist aus monarch.json › skills, was die Effekte brauchen: Linie, Art und Schild-HP.
type skillData struct {
	Line, Kind string
	Effect     struct {
		Type string
		HP   float64
	}
}

var skills = readSkills()

func readSkills() map[string]skillData {
	var m struct{ Skills map[string]skillData }
	raw, err := data.Files.ReadFile("monarch.json")
	if err == nil {
		err = json.Unmarshal(raw, &m)
	}
	if err != nil {
		slog.Error("💥 Effekt-Werte nicht lesbar", "datei", "monarch.json", "err", err)
	}
	return m.Skills
}

// shieldMax ist der größte Schild eines Skills (effect.type shield) als volle Balkenbreite.
var shieldMax = func() float64 {
	m := 1.0
	for _, s := range skills {
		if s.Effect.Type == "shield" {
			m = math.Max(m, s.Effect.HP)
		}
	}
	return m
}()

// skillLook liefert den Effekt eines Skills: eigener Look, sonst der Ring seiner Linie; unbekannt: ok false.
func skillLook(id string) (look, bool) {
	if l, ok := skillLooks[id]; ok {
		return l, true
	}
	c, ok := lineColors[skills[id].Line]
	if !ok {
		return look{}, false
	}
	base := config[kindSkill].look
	return look{c, base.radius, base.duration, base.particles, motionBurst}, true
}

// particleOffset ist der Ziel-Versatz des Partikels i von n je Bewegung (Welt-px; y nach unten positiv).
func particleOffset(m motion, i, n int, radius float64) (dx, dy float64) {
	spread := 0.0
	if n > 1 {
		spread = float64(i)/float64(n-1) - 0.5
	}
	switch m {
	case motionRise:
		return spread * radius * 1.6, -radius * 2
	case motionFall:
		return spread * radius * 1.6, radius * 1.2
	case motionWave:
		side := 1.0
		if i%2 == 0 {
			side = -1
		}
		return side * radius * (1.2 + float64(i/2)*0.5), 30
	}
	a := float64(i) / float64(n) * math.Pi * 2
	return math.Cos(a) * radius * 1.4, math.Sin(a)*radius*1.4 - 12
}

// piece ist ein Teil eines Effekts zum Fortschritt p: Versatz (Welt-px), Deckkraft, Radius (Welt-px).
type piece struct {
	dx, dy, alpha, radius float64
}

// pieceAt ist Teil i des Effekts (i = 0: Ring, wächst auf das 1,8-Fache; danach Partikel, schrumpfen auf die Hälfte)
// beim Fortschritt p (0..1, linear wie die Tweens im Browser).
func pieceAt(l look, i int, p float64) piece {
	k := math.Min(1, math.Max(0, p))
	if i == 0 {
		return piece{0, 0, 0.8 * (1 - k), l.radius * (1 + 0.8*k)}
	}
	dx, dy := particleOffset(l.motion, i-1, l.particles, l.radius)
	return piece{dx * k, dy * k, 0.8 * (1 - k), 5 * (1 - 0.5*k)}
}
