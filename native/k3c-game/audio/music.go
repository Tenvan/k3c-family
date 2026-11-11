package audio

import "math"

// musicState ist ein Musikzustand wie src/audio/music.ts › MUSIC_STATES; stateNone = noch keiner bzw. Lücke.
type musicState int

const (
	stateNone musicState = iota
	stateDay
	stateDusk
	stateNight
	stateBattle
	stateDepth
	stateBoss
	stateLobby
	stateDefeat
	stateEnding
)

// musicSlots bildet jeden Zustand auf genau einen Musik-Slot ab (music.ts › MUSIC_SLOTS).
var musicSlots = [...]string{
	stateNone:   "",
	stateDay:    "music:tag",
	stateDusk:   "music:abend",
	stateNight:  "music:nacht",
	stateBattle: "music:kampf",
	stateDepth:  "music:tiefe",
	stateBoss:   "music:boss",
	stateLobby:  "music:lobby",
	stateDefeat: "music:niederlage",
	stateEnding: "music:ende",
}

// musicView ist der Auszug, den der Automat braucht (music.ts › MusicView). boss und victory setzt der Browser
// (musicViewOf) bisher nie; der Automat kennt sie, damit die Priorität vollständig bleibt.
type musicView struct {
	lobby          bool
	phase          string
	enemies, depth int
	boss, victory  bool
}

// nextMusicState ist der Zustands-Automat wie music.ts › nextMusicState. Priorität: Lobby > Niederlage > Sieg > Boss >
// Kampf > Tiefe > Tageszeit; Niederlage (castleFallen) und Sieg bleiben, bis die Lobby wieder läuft. Morgengrauen hat
// kein eigenes Stück, die Nachtmusik läuft weiter. Eine unbekannte Phase ist eine Lücke (Stille).
func nextMusicState(prev musicState, v musicView, castleFallen bool) musicState {
	switch {
	case v.lobby:
		return stateLobby
	case prev == stateDefeat || castleFallen:
		return stateDefeat
	case prev == stateEnding || v.victory:
		return stateEnding
	case v.boss:
		return stateBoss
	case v.enemies > 0:
		return stateBattle
	case v.depth > 0:
		return stateDepth
	}
	switch v.phase {
	case "day":
		return stateDay
	case "dusk":
		return stateDusk
	case "night", "dawn":
		return stateNight
	default:
		return stateNone
	}
}

// crossfadeTicks ist die Dauer eines Stückwechsels in Takten: 2 s (crossfade.ts › CROSSFADE_SECONDS) bei 30 Hz.
const crossfadeTicks = 2 * tps

// MusicVoice ist ein Musikstück mit seinem Pegel 0 … 1 (vor Bus-Lautstärke); Slot "" = keins.
type MusicVoice struct {
	Slot string
	Gain float64
}

// fader blendet die Musik über Takte über (crossfade.ts: gleiche Leistung, sin hinein, cos hinaus). Ein Wechsel mitten
// im Übergang blendet das laufende Stück vom aktuellen Pegel aus (fadeOutFrom); kehrt das ausblendende Stück zurück,
// steigt es ohne Sprung vom aktuellen Pegel wieder an.
type fader struct {
	in, out         string
	inStep, outStep int
	outFrom         float64
}

// inLevel ist der Pegel des einblendenden Stücks.
func (d *fader) inLevel() float64 {
	if d.in == "" {
		return 0
	}
	return math.Sin(math.Pi / 2 * float64(min(d.inStep, crossfadeTicks)) / crossfadeTicks)
}

// outLevel ist der Pegel des ausblendenden Stücks.
func (d *fader) outLevel() float64 {
	if d.out == "" {
		return 0
	}
	return d.outFrom * math.Cos(math.Pi/2*float64(min(d.outStep, crossfadeTicks))/crossfadeTicks)
}

// target setzt das gewünschte Stück ("" = Stille). Dasselbe Stück läuft einfach weiter.
func (d *fader) target(slot string) {
	if slot == d.in {
		return
	}
	back := slot != "" && slot == d.out
	backLevel := d.outLevel()
	if d.in != "" {
		d.out, d.outFrom, d.outStep = d.in, d.inLevel(), 0
	} else if back {
		d.out = ""
	}
	d.in, d.inStep = slot, 0
	if back {
		// Rückkehr: dort weitermachen, wo der Pegel gerade steht (asin, gleiche Kurve)
		d.inStep = int(math.Round(math.Asin(math.Min(1, backLevel)) * 2 / math.Pi * crossfadeTicks))
	}
}

// step rückt den Übergang um einen Takt vor; ein ausgeblendetes Stück fällt weg.
func (d *fader) step() {
	if d.in != "" && d.inStep < crossfadeTicks {
		d.inStep++
	}
	if d.out != "" {
		d.outStep++
		if d.outStep >= crossfadeTicks {
			d.out, d.outStep = "", 0
		}
	}
}

// voices liefert das einblendende bzw. laufende Stück [0] und das ausblendende [1].
func (d *fader) voices() [2]MusicVoice {
	return [2]MusicVoice{{Slot: d.in, Gain: d.inLevel()}, {Slot: d.out, Gain: d.outLevel()}}
}
