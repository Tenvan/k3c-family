package sim

import (
	"fmt"
	"io/fs"
)

// Events im Wertesatz der Welt (docs/rules/welten.md § 1.1, B-982): `events` ist eine erlaubte Override-Datei. Die Stufen
// einer Insel rechnen mit WorldValues.Events statt mit der eingebetteten Basis (nightEvents). Abstand 0 (everyNights,
// everyVisits) schaltet ein Event aus; neue Events stehen in der Basis mit Abstand 0 und werden je Welt eingeschaltet.

// eventsFile ist der Lese-Typ von data/events.json.
type eventsFile struct{ Events []NightEvent }

// loadEvents liest data/events.json aus fsys nach dst.
func loadEvents(fsys fs.FS, dst *[]NightEvent) error {
	var v eventsFile
	if err := loadFrom(fsys, "events.json", &v); err != nil {
		return err
	}
	*dst = v.Events
	return nil
}

// night liefert das Event id aus dem Wertesatz der Stufe (nil, wenn es fehlt).
func (w *World) night(id string) *NightEvent { return findEvent(w.values().Events, id) }

// checkNightEvents prüft die Events gegen die Gegnerarten enemies: Vollmond, Blutmond und Händler-Überfall vorhanden,
// Abstände ≥ 0 (0 = aus), Gegnerarten bekannt, Faktoren > 0, Mengen ≥ 0. Die Meldungen nennen die Datei und das Event.
func checkNightEvents(events []NightEvent, enemies map[string]EnemyData) error {
	full, blood, raid := findEvent(events, "fullMoon"), findEvent(events, "bloodMoon"), findEvent(events, "merchantRaid")
	if full == nil || blood == nil || raid == nil {
		return fmt.Errorf("data/events.json: fullMoon, bloodMoon und merchantRaid fehlen")
	}
	for _, ev := range events {
		if ev.EveryNights < 0 || ev.EveryVisits < 0 {
			return fmt.Errorf("data/events.json: %s: Abstand muss >= 0 sein (0 = aus)", ev.ID)
		}
	}
	if _, ok := enemies[full.Kind]; !ok {
		return fmt.Errorf("data/events.json: fullMoon nennt unbekannte Gegner %q, %q", full.Kind, full.Elite)
	}
	if _, ok := enemies[full.Elite]; !ok {
		return fmt.Errorf("data/events.json: fullMoon nennt unbekannte Gegner %q, %q", full.Kind, full.Elite)
	}
	switch {
	case full.WolfShare < 0 || full.RewardGold < 0 || raid.RewardMaterial < 0:
		return fmt.Errorf("data/events.json: Anteil und Belohnung müssen >= 0 sein")
	case blood.DamageFactor <= 0 || blood.DropFactor <= 0:
		return fmt.Errorf("data/events.json: bloodMoon braucht Faktoren > 0")
	}
	return nil
}
