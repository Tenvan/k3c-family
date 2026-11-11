package audio

import (
	"bytes"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"

	"github.com/hajimehoshi/ebiten/v2/audio"
)

// sampleRate des Ton-Kontexts: die Rate des Atlas (public/audio/README.md: 44,1 kHz).
const sampleRate = 44100

// maxVoices begrenzt gleichzeitige Instanzen je Effekt-Slot (audioCore.ts › MAX_SAMPLE_VOICES).
const maxVoices = 4

// output setzt einen Mix um.
type output interface{ apply(m *Mix) }

// silent ist die Wiedergabe ohne Ton (Tests, Aufnahme-Modus, kein Gerät).
type silent struct{}

func (silent) apply(*Mix) {}

type voice struct {
	slot string
	gain float64
	p    *audio.Player
}

type track struct {
	slot string
	f    *os.File
	p    *audio.Player
}

type loopVoice struct {
	key int
	p   *audio.Player
}

// device ist die dünne Schicht über Ebitengine: ein Kontext, erst beim ersten hörbaren Mix angelegt; der Ton lädt
// danach im Hintergrund (bis dahin klingen nur Musikstücke). Fehler machen stumm, nie einen Absturz.
type device struct {
	dir     string
	reg     *Register
	ctx     *audio.Context
	off     bool
	loading chan *bank
	bank    *bank
	voices  []voice
	tracks  []track
	loops   []loopVoice
	warned  map[string]bool
	bus     float64 // zuletzt angewandte Effekt-Lautstärke
}

func newDevice(dir string, reg *Register) *device {
	return &device{dir: dir, reg: reg, warned: map[string]bool{}, bus: -1}
}

// apply setzt den Mix um: Effekte starten, Musik und Schleifen auf Stand bringen, fertige Stimmen freigeben.
func (d *device) apply(m *Mix) {
	if d.off || (d.ctx == nil && !audible(m)) {
		return
	}
	if d.ctx == nil && !d.start() {
		return
	}
	d.poll()
	d.reap(m)
	for _, r := range m.Effects {
		d.play(r, m)
	}
	d.syncMusic(m)
	d.syncLoops(m)
}

// audible meldet, ob der Mix etwas Hörbares verlangt (erst dann entsteht der Kontext).
func audible(m *Mix) bool {
	music := m.MusicBus > 0 && (m.Music[0].Gain > 0 || m.Music[1].Gain > 0)
	return music || (m.EffectBus > 0 && (len(m.Effects) > 0 || len(m.Loops) > 0))
}

// start legt den Kontext an (oder übernimmt einen vorhandenen) und lädt den Ton im Hintergrund.
func (d *device) start() (ok bool) {
	defer func() {
		if r := recover(); r != nil {
			slog.Warn("❌ Ton-Kontext nicht startbar, Spiel läuft stumm", "err", fmt.Sprint(r))
			d.off, ok = true, false
		}
	}()
	d.ctx = audio.CurrentContext()
	if d.ctx == nil {
		d.ctx = audio.NewContext(sampleRate)
	}
	d.loading = make(chan *bank, 1)
	go func(rate int) { d.loading <- loadBank(d.dir, d.reg, rate) }(d.ctx.SampleRate())
	slog.Info("🚀 Ton-Kontext angelegt", "rate", d.ctx.SampleRate())
	return true
}

// poll übernimmt den geladenen Ton, sobald er fertig ist (ohne zu warten).
func (d *device) poll() {
	if d.bank != nil {
		return
	}
	select {
	case b := <-d.loading:
		d.bank = b
	default:
	}
}

// reap gibt fertige Effekt-Stimmen frei und stellt laufende auf eine geänderte Effekt-Lautstärke.
func (d *device) reap(m *Mix) {
	keep := d.voices[:0]
	for _, v := range d.voices {
		if !v.p.IsPlaying() {
			v.p.PauseAndStopReading()
			continue
		}
		if m.EffectBus != d.bus {
			v.p.SetVolume(m.EffectVolume(v.gain))
		}
		keep = append(keep, v)
	}
	clear(d.voices[len(keep):])
	d.voices = keep
	d.bus = m.EffectBus
}

// play startet einen Effekt, wenn er hörbar ist, sein Ton geladen ist und weniger als maxVoices davon laufen.
func (d *device) play(r Request, m *Mix) {
	vol := m.EffectVolume(r.Gain)
	pcm := d.bank.clip(r.Slot)
	if vol <= 0 || len(pcm) == 0 || d.count(r.Slot) >= maxVoices {
		return
	}
	p, err := d.ctx.NewPlayer(bytes.NewReader(pcm))
	if err != nil {
		d.warnOnce(r.Slot, err)
		return
	}
	p.SetVolume(vol)
	p.Play()
	d.voices = append(d.voices, voice{slot: r.Slot, gain: r.Gain, p: p})
}

func (d *device) count(slot string) int {
	n := 0
	for _, v := range d.voices {
		if v.slot == slot {
			n++
		}
	}
	return n
}

// syncMusic hält genau die Stücke des Mix offen und stellt ihre Lautstärke; andere enden.
func (d *device) syncMusic(m *Mix) {
	keep := d.tracks[:0]
	for _, t := range d.tracks {
		if t.slot == m.Music[0].Slot || t.slot == m.Music[1].Slot {
			keep = append(keep, t)
			continue
		}
		t.p.PauseAndStopReading()
		_ = t.f.Close()
	}
	clear(d.tracks[len(keep):])
	d.tracks = keep
	for _, mv := range m.Music {
		if mv.Slot == "" {
			continue
		}
		t := d.track(mv.Slot)
		if t != nil {
			t.p.SetVolume(m.MusicVolume(mv.Gain))
		}
	}
}

// track liefert das offene Stück eines Musik-Slots oder öffnet es als Schleife (Datei gestreamt); nil = stumm.
func (d *device) track(slot string) *track {
	for i := range d.tracks {
		if d.tracks[i].slot == slot {
			return &d.tracks[i]
		}
	}
	if d.warned[slot] {
		return nil
	}
	path := filepath.Join(d.dir, d.reg.Music[slot])
	f, err := os.Open(path)
	if err != nil {
		d.warnOnce(slot, err)
		return nil
	}
	s, err := decodeStream(path, f, d.ctx.SampleRate())
	var p *audio.Player
	if err == nil {
		p, err = d.ctx.NewPlayer(audio.NewInfiniteLoop(s, s.Length()))
	}
	if err != nil {
		_ = f.Close()
		d.warnOnce(slot, err)
		return nil
	}
	p.SetVolume(0)
	p.Play()
	d.tracks = append(d.tracks, track{slot: slot, f: f, p: p})
	return &d.tracks[len(d.tracks)-1]
}

// syncLoops hält die Hufschlag-Schleifen auf dem gewünschten Stand: neue starten, fehlende enden, vorhandene stellen
// nur die Lautstärke nach (audioCore.ts › syncLoops).
func (d *device) syncLoops(m *Mix) {
	keep := d.loops[:0]
	for _, l := range d.loops {
		if gain, ok := loopGain(m, l.key); ok {
			l.p.SetVolume(m.EffectVolume(gain))
			keep = append(keep, l)
			continue
		}
		l.p.PauseAndStopReading()
	}
	clear(d.loops[len(keep):])
	d.loops = keep
	pcm := d.bank.clip(slotRide)
	if len(pcm) == 0 {
		return
	}
	for _, w := range m.Loops {
		if d.hasLoop(w.Key) {
			continue
		}
		p, err := d.ctx.NewPlayer(audio.NewInfiniteLoop(bytes.NewReader(pcm), int64(len(pcm))))
		if err != nil {
			d.warnOnce(slotRide, err)
			return
		}
		p.SetVolume(m.EffectVolume(w.Gain))
		p.Play()
		d.loops = append(d.loops, loopVoice{key: w.Key, p: p})
	}
}

func loopGain(m *Mix, key int) (float64, bool) {
	for _, l := range m.Loops {
		if l.Key == key {
			return l.Gain, true
		}
	}
	return 0, false
}

func (d *device) hasLoop(key int) bool {
	for _, l := range d.loops {
		if l.key == key {
			return true
		}
	}
	return false
}

// warnOnce meldet einen Ton-Fehler je Slot genau einmal; der Slot bleibt stumm.
func (d *device) warnOnce(slot string, err error) {
	if d.warned[slot] {
		return
	}
	d.warned[slot] = true
	slog.Warn("❌ Ton nicht abspielbar, Slot stumm", "slot", slot, "err", err)
}
