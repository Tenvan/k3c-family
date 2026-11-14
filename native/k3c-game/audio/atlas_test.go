package audio

import (
	"encoding/json"
	"math"
	"os"
	"path/filepath"
	"testing"

	"k3c/data"
	"k3c/native/k3c-game/view"
)

// repoPublic ist public/ aus Sicht des Paket-Ordners (go test läuft in native/k3c-game/audio).
var repoPublic = filepath.Join("..", "..", "..", DefaultPublicDir)

func realRegister(t *testing.T) *Register {
	t.Helper()
	reg, err := LoadRegister(data.Files)
	if err != nil {
		t.Fatal(err)
	}
	return &reg
}

// AC-01: atlas.json lesen, Start und Dauer je Slot wie in der Datei; jeder Effekt-Slot des Registers außer der
// Hufschlag-Schleife liegt im Atlas (public/audio/README.md).
func TestAtlasStartAndDurationPerSlot(t *testing.T) {
	raw, err := os.ReadFile(filepath.Join(repoPublic, atlasJSON))
	if err != nil {
		t.Fatal(err)
	}
	a, err := ParseAtlas(raw)
	if err != nil {
		t.Fatal(err)
	}
	var want struct {
		Sprites map[string]map[string]float64 `json:"sprites"`
	}
	if err := json.Unmarshal(raw, &want); err != nil {
		t.Fatal(err)
	}
	if len(a.Sprites) != len(want.Sprites) || len(a.Sprites) == 0 {
		t.Fatalf("Sprites %d, Datei %d", len(a.Sprites), len(want.Sprites))
	}
	for slot, w := range want.Sprites {
		if s := a.Sprites[slot]; s.Start != w["start"] || s.Duration != w["duration"] {
			t.Errorf("%s: %+v, Datei %v", slot, s, w)
		}
	}
	for slot := range realRegister(t).Effects {
		if _, ok := a.Sprites[slot]; !ok && slot != slotRide {
			t.Errorf("Effekt-Slot %s fehlt im Atlas", slot)
		}
	}
	name, ok := a.Pick(func(string) bool { return true })
	if !ok || name != "atlas.ogg" {
		t.Errorf("Format: %q, erwartet atlas.ogg (Ogg Vorbis zuerst)", name)
	}
}

func TestParseAtlasDropsInvalidSprites(t *testing.T) {
	raw := `{"files":{"mp3":"a.mp3"},"sprites":{"ok":{"start":1,"duration":0.5},"neg":{"start":-1,"duration":1},"zero":{"start":2,"duration":0}}}`
	a, err := ParseAtlas([]byte(raw))
	if err != nil {
		t.Fatal(err)
	}
	if len(a.Sprites) != 1 || a.Sprites["ok"].Duration != 0.5 {
		t.Errorf("Sprites %+v", a.Sprites)
	}
	if n, ok := a.Pick(func(n string) bool { return n == "a.mp3" }); !ok || n != "a.mp3" {
		t.Errorf("Rückfall auf mp3: %q %v", n, ok)
	}
	if _, ok := a.Pick(func(string) bool { return false }); ok {
		t.Error("ohne vorhandene Datei kein Format")
	}
	for _, bad := range []string{`{"sprites":{}}`, `{"files":{"ogg":"a.ogg"}}`, `kein json`} {
		if _, err := ParseAtlas([]byte(bad)); err == nil {
			t.Errorf("%s: Fehler erwartet", bad)
		}
	}
}

func TestSpriteRangeWholeFramesAndClamped(t *testing.T) {
	const rate = 100
	total := 1000 * pcmFrame // 10 s
	cases := []struct {
		s      Sprite
		lo, hi int
	}{
		{Sprite{Start: 1, Duration: 0.5}, 100 * pcmFrame, 150 * pcmFrame},
		{Sprite{Start: 9.8, Duration: 1}, 980 * pcmFrame, 1000 * pcmFrame}, // am Ende begrenzt
		{Sprite{Start: 20, Duration: 1}, total, total},                    // außerhalb: leer
	}
	for _, c := range cases {
		if lo, hi := spriteRange(c.s, rate, total); lo != c.lo || hi != c.hi {
			t.Errorf("%+v: [%d,%d), erwartet [%d,%d)", c.s, lo, hi, c.lo, c.hi)
		}
	}
}

// AC-01 ohne Gerät: Der Ton lädt aus public/ (dekodiert ohne Kontext), jeder Atlas-Slot hat die Länge seines Sprites,
// der Hufschlag lädt als Einzeldatei (Rückfall).
func TestLoadBankDecodesAtlasAndSingles(t *testing.T) {
	if testing.Short() {
		t.Skip("dekodiert den Atlas")
	}
	b := loadBank(repoPublic, realRegister(t), sampleRate)
	if b.pcm == nil {
		t.Fatal("Atlas nicht geladen")
	}
	for slot, s := range b.atlas.Sprites {
		got := float64(len(b.clip(slot))) / pcmFrame / sampleRate
		if math.Abs(got-s.Duration) > 0.001 {
			t.Errorf("%s: %.3f s, Sprite %.3f s", slot, got, s.Duration)
		}
	}
	if len(b.clip(slotRide)) == 0 {
		t.Error("Hufschlag fehlt (Einzeldatei)")
	}
	if b.clip("sfx:gibtsnicht") != nil {
		t.Error("unbekannter Slot muss stumm sein")
	}
}

// Fehlende Ton-Dateien: stumm mit Meldung, kein Absturz.
func TestLoadBankMissingFilesIsSilent(t *testing.T) {
	reg := &Register{Effects: map[string]string{"sfx:coin": "audio/fehlt.ogg"}}
	b := loadBank(t.TempDir(), reg, sampleRate)
	if b.pcm != nil || b.clip("sfx:coin") != nil {
		t.Error("ohne Dateien muss alles stumm sein")
	}
	var nilBank *bank
	if nilBank.clip("sfx:coin") != nil {
		t.Error("noch nicht geladen: stumm")
	}
}

// AC-01: Lautstärke je Bus wirkt (ohne Gerät): Effekte und Musik folgen getrennt den Einstellungen.
func TestBusVolumesFromSettings(t *testing.T) {
	p := newPlanner(realRegister(t))
	f := &view.Frame{Settings: view.Settings{Music: 0.25, Effects: 0.5}, Cues: []view.Cue{{Kind: view.CueConfirm}}}
	m := p.plan(f)
	if m.EffectBus != 0.5 || m.MusicBus != 0.25 {
		t.Fatalf("Busse %v / %v", m.EffectBus, m.MusicBus)
	}
	if v := m.EffectVolume(m.Effects[0].Gain); v != 0.5 {
		t.Errorf("Effekt %v, erwartet 0.5", v)
	}
	if v := m.MusicVolume(0.8); v != 0.2 {
		t.Errorf("Musik %v, erwartet 0.2", v)
	}
	f.Settings = view.Settings{Music: 7, Effects: math.NaN()}
	if m = p.plan(f); m.MusicBus != 1 || m.EffectBus != 1 {
		t.Errorf("begrenzt: %v / %v", m.MusicBus, m.EffectBus)
	}
}
