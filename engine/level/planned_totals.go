package level

import "fmt"

// checkTotals prüft die Biom-Summen inklusive Planung und ob für den Rest der Level genug freie Plätze bleiben
// (docs/rules/geplante-chunks.md § 3 und § 5). Gerechnet wird mit der kürzesten Länge und dem ungünstigsten Würfelwurf.
func checkTotals(b Biome, g geometry, plan []PlannedChunk, all []placed) error {
	count := map[string]int{}
	for _, p := range all {
		count[p.kind]++
	}
	if count["portal"] > b.Portals.Count {
		return fmt.Errorf("%d feste Portale, das Biom hat %d", count["portal"], b.Portals.Count)
	}
	for _, kind := range eventKinds {
		if want := b.EventChunks[kind]; count[kind] > want.Max {
			return fmt.Errorf("%d feste %s, das Biom erlaubt höchstens %d", count[kind], kind, want.Max)
		}
	}
	if b.Veins.Count > 0 && count[b.Veins.Kind] > b.Veins.Count {
		return fmt.Errorf("%d feste Adern (%s), das Biom hat %d", count[b.Veins.Kind], b.Veins.Kind, b.Veins.Count)
	}
	return checkRoom(b, g, plan, count)
}

// checkRoom: Die übrigen Portale, Ereignisse (im ungünstigsten Fall bis zum Maximum) und Adern brauchen freie Chunks.
func checkRoom(b Biome, g geometry, plan []PlannedChunk, count map[string]int) error {
	taken := map[int]bool{}
	for _, c := range plan {
		taken[c.Slot] = true
	}
	free := 2*g.usable - len(plan)
	remainingPortals := b.Portals.Count - count["portal"]
	if n := eligibleFree(b, g, taken); n < remainingPortals {
		return fmt.Errorf("kein Platz mehr für %d weitere Portale (frei und weit genug vom Hub: %d)", remainingPortals, n)
	}
	events := 0
	for _, kind := range eventKinds {
		events += max(0, b.EventChunks[kind].Max-count[kind])
	}
	remainingVeins := 0
	if b.Veins.Count > 0 {
		remainingVeins = b.Veins.Count - count[b.Veins.Kind]
	}
	if need := remainingPortals + events + remainingVeins; free < need {
		return fmt.Errorf("kein Platz mehr: %d freie Chunks, gebraucht bis zu %d (Portale %d, Ereignisse %d, Adern %d)", free, need, remainingPortals, events, remainingVeins)
	}
	return nil
}

// eligibleFree zählt freie Plätze, deren Mitte mindestens minDistanceFromHubUnits vom Hub entfernt liegt.
func eligibleFree(b Biome, g geometry, taken map[int]bool) int {
	n := 0
	for slot := 1; slot <= g.usable; slot++ {
		if g.dist(slot, g.cw/2) < b.Portals.MinDistanceFromHubUnits {
			continue
		}
		for _, s := range []int{slot, -slot} {
			if !taken[s] {
				n++
			}
		}
	}
	return n
}
