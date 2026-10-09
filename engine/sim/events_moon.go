package sim

import (
	"fmt"
	"math"

	"k3c/data"
)

// Events der Nacht (B-131, docs/rules/bosse.md § 2): Vollmond jede 7. Nacht (mehr Wölfe, ein Alpha-Wolf, Belohnung bei
// Tagesanbruch), Blutmond jede 13. (Schaden und Gold-Drop aller Gegner höher). Werte in data/events.json. Sie gelten für
// die Nacht des globalen Zyklus und nur auf Inseln; Campaign und einzelne Welten (Golden-Läufe) bleiben unverändert.

// NightEvent ist ein Event aus data/events.json; nicht genutzte Felder bleiben 0.
type NightEvent struct {
	ID           string
	EveryNights  int
	Kind         string  // fullMoon: Gegnerart, deren Anteil steigt
	WolfShare    float64 // fullMoon: zusätzlich dieser Anteil der gezogenen Gegner der Art Kind (aufgerundet)
	Elite        string  // fullMoon: genau ein Gegner dieser Art zusätzlich
	RewardGold   int     // fullMoon: Münzen am Hub bei Tagesanbruch, wenn die Burg hielt
	DamageFactor float64 // bloodMoon: Schaden beim Angriff
	DropFactor   float64 // bloodMoon: Gold-Drop beim Tod
}

var nightEvents = loadNightEvents()

func loadNightEvents() []NightEvent {
	var v struct{ Events []NightEvent }
	if err := loadFrom(data.Files, "events.json", &v); err != nil {
		panic(err)
	}
	if err := checkNightEvents(v.Events); err != nil {
		panic(err)
	}
	return v.Events
}

// checkNightEvents: Vollmond und Blutmond vorhanden, Rhythmen > 0, Gegnerarten bekannt, Faktoren > 0.
func checkNightEvents(events []NightEvent) error {
	full, blood := findEvent(events, "fullMoon"), findEvent(events, "bloodMoon")
	switch {
	case full == nil || blood == nil:
		return fmt.Errorf("data/events.json: fullMoon und bloodMoon fehlen")
	case full.EveryNights <= 0 || blood.EveryNights <= 0:
		return fmt.Errorf("data/events.json: everyNights muss > 0 sein")
	case !knownEnemy(full.Kind) || !knownEnemy(full.Elite):
		return fmt.Errorf("data/events.json: fullMoon nennt unbekannte Gegner %q, %q", full.Kind, full.Elite)
	case blood.DamageFactor <= 0 || blood.DropFactor <= 0:
		return fmt.Errorf("data/events.json: bloodMoon braucht Faktoren > 0")
	}
	return nil
}

func knownEnemy(kind string) bool {
	_, ok := enemyData[kind]
	return ok
}

func findEvent(events []NightEvent, id string) *NightEvent {
	for i := range events {
		if events[i].ID == id {
			return &events[i]
		}
	}
	return nil
}

func fullMoon() *NightEvent  { return findEvent(nightEvents, "fullMoon") }
func bloodMoon() *NightEvent { return findEvent(nightEvents, "bloodMoon") }

// eventInNight: Das Event läuft in Nacht n der Insel (Nacht n = Cycle.Day n, die Nacht gehört zum Tag davor).
func eventInNight(w *World, ev *NightEvent, n int) bool {
	return w.island != nil && n > 0 && n%ev.EveryNights == 0
}

// eventActive: Das Event läuft jetzt (Phase night).
func eventActive(w *World, ev *NightEvent) bool {
	return w.Cycle.Phase == "night" && eventInNight(w, ev, w.Cycle.Day)
}

// eventStage: Die Stufe meldet die Events der Insel (Stufe 0), damit jedes Event je Nacht genau einmal erscheint.
func eventStage(w *World) bool { return w.island != nil && len(w.island.Stages) > 0 && w.island.Stages[0] == w }

// nightEventsStart meldet bei Einbruch der Nacht die Events dieser Nacht (stepCycle).
func nightEventsStart(w *World) {
	if !eventStage(w) {
		return
	}
	for i := range nightEvents {
		if ev := &nightEvents[i]; eventActive(w, ev) {
			emit(w, "eventStarted", Event{"event": ev.ID, "day": w.Cycle.Day})
		}
	}
}

// nightEventsEnd meldet bei Tagesanbruch das Ende der Events der vergangenen Nacht und zahlt die Vollmond-Belohnung,
// wenn die Burg der Stufe in dieser Nacht nicht gefallen ist (stepCycle, Phase day).
func nightEventsEnd(w *World) {
	night := w.Cycle.Day - 1
	if !eventStage(w) {
		return
	}
	for i := range nightEvents {
		if ev := &nightEvents[i]; eventInNight(w, ev, night) {
			emit(w, "eventEnded", Event{"event": ev.ID, "day": night})
		}
	}
	if full := fullMoon(); eventInNight(w, full, night) && w.castleFellNight != night {
		scatterCoins(w, w.HubX, full.RewardGold)
	}
}

// fullMoonExtra: In einer Vollmondnacht kommen zur Welle einer Tag/Nacht-Stufe zusätzliche Gegner der Art Kind
// (Anteil der gezogenen, aufgerundet) und genau ein Elite-Gegner (startWave).
func fullMoonExtra(w *World, plan []spawnOrder) []spawnOrder {
	full := fullMoon()
	if w.Biome.Cycle.Type != "dayNight" || !eventActive(w, full) {
		return nil
	}
	n := 0
	for _, s := range plan {
		if s.kind == full.Kind {
			n++
		}
	}
	picks := []string{full.Elite}
	for range int(math.Ceil(float64(n) * full.WolfShare)) {
		picks = append(picks, full.Kind)
	}
	return spawnOrders(picks, w.Portals, w.rng)
}

// moonDamage ist der Schaden eines Gegnerangriffs, im Blutmond mit dem Faktor der Daten (attack).
func moonDamage(w *World, damage float64) float64 {
	if blood := bloodMoon(); eventActive(w, blood) {
		return damage * blood.DamageFactor
	}
	return damage
}

// moonDrop ist der Gold-Drop eines besiegten Gegners, im Blutmond mit dem Faktor der Daten (enemyDrop).
func moonDrop(w *World, gold int) int {
	if blood := bloodMoon(); eventActive(w, blood) {
		return int(math.Round(float64(gold) * blood.DropFactor))
	}
	return gold
}
