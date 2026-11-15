package sim

import (
	"encoding/json"
	"strings"
	"testing"

	"k3c/data"
)

// Events im Wertesatz der Welt (B-982, docs/rules/welten.md § 1.1): `events` ist eine erlaubte Override-Datei.

// eventWorld ist die Standard-Welt mit Overrides auf events, aufgelöst für den Raum.
func eventWorld(t *testing.T, events string) WorldDef {
	t.Helper()
	w := StandardWorld()
	w.ID, w.Name, w.Extends = "nacht", "Nacht", StandardWorldID
	w.Overrides = map[string]json.RawMessage{"events": json.RawMessage(events)}
	return need(ResolveWorld(w, standardLookup))(t)
}

// eventIsland legt eine Insel mit zwei Stufen und zwei Spielern in der Welt an, kurz vor Einbruch der Nacht n.
func eventIsland(t *testing.T, world WorldDef, n int) *Island {
	t.Helper()
	isl := need(CreateIslandIn(world, "nacht", []int{0, 1}, islandSpeed, CreateOptions{Nature: neutralNature([]int{0, 1})}))(t)
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	setCycle(isl, n, -0.5)
	return isl
}

// B-982/AC-01: Der Raum nutzt den geänderten Abstand (Vollmond jede 5. Nacht), in jeder Stufe; Blutmond und die Basis bleiben.
func TestEventsOverrideAendertAbstand(t *testing.T) {
	world := eventWorld(t, `{"events": {"fullMoon": {"everyNights": 5}}}`)
	isl := eventIsland(t, world, 5)
	for _, w := range isl.Stages {
		full, blood := w.night("fullMoon"), w.night("bloodMoon")
		if full.EveryNights != 5 || blood.EveryNights != bloodMoon().EveryNights || full.RewardGold != fullMoon().RewardGold {
			t.Fatalf("Stufe %d: Vollmond %+v, Blutmond %+v", w.Biome.Depth, *full, *blood)
		}
		if !eventInNight(w, full, 5) || eventInNight(w, full, 7) || !eventInNight(w, blood, 13) {
			t.Errorf("Stufe %d: Vollmond Nacht 5 an, Nacht 7 aus, Blutmond Nacht 13 an erwartet", w.Biome.Depth)
		}
	}
	_, length := cycleSeconds(1)
	started := moonEvents(runIsland(isl, nil, length/islandSpeed), "eventStarted")
	if len(started) != 1 || started[0]["event"] != "fullMoon" || started[0]["day"] != 5 {
		t.Errorf("eventStarted %v: genau fullMoon in Nacht 5 erwartet", started)
	}
	if fullMoon().EveryNights != 7 {
		t.Error("die Basis bleibt unverändert")
	}
}

// B-982/AC-01: Abstand 0 schaltet das Event aus (Blutmond, Vollmond, Händler-Überfall); ohne Override bleibt alles an.
func TestEventsAbstandNullIstAus(t *testing.T) {
	world := eventWorld(t, `{"events": {"bloodMoon": {"everyNights": 0}, "merchantRaid": {"everyVisits": 0}}}`)
	w := eventIsland(t, world, 13).Stages[0]
	if eventInNight(w, w.night("bloodMoon"), 13) || eventInNight(w, w.night("bloodMoon"), 91) || !eventInNight(w, w.night("fullMoon"), 91) {
		t.Error("Blutmond aus (auch Nacht 91), Vollmond an erwartet")
	}
	isl := eventIsland(t, world, 13)
	m := isl.Stages[0]
	m.Merchant = nil
	for visit := 1; visit <= 8; visit++ {
		isl.MerchantVisits = visit
		merchantArrive(m)
		merchantRaidStart(m) // everyVisits 0: keine Division durch 0, kein Überfall
		if m.Merchant.Raid {
			t.Fatalf("Besuch %d: Überfall trotz Abstand 0", visit)
		}
		m.Merchant = nil
	}
	std := moonIsland(t, 13).Stages[0]
	if !eventInNight(std, std.night("bloodMoon"), 13) {
		t.Error("Standard-Welt: Blutmond bleibt an")
	}
}

// B-982: Kind-Welt erbt die Event-Overrides der Eltern-Welt.
func TestEventsKindErbtOverride(t *testing.T) {
	parent := StandardWorld()
	parent.ID, parent.Extends = "eltern", StandardWorldID
	parent.Overrides = ov(map[string]string{"events": `{"events": {"fullMoon": {"everyNights": 3}}}`})
	kid := WorldDef{ID: "kind", Extends: "eltern"}
	v, err := ResolveWorldValues(data.Files, testWorlds(StandardWorld(), parent, kid), "kind")
	if err != nil {
		t.Fatal(err)
	}
	if got := findEvent(v.Events, "fullMoon").EveryNights; got != 3 {
		t.Errorf("everyNights %d, 3 geerbt erwartet", got)
	}
}

// B-982/AC-02: Fehlerfälle laden nicht; die Meldung nennt Welt und Pfad.
func TestEventsOverrideAbgelehnt(t *testing.T) {
	for override, want := range map[string]string{
		`{"events": {"taifun": {"everyNights": 3}}}`:                  "events.taifun: unbekannter Eintrag (neue Einträge sind nicht erlaubt)",
		`{"events": {"fullMoon": {"everyNights": -1}}}`:               "Abstand muss >= 0 sein",
		`{"events": {"merchantRaid": {"everyVisits": -2}}}`:           "Abstand muss >= 0 sein",
		`{"events": {"bloodMoon": {"damageFactor": 0}}}`:              "bloodMoon braucht Faktoren > 0",
		`{"events": {"bloodMoon": {"dropFactor": -1}}}`:               "bloodMoon braucht Faktoren > 0",
		`{"events": {"fullMoon": {"kind": "drache"}}}`:                "unbekannte Gegner",
		`{"events": {"fullMoon": {"elite": "drache"}}}`:               "unbekannte Gegner",
		`{"events": {"fullMoon": {"everyNights": "oft"}}}`:            "events.fullMoon.everyNights",
		`{"events": {"fullMoon": {"gibtEsNicht": 1}}}`:                "unbekanntes Feld",
		`{"events": {"fullMoon": {"id": "vollmond"}}}`:                "fullMoon, bloodMoon und merchantRaid fehlen",
		`{"events": [{"id": "fullMoon", "everyNights": 5}]}`:          "events",
		`{"notes": "x"}`:                                              "unbekanntes Feld",
		`{"events": {"fullMoon": {"everyNights": 5}}, "extra": true}`: "unbekanntes Feld",
	} {
		w := StandardWorld()
		w.ID, w.Extends = "kaputt", StandardWorldID
		w.Overrides = map[string]json.RawMessage{"events": json.RawMessage(override)}
		_, err := ResolveWorld(w, standardLookup)
		if err == nil || !strings.Contains(err.Error(), want) || !strings.Contains(err.Error(), "kaputt") {
			t.Errorf("%s: Fehler %v, enthält %q und die Welt erwartet", override, err, want)
		}
	}
}
