package main

import "testing"

func TestPickEntriesNimmtErstePraeferenzOhneSchleifen(t *testing.T) {
	slots := []slot{
		{ID: "sfx:warn", Art: "sound", Bus: "sfx", Wahl: []slotFile{{Datei: "audio/w.ogg"}, {Datei: "audio/w2.ogg"}}},
		{ID: "sfx:coin", Art: "sound", Bus: "sfx", Wahl: []slotFile{{Datei: "audio/c.ogg"}}},
		{ID: "sfx:ride", Art: "sound", Bus: "sfx", Wahl: []slotFile{{Datei: "audio/r.ogg"}}},
		{ID: "music:tag", Art: "sound", Bus: "music", Wahl: []slotFile{{Datei: "audio/m.ogg"}}},
		{ID: "sfx:leer", Art: "sound", Bus: "sfx"},
		{ID: "sfx:alt", Art: "sound", Bus: "sfx", Wahl: []slotFile{{Datei: "audio/atlas.ogg", Sprite: "coin"}}},
		{ID: "bild", Art: "grafik", Bus: "sfx", Wahl: []slotFile{{Datei: "x.png"}}},
	}
	got := pickEntries(slots)
	want := []entry{{Slot: "sfx:coin", File: "audio/c.ogg"}, {Slot: "sfx:warn", File: "audio/w.ogg"}}
	if len(got) != len(want) {
		t.Fatalf("got %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("got %v, want %v", got, want)
		}
	}
}

func TestLayoutLegtPausenZwischenDieEffekte(t *testing.T) {
	entries := []entry{{Slot: "a"}, {Slot: "b"}, {Slot: "c"}}
	sprites, pads := layout(entries, []int{44100, 22050, 4410})
	gap := 6615 // 0,15 s
	if pads[0] != 0 || pads[1] != gap || pads[2] != gap {
		t.Fatalf("pads %v", pads)
	}
	if sprites["a"] != (sprite{Start: 0, Duration: 1}) {
		t.Fatalf("a %v", sprites["a"])
	}
	if sprites["b"] != (sprite{Start: 1.15, Duration: 0.5}) {
		t.Fatalf("b %v", sprites["b"])
	}
	if sprites["c"] != (sprite{Start: 1.8, Duration: 0.1}) {
		t.Fatalf("c %v", sprites["c"])
	}
}
