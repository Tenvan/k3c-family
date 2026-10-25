package room

import (
	"errors"

	"k3c/engine/sim"
)

// devCombat führt die Kampf-Aktionen aus (B-603) und liefert die Log-Felder: boss stellt den Boss an das nächste
// Portal der Stufe des Monarchen von slot, bossPhase schaltet den laufenden Endboss eine Phase weiter, event startet
// ein Event (Mond-Events in der nächsten Nacht, zu deren Beginn alle Stufen springen), night springt zur Nacht.
// Lebt schon ein Boss, ist boss abgelehnt (bad_request) mit Hinweis im Log.
func (r *Room) devCombat(who string, a DevAction, player func(int) (int, bool)) ([]any, bool) {
	switch a.Action {
	case "boss":
		idx, ok := devIndex(a, player)
		if !ok {
			return nil, false
		}
		err := sim.DevSpawnBoss(r.isl, idx, a.Boss)
		if errors.Is(err, sim.ErrDevBossPresent) {
			r.log().Warn("🚫 Dev-Aktion abgelehnt: Boss schon da", "device", who, "aktion", a.Action, "boss", a.Boss)
		}
		return []any{"slot", *a.Slot, "boss", a.Boss}, err == nil
	case "bossPhase":
		phase, ok := sim.DevBossPhase(r.isl)
		return []any{"phase", phase}, ok
	case "event":
		return []any{"event", a.Event}, sim.DevStartEvent(r.isl, a.Event)
	case "night":
		return []any{"phase", "night"}, sim.DevSetPhase(r.isl, "night")
	}
	return nil, false
}
