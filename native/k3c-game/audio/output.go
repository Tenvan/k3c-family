package audio

import (
	"bytes"
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
	p    player
}

type track struct {
	slot string
	f    *os.File
	p    player
}

// stop beendet ein Stück: PauseAndStopReading wartet auf einen laufenden Lesevorgang des oto-Threads, erst danach
// ist das Schließen der Datei sicher.
func (t track) stop() {
	t.p.PauseAndStopReading()
	_ = t.f.Close()
}

type loopVoice struct {
	key int
	p   player
}

// device ist die dünne Schicht über oto (sink.go): ein Kontext, erst beim ersten hörbaren Mix angelegt; der Ton lädt
// danach im Hintergrund (bis dahin klingen nur Musikstücke). oto mischt die Stimmen, jede mit der Lautstärke aus dem
// Mix (Pegel mal Bus). Fehler machen stumm, nie einen Absturz (B-805).
//
// Threads: Alle Felder gehören dem Spiel-Thread (Listen). Der Lader übergibt die Bank einmal über loading; danach
// wird sie nur gelesen. oto liest die Quellen der Player in seinem Thread; jede Quelle gehört nach Play allein oto,
// die Datei eines Stücks schließt erst stop. Treiberfehler liest Err unter oto-eigenem Mutex. Kein eigener Mutex nötig.
type device struct {
	dir     string
	reg     *Register
	open    func() (sink, error) // openOto, in Tests ein Fake
	ctx     sink
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
	return &device{dir: dir, reg: reg, open: openOto, warned: map[string]bool{}, bus: -1}
}

// apply setzt den Mix um: Effekte starten, Musik und Schleifen auf Stand bringen, fertige Stimmen freigeben.
func (d *device) apply(m *Mix) {
	if d.off || (d.ctx == nil && !audible(m)) {
		return
	}
	if d.ctx == nil && !d.start() {
		return
	}
	if err := d.ctx.Err(); err != nil {
		d.fail(err)
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

// start legt den Kontext an und lädt den Ton im Hintergrund; ohne Kontext bleibt der Ton aus.
func (d *device) start() bool {
	ctx, err := d.open()
	if err != nil {
		slog.Warn("❌ Ton-Gerät nicht startbar, Spiel läuft stumm", "err", err)
		d.off = true
		return false
	}
	d.ctx = ctx
	ch := make(chan *bank, 1) // gepuffert: der Lader endet auch, wenn niemand mehr abholt (Ton aus)
	d.loading = ch
	go func(dir string, reg *Register) { ch <- loadBank(dir, reg, sampleRate) }(d.dir, d.reg)
	slog.Info("🚀 Ton-Gerät angelegt (oto)", "rate", sampleRate)
	return true
}

// fail schaltet den Ton nach einem Treiberfehler ab (B-805): eine Log-Zeile, alle Stimmen enden, danach bleibt der
// Ton aus bis zum Neustart (kein Wiederverbinden). Der Fehler geht nicht nach außen, Listen und das Spiel laufen weiter.
func (d *device) fail(err error) {
	slog.Warn("💥 Ton-Gerät ausgefallen, Spiel läuft stumm bis zum Neustart", "err", err)
	for _, v := range d.voices {
		v.p.PauseAndStopReading()
	}
	for _, l := range d.loops {
		l.p.PauseAndStopReading()
	}
	for _, t := range d.tracks {
		t.stop()
	}
	d.voices, d.loops, d.tracks = nil, nil, nil
	d.off = true
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
	p := d.ctx.NewPlayer(bytes.NewReader(pcm))
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

// syncMusic hält genau die Stücke des Mix offen und stellt ihre Lautstärke; andere enden, ein Stück mit Lesefehler
// auch (Slot danach stumm).
func (d *device) syncMusic(m *Mix) {
	keep := d.tracks[:0]
	for _, t := range d.tracks {
		if err := t.p.Err(); err != nil {
			d.warnOnce(t.slot, err)
		} else if t.slot == m.Music[0].Slot || t.slot == m.Music[1].Slot {
			keep = append(keep, t)
			continue
		}
		t.stop()
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
	s, err := decodeStream(path, f, sampleRate)
	if err != nil {
		_ = f.Close()
		d.warnOnce(slot, err)
		return nil
	}
	// audio.NewInfiniteLoop ist ein reiner Strom (Schleife mit Überblendung an der Naht), kein Ebitengine-Kontext
	p := d.ctx.NewPlayer(audio.NewInfiniteLoop(s, s.Length()))
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
		p := d.ctx.NewPlayer(audio.NewInfiniteLoop(bytes.NewReader(pcm), int64(len(pcm))))
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
