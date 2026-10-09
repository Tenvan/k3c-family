package sim

import (
	"encoding/json"
	"io/fs"
	"math"
	"testing"

	"k3c/data"
)

// Vollmond und Blutmond (K3.1, B-131/AC-01).

// cycleSeconds: Beginn der Nacht n und Länge eines Zyklus in Zyklus-Sekunden (globalDayNight).
func cycleSeconds(n int) (nightStart, length float64) {
	c := globalDayNight
	day, dusk, night := float64(c.DayMinutes*60), float64(c.TwilightMinutes*60), float64(c.NightMinutes*60)
	length = day + dusk + night
	return float64(n-1)*length + day + dusk, length
}

// moonIsland ist eine Insel (Oberwelt und Höhle) mit zwei Spielern, kurz vor Einbruch der Nacht n.
func moonIsland(t *testing.T, n int) *Island {
	t.Helper()
	isl := mustIsland(t, "mond", []int{0, 1})
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	setCycle(isl, n, -0.5)
	return isl
}

// setCycle stellt alle Stufen auf Nacht n plus offset Sekunden Spielzeit (negativ = davor).
func setCycle(isl *Island, n int, offset float64) {
	start, _ := cycleSeconds(n)
	for _, w := range isl.Stages {
		w.Time = start/w.CycleSpeed + offset
		w.Cycle = cycleAt(globalDayNight, w.Time*w.CycleSpeed)
	}
}

// moonEvents sammelt Ereignisse eines Typs aus allen Stufen.
func moonEvents(events [][]Event, typ string) []Event {
	var out []Event
	for _, stage := range events {
		for _, e := range stage {
			if e["type"] == typ {
				out = append(out, e)
			}
		}
	}
	return out
}

func TestVollmondBlutmondRhythmus(t *testing.T) {
	w := moonIsland(t, 1).Stages[0]
	full, blood := fullMoon(), bloodMoon()
	for _, c := range []struct {
		night       int
		full, blood bool
	}{{6, false, false}, {7, true, false}, {8, false, false}, {13, false, true}, {14, true, false}, {91, true, true}} {
		if eventInNight(w, full, c.night) != c.full || eventInNight(w, blood, c.night) != c.blood {
			t.Errorf("Nacht %d: Vollmond %v, Blutmond %v erwartet", c.night, c.full, c.blood)
		}
	}
	if eventInNight(newWorld(0, "x", Options{}), full, 7) {
		t.Error("ohne Insel kein Event (Golden-Läufe bleiben gleich)")
	}
}

func TestVollmondEreignisseJeNachtEinmal(t *testing.T) {
	isl := moonIsland(t, 7)
	_, length := cycleSeconds(1)
	events := runIsland(isl, nil, length/islandSpeed)
	started, ended := moonEvents(events, "eventStarted"), moonEvents(events, "eventEnded")
	if len(started) != 1 || len(ended) != 1 || started[0]["event"] != "fullMoon" || ended[0]["day"] != 7 {
		t.Fatalf("eventStarted %v, eventEnded %v: je genau einmal fullMoon, Nacht 7 erwartet", started, ended)
	}
}

func TestVollmondUndBlutmondInNacht91(t *testing.T) {
	isl := moonIsland(t, 91)
	events := runIsland(isl, nil, 1)
	if got := moonEvents(events, "eventStarted"); len(got) != 2 {
		t.Fatalf("Nacht 91: zwei Events erwartet, %v", got)
	}
	w := isl.Stages[0]
	if moonDamage(w, 10) != 13 || len(fullMoonExtra(w, nil)) != 1 {
		t.Error("Nacht 91: Blutmond-Schaden und Vollmond-Welle wirken zusammen")
	}
}

func TestVollmondWelleMehrWoelfeUndEinAlphaWolf(t *testing.T) {
	isl := moonIsland(t, 7)
	runIsland(isl, nil, 1)
	w := isl.Stages[0]
	plan := []spawnOrder{{kind: "wolf"}, {kind: "wolf"}, {kind: "wolf"}, {kind: "greed"}}
	kinds := map[string]int{}
	for _, s := range fullMoonExtra(w, plan) {
		kinds[s.kind]++
	}
	if kinds["wolf"] != 2 || kinds["alphaWolf"] != 1 || len(kinds) != 2 {
		t.Fatalf("Vollmond zu 3 Wölfen: 2 Wölfe (50 %%, aufgerundet) und 1 Alpha-Wolf erwartet, %v", kinds)
	}
	alpha := 0
	for _, s := range w.SpawnQueue {
		if s.Kind == "alphaWolf" {
			alpha++
		}
	}
	if alpha != 1 {
		t.Errorf("Welle der Vollmondnacht: genau ein Alpha-Wolf erwartet, %d", alpha)
	}
	if cave := isl.Stages[1]; len(fullMoonExtra(cave, plan)) != 0 {
		t.Error("Vollmond ändert nur Wellen der Tag/Nacht-Stufen")
	}
}

func TestBlutmondSchadenUndDropNurInDieserNacht(t *testing.T) {
	isl := moonIsland(t, 13)
	runIsland(isl, nil, 1)
	for _, w := range isl.Stages { // alle Stufen der Insel, auch unter Tage
		if got := moonDamage(w, 20); math.Abs(got-26) > 1e-9 {
			t.Errorf("Tiefe %d: Blutmond-Schaden 26 erwartet, %v", w.Biome.Depth, got)
		}
		if got := moonDrop(w, 7); got != 14 {
			t.Errorf("Tiefe %d: Blutmond-Drop 14 erwartet, %d", w.Biome.Depth, got)
		}
	}
	setCycle(isl, 14, -0.5) // Nacht 14: Tag davor, kein Blutmond
	runIsland(isl, nil, 1)
	if w := isl.Stages[0]; moonDamage(w, 20) != 20 || moonDrop(w, 7) != 7 {
		t.Error("Nacht 14: kein Blutmond")
	}
}

func TestBlutmondVerdoppeltMuenzenBeimTod(t *testing.T) {
	coins := func(n int) int {
		isl := moonIsland(t, n)
		runIsland(isl, nil, 1)
		w := isl.Stages[0]
		before := len(w.Coins)
		enemyDrop(w, &Enemy{Kind: "greed", X: w.HubX})
		return len(w.Coins) - before
	}
	if normal, blood := coins(12), coins(13); blood != 2*normal || normal == 0 {
		t.Fatalf("Blutmond: doppelter Gold-Drop erwartet, normal %d, Blutmond %d", normal, blood)
	}
}

func TestBlutmondSchadenBeimAngriff(t *testing.T) {
	isl := moonIsland(t, 13)
	runIsland(isl, nil, 1)
	w := isl.Stages[0]
	hp := w.Castle.HP
	e := &Enemy{ID: w.newID(), Kind: "wolf", X: w.Castle.X, Damage: 10}
	attack(w, e, &target{id: w.Castle.ID})
	if got := hp - w.Castle.HP; math.Abs(got-13) > 1e-9 {
		t.Fatalf("Angriff auf die Burg im Blutmond: 13 Schaden erwartet, %v", got)
	}
}

func TestVollmondBelohnungNurWennDieBurgHielt(t *testing.T) {
	reward := func(fell bool) int {
		isl := moonIsland(t, 8) // vor Nacht 8 = nach dem Tagesanbruch von Tag 8 ist Nacht 7 vorbei
		w := isl.Stages[0]
		w.Cycle = CycleInfo{Phase: "day", Day: 8}
		if fell {
			w.castleFellNight = 7
		}
		before := len(w.Coins)
		nightEventsEnd(w)
		return len(w.Coins) - before
	}
	if got := reward(false); got != fullMoon().RewardGold {
		t.Errorf("Burg hielt: %d Münzen erwartet, %d", fullMoon().RewardGold, got)
	}
	if got := reward(true); got != 0 {
		t.Errorf("Burg gefallen: keine Belohnung erwartet, %d", got)
	}
	isl := moonIsland(t, 7)
	runIsland(isl, nil, 1)
	w := isl.Stages[0]
	w.Castle.HP = 0
	StepIsland(isl, nil, dt)
	if w.castleFellNight != 7 {
		t.Errorf("Burgfall in Nacht 7 merkt sich die Nacht, %d", w.castleFellNight)
	}
}

func TestMondEventsAusDenDaten(t *testing.T) {
	raw, err := fs.ReadFile(data.Files, "events.json")
	if err != nil {
		t.Fatal(err)
	}
	var v struct{ Events []NightEvent }
	if err := json.Unmarshal(raw, &v); err != nil {
		t.Fatal(err)
	}
	if *findEvent(v.Events, "fullMoon") != *fullMoon() || *findEvent(v.Events, "bloodMoon") != *bloodMoon() {
		t.Error("Werte stammen aus data/events.json")
	}
	bad := []NightEvent{{ID: "fullMoon", EveryNights: 7, Kind: "wolf", Elite: "unbekannt"}, {ID: "bloodMoon", EveryNights: 13, DamageFactor: 1, DropFactor: 1}}
	if checkNightEvents(bad) == nil {
		t.Error("unbekannter Elite-Gegner wird abgelehnt")
	}
	bad[0].Elite, bad[1].EveryNights = "alphaWolf", 0
	if checkNightEvents(bad) == nil {
		t.Error("Rhythmus 0 wird abgelehnt")
	}
}
