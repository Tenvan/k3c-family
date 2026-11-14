package hud

import (
	"testing"

	"k3c/engine/sim"
)

type textCase struct {
	name, got, want string
}

func checkTexts(t *testing.T, cases []textCase) {
	t.Helper()
	for _, c := range cases {
		if c.got != c.want {
			t.Errorf("%s: %q, erwartet %q", c.name, c.got, c.want)
		}
	}
}

func TestUhrUndZeit(t *testing.T) {
	agg := 37.8
	checkTexts(t, []textCase{
		{"m:ss", formatTime(90), "1:30"},
		{"aufgerundet", formatTime(60.2), "1:01"},
		{"nie negativ", formatTime(-3), "0:00"},
		{"Tag", clockText(newClockKey("de", sim.CycleInfo{Phase: "day", Day: 2, SecondsLeft: 125}, 1, nil, false)),
			"Tag 2  ·  Dämmerung in 2:05"},
		{"Zeitraffer", clockText(newClockKey("de", sim.CycleInfo{Phase: "night", Day: 1, SecondsLeft: 120}, 2, nil, false)),
			"Nacht 1  ·  Morgen in 1:00"},
		{"Aggression und gesichert", clockText(newClockKey("de", sim.CycleInfo{Phase: "dusk", SecondsLeft: 5}, 1, &agg, true)),
			"Dämmerung  ·  Nacht in 0:05  ·  Aggression 37%  ·  gesichert"},
		{"englisch", clockText(newClockKey("en", sim.CycleInfo{Phase: "dawn", Day: 3, SecondsLeft: 61}, 1, nil, false)),
			"Day 3  ·  Sunrise in 1:01"},
	})
}

// Gold nur im HUD (goldOnlyInHud.test.ts): die Spielerkarte nennt Gold und Maximum, gefallen die Sekunden.
func TestGoldNurAufDerKarte(t *testing.T) {
	checkTexts(t, []textCase{
		{"Gold/Maximum", goldText(goldKey{"de", 12, 30, 0}), "12/30"},
		{"gefallen", goldText(goldKey{"de", 12, 30, 4}), "4 s"},
	})
}

func TestGoldFlash(t *testing.T) {
	var g goldFlash
	if g.step(5, 0) {
		t.Error("der erste Wert blinkt nicht")
	}
	if !g.step(6, 1) || !g.step(6, 1.79) {
		t.Error("eine Änderung hebt den Topf hervor")
	}
	if g.step(6, 1.8) {
		t.Error("nach 0,8 s ist es vorbei")
	}
	g.step(7, 2)
	if !g.step(8, 2.5) || !g.step(8, 3.29) {
		t.Error("eine neue Änderung startet die Zeit neu")
	}
}

func TestVorratZeile(t *testing.T) {
	k := func(lang, primary string, stock [5]int, maxStock, points int) string {
		return sharedText(stockKey{lang, primary, stock, maxStock, points})
	}
	checkTexts(t, []textCase{
		{"nur mit Vorrat", k("de", "wood", [5]int{0, 40}, 300, 0), "Holz 0/300  ·  Stein 40/300"},
		{"voll", k("de", "", [5]int{300}, 300, 0), "Holz 300/300 voll"},
		{"fünf in Reihenfolge", k("de", "", [5]int{1, 2, 3, 7, 5}, 600, 0),
			"Holz 1/600  ·  Stein 2/600  ·  Kupfer 3/600  ·  Eisen 7/600  ·  Kristall 5/600"},
		{"Primär mit 0", k("de", "iron", [5]int{0, 0, 4}, 300, 0), "Kupfer 4/300  ·  Eisen 0/300"},
		{"ohne Maximum", k("de", "", [5]int{12}, 0, 0), "Holz 12"},
		{"Skill-Punkte", k("de", "", [5]int{12}, 0, 2), "Holz 12  ·  Skill-Punkte 2"},
		{"englisch", k("en", "", [5]int{0, 0, 0, 300}, 300, 0), "Iron 300/300 full"},
		{"leer", k("de", "", [5]int{}, 0, 0), ""},
	})
}

func TestHubUndBuerger(t *testing.T) {
	up := hubKey{lang: "de", level: 2, has: true, gold: 40, paid: 15, material: [5]int{0, 100, 0, 20}}
	step := hubKey{lang: "de", level: 1, has: true, step: 2, steps: 3, gold: 25, material: [5]int{0, 50, 20}}
	long, short := up, step
	long.compact, short.compact = false, true
	step.compact = false
	checkTexts(t, []textCase{
		{"lang", hubText(long), "Hub-Stufe 2 → 3: 15/40 Gold, 100 Stein, 20 Eisen"},
		{"Schritt lang", hubText(step), "Hub-Stufe 1 → 2, Ausbau 2/3: 0/25 Gold, 50 Stein, 20 Kupfer"},
		{"Schritt kurz", hubText(short), "Hub 1→2 2/3: 0/25 Gold, 50 Stein, 20 Kupfer"},
		{"höchste Stufe", hubText(hubKey{lang: "de", level: 5, compact: true}), "Hub 5"},
		{"ohne Stufe", hubText(hubKey{lang: "de"}), ""},
		{"Limit lang", limitText("de", 12, 20, false), "Kämpfer 12 von 20"},
		{"Limit kurz", limitText("de", 0, 10, true), "Limit 0/10"},
		{"Limit voll", limitText("de", 20, 20, false), "Kämpfer 20 von 20: Limit erreicht"},
		{"Limit voll kurz", limitText("de", 20, 20, true), "Limit 20/20 voll"},
		{"Händler", merchantText("de", "stone", 7, 0, false), "Händler: Stein · bis Tag 7"},
		{"Händler Kauf", merchantText("de", "iron", 7, 3, false), "Händler: Eisen · bis Tag 7 · Kauf 3 Gold"},
		{"Händler kurz", merchantText("de", "stone", 7, 3, true), "Händler Stein"},
		{"Zeile", citizensText(hubKey{lang: "de", level: 5, compact: true, fighters: 3, limit: 10, merchant: true, mResource: "wood"}),
			"Hub 5  ·  Limit 3/10  ·  Händler Holz"},
		{"Beruf", professionText("de", "miner", false), "Bergmann"},
		{"Beruf englisch", professionText("en", "builder", false), "Builder"},
		{"Beruf unbekannt", professionText("de", "wizard", true), "Beruf"},
		{"kein Beruf", professionText("de", "", true), ""},
	})
}

func TestKampfReiseOrt(t *testing.T) {
	checkTexts(t, []textCase{
		{"Welle", waveText(waveKey{"de", 2, 7}), "Welle 2: 7 Gegner"},
		{"keine Gegner", waveText(waveKey{"de", 2, 0}), ""},
		{"Reise", travelText(newTravelKey("de", &sim.Travel{ToDepth: 1, Via: "stairsDown", Progress: 0.31})),
			"Abstieg in Tiefe 1  ■■■■□□□□□□"},
		{"keine Reise", travelText(newTravelKey("de", nil)), ""},
		{"Ort", placeText(placeKey{"de", "forest", "Wald", 0}), "P1 · Oberwelt (Wald)"},
		{"Ort unbekanntes Biom", placeText(placeKey{"en", "neu", "Neuland", 1}), "P2 · Neuland"},
	})
}

func TestMemoBautNurBeiNeuemSchluessel(t *testing.T) {
	var m memo[waveKey]
	n := 0
	build := func(k waveKey) string { n++; return waveText(k) }
	a := m.get(waveKey{"de", 1, 3}, build)
	b := m.get(waveKey{"de", 1, 3}, build)
	if a != b || n != 1 {
		t.Errorf("gleicher Schlüssel baut nicht neu: %d", n)
	}
	m.get(waveKey{"de", 1, 4}, build)
	if n != 2 {
		t.Error("neuer Schlüssel baut neu")
	}
}
