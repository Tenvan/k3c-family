package audio

import (
	"math"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// soundCue ist der Klang eines Ereignisses: Slot im Register und ob er überall gleich laut klingt (Warnung).
type soundCue struct {
	slot   string
	global bool
}

// eventCues ist die Zuordnung Ereignis → Klang wie src/audio/events.ts › CUES; Warnungen (global) senken die Musik.
var eventCues = map[string]soundCue{
	"coinPickup":    {"sfx:coin", false},
	"hit":           {"sfx:hit", false},
	"kill":          {"sfx:kill", false},
	"arrow":         {"sfx:arrow", false},
	"strike":        {"sfx:strike", false},
	"built":         {"sfx:build", false},
	"playerDown":    {"sfx:death", false},
	"revive":        {"sfx:revive", false},
	"skillPoint":    {"sfx:skill", false},
	"arrived":       {"sfx:portal", false},
	"coinGive":      {"sfx:coinGive", false},
	"buildProgress": {"sfx:buildProgress", false},
	"revived":       {"sfx:revived", false},
	"bossSpawned":   {"sfx:boss", true},
	"dusk":          {"sfx:warn", true},
	"wave":          {"sfx:warn", true},
	"castleFallen":  {"sfx:warn", true},
}

// skillCastEvent ist das Ereignis eines gewirkten Skills; sein Slot kommt aus Register.SkillSlot (skills.ts).
const skillCastEvent = "skillCast"

// Menü-Klänge wie src/audio/ui.ts › UI_SLOTS: Auswahl bewegt und Bestätigen; alle anderen Cues bleiben still (B unbelegt).
const (
	slotUISelect  = "sfx:uiSelect"
	slotUIConfirm = "sfx:uiConfirm"
	slotHub       = "sfx:hub"
	slotRide      = "sfx:ride"
)

// cueSlot liefert den Menü-Klang eines Cues ("" = still).
func cueSlot(k view.CueKind) string {
	switch k {
	case view.CueMove:
		return slotUISelect
	case view.CueConfirm:
		return slotUIConfirm
	default:
		return ""
	}
}

// Pegel wie events.ts: Rand des sichtbaren Bereichs, Nacht leise, Hufschlag als Teppich.
const (
	edgeGain     = 0.5
	nightGainVal = 0.6
	rideMinSpeed = 0.3
	rideGain     = 0.5
)

// nightGain ist der Pegelfaktor der Effekte zur Tageszeit (events.ts › nightGain).
func nightGain(phase string) float64 {
	if phase == "night" {
		return nightGainVal
	}
	return 1
}

// ear ist der sichtbare Ausschnitt eines lokalen Spielers in Units (events.ts › Listener).
type ear struct{ center, span float64 }

// attenuation ist die Lautstärke 0 … 1 einer Quelle bei x wie events.ts › attenuation: im Ausschnitt eines Ohrs 1,
// darüber hinaus ab edgeGain fallend bis 0 nach einer Bildschirmbreite; das lauteste Ohr gilt.
func attenuation(x float64, hasX bool, ears []ear, global bool) float64 {
	if global || len(ears) == 0 || !hasX {
		return 1
	}
	best := math.Inf(1)
	for _, e := range ears {
		d := math.Max(0, math.Abs(x-e.center)-e.span/2) / math.Max(e.span, 1)
		best = math.Min(best, d)
	}
	if best == 0 {
		return 1
	}
	return math.Max(0, edgeGain*(1-best))
}

// eventX liefert den Ort eines Ereignisses in Units: Feld x, sonst der Ort des genannten Spielers (player) in w;
// ohne beides false (keine Dämpfung). Vorbild events.ts › eventX mit dem Ort des Effekts als Rückfall.
func eventX(e view.Event, w *sim.World) (float64, bool) {
	if x, ok := number(e.Data["x"]); ok {
		return x, true
	}
	p, ok := number(e.Data["player"])
	if !ok || w == nil {
		return 0, false
	}
	for _, m := range w.Players {
		if m != nil && float64(m.Index) == p {
			return m.X, true
		}
	}
	return 0, false
}

// number liest eine Zahl aus einem Ereignis-Feld (Go-Simulation: float64 oder int).
func number(v any) (float64, bool) {
	switch n := v.(type) {
	case float64:
		return n, true
	case int:
		return float64(n), true
	default:
		return 0, false
	}
}

// riding meldet, ob man den Hufschlag eines Monarchen hört (events.ts › rideLoops): lebt und bewegt sich.
func riding(p *sim.Player) bool {
	return p != nil && p.RespawnIn <= 0 && math.Abs(p.VX) >= rideMinSpeed
}
