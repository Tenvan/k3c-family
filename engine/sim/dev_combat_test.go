package sim

import (
	"errors"
	"slices"
	"testing"
)

// B-603/AC-01: Dev-Aktionen boss, bossPhase und event wirken in der Simulation.

// Der Miniboss erscheint am nächsten Portal, ein zweiter wird abgelehnt, unbekannte Ids und Spieler auch.
func TestDevSpawnBoss(t *testing.T) {
	isl := mustIsland(t, "dev-boss", []int{0})
	p := AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	if err := DevSpawnBoss(isl, p.Index, "goblinLeader"); err != nil {
		t.Fatal(err)
	}
	b := bossesIn(w)
	if len(b) != 1 || b[0].Kind != "goblinLeader" || !slices.Contains(w.Portals, b[0].X) {
		t.Fatalf("Bosse %v", b)
	}
	if err := DevSpawnBoss(isl, p.Index, "trollKing"); !errors.Is(err, ErrDevBossPresent) {
		t.Fatalf("zweiter Boss: %v", err)
	}
	if err := DevSpawnBoss(isl, p.Index, "drache"); err == nil || errors.Is(err, ErrDevBossPresent) {
		t.Fatalf("unbekannte Id: %v", err)
	}
	if err := DevSpawnBoss(isl, 7, "goblinLeader"); err == nil {
		t.Fatal("unbekannter Spieler angenommen")
	}
}

// Ein Endboss außerhalb seines Baus kämpft in der Stufe des Monarchen; bossPhase schaltet je Aufruf eine Phase weiter.
func TestDevBossPhase(t *testing.T) {
	isl := mustIsland(t, "dev-phase-boss", []int{0})
	p := AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	if _, ok := DevBossPhase(isl); ok {
		t.Fatal("Phase ohne Endboss")
	}
	if err := DevSpawnBoss(isl, p.Index, wardenID); err != nil {
		t.Fatal(err)
	}
	e := bossesIn(w)[0]
	for want := 2; want <= 3; want++ {
		phase, ok := DevBossPhase(isl)
		if !ok || phase != want || isl.endboss.phase+1 != want {
			t.Fatalf("Phase %d (%v), erwartet %d", phase, ok, want)
		}
		if count(w.Events, "bossPhase") != want-1 {
			t.Fatalf("%d bossPhase-Meldungen", count(w.Events, "bossPhase"))
		}
	}
	if _, ok := DevBossPhase(isl); ok {
		t.Fatal("über die letzte Phase hinaus")
	}
	StepIsland(isl, nil, dt)
	if e.HP <= 0 || !slices.Contains(w.Enemies, e) || e.Phase != 3 {
		t.Fatalf("Endboss nach dem Tick: HP %.0f, Phase %d", e.HP, e.Phase)
	}
}

// Blutmond: Sprung zur nächsten Nacht, das Event läuft genau in ihr (Meldung, Zustand) und endet beim Morgen.
func TestDevStartEventMond(t *testing.T) {
	isl := mustIsland(t, "dev-mond", []int{0})
	AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	StepIsland(isl, nil, dt)
	if !DevStartEvent(isl, "bloodMoon") {
		t.Fatal("bloodMoon abgelehnt")
	}
	StepIsland(isl, nil, dt)
	if w.Cycle.Phase != "night" || !eventActive(w, bloodMoon()) || eventActive(w, fullMoon()) {
		t.Fatalf("Phase %s, Blutmond %v", w.Cycle.Phase, eventActive(w, bloodMoon()))
	}
	if !slices.ContainsFunc(w.Events, func(e Event) bool { return e["type"] == "eventStarted" && e["event"] == "bloodMoon" }) {
		t.Fatalf("kein eventStarted: %v", w.Events)
	}
	if len(w.NightEvents) != 1 || w.NightEvents[0].ID != "bloodMoon" {
		t.Fatalf("Zustand %v", w.NightEvents)
	}
	DevSetPhase(isl, "dawn") // Events enden mit dem Morgengrauen (W12, nightEventsEnd)
	StepIsland(isl, nil, dt)
	if eventActive(w, bloodMoon()) || !slices.ContainsFunc(w.Events, func(e Event) bool { return e["type"] == "eventEnded" }) {
		t.Fatalf("Blutmond endet nicht: %v", w.Events)
	}
	if DevStartEvent(isl, "neumond") {
		t.Fatal("unbekanntes Event angenommen")
	}
}

// Händler-Überfall: Der Händler kommt (wenn er fehlt) und wird überfallen; ein zweiter Überfall wird abgelehnt.
func TestDevStartEventUeberfall(t *testing.T) {
	isl := raidIsland(t)
	w := isl.Stages[0]
	queued := len(w.SpawnQueue)
	if !DevStartEvent(isl, "merchantRaid") {
		t.Fatal("merchantRaid abgelehnt")
	}
	if w.Merchant == nil || !w.Merchant.Raid || len(w.SpawnQueue) <= queued || isl.MerchantVisits != 1 {
		t.Fatalf("Händler %+v, Warteschlange %d", w.Merchant, len(w.SpawnQueue))
	}
	if DevStartEvent(isl, "merchantRaid") {
		t.Fatal("zweiter Überfall angenommen")
	}
}
