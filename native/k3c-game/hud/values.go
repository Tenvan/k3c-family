package hud

// Texte der Werte (Spiegel von HudScene.ts › showWorld/showCells/clockText, economyLabels.ts und citizenLabels.ts):
// reine Funktionen auf einfachen Werten, gebaut nur, wenn sich ihr Schlüssel ändert (memo).

import (
	"math"
	"strconv"
	"strings"

	"k3c/engine/sim"
)

// sep trennt Einträge einer Zeile (wie der Browser).
const sep = "  ·  "

// memo hält einen Text, bis sich sein Schlüssel ändert: so entsteht je Bild kein neuer Text.
type memo[K comparable] struct {
	key K
	set bool
	s   string
}

// get liefert den Text zum Schlüssel k und baut ihn mit build nur bei einem neuen Schlüssel.
func (m *memo[K]) get(k K, build func(K) string) string {
	if !m.set || m.key != k {
		m.key, m.s, m.set = k, build(k), true
	}
	return m.s
}

// formatTime ist eine Restzeit als m:ss (aufgerundet, nie negativ).
func formatTime(seconds float64) string {
	s := int(math.Max(0, math.Ceil(seconds)))
	sec := strconv.Itoa(s % 60)
	if s%60 < 10 {
		sec = "0" + sec
	}
	return strconv.Itoa(s/60) + ":" + sec
}

// clockKey sind die Werte der Uhr: Phase, Tag, Restzeit in ganzen Sekunden (Zeitraffer eingerechnet), Aggression in
// ganzen Prozent (-1 = keine), gesichert.
type clockKey struct {
	lang, phase string
	day, left   int
	aggression  int
	saved       bool
}

// newClockKey liest die Uhr aus dem Zyklus der Welt.
func newClockKey(lang string, c sim.CycleInfo, speed float64, aggression *float64, saved bool) clockKey {
	if speed <= 0 {
		speed = 1
	}
	agg := -1
	if aggression != nil {
		agg = int(math.Floor(*aggression))
	}
	return clockKey{lang, c.Phase, c.Day, int(math.Max(0, math.Ceil(c.SecondsLeft/speed))), agg, saved}
}

// clockText: Tageszeit mit Restzeit, unter Tage mit Aggression, nach dem Speichern „gesichert“.
func clockText(k clockKey) string {
	key := "hud.clockNight"
	switch k.phase {
	case "day":
		key = "hud.clockDay"
	case "dusk":
		key = "hud.clockDusk"
	case "dawn":
		key = "hud.clockDawn"
	}
	s := tf(k.lang, key, "day", k.day, "left", formatTime(float64(k.left)))
	if k.aggression >= 0 {
		s += tf(k.lang, "hud.aggression", "pct", k.aggression)
	}
	if k.saved {
		s += tr(k.lang, "hud.saved")
	}
	return s
}

// goldKey: Gold, Maximum und Sekunden bis zum Respawn (aufgerundet, 0 = lebt).
type goldKey struct {
	lang           string
	gold, max, out int
}

// goldText ist der Wert der Spielerkarte: „Gold/Maximum“, gefallen die Sekunden bis zur Rückkehr. Gold steht nur
// hier (Regel „Gold nur im HUD“, B-398).
func goldText(k goldKey) string {
	if k.out > 0 {
		return tf(k.lang, "hud.downShort", "s", k.out)
	}
	return strconv.Itoa(k.gold) + "/" + strconv.Itoa(k.max)
}

// goldFlashSeconds: so lange hebt das HUD den Goldtopf nach einer Änderung hervor (B-398).
const goldFlashSeconds = 0.8

// goldFlash ist die Hervorhebung des Goldtopfs eines Spielers (goldFlash.ts).
type goldFlash struct {
	gold      int
	changedAt float64
	seen      bool
}

// step übernimmt den Goldstand zur Zeit now und meldet, ob der Topf hervorgehoben wird; der erste Wert zählt nicht
// als Änderung.
func (g *goldFlash) step(gold int, now float64) bool {
	switch {
	case !g.seen:
		g.gold, g.changedAt, g.seen = gold, math.Inf(-1), true
	case g.gold != gold:
		g.gold, g.changedAt = gold, now
	}
	return now-g.changedAt < goldFlashSeconds
}

// resourceOrder ist die feste Reihenfolge der Rohstoffe (wie sim.Stock).
var resourceOrder = [5]string{"wood", "stone", "copper", "iron", "crystal"}

// resName ist der Name eines Rohstoffs aus den Texten, sonst die ID.
func resName(lang, res string) string { return nameOf(lang, "res", res, res) }

// stockKey: Vorrat je Rohstoff, Primärrohstoff des Bioms, Lager-Maximum (0 = keins), Skill-Punkte.
type stockKey struct {
	lang, primary string
	stock         [5]int
	max, points   int
}

// sharedText ist die Vorrats-Zeile: Rohstoffe mit Vorrat oder der Primärrohstoff, mit Maximum „n/max“ bzw. „voll“,
// dahinter die Skill-Punkte, wenn welche frei sind (HudScene.ts › showWorld, economyLabels.ts › stockTexts).
func sharedText(k stockKey) string {
	parts := make([]string, 0, 6)
	for i, r := range resourceOrder {
		n := k.stock[i]
		if n <= 0 && r != k.primary {
			continue
		}
		name := resName(k.lang, r)
		switch {
		case k.max <= 0:
			parts = append(parts, tf(k.lang, "stock.amount", "res", name, "n", n))
		case n >= k.max:
			parts = append(parts, tf(k.lang, "stock.full", "res", name, "n", n, "max", k.max))
		default:
			parts = append(parts, tf(k.lang, "stock.ofMax", "res", name, "n", n, "max", k.max))
		}
	}
	if k.points > 0 {
		parts = append(parts, tf(k.lang, "hud.skillPoints", "n", k.points))
	}
	return strings.Join(parts, sep)
}

// stockOf liest den Vorrat in fester Reihenfolge (nil = leer).
func stockOf(s *sim.Stock) [5]int {
	if s == nil {
		return [5]int{}
	}
	return [5]int{s.Wood, s.Stone, s.Copper, s.Iron, s.Crystal}
}

// hubKey: Hub-Stufe, Schritt und Ausbau (has = es gibt eine nächste Stufe).
type hubKey struct {
	lang               string
	level, step, steps int
	has                bool
	gold, paid         int
	material           [5]int
	fighters, limit    int
	merchant           bool
	mResource          string
	mLeaves, mPaid     int
	compact            bool
}

// newHubKey liest Hub, Bürger und Händler aus dem Wirtschaftsstand und der Welt.
func newHubKey(lang string, e sim.Economy, m *sim.Merchant) hubKey {
	k := hubKey{lang: lang, level: e.HubLevel, fighters: e.Fighters, limit: e.TroopLimit, compact: true}
	if up := e.HubUpgrade; up != nil {
		k.has, k.step, k.steps, k.gold, k.paid, k.material = true, e.HubStep, up.Steps, up.Gold, up.Paid, stockOf(&up.Material)
	}
	if m != nil {
		k.merchant, k.mResource, k.mLeaves, k.mPaid = true, m.Resource, m.Leaves, m.BuyPaid
	}
	return k
}

// citizensText ist die Zeile Hub, Truppen-Limit und Händler (kurze Form wie die Leiste des Browsers).
func citizensText(k hubKey) string {
	parts := make([]string, 0, 3)
	if h := hubText(k); h != "" {
		parts = append(parts, h)
	}
	parts = append(parts, limitText(k.lang, k.fighters, k.limit, k.compact))
	if k.merchant {
		parts = append(parts, merchantText(k.lang, k.mResource, k.mLeaves, k.mPaid, k.compact))
	}
	return strings.Join(parts, sep)
}

// hubText ist die Hub-Stufe mit Schritt und Kosten des laufenden Ausbauschritts; ohne Ausbau nur die Stufe, ohne
// Stufe "".
func hubText(k hubKey) string {
	if k.level <= 0 {
		return ""
	}
	if !k.has {
		return tf(k.lang, pick(k.compact, "hub.levelShort", "hub.level"), "n", k.level)
	}
	parts := []string{tf(k.lang, "cost.gold", "paid", k.paid, "gold", k.gold)}
	for i, r := range resourceOrder {
		if k.material[i] > 0 {
			parts = append(parts, tf(k.lang, "cost.item", "n", k.material[i], "res", resName(k.lang, r)))
		}
	}
	cost := strings.Join(parts, ", ")
	if k.steps > 0 {
		return tf(k.lang, pick(k.compact, "hub.upgradeStepShort", "hub.upgradeStep"), "n", k.level, "next", k.level+1,
			"cost", cost, "step", k.step, "steps", k.steps)
	}
	return tf(k.lang, pick(k.compact, "hub.upgradeShort", "hub.upgrade"), "n", k.level, "next", k.level+1, "cost", cost)
}

func pick(short bool, a, b string) string {
	if short {
		return a
	}
	return b
}

// limitText: „Kämpfer/Limit“, voll, sobald die Zahl das Limit erreicht.
func limitText(lang string, n, limit int, compact bool) string {
	full := n >= limit
	key := pick(compact, pick(full, "limit.fullShort", "limit.short"), pick(full, "limit.full", "limit.text"))
	return tf(lang, key, "n", n, "max", limit)
}

// merchantText: Händler mit Angebot und Abreisetag, laufender Kauf in der langen Form.
func merchantText(lang, resource string, leaves, paid int, compact bool) string {
	res := resName(lang, resource)
	switch {
	case compact:
		return tf(lang, "merchant.short", "res", res)
	case paid > 0:
		return tf(lang, "merchant.paid", "res", res, "day", leaves, "paid", paid)
	}
	return tf(lang, "merchant.text", "res", res, "day", leaves)
}

// professionText ist der Name eines Berufs (bekannte Berufe, sonst neutral; kein Beruf "").
func professionText(lang, profession string, compact bool) string {
	if profession == "" {
		return ""
	}
	keys, ok := professionKeys[profession]
	if !ok {
		keys = professionKeys["unknown"]
	}
	if compact {
		return tr(lang, keys[1])
	}
	return tr(lang, keys[0])
}

// professionKeys: Text-Schlüssel je Beruf, lang und kurz (fest, damit je Bild kein Schlüssel entsteht).
var professionKeys = map[string][2]string{
	"miner": {"prof.miner", "prof.minerShort"}, "builder": {"prof.builder", "prof.builderShort"},
	"craftsman": {"prof.craftsman", "prof.craftsmanShort"}, "unknown": {"prof.unknown", "prof.unknownShort"},
}

// waveKey: Welle und Zahl der Gegner (laufend und wartend).
type waveKey struct {
	lang    string
	wave, n int
}

// waveText ist die Kampf-Zeile; ohne Gegner "".
func waveText(k waveKey) string {
	if k.n <= 0 {
		return ""
	}
	return tf(k.lang, "hud.wave", "wave", k.wave, "n", k.n)
}

// travelKey: Reise zur Tiefe über Treppe (up) mit Fortschritt in Zehnteln; on false = keine Reise.
type travelKey struct {
	lang         string
	on, up       bool
	depth, tenth int
}

// newTravelKey liest den Stufenwechsel der Welt.
func newTravelKey(lang string, t *sim.Travel) travelKey {
	if t == nil {
		return travelKey{lang: lang}
	}
	return travelKey{lang, true, t.Via == "stairsUp", t.ToDepth, int(math.Ceil(math.Max(0, math.Min(1, t.Progress)) * 10))}
}

// travelText: Ziel der Reise und ein Balken aus zehn Kästchen.
func travelText(k travelKey) string {
	if !k.on {
		return ""
	}
	s := tf(k.lang, pick(k.up, "hud.travelUp", "hud.travelDown"), "depth", k.depth)
	return s + "  " + strings.Repeat("▮", k.tenth) + strings.Repeat("▯", 10-k.tenth)
}

// placeKey: Spieler und Biom der Stufe.
type placeKey struct {
	lang, id, name string
	player         int
}

// placeText ist der Ort des Spielers: „P1 · Oberwelt (Wald)“.
func placeText(k placeKey) string {
	return tf(k.lang, "hud.place", "p", k.player+1, "stage", nameOf(k.lang, "biome", k.id, k.name))
}
