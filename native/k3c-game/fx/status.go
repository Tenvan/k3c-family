package fx

// Status-Effekte der Skills (Spiegel von src/scenes/effects/statusLooks.ts, B-675): Auren und Sturm am Boden, Schild
// und Last Stand am Monarchen, Verlangsamung, Betäubung und Spott über dem Gegner; dazu der Warnkreis vor dem
// Flächenschlag eines Bosses (kampfView.ts › WarnView). Nur Optik aus dem Zustand, Bewegung allein aus der Zeit.

import (
	"slices"

	"k3c/engine/sim"
)

// statusKind in Zeichen-Reihenfolge (Boden zuerst).
type statusKind int

const (
	statusAura statusKind = iota
	statusStorm
	statusShield
	statusLastStand
	statusSlow
	statusStun
	statusTaunt
	statusKinds
)

// mark ist eine Markierung: x in Units, y Höhe über dem Boden in Welt-px (Mitte bzw. Boden), Radius in Welt-px,
// Farbe und Anteil (Schild-Balken).
type mark struct {
	kind         statusKind
	x, y, radius float64
	color        uint32
	share        float64
}

// Werte der Looks (STATUS_LOOKS): Farben DB32, Größen und Höhen in Welt-px, Takte in Sekunden.
const (
	stunColor, slowColor, tauntColor = 0xfbf236, 0x5fcde4, 0xdf7126
	stormColor, stormFill            = 0xfbf236, 0x5b6ee1
	shieldColor, lastStandColor      = 0x9badb7, 0xac3232
	stunOrbit, slowSize, tauntHeight = 16.0, 9.0, 18.0
	statusY, statusBossY             = 104.0, 190.0
	tauntY, tauntBossY               = 112.0, 200.0
	shieldRadius, shieldY            = 46.0, 48.0
	lastStandRadius                  = 40.0
	auraFallback                     = 0xffffff
)

// auras liefert die sichtbaren Passive eines Monarchen (sim.AurasOf; Tests geben eigene).
type auras func(p *sim.Player) []sim.Aura

// auraCache merkt sim.AurasOf je Monarch der Welt w, solange seine Skills gleich bleiben: Die Auren hängen nur von
// den gelernten Skills und den Werten der Welt ab, AurasOf legt aber je Aufruf einen neuen Slice an.
type auraCache struct {
	w       *sim.World
	entries []auraEntry
}

type auraEntry struct {
	w      *sim.World
	index  int
	skills []string
	auras  []sim.Aura
}

// of liefert die Auren des Monarchen p in c.w (auras).
func (c *auraCache) of(p *sim.Player) []sim.Aura {
	if p.RespawnIn > 0 { // gefallen: keine Auren (wie sim.AurasOf), nichts merken
		return nil
	}
	for i := range c.entries {
		e := &c.entries[i]
		if e.index != p.Index {
			continue
		}
		if e.w != c.w || !slices.Equal(e.skills, p.Skills) {
			e.w, e.skills, e.auras = c.w, append(e.skills[:0], p.Skills...), sim.AurasOf(c.w, p)
		}
		return e.auras
	}
	c.entries = append(c.entries, auraEntry{w: c.w, index: p.Index, skills: slices.Clone(p.Skills), auras: sim.AurasOf(c.w, p)})
	return c.entries[len(c.entries)-1].auras
}

// statusMarks schreibt alle Markierungen der Welt w in out (geleert), nach Art sortiert (Boden zuerst, stabil).
func statusMarks(out []mark, w *sim.World, aurasOf auras) []mark {
	out = out[:0]
	for kind := range statusKinds {
		switch kind {
		case statusAura:
			out = auraMarks(out, w, aurasOf)
		case statusStorm:
			for _, s := range w.Storms {
				if s != nil && s.Left > 0 {
					out = append(out, mark{statusStorm, s.X, 0, s.Radius * unitPx, stormColor, 1})
				}
			}
		case statusShield, statusLastStand:
			out = playerMarks(out, w, kind)
		default:
			out = enemyMarks(out, w, kind)
		}
	}
	return out
}

// auraMarks: Aura-Ringe der lebenden Monarchen am Boden (Holy Ground an der Burg), mindestens eine Unit groß.
func auraMarks(out []mark, w *sim.World, aurasOf auras) []mark {
	for _, p := range w.Players {
		if p == nil || p.RespawnIn > 0 {
			continue
		}
		for _, a := range aurasOf(p) {
			x := p.X
			if a.Castle {
				x = w.Castle.X
			}
			c := uint32(auraFallback)
			if l, ok := skillLook(a.Skill); ok {
				c = l.color
			}
			out = append(out, mark{statusAura, x, 0, max(a.Radius*unitPx, unitPx), c, 1})
		}
	}
	return out
}

// playerMarks: Schild-Blase mit Balken bzw. Last Stand der lebenden Monarchen.
func playerMarks(out []mark, w *sim.World, kind statusKind) []mark {
	for _, p := range w.Players {
		if p == nil || p.RespawnIn > 0 {
			continue
		}
		switch {
		case kind == statusShield && p.ShieldFor > 0 && p.Shield > 0:
			out = append(out, mark{statusShield, p.X, shieldY, shieldRadius, shieldColor, min(1, p.Shield/shieldMax)})
		case kind == statusLastStand && p.LastStandFor > 0:
			out = append(out, mark{statusLastStand, p.X, shieldY, lastStandRadius, lastStandColor, 1})
		}
	}
	return out
}

// enemyMarks: Verlangsamung (links), Betäubung (Mitte), Spott (rechts) über dem Kopf lebender Gegner.
func enemyMarks(out []mark, w *sim.World, kind statusKind) []mark {
	for _, e := range w.Enemies {
		if e == nil || e.HP <= 0 {
			continue
		}
		y, ty := statusY, tauntY
		if e.Boss {
			y, ty = statusBossY, tauntBossY
		}
		switch {
		case kind == statusSlow && e.SlowFor > 0:
			out = append(out, mark{statusSlow, e.X - 18/unitPx, y, slowSize, slowColor, 1})
		case kind == statusStun && e.Stun > 0:
			out = append(out, mark{statusStun, e.X, y, stunOrbit, stunColor, 1})
		case kind == statusTaunt && e.TauntFor > 0:
			out = append(out, mark{statusTaunt, e.X + 18/unitPx, ty, tauntHeight, tauntColor, 1})
		}
	}
	return out
}

// warnWindow: Sekunden vor dem Flächenschlag, in denen der Warnkreis kräftiger wird.
const warnWindow = 3.0

// urgency ist die Dringlichkeit des Warnkreises: 0 = Schlag noch weit, 1 = jetzt.
func urgency(in float64) float64 {
	return 1 - min(warnWindow, max(0, in))/warnWindow
}
