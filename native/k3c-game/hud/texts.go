package hud

// Spieltexte des HUD (Spiegel von src/core/texts.de.ts und texts.en.ts, nur die Schlüssel, die das HUD zeigt). Einzige
// Stelle mit deutschen und englischen Texten im Paket (textrule_test.go prüft das). Deutsch ist Standard und Rückfall.

import (
	"strconv"
	"strings"

	"k3c/native/k3c-game/view"
)

var textsDE = map[string]string{
	"hud.join.pad": "A drücken zum Beitreten", "hud.join.keyboard": "Leertaste drücken zum Beitreten",
	"hud.join.keyboard2": "Enter drücken zum Beitreten",
	"hud.travelUp":       "Aufstieg in Tiefe {depth}", "hud.travelDown": "Abstieg in Tiefe {depth}",
	"hud.skillPoints": "Skill-Punkte {n}", "hud.wave": "Welle {wave}: {n} Gegner",
	"hud.downShort": "{s} s", "hud.place": "P{p} · {stage}",
	"hud.clockDay": "Tag {day}  ·  Dämmerung in {left}", "hud.clockDusk": "Dämmerung  ·  Nacht in {left}",
	"hud.clockNight": "Nacht {day}  ·  Morgen in {left}", "hud.clockDawn": "Tag {day}  ·  Sonnenaufgang in {left}",
	"hud.aggression": "  ·  Aggression {pct}%", "hud.saved": "  ·  gesichert",
	"ev.dusk": "Nacht naht!", "ev.night": "Nacht {day}", "ev.dawn": "Tag {day}  –  die Sonne geht auf",
	"ev.wave": "Portal öffnet sich!\n{n} Gegner kommen", "ev.chest": "P{p} findet {gold} Gold",
	"ev.skillPoint": "Skill-Punkt gefunden!", "ev.armed": "Neuer Bogenschütze", "ev.built": "{name} fertig",
	"ev.destroyed": "{name} zerstört!", "ev.goldStolen": "P{p}: {amount} Gold geklaut!", "ev.playerDown": "P{p} ist gefallen",
	"ev.castleFallen": "Die Burg ist gefallen!\nGebäude, Truppen und die Hälfte der Vorräte sind verloren",
	"site.wall":       "Mauer", "site.tower": "Turm", "site.workshop": "Werkstatt", "site.ship": "Schiff", "site.storage": "Lager",
	"site.stairsUp": "Treppe hoch", "site.stairsDown": "Treppe runter", "site.nursery": "Baumschule", "site.barracks": "Kaserne",
	"site.tavern": "Taverne", "site.healer": "Heilplatz", "site.smithy": "Schmiede", "site.armory": "Rüstkammer",
	"res.wood": "Holz", "res.stone": "Stein", "res.copper": "Kupfer", "res.iron": "Eisen", "res.crystal": "Kristall",
	"stock.amount": "{res} {n}", "stock.ofMax": "{res} {n}/{max}", "stock.full": "{res} {n}/{max} voll", "stock.fullShort": "{res} voll",
	"cost.item": "{n} {res}", "cost.gold": "{paid}/{gold} Gold", "hub.level": "Hub-Stufe {n}", "hub.levelShort": "Hub {n}",
	"hub.upgrade": "Hub-Stufe {n} → {next}: {cost}", "hub.upgradeShort": "Hub {n}→{next}: {cost}",
	"hub.upgradeStep":      "Hub-Stufe {n} → {next}, Ausbau {step}/{steps}: {cost}",
	"hub.upgradeStepShort": "Hub {n}→{next} {step}/{steps}: {cost}",
	"limit.text":           "Kämpfer {n} von {max}", "limit.full": "Kämpfer {n} von {max}: Limit erreicht",
	"limit.short": "Limit {n}/{max}", "limit.fullShort": "Limit {n}/{max} voll",
	"prof.miner": "Bergmann", "prof.builder": "Baumeister", "prof.craftsman": "Handwerker", "prof.unknown": "Beruf",
	"prof.minerShort": "Bergm.", "prof.builderShort": "Baum.", "prof.craftsmanShort": "Handw.", "prof.unknownShort": "Beruf",
	"merchant.text": "Händler: {res} · bis Tag {day}", "merchant.paid": "Händler: {res} · bis Tag {day} · Kauf {paid} Gold",
	"merchant.short": "Händler {res}",
	"biome.forest":   "Oberwelt (Wald)", "biome.cave": "Tiefe 1 (Höhle)", "biome.mine": "Tiefe 2 (Mine)",
	"skill.title": "Skills  ·  {n} Punkte frei", "skill.learned": "gelernt", "skill.learnable": "lernen", "skill.locked": "–",
	"skill.passive": "passiv", "skill.respec": "Alle Skills zurücksetzen (Respec)", "skill.free": "–", "skill.ready": "bereit",
	"skill.hint.pad":      "◀ ▶ wählen · A bestätigen · {menu} schließen",
	"skill.hint.keyboard": "{move} wählen · {ok} bestätigen · {menu} schließen",
	"hint.space":          "Leertaste", "hint.left": "Pfeil links", "hint.right": "Pfeil rechts", "hint.train": "{name} ausbilden", "hint.buy": "{name} kaufen", "hint.sell": "{name} verkaufen",
	"hint.lair":  "Bau des Endbosses: Ankunft löst ihn aus",
	"guide.coin": "Hinlaufen: Münze aufheben", "guide.dusk": "Die Nacht naht! Zurück zur Burg",
	"guide.recruit": "Münze geben, er wird Bauer",
	"boss.phase":    "Phase {n} von {of}", "boss.goblinLeader": "Goblin-Anführer", "boss.trollKing": "Troll-König",
	"boss.ratQueen": "Ratten-Königin", "boss.lavaGolem": "Lava-Golem", "boss.shardTitan": "Splitter-Titan",
	"boss.crystalHeartWarden": "Kristallherz-Wächter",
	"event.timed":             "{name}  ·  {left}", "event.fullMoon": "Vollmond", "event.bloodMoon": "Blutmond",
	"event.merchantRaid": "Händler-Überfall",
}

var textsEN = map[string]string{
	"hud.join.pad": "Press A to join", "hud.join.keyboard": "Press Space to join", "hud.join.keyboard2": "Press Enter to join",
	"hud.travelUp": "Going up to depth {depth}", "hud.travelDown": "Going down to depth {depth}",
	"hud.skillPoints": "Skill points {n}", "hud.wave": "Wave {wave}: {n} enemies",
	"hud.downShort": "{s} s", "hud.place": "P{p} · {stage}",
	"hud.clockDay": "Day {day}  ·  Dusk in {left}", "hud.clockDusk": "Dusk  ·  Night in {left}",
	"hud.clockNight": "Night {day}  ·  Morning in {left}", "hud.clockDawn": "Day {day}  ·  Sunrise in {left}",
	"hud.aggression": "  ·  Aggression {pct}%", "hud.saved": "  ·  saved",
	"ev.dusk": "Night is coming!", "ev.night": "Night {day}", "ev.dawn": "Day {day}  –  the sun rises",
	"ev.wave": "Portal opens!\n{n} enemies are coming", "ev.chest": "P{p} finds {gold} gold",
	"ev.skillPoint": "Skill point found!", "ev.armed": "New archer", "ev.built": "{name} built",
	"ev.destroyed": "{name} destroyed!", "ev.goldStolen": "P{p}: {amount} gold stolen!", "ev.playerDown": "P{p} is down",
	"ev.castleFallen": "The castle has fallen!\nBuildings, troops and half the supplies are lost",
	"site.wall":       "Wall", "site.tower": "Tower", "site.workshop": "Workshop", "site.ship": "Ship", "site.storage": "Storage",
	"site.stairsUp": "Stairs up", "site.stairsDown": "Stairs down", "site.nursery": "Baumschule", "site.barracks": "Barracks",
	"site.tavern": "Tavern", "site.healer": "Healer", "site.smithy": "Smithy", "site.armory": "Armory",
	"res.wood": "Wood", "res.stone": "Stone", "res.copper": "Copper", "res.iron": "Iron", "res.crystal": "Crystal",
	"stock.amount": "{res} {n}", "stock.ofMax": "{res} {n}/{max}", "stock.full": "{res} {n}/{max} full", "stock.fullShort": "{res} full",
	"cost.item": "{n} {res}", "cost.gold": "{paid}/{gold} gold", "hub.level": "Hub level {n}", "hub.levelShort": "Hub {n}",
	"hub.upgrade": "Hub level {n} → {next}: {cost}", "hub.upgradeShort": "Hub {n}→{next}: {cost}",
	"hub.upgradeStep":      "Hub level {n} → {next}, step {step}/{steps}: {cost}",
	"hub.upgradeStepShort": "Hub {n}→{next} {step}/{steps}: {cost}",
	"limit.text":           "Fighters {n} of {max}", "limit.full": "Fighters {n} of {max}: limit reached",
	"limit.short": "Limit {n}/{max}", "limit.fullShort": "Limit {n}/{max} full",
	"prof.miner": "Miner", "prof.builder": "Builder", "prof.craftsman": "Craftsman", "prof.unknown": "Profession",
	"prof.minerShort": "Min.", "prof.builderShort": "Bld.", "prof.craftsmanShort": "Cft.", "prof.unknownShort": "Prof.",
	"merchant.text": "Merchant: {res} · until day {day}", "merchant.paid": "Merchant: {res} · until day {day} · buying {paid} gold",
	"merchant.short": "Merchant {res}",
	"biome.forest":   "Overworld (forest)", "biome.cave": "Depth 1 (cave)", "biome.mine": "Depth 2 (mine)",
	"skill.title": "Skills  ·  {n} points free", "skill.learned": "learned", "skill.learnable": "learn", "skill.locked": "–",
	"skill.passive": "passive", "skill.respec": "Reset all skills (respec)", "skill.free": "–", "skill.ready": "ready",
	"skill.hint.pad":      "◀ ▶ select · A confirm · {menu} close",
	"skill.hint.keyboard": "{move} select · {ok} confirm · {menu} close",
	"hint.space":          "Space", "hint.left": "Left arrow", "hint.right": "Right arrow", "hint.train": "train {name}", "hint.buy": "buy {name}", "hint.sell": "sell {name}",
	"hint.lair":  "boss lair: arriving wakes the final boss",
	"guide.coin": "Walk over it: pick up the coin", "guide.dusk": "Night is coming! Back to the castle",
	"guide.recruit": "Give a coin, he becomes a peasant",
	"boss.phase":    "Phase {n} of {of}", "boss.goblinLeader": "Goblin Leader", "boss.trollKing": "Troll King",
	"boss.ratQueen": "Rat Queen", "boss.lavaGolem": "Lava Golem", "boss.shardTitan": "Shard Titan",
	"boss.crystalHeartWarden": "Crystal Heart Warden",
	"event.timed":             "{name}  ·  {left}", "event.fullMoon": "Full moon", "event.bloodMoon": "Blood moon",
	"event.merchantRaid": "Merchant raid",
}

// tr liefert den Text zum Schlüssel in der Sprache lang (fehlt er: Deutsch, fehlt auch der: der Schlüssel selbst).
func tr(lang, key string) string {
	if lang == view.LanguageEnglish {
		if s, ok := textsEN[key]; ok {
			return s
		}
	}
	if s, ok := textsDE[key]; ok {
		return s
	}
	return key
}

// has meldet, ob es den Schlüssel gibt (wie `key in de` im Browser).
func has(key string) bool {
	_, ok := textsDE[key]
	return ok
}

// nameOf ist der Name aus den Texten (`group.id`), sonst fallback (wie texts.ts › nameOf).
func nameOf(lang, group, id, fallback string) string {
	if key := group + "." + id; has(key) {
		return tr(lang, key)
	}
	return fallback
}

// tf liefert den Text zum Schlüssel mit ersetzten Platzhaltern; kv sind Paare Name, Wert (string, int oder float64).
// Ein unbekannter Platzhalter bleibt stehen.
func tf(lang, key string, kv ...any) string {
	return fill(tr(lang, key), kv...)
}

// fill ersetzt `{name}` in tpl durch die Werte aus kv (Paare Name, Wert).
func fill(tpl string, kv ...any) string {
	if !strings.Contains(tpl, "{") {
		return tpl
	}
	var b strings.Builder
	for {
		i := strings.IndexByte(tpl, '{')
		j := strings.IndexByte(tpl[max(i, 0):], '}')
		if i < 0 || j < 0 {
			b.WriteString(tpl)
			return b.String()
		}
		name := tpl[i+1 : i+j]
		b.WriteString(tpl[:i])
		if v, ok := lookup(kv, name); ok {
			b.WriteString(v)
		} else {
			b.WriteString(tpl[i : i+j+1])
		}
		tpl = tpl[i+j+1:]
	}
}

// lookup sucht den Wert zu name in den Paaren kv.
func lookup(kv []any, name string) (string, bool) {
	for k := 0; k+1 < len(kv); k += 2 {
		if kv[k] == name {
			return str(kv[k+1]), true
		}
	}
	return "", false
}

// str schreibt einen Wert als Text (ganze Zahlen ohne Nachkommastellen).
func str(v any) string {
	switch x := v.(type) {
	case string:
		return x
	case int:
		return strconv.Itoa(x)
	case float64:
		return strconv.FormatFloat(x, 'f', -1, 64)
	}
	return ""
}
