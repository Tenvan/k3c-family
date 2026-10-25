package sim

import (
	"errors"
	"math"
	"slices"
)

// Dev-Aktionen für den Kampf (B-603): Boss, Phasenwechsel des Endbosses, Events und Nacht sofort. Nur für den
// Dev-Mode des Raums (engine/room/dev_combat.go), keine Spielregel. Sie nutzen die Abläufe des Spiels (spawnBoss,
// endbossPhase, Nachtbeginn, Händler-Überfall) und damit deren rng-Strom, sonst kein rng.

// ErrDevBossPresent: In der Stufe lebt schon ein Boss oder der Endboss-Kampf der Insel läuft.
var ErrDevBossPresent = errors.New("boss schon da")

// errDevInvalid: unbekannte Boss-Id, unbekannter Spieler oder Stufe ohne Portal.
var errDevInvalid = errors.New("dev-aktion ungültig")

// DevSpawnBoss stellt den Boss id (data/bosses.json, Mini- oder Endboss) in der Stufe des Spielers index an das
// nächste Portal; ein Endboss beginnt dort seinen Kampf mit Phase 1. Besiegte Bosse kommen trotzdem.
func DevSpawnBoss(isl *Island, index int, id string) error {
	stage, b := isl.StageOf(index), bossByID(id)
	if stage < 0 || b == nil {
		return errDevInvalid
	}
	w := isl.Stages[stage]
	if isl.endboss != nil || slices.ContainsFunc(w.Enemies, func(e *Enemy) bool { return e.Boss }) {
		return ErrDevBossPresent
	}
	x, ok := nearestPortal(w, index)
	if !ok {
		return errDevInvalid
	}
	e := spawnBoss(w, b, x)
	if b.Kind == "end" {
		armBoss(e, *b.Phases[0].Ability)
		isl.endboss = &endbossFight{boss: e, home: w}
	}
	return nil
}

// nearestPortal: das Portal der Stufe am nächsten beim Spieler index (bei Gleichstand das erste); false: keins.
func nearestPortal(w *World, index int) (float64, bool) {
	i := slices.IndexFunc(w.Players, func(p *Player) bool { return p.Index == index })
	if i < 0 || len(w.Portals) == 0 {
		return 0, false
	}
	best := w.Portals[0]
	for _, x := range w.Portals[1:] {
		if math.Abs(x-w.Players[i].X) < math.Abs(best-w.Players[i].X) {
			best = x
		}
	}
	return best, true
}

// DevBossPhase wechselt den laufenden Endboss in seine nächste Phase: Seine HP fallen knapp unter deren Schwelle
// (höher bleiben sie nie), der Wechsel samt Meldung läuft über endbossPhase. Liefert die neue Phase (ab 1);
// false: kein Endboss-Kampf oder schon die letzte Phase.
func DevBossPhase(isl *Island) (int, bool) {
	f := isl.endboss
	if f == nil || f.home == nil {
		return 0, false
	}
	b := bossByID(f.boss.Kind)
	if f.phase+1 >= len(b.Phases) {
		return 0, false
	}
	e := f.boss
	e.HP = math.Max(1, math.Min(e.HP, math.Floor(b.Phases[f.phase+1].BelowHPPercent*e.MaxHP/100)-1))
	endbossPhase(f.home, f, 0)
	return f.phase + 1, true
}

// DevStartEvent startet ein Event aus data/events.json: fullMoon und bloodMoon laufen in der nächsten Nacht, zu deren
// Beginn alle Stufen springen (DevSetPhase); merchantRaid lässt den Händler in seiner Stufe ankommen, wenn er fehlt,
// und überfällt seinen Besuch. false: unbekannte Id, keine Händler-Stufe oder der Besuch wird schon überfallen.
func DevStartEvent(isl *Island, id string) bool {
	switch id {
	case "fullMoon", "bloodMoon":
		if !DevSetPhase(isl, "night") {
			return false
		}
		if isl.Dev.Nights == nil {
			isl.Dev.Nights = map[string]int{}
		}
		isl.Dev.Nights[id] = isl.Stages[0].Cycle.Day // die Nacht gehört zum Tag davor
		return true
	case "merchantRaid":
		return devMerchantRaid(isl)
	}
	return false
}

// devMerchantRaid: Händler-Überfall in der Stufe des Händlers (hub.merchant.onlyDepth).
func devMerchantRaid(isl *Island) bool {
	for _, w := range isl.Stages {
		if w.Biome.Depth != hubOf(w).Merchant.OnlyDepth {
			continue
		}
		if w.Merchant == nil {
			merchantArrive(w)
		}
		if w.Merchant.Raid {
			return false
		}
		raidMerchant(w)
		return true
	}
	return false
}
