package fx

// Ereignis → Effekt (Spiegel von src/scenes/effects/effects.ts, B-164, GR5.1): reine Zuordnung, liest nur das
// Ereignis, ändert keinen Spielzustand. Dazu die Schadenszahl je Treffer (B-799 › Beispiele: „Monarch trifft einen
// Gegner → Schadenszahl steigt auf“; der Browser zeigt nur den Treffer-Blitz).

import (
	"math"
	"strconv"

	"k3c/engine/sim"
)

// kind ist die Art eines Effekts.
type kind int

const (
	kindHit kind = iota
	kindKill
	kindCoin
	kindCoinGive
	kindBuild
	kindBuilt
	kindDown
	kindSwing
	kindNoTarget
	kindSkill
	kindDamage
)

// look ist das Aussehen eines Effekts: Ring plus Partikel in Farbe, Größe (Welt-px), Dauer (s), Zahl und Bewegung.
type look struct {
	color     uint32
	radius    float64
	duration  float64
	particles int
	motion    motion
}

// motion ist die Bewegung der Partikel: burst im Kreis, rise steigt auf, fall fällt herab, wave am Boden zur Seite.
type motion int

const (
	motionBurst motion = iota
	motionRise
	motionFall
	motionWave
)

// config: Aussehen und Grenzen aller Effekte (EFFECT_CONFIG); flash = Blitz-Effekt (Grenze 3 je Sekunde).
var config = [...]struct {
	look
	flash bool
}{
	kindHit:      {look{0xffffff, 22, 0.16, 0, motionBurst}, true},
	kindKill:     {look{0xff5533, 34, 0.38, 6, motionBurst}, false},
	kindCoin:     {look{0xffd23c, 10, 0.3, 3, motionBurst}, false},
	kindCoinGive: {look{0xffe680, 12, 0.3, 3, motionBurst}, false},
	kindBuild:    {look{0xc8b078, 26, 0.3, 4, motionBurst}, false},
	kindBuilt:    {look{0x7be07b, 44, 0.5, 8, motionBurst}, false},
	kindDown:     {look{0xb02030, 40, 0.6, 8, motionBurst}, false},
	kindSwing:    {look{0xdddddd, 18, 0.14, 0, motionBurst}, false},
	kindNoTarget: {look{0xffffff, 0, 0.7, 0, motionBurst}, false},
	kindSkill:    {look{0xffffff, 30, 0.45, 4, motionBurst}, false},
	kindDamage:   {look{0xffffff, 0, 0.8, 0, motionBurst}, false},
}

// Grenzen (effects.ts): höchstens 3 Blitze je Sekunde, höchstens 48 lebende Teile je Stufe.
const (
	maxFlashesPerSecond = 3
	flashMinGap         = 1.0 / maxFlashesPerSecond
	maxLivePieces       = 48
)

// Höhen über dem Boden in Welt-px (Browser): Treffer und Münzen, Bau, „kein Ziel“, Skill-Ring.
const (
	bodyPx     = 40.0
	buildPx    = 60.0
	noTargetPx = 90.0
	ringPx     = 50.0
	unitPx     = 32.0
)

// effect ist ein laufender Effekt einer Stufe.
type effect struct {
	kind   kind
	x, y   float64 // x in Units, y Höhe über dem Boden in Welt-px
	look   look
	text   string // noTarget und Schadenszahl
	target bool   // Schadenszahl: Treffer an einem Monarchen (rot)
	start  float64
	by     int // Schlag: ID des ausholenden Spielers, sonst -1
}

// pieces ist die Zahl der Teile (Ring plus Partikel; Text = 1).
func (e effect) pieces() int {
	if e.text != "" {
		return 1
	}
	return 1 + e.look.particles
}

// flashAllowed: Blitz erlaubt, wenn der letzte mindestens flashMinGap Sekunden zurückliegt (last < 0: noch keiner).
func flashAllowed(now, last float64) bool {
	return last < 0 || now-last >= flashMinGap-1e-9
}

// playerX liefert den Ort des Monarchen mit Index i (ok false, wenn es ihn nicht gibt).
type playerX func(index int) (float64, bool)

// effectFor ordnet einem Ereignis genau einen Effekt zu; unbekannte Typen und Ereignisse ohne Ort: ok false.
// spots merkt den Ort des letzten Baufortschritts je Bauart (Rückfall für built ohne x).
func effectFor(lang string, d sim.Event, typ string, px playerX, spots map[string]float64) (effect, bool) {
	at := func(k kind, x float64, ok bool, up float64) (effect, bool) {
		return effect{kind: k, x: x, y: up, look: config[k].look, by: -1}, ok
	}
	x, hasX := numOf(d, "x")
	switch typ {
	case "hit":
		return at(kindHit, x, hasX, bodyPx)
	case "kill":
		return at(kindKill, x, hasX, bodyPx)
	case "coinPickup":
		return at(kindCoin, x, hasX, bodyPx)
	case "coinGive":
		return at(kindCoinGive, x, hasX, bodyPx)
	case "buildProgress":
		if hasX {
			spots[strOf(d, "kind")] = x
		}
		return at(kindBuild, x, hasX, buildPx)
	case "built":
		if !hasX {
			x, hasX = spots[strOf(d, "kind")]
		}
		return at(kindBuilt, x, hasX, buildPx)
	case "playerDown":
		if !hasX {
			p, _ := numOf(d, "player")
			x, hasX = px(int(p))
		}
		return at(kindDown, x, hasX, bodyPx)
	case "strike":
		return swing(d, x, hasX)
	case "castFailed":
		e, ok := at(kindNoTarget, x, hasX, noTargetPx)
		e.text = noTargetText(lang)
		return e, ok
	}
	return effect{}, false
}

// swing: Nur der Monarch holt aus (Schlag mit „hit“); der Schlag eines Gegners trägt kein „hit“.
func swing(d sim.Event, x float64, hasX bool) (effect, bool) {
	if _, ok := d["hit"]; !ok || !hasX {
		return effect{}, false
	}
	from, _ := numOf(d, "from")
	return effect{kind: kindSwing, x: x, y: bodyPx, look: config[kindSwing].look, by: int(from)}, true
}

// damageFor ist die Schadenszahl eines Treffers: aufgerundeter Schaden über dem Ziel, rot an einem Monarchen.
func damageFor(d sim.Event) (effect, bool) {
	x, hasX := numOf(d, "x")
	dmg, _ := numOf(d, "damage")
	if !hasX || dmg <= 0 {
		return effect{}, false
	}
	n := int(math.Ceil(dmg - 1e-9))
	return effect{kind: kindDamage, x: x, y: bodyPx + 40, look: config[kindDamage].look, text: strconv.Itoa(n),
		target: strOf(d, "target") == "player", by: -1}, true
}

// numOf liest eine Zahl aus einem Ereignis (int oder float64); ok false, wenn sie fehlt.
func numOf(d sim.Event, key string) (float64, bool) {
	switch v := d[key].(type) {
	case int:
		return float64(v), true
	case float64:
		return v, true
	case int64:
		return float64(v), true
	}
	return 0, false
}

func strOf(d sim.Event, key string) string {
	s, _ := d[key].(string)
	return s
}
