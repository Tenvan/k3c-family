package sim

import (
	"fmt"
	"math"
	"slices"
	"strings"
	"testing"
	"testing/fstest"

	"k3c/data"
)

// Endboss der Kristallhöhle (K2.1c, B-130/AC-02, AC-03): Bau, Warten, Auslösung, Phasen, Skalierung, Belohnung,
// keine Rückkehr, Determinismus.

const wardenID = "crystalHeartWarden"

// endIsland: Insel nur mit der Kristallhöhle und `players` Spielern an der Burg; ohne Truppen, Gegner und Aggression,
// damit nur der Endboss und seine Beschwörungen kämpfen.
func endIsland(t *testing.T, players int) (*Island, *World, float64) {
	t.Helper()
	isl, err := CreateIsland("endboss", []int{4}, 1)
	if err != nil {
		t.Fatal(err)
	}
	for range players {
		AddIslandPlayer(isl, 0)
	}
	w := isl.Stages[0]
	w.Troops, w.Enemies, w.SpawnQueue, w.Aggression = []*Troop{}, []*Enemy{}, []QueuedSpawn{}, nil
	x, ok := lairX(w)
	if !ok {
		t.Fatal("Kristallhöhle ohne Bau")
	}
	return isl, w, x
}

func warden(w *World) *Enemy {
	for _, e := range w.Enemies {
		if e.Kind == wardenID {
			return e
		}
	}
	return nil
}

// protect hält die Spieler am Leben: volle HP, und ein tödlicher Treffer lässt 1 HP stehen (Last Stand).
func protect(w *World) {
	for _, p := range w.Players {
		p.HP, p.LastStandFor = p.MaxHP, 1
	}
}

// endSteps tickt die Stufe `seconds` lang; keep hält die Spieler am Leben. Liefert alle Ereignisse.
func endSteps(w *World, seconds float64, keep bool) []Event {
	var events []Event
	for tm := 0.0; tm < seconds-1e-9; tm += dt {
		if keep {
			protect(w)
		}
		Step(w, nil, dt)
		events = append(events, w.Events...)
	}
	return events
}

// (a) Ohne Spieler am Bau wartet der Endboss, auch nach langer Zeit: kein Gegner, nichts kann ihn treffen. Ein
// Spieler genau auf triggerUnits, ein toter und ein getrennter Spieler am Bau lösen ihn nicht aus.
func TestEndbossWartetOhneZeitdruck(t *testing.T) {
	_, w, x := endIsland(t, 3)
	trigger := endBoss(4).Lair.TriggerUnits
	w.Players[0].X = x - trigger
	w.Players[1].X, w.Players[1].RespawnIn = x, 10
	w.Players[2].X, w.Players[2].Free = x, true
	for tm := 0.0; tm < 20*60; tm++ {
		w.Players[1].RespawnIn = 10
		Step(w, nil, 1)
		if warden(w) != nil || countEvents(w, "bossSpawned") > 0 {
			t.Fatalf("Endboss nach %.0f s ohne Ankunft aktiv", tm)
		}
	}
}

// (b) Ein Spieler kommt an: Der Boss erscheint einmal am Bau, läuft nicht und ist verwundbar; mit zwei Spielern an
// verschiedenen Orten genügt einer.
func TestEndbossWirdBeiAnkunftAusgeloest(t *testing.T) {
	_, w, x := endIsland(t, 2)
	w.Players[1].X = x - endBoss(4).Lair.TriggerUnits + 1
	events := endSteps(w, 10, true)
	e := warden(w)
	if e == nil || count(events, "bossSpawned") != 1 || e.X != x || e.HomeX != x {
		t.Fatalf("Endboss %+v, %d× bossSpawned, erwartet einmal am Bau %.1f", e, count(events, "bossSpawned"), x)
	}
	hp := e.HP
	applyDamageBy(w, e.ID, 10, "test")
	if e.HP != hp-10 {
		t.Fatalf("aktiver Endboss unverwundbar: HP %.0f → %.0f", hp, e.HP)
	}
	w.Players[1].X = w.HubX // er bleibt aktiv, auch wenn der Spieler geht
	if events = endSteps(w, 30, true); warden(w) != e || count(events, "bossSpawned") != 0 {
		t.Fatal("Endboss nach dem Weggehen nicht mehr aktiv oder neu erschienen")
	}
}

// strikesOf: Zeitpunkte der Flächenschläge des Bosses (ein `strike` je Schlag), Phasenwechsel und Treffer am Spieler
// mit Index 1.
func strikesOf(w *World, boss *Enemy, seconds float64) (times []float64, phases, hitsOnSecond int) {
	for tm := 0.0; tm < seconds-1e-9; tm += dt {
		protect(w)
		Step(w, nil, dt)
		for _, ev := range w.Events {
			switch {
			case ev["type"] == "strike" && ev["from"] == boss.ID:
				times = append(times, w.Time)
			case ev["type"] == "bossPhase":
				phases++
			case ev["type"] == "hit" && ev["target"] == "player" && ev["id"] == 1:
				hitsOnSecond++
			}
		}
	}
	return times, phases, hitsOnSecond
}

// gaps: Abstände der Zeitpunkte, gerundet auf 0,1 s.
func gaps(times []float64) []float64 {
	var out []float64
	for i := 1; i < len(times); i++ {
		out = append(out, math.Round((times[i]-times[i-1])*10)/10)
	}
	return out
}

// (c) Phasen: Phase 1 ruft Kristallspinnen, Phase 2 (unter 60 %) schlägt im Radius 5 alle 6 s und beschwört nicht
// mehr, Phase 3 (unter 25 %) schlägt mit +50 % Tempo alle 4 s; jeder Wechsel wird genau einmal gemeldet.
func TestEndbossPhasen(t *testing.T) {
	_, w, x := endIsland(t, 2)
	w.Players[0].X, w.Players[1].X = x+4, x+6 // hinter dem Bau, nicht im Weg der Beschwörungen
	endSteps(w, dt, true)
	boss := warden(w)
	if boss == nil {
		t.Fatal("Endboss nicht ausgelöst")
	}
	if events := endSteps(w, 10.5, true); countKind(w, "crystalSpider") < 2 || count(events, "bossPhase") != 0 {
		t.Fatalf("Phase 1: %d Kristallspinnen, %d Phasenwechsel", countKind(w, "crystalSpider"), count(events, "bossPhase"))
	}
	w.Enemies = []*Enemy{boss}
	boss.HP = math.Floor(boss.MaxHP * 0.59)
	times, phases, outside := strikesOf(w, boss, 25)
	if phases != 1 || !slices.Equal(gaps(times), []float64{6, 6, 6}) || countKind(w, "crystalSpider") != 0 {
		t.Fatalf("Phase 2: %d Wechsel, Schläge alle %v s, %d Spinnen", phases, gaps(times), countKind(w, "crystalSpider"))
	}
	if outside != 0 {
		t.Fatalf("Flächenschlag trifft %d× außerhalb von Radius 5", outside)
	}
	boss.HP = math.Floor(boss.MaxHP * 0.24)
	times, phases, _ = strikesOf(w, boss, 17)
	if phases != 1 || !slices.Equal(gaps(times), []float64{4, 4, 4}) || boss.Speed != 0 {
		t.Fatalf("Phase 3: %d Wechsel, Schläge alle %v s, Tempo %.1f", phases, gaps(times), boss.Speed)
	}
	if _, phases, _ = strikesOf(w, boss, 5); phases != 0 {
		t.Fatal("Phase 3 erneut gemeldet")
	}
}

// (d) HP bei 3 Spielern = 2 × HP bei 1 Spieler (1 + 0,5 × 2); später beitretende Spieler ändern sie nicht.
func TestEndbossHPSkaliertMitSpielern(t *testing.T) {
	hpWith := func(players int) float64 {
		isl, w, x := endIsland(t, players)
		w.Players[0].X = x
		endSteps(w, dt, true)
		e := warden(w)
		AddIslandPlayer(isl, 0)
		endSteps(w, dt, true)
		if e == nil || e.HP != e.MaxHP {
			t.Fatalf("%d Spieler: Endboss %+v", players, e)
		}
		return e.MaxHP
	}
	one, three := hpWith(1), hpWith(3)
	if three != 2*one {
		t.Fatalf("HP 1 Spieler %.0f, 3 Spieler %.0f, erwartet das Doppelte", one, three)
	}
	t.Logf("Endboss-HP: 1 Spieler %.0f, 3 Spieler %.0f", one, three)
}

// (e) Belohnung: 500 Gold als Münzen am Bau, 250 Kristall in den Insel-Vorrat, 3 Skill-Punkte in den Pool der Insel
// (für alle Spieler); Merker gesetzt.
func TestEndbossBelohnung(t *testing.T) {
	isl, w, x := endIsland(t, 2)
	*isl.Stock = Stock{}
	w.Players[0].X = x - 10
	endSteps(w, dt, true)
	boss := warden(w)
	pool, coins := isl.SkillPool, len(w.Coins)
	boss.HP = 0
	events := endSteps(w, dt, true)
	near := 0
	for _, c := range w.Coins {
		if math.Abs(c.X-x) <= 1.5 {
			near++
		}
	}
	if count(events, "bossDefeated") != 1 || len(w.Coins)-coins != 500 || near != 500 {
		t.Fatalf("%d× bossDefeated, %d neue Münzen (%d am Bau), erwartet 500", count(events, "bossDefeated"), len(w.Coins)-coins, near)
	}
	if isl.Stock.Crystal != 250 || isl.SkillPool-pool != 3 || w.SkillPoints != isl.SkillPool {
		t.Fatalf("Kristall %d, Pool +%d (Stufe %d), erwartet 250 und +3", isl.Stock.Crystal, isl.SkillPool-pool, w.SkillPoints)
	}
	if !isl.EndbossDefeated || !slices.Contains(isl.DefeatedBosses, wardenID) {
		t.Fatalf("Merker fehlt: EndbossDefeated %v, besiegt %v", isl.EndbossDefeated, isl.DefeatedBosses)
	}
}

// (f) Ein aktiver Endboss übersteht einen Burgfall; nach seinem Tod kehrt er nie zurück, auch nicht nach einem
// Burgfall mit einem Spieler am Bau.
func TestEndbossKehrtNieZurueck(t *testing.T) {
	isl, w, x := endIsland(t, 1)
	w.Players[0].X = x
	endSteps(w, dt, true)
	boss := warden(w)
	boss.HP--
	w.Castle.HP = 0
	endSteps(w, 2*dt, true)
	if warden(w) != boss || boss.HP != boss.MaxHP-1 {
		t.Fatal("aktiver Endboss nach dem Burgfall verschwunden oder geheilt")
	}
	boss.HP = 0
	endSteps(w, dt, true)
	for range 3 {
		w.Castle.HP = 0
		events := endSteps(w, 60, false)
		w.Players[0].X = x
		events = append(events, endSteps(w, 60, false)...)
		if warden(w) != nil || count(events, "bossSpawned") != 0 || count(events, "castleFallen") != 1 {
			t.Fatalf("besiegter Endboss zurück (%d× bossSpawned, %d Burgfälle)", count(events, "bossSpawned"), count(events, "castleFallen"))
		}
	}
	if len(isl.DefeatedBosses) != 1 {
		t.Fatalf("besiegt %v", isl.DefeatedBosses)
	}
}

// (g) Gleicher Seed, gleicher Verlauf: zwei Spieler am Bau, alle drei Phasen, Sieg.
func TestEndbossDeterministisch(t *testing.T) {
	trace := func() string {
		_, w, x := endIsland(t, 2)
		w.Players[0].X, w.Players[1].X = x-3, x-8
		var b strings.Builder
		for tm := 0.0; tm < 120; tm += dt {
			protect(w)
			if e := warden(w); e != nil && math.Mod(tm, 1) < dt {
				applyDamageBy(w, e.ID, e.MaxHP/90, "test")
			}
			Step(w, nil, dt)
			for _, ev := range w.Events {
				if s, _ := ev["type"].(string); strings.HasPrefix(s, "boss") || s == "strike" {
					fmt.Fprintf(&b, "%.3f %v\n", w.Time, ev)
				}
			}
		}
		fmt.Fprintf(&b, "Gegner %d, Münzen %d", len(w.Enemies), len(w.Coins))
		return b.String()
	}
	first := trace()
	if first != trace() {
		t.Fatal("Endboss-Verlauf nicht deterministisch")
	}
	for _, want := range []string{"bossSpawned", "phase:2", "phase:3", "bossDefeated"} {
		if !strings.Contains(first, want) {
			t.Fatalf("Verlauf ohne %s", want)
		}
	}
}

// Daten: ein Endboss je Insel 1 in der Kristallhöhle mit Werten laut bosse.md § 1; kaputte Phasen oder Baue werden
// beim Laden abgelehnt.
func TestEndbossDaten(t *testing.T) {
	b := endBoss(4)
	if len(endBosses) != 1 || b == nil || b.ID != wardenID || b.HPFactor != 30 || len(b.Phases) != 3 ||
		b.Phases[2].BelowHPPercent != 25 || b.Phases[2].Tempo != 1.5 || b.Reward.Gold != 500 || b.Reward.Material != 250 {
		t.Fatalf("Endbosse %+v", endBosses)
	}
	raw, err := data.Files.ReadFile("bosses.json")
	if err != nil {
		t.Fatal(err)
	}
	for _, c := range [][2]string{
		{`"triggerUnits": 15`, `"triggerUnits": 0`},
		{`"belowHpPercent": 25`, `"belowHpPercent": 70`},
		{`"tempo": 1.5`, `"tempo": -1`},
		{`"kind": "end"`, `"kind": "mini"`},
		{`{ "ability": { "type": "summon", "enemy": "crystalSpider"`, `{ "ability": { "type": "summon", "enemy": "nope"`},
	} {
		broken := strings.Replace(string(raw), c[0], c[1], 1)
		if broken == string(raw) {
			t.Fatalf("Testdaten: %s nicht gefunden", c[0])
		}
		var got []BossData
		if err := loadEndBosses(fstest.MapFS{"bosses.json": {Data: []byte(broken)}}, &got); err == nil {
			t.Fatalf("%s: ungültige Daten geladen", c[1])
		}
	}
}
