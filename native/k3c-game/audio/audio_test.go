package audio

import (
	"testing"

	"k3c/native/k3c-game/view"
)

type recorder struct{ mixes int }

func (r *recorder) apply(*Mix) { r.mixes++ }

// In Tests ist der Ton stumm (kein Kontext); Listen läuft je Frame genau einmal durch die Wiedergabe.
func TestNewIsSilentInTestsAndListens(t *testing.T) {
	s, ok := New(nil).(*Sound)
	if !ok {
		t.Fatal("New liefert keinen *Sound")
	}
	if _, isSilent := s.out.(silent); !isSilent {
		t.Errorf("Wiedergabe in Tests: %T", s.out)
	}
	r := &recorder{}
	s.out = r
	s.Listen(&view.Frame{Settings: view.DefaultSettings()})
	s.Listen(&view.Frame{Settings: view.DefaultSettings()})
	if r.mixes != 2 {
		t.Errorf("%d Mixe", r.mixes)
	}
}

// Aufnahme-Modus: Der Mix wird gerechnet, aber nicht ausgegeben.
func TestRecordingStaysSilent(t *testing.T) {
	s := New(nil).(*Sound)
	r := &recorder{}
	s.out = r
	s.Listen(&view.Frame{Settings: view.DefaultSettings(), Recording: true})
	if r.mixes != 0 || s.plan.tick != 1 {
		t.Errorf("Aufnahme: %d Mixe ausgegeben, %d gerechnet", r.mixes, s.plan.tick)
	}
}

// Ohne hörbaren Mix entsteht kein Kontext; ein abgeschalteter device bleibt still.
func TestDeviceStaysOffWithoutSound(t *testing.T) {
	d := newDevice(t.TempDir(), &Register{})
	d.apply(&Mix{EffectBus: 1, MusicBus: 1})
	if d.ctx != nil {
		t.Error("Kontext ohne hörbaren Klang angelegt")
	}
	if audible(&Mix{EffectBus: 0, Effects: []Request{{Slot: "sfx:coin", Gain: 1}}}) {
		t.Error("Effekte bei Lautstärke 0 sind nicht hörbar")
	}
	if !audible(&Mix{MusicBus: 1, Music: [2]MusicVoice{{Slot: "music:tag", Gain: 0.1}}}) {
		t.Error("Musik ist hörbar")
	}
	d.off = true
	d.apply(&Mix{EffectBus: 1, Effects: []Request{{Slot: "sfx:coin", Gain: 1}}})
	if d.ctx != nil {
		t.Error("abgeschaltet: kein Kontext")
	}
}
