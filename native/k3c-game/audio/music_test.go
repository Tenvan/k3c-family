package audio

import (
	"math"
	"regexp"
	"testing"
)

// AC-02: Zustand → Slot wie music.ts › MUSIC_SLOTS, jeder Slot mit Datei im Register, kein Stück doppelt.
func TestMusicSlotsMatchBrowser(t *testing.T) {
	src := browserSource(t, "music.ts")
	re := regexp.MustCompile(`(?m)^\s+(day|dusk|night|battle|depth|boss|lobby|defeat|ending): '([^']+)',`)
	found := re.FindAllStringSubmatch(src, -1)
	byName := map[string]musicState{"day": stateDay, "dusk": stateDusk, "night": stateNight, "battle": stateBattle,
		"depth": stateDepth, "boss": stateBoss, "lobby": stateLobby, "defeat": stateDefeat, "ending": stateEnding}
	if len(found) != len(byName) {
		t.Fatalf("%d Einträge in music.ts, erwartet %d", len(found), len(byName))
	}
	reg := realRegister(t)
	files := map[string]bool{}
	for _, m := range found {
		slot := musicSlots[byName[m[1]]]
		if slot != m[2] {
			t.Errorf("%s: nativ %q, Browser %q", m[1], slot, m[2])
		}
		f := reg.Music[slot]
		if f == "" || files[f] {
			t.Errorf("%s: Datei %q fehlt oder doppelt", slot, f)
		}
		files[f] = true
	}
}

// AC-02: Musikwechsel nach music.ts › nextMusicState (Fälle aus music.test.ts).
func TestNextMusicState(t *testing.T) {
	all := musicView{phase: "night", enemies: 3, depth: 2, boss: true, victory: true}
	cases := []struct {
		name   string
		prev   musicState
		v      musicView
		fallen bool
		want   musicState
	}{
		{"Tag", stateNone, musicView{phase: "day"}, false, stateDay},
		{"Abend", stateNone, musicView{phase: "dusk"}, false, stateDusk},
		{"Nacht", stateNone, musicView{phase: "night"}, false, stateNight},
		{"Morgengrauen = Nacht", stateNone, musicView{phase: "dawn"}, false, stateNight},
		{"Lobby vor allem", stateNone, musicView{lobby: true, phase: "night", enemies: 3, boss: true, victory: true}, false, stateLobby},
		{"Sieg", stateNone, all, false, stateEnding},
		{"Boss", stateNone, musicView{phase: "night", enemies: 3, depth: 2, boss: true}, false, stateBoss},
		{"Kampf", stateNone, musicView{phase: "night", enemies: 3, depth: 2}, false, stateBattle},
		{"Tiefe", stateNone, musicView{phase: "night", depth: 2}, false, stateDepth},
		{"anderes Ereignis", stateNight, musicView{phase: "night"}, false, stateNight},
		{"Burg fällt", stateNight, musicView{phase: "night", enemies: 2}, true, stateDefeat},
		{"Niederlage bleibt", stateDefeat, musicView{phase: "day", victory: true}, false, stateDefeat},
		{"Lobby beendet Niederlage", stateDefeat, musicView{lobby: true}, false, stateLobby},
		{"Sieg bleibt", stateEnding, musicView{phase: "day"}, false, stateEnding},
		{"unbekannte Phase = Lücke", stateNone, musicView{phase: "?"}, false, stateNone},
	}
	for _, c := range cases {
		if got := nextMusicState(c.prev, c.v, c.fallen); got != c.want {
			t.Errorf("%s: %d, erwartet %d", c.name, got, c.want)
		}
	}
}

// Überblenden über Takte: neues Stück steigt in crossfadeTicks auf 1, altes fällt auf 0; gleiche Leistung im Übergang.
func TestFaderCrossfade(t *testing.T) {
	var d fader
	d.target("a")
	for range crossfadeTicks {
		d.step()
	}
	if v := d.voices(); v[0].Slot != "a" || v[0].Gain != 1 || v[1].Slot != "" {
		t.Fatalf("eingeblendet: %+v", v)
	}
	d.target("b")
	for i := range crossfadeTicks {
		d.step()
		v := d.voices()
		if i < crossfadeTicks-1 {
			if p := v[0].Gain*v[0].Gain + v[1].Gain*v[1].Gain; math.Abs(p-1) > 1e-9 {
				t.Fatalf("Takt %d: Leistung %v", i, p)
			}
		}
	}
	if v := d.voices(); v[0].Slot != "b" || v[0].Gain != 1 || v[1].Slot != "" {
		t.Errorf("nach dem Wechsel: %+v", v)
	}
	d.target("b")
	if d.voices()[0].Gain != 1 {
		t.Error("dasselbe Stück läuft einfach weiter")
	}
}

// Wechsel mitten im Übergang: kein Sprung; kehrt das ausblendende Stück zurück, steigt es vom aktuellen Pegel.
func TestFaderRetargetWithoutJump(t *testing.T) {
	var d fader
	d.target("a")
	for range crossfadeTicks {
		d.step()
	}
	d.target("b")
	for range crossfadeTicks / 2 {
		d.step()
	}
	before := d.voices()
	d.target("a") // zurück
	after := d.voices()
	if after[0].Slot != "a" || math.Abs(after[0].Gain-before[1].Gain) > 0.03 {
		t.Errorf("Rückkehr: vorher %+v, nachher %+v", before, after)
	}
	if after[1].Slot != "b" || after[1].Gain != before[0].Gain {
		t.Errorf("b blendet vom aktuellen Pegel aus: vorher %+v, nachher %+v", before, after)
	}
	d.target("")
	if v := d.voices(); v[0].Slot != "" || v[1].Slot != "a" {
		t.Errorf("Lücke: Stille, a blendet aus: %+v", v)
	}
}

// Ducking wie ducking.test.ts: sinkt, hält, steigt; überlappend verlängert; in der Rampe hinauf ohne Sprung.
func TestDuck(t *testing.T) {
	near := func(a, b float64) bool { return math.Abs(a-b) < 1e-9 }
	if duckGain(3, duckWindow{}) != 1 {
		t.Error("ohne Warnung voll")
	}
	w := extendDuck(duckWindow{}, 10)
	checks := [][2]float64{
		{10, 1}, {10 + duckAttack/2, (1 + duckLevel) / 2}, {10 + duckAttack, duckLevel}, {w.end, duckLevel},
		{w.end + duckRelease/2, (1 + duckLevel) / 2}, {w.end + duckRelease, 1},
	}
	for _, c := range checks {
		if g := duckGain(c[0], w); !near(g, c[1]) {
			t.Errorf("t=%v: %v, erwartet %v", c[0], g, c[1])
		}
	}
	o := extendDuck(extendDuck(extendDuck(duckWindow{}, 0), 0.5), 1)
	if o.start != 0 || !near(o.end, 1+duckHold) {
		t.Errorf("verlängert: %+v", o)
	}
	for x := 0.0; x < 4; x += 0.05 {
		if duckGain(x, o) < duckLevel-1e-9 {
			t.Fatalf("t=%v unter level", x)
		}
	}
	at := w.end + duckRelease/2
	w2 := extendDuck(w, at)
	if !near(duckGain(at, w2), duckGain(at, w)) || !near(duckGain(at+duckAttack, w2), duckLevel) {
		t.Errorf("Sprung in der Rampe hinauf: %+v", w2)
	}
}
