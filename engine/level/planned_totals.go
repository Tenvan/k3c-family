package level

import (
	"fmt"
	"slices"
)

// counts zählt feste Objekte je Art, die für die Biom-Summen zählen.
type counts struct{ portal, chest, camp, vein int }

func (c *counts) add(kind string) {
	switch kind {
	case "portal":
		c.portal++
	case "chest":
		c.chest++
	case "recruitCamp":
		c.camp++
	}
}

func (c counts) of(kind string) int {
	if kind == "chest" {
		return c.chest
	}
	return c.camp
}

// checkTotals prüft die Biom-Summen inklusive Planung und ob für den Rest der Level genug freie Plätze bleiben
// (docs/rules/geplante-chunks.md § 3 und § 5). Gerechnet wird mit der kürzesten Länge und dem ungünstigsten Würfelwurf.
func checkTotals(b Biome, g geometry, plan []PlannedChunk, all []placed) error {
	var count counts
	for _, p := range all {
		count.add(p.kind)
		if b.Veins.Count > 0 && p.kind == b.Veins.Kind {
			count.vein++
		}
	}
	if count.portal > b.Portals.Count {
		return fmt.Errorf("%d feste Portale, das Biom hat %d", count.portal, b.Portals.Count)
	}
	for _, kind := range eventKinds {
		if want := b.EventChunks[kind]; count.of(kind) > want.Max {
			return fmt.Errorf("%d feste %s, das Biom erlaubt höchstens %d", count.of(kind), kind, want.Max)
		}
	}
	if b.Veins.Count > 0 && count.vein > b.Veins.Count {
		return fmt.Errorf("%d feste Adern (%s), das Biom hat %d", count.vein, b.Veins.Kind, b.Veins.Count)
	}
	return checkRoom(b, g, plan, count)
}

// checkRoom: Die übrigen Portale, Ereignisse (im ungünstigsten Fall bis zum Maximum) und Adern brauchen freie Chunks.
func checkRoom(b Biome, g geometry, plan []PlannedChunk, count counts) error {
	free := 2*g.usable - len(plan)
	remainingPortals := b.Portals.Count - count.portal
	if n := eligibleFree(b, g, plan); n < remainingPortals {
		return fmt.Errorf("kein Platz mehr für %d weitere Portale (frei und weit genug vom Hub: %d)", remainingPortals, n)
	}
	events := 0
	for _, kind := range eventKinds {
		events += max(0, b.EventChunks[kind].Max-count.of(kind))
	}
	remainingVeins := 0
	if b.Veins.Count > 0 {
		remainingVeins = b.Veins.Count - count.vein
	}
	if need := remainingPortals + events + remainingVeins; free < need {
		return fmt.Errorf("kein Platz mehr: %d freie Chunks, gebraucht bis zu %d (Portale %d, Ereignisse %d, Adern %d)", free, need, remainingPortals, events, remainingVeins)
	}
	return nil
}

// eligibleFree zählt freie Plätze, deren Mitte mindestens minDistanceFromHubUnits vom Hub entfernt liegt.
func eligibleFree(b Biome, g geometry, plan []PlannedChunk) int {
	n := 0
	for slot := 1; slot <= g.usable; slot++ {
		if g.dist(slot, g.cw/2) < b.Portals.MinDistanceFromHubUnits {
			continue
		}
		for _, s := range []int{slot, -slot} {
			if !slices.ContainsFunc(plan, func(c PlannedChunk) bool { return c.Slot == s }) {
				n++
			}
		}
	}
	return n
}
