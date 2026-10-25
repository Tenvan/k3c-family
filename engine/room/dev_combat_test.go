package room

import (
	"strings"
	"testing"
)

// B-603/AC-01: Die Kampf-Aktionen wirken im Dev-Mode (Gerät und /dm), sonst forbidden; falsche Felder sind bad_request.

func bossCount(r *Room) int {
	n := 0
	for _, w := range r.isl.Stages {
		for _, e := range w.Enemies {
			if e.Boss {
				n++
			}
		}
	}
	return n
}

// boss stellt den Boss in die Stufe des Monarchen; ein zweiter wird mit Hinweis abgelehnt.
func TestDevBoss(t *testing.T) {
	r, p, out := devRoom(t)
	if err := r.Dev("a", p, DevAction{Action: "boss", Slot: slot(1), Boss: "goblinLeader"}); err != nil {
		t.Fatal(err)
	}
	if n := bossCount(r); n != 1 {
		t.Fatalf("%d Bosse, erwartet 1", n)
	}
	if err := r.Dev("a", p, DevAction{Action: "boss", Slot: slot(0), Boss: "trollKing"}); err != ErrBadRequest {
		t.Fatalf("zweiter Boss: %v", err)
	}
	log := out.String()
	if !strings.Contains(log, "aktion=boss") || !strings.Contains(log, "Boss schon da") {
		t.Fatalf("Log ohne Aktion oder Hinweis:\n%s", log)
	}
	for _, a := range []DevAction{{Action: "boss", Slot: slot(0), Boss: "drache"}, {Action: "boss", Boss: "goblinLeader"},
		{Action: "boss", Slot: slot(3), Boss: "goblinLeader"}} {
		if err := r.Dev("a", p, a); err != ErrBadRequest {
			t.Fatalf("%+v: %v", a, err)
		}
	}
}

// bossPhase schaltet den Endboss (über /dm gerufen) weiter; ohne Endboss und nach der letzten Phase bad_request.
func TestDevBossPhase(t *testing.T) {
	r, p, _ := devRoom(t)
	if err := r.Dev("a", p, DevAction{Action: "bossPhase"}); err != ErrBadRequest {
		t.Fatalf("ohne Endboss: %v", err)
	}
	m0 := islPlayer(t, r, r.devices["a"].slots[0])
	if err := r.DevPage(DevAction{Action: "boss", Slot: slot(m0.Index), Boss: "crystalHeartWarden"}); err != nil {
		t.Fatal(err)
	}
	for range 2 {
		if err := r.Dev("a", p, DevAction{Action: "bossPhase"}); err != nil {
			t.Fatal(err)
		}
	}
	if err := r.Dev("a", p, DevAction{Action: "bossPhase"}); err != ErrBadRequest {
		t.Fatalf("nach der letzten Phase: %v", err)
	}
}

// event springt zur Nacht mit dem Event, night zur nächsten Nacht; unbekannte Events sind bad_request.
func TestDevEventUndNacht(t *testing.T) {
	r, p, _ := devRoom(t)
	if err := r.Dev("a", p, DevAction{Action: "event", Event: "fullMoon"}); err != nil {
		t.Fatal(err)
	}
	r.Tick()
	w := r.isl.Stages[0]
	if w.Cycle.Phase != "night" || len(w.NightEvents) != 1 || w.NightEvents[0].ID != "fullMoon" {
		t.Fatalf("Phase %s, Events %v", w.Cycle.Phase, w.NightEvents)
	}
	day := w.Cycle.Day
	if err := r.Dev("a", p, DevAction{Action: "night"}); err != nil {
		t.Fatal(err)
	}
	r.Tick()
	if w.Cycle.Phase != "night" || w.Cycle.Day != day+1 {
		t.Fatalf("Phase %s Tag %d, erwartet Nacht %d", w.Cycle.Phase, w.Cycle.Day, day+1)
	}
	if err := r.Dev("a", p, DevAction{Action: "event", Event: "merchantRaid"}); err != nil {
		t.Fatal(err)
	}
	for _, ev := range []string{"", "neumond", "merchantRaid"} {
		if err := r.Dev("a", p, DevAction{Action: "event", Event: ev}); err != ErrBadRequest {
			t.Fatalf("event %q: %v", ev, err)
		}
	}
}

// Ohne Dev-Mode sind alle Kampf-Aktionen verboten und ändern nichts.
func TestDevKampfVerboten(t *testing.T) {
	r, p, _ := devRoom(t)
	r.m.Dev = false
	for _, a := range []DevAction{{Action: "boss", Slot: slot(0), Boss: "goblinLeader"}, {Action: "bossPhase"},
		{Action: "event", Event: "bloodMoon"}, {Action: "night"}} {
		if err := r.Dev("a", p, a); err != ErrForbidden {
			t.Fatalf("%s: %v", a.Action, err)
		}
	}
	if bossCount(r) != 0 || r.isl.Dev.Nights != nil {
		t.Fatal("verbotene Aktion hat gewirkt")
	}
}
