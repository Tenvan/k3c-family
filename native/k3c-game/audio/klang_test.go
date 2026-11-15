package audio

import (
	"encoding/json"
	"path"
	"strings"
	"testing"

	"k3c/data"
)

// excludedSounds sind Namensteile der Kandidaten, die das Klangbild ausschließt (docs/rules/klang.md, B-935):
// Synth-Piepen und Interface-Klänge (Kenney Digital Audio, Interface Sounds) sowie 8-Bit und Chiptune.
var excludedSounds = []string{"kenney-digital-audio-", "kenney-interface-sounds-", "8-bit", "chiptune", "nes-"}

// pendingMusic sind Musik-Slots, deren Stück noch gegen das Klangbild verstößt und erst mit B-936 ersetzt wird
// (kein ruhiger Kandidat vorhanden). Nur Musik darf hier stehen; ein Effekt-Slot hat keine Ausnahme.
var pendingMusic = map[string]bool{
	"music:tag": true, "music:abend": true, "music:nacht": true, "music:kampf": true, "music:tiefe": true,
	"music:boss": true, "music:lobby": true, "music:ende": true,
}

func excluded(file string) bool {
	name := path.Base(file)
	for _, part := range excludedSounds {
		if strings.Contains(name, part) {
			return true
		}
	}
	return false
}

// B-935/AC-02: Kein Slot spielt als erste Wahl einen ausgeschlossenen Kandidaten (die erste Wahl klingt, register.go
// und tools/soundatlas); die Ausnahmen für Musik bis B-936 sind ausdrücklich gelistet.
func TestFirstChoiceFollowsSoundImage(t *testing.T) {
	raw, err := data.Files.ReadFile("sounds.json")
	if err != nil {
		t.Fatal(err)
	}
	var sf soundsFile
	if err := json.Unmarshal(raw, &sf); err != nil {
		t.Fatal(err)
	}
	for _, s := range sf.Slots {
		if len(s.Wahl) == 0 {
			continue
		}
		if pendingMusic[s.ID] {
			if s.Bus != "music" {
				t.Errorf("%s: Ausnahme nur für Musik", s.ID)
			}
			continue
		}
		if f := s.Wahl[0].Datei; excluded(f) {
			t.Errorf("%s: erste Wahl %s verstößt gegen das Klangbild", s.ID, f)
		}
	}
	if !excluded("audio/kandidaten/kenney-digital-audio-x.ogg") || excluded("audio/kandidaten/heal-rpg.mp3") {
		t.Error("Namensprüfung")
	}
}

// B-935: Die Mischpegel aus sounds.json dämpfen beide Busse und das Ducking gegenüber dem Stand davor.
func TestMixingSoftened(t *testing.T) {
	mx := realRegister(t).Mix
	if mx.Effects >= 1 || mx.Music >= 1 {
		t.Errorf("Busse nicht gedämpft: %+v", mx)
	}
	d := mx.Duck
	if d.attack <= defaultDuck.attack || d.release <= defaultDuck.release || d.level <= defaultDuck.level {
		t.Errorf("Ducking nicht sanfter: %+v", d)
	}
}

// Ungültige Mischpegel sind ein Fehler; ohne Abschnitt gilt defaultMixing.
func TestMixingParse(t *testing.T) {
	monarch := []byte(`{"skills":{}}`)
	r, err := ParseRegister([]byte(`{"slots":[]}`), monarch)
	if err != nil || r.Mix != defaultMixing {
		t.Fatalf("ohne mischung: %+v, %v", r.Mix, err)
	}
	if (&Register{}).mixing() != defaultMixing {
		t.Error("Null-Wert ohne defaultMixing")
	}
	for _, bad := range []string{
		`{"effekte":0,"musik":1,"duckPegel":0.5,"duckAnstieg":0.1,"duckHalten":1,"duckAbklingen":1}`,
		`{"effekte":1,"musik":1.2,"duckPegel":0.5,"duckAnstieg":0.1,"duckHalten":1,"duckAbklingen":1}`,
		`{"effekte":1,"musik":1,"duckPegel":1,"duckAnstieg":0.1,"duckHalten":1,"duckAbklingen":1}`,
		`{"effekte":1,"musik":1,"duckPegel":0.5,"duckAnstieg":0,"duckHalten":1,"duckAbklingen":1}`,
	} {
		if _, err := ParseRegister([]byte(`{"mischung":`+bad+`,"slots":[]}`), monarch); err == nil {
			t.Errorf("angenommen: %s", bad)
		}
	}
}
