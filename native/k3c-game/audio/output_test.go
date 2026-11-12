package audio

import (
	"bytes"
	"encoding/binary"
	"errors"
	"io"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/native/k3c-game/view"
)

// fakePlayer und fakeSink stellen das Ton-Gerät nach (alles im Test-Thread, ohne oto).
type fakePlayer struct {
	playing, stopped bool
	vol              float64
	err              error
}

func (p *fakePlayer) Play()                { p.playing = true }
func (p *fakePlayer) IsPlaying() bool      { return p.playing && !p.stopped }
func (p *fakePlayer) SetVolume(v float64)  { p.vol = v }
func (p *fakePlayer) PauseAndStopReading() { p.stopped = true }
func (p *fakePlayer) Err() error           { return p.err }

type fakeSink struct {
	players []*fakePlayer
	err     error
	opens   int
}

func (s *fakeSink) NewPlayer(io.Reader) player {
	p := &fakePlayer{}
	s.players = append(s.players, p)
	return p
}

func (s *fakeSink) Err() error { return s.err }

// captureLog lenkt slog in einen Puffer (Tests im Paket laufen nicht parallel).
func captureLog(t *testing.T) *bytes.Buffer {
	t.Helper()
	var buf bytes.Buffer
	prev := slog.Default()
	slog.SetDefault(slog.New(slog.NewTextHandler(&buf, nil)))
	t.Cleanup(func() { slog.SetDefault(prev) })
	return &buf
}

// fakeDevice liefert ein device über fs; awaitBank wartet den Lader ab (danach schreibt er nicht mehr ins Log).
func fakeDevice(t *testing.T, reg *Register, fs *fakeSink) *device {
	t.Helper()
	d := newDevice(t.TempDir(), reg)
	d.open = func() (sink, error) { fs.opens++; return fs, nil }
	return d
}

func awaitBank(d *device) {
	if d.bank == nil {
		d.bank = <-d.loading
	}
}

// AC-01 (B-805): Das Gerät lässt sich nicht anlegen → Log-Zeile, stumm, kein zweiter Versuch, kein Fehler nach außen.
func TestDeviceOpenFailureStaysSilent(t *testing.T) {
	log := captureLog(t)
	d := newDevice(t.TempDir(), &Register{})
	opens := 0
	d.open = func() (sink, error) { opens++; return nil, errors.New("oto: context is already created") }
	m := &Mix{EffectBus: 1, Effects: []Request{{Slot: "sfx:coin", Gain: 1}}}
	d.apply(m)
	d.apply(m)
	if !d.off || d.ctx != nil || opens != 1 {
		t.Errorf("off=%v ctx=%v Versuche=%d", d.off, d.ctx, opens)
	}
	if !strings.Contains(log.String(), "❌ Ton-Gerät nicht startbar") {
		t.Errorf("Log-Zeile fehlt: %q", log.String())
	}
}

// AC-01 (B-805): Der Treiber fällt im Lauf aus → eine Log-Zeile, alle Stimmen enden, keine neuen, Listen läuft weiter.
// Davor: Lautstärken wie im Mix (Pegel mal Bus), auch nach einer Änderung des Busses.
func TestDeviceDriverFailureMutesAndGameGoesOn(t *testing.T) {
	reg := &Register{Effects: map[string]string{}, Music: map[string]string{}}
	fs := &fakeSink{}
	d := fakeDevice(t, reg, fs)
	m := &Mix{EffectBus: 0.5, MusicBus: 1, Effects: []Request{{Slot: "sfx:coin", Gain: 0.8}}, Loops: []Loop{{Key: 0, Gain: 1}}}
	d.apply(m)
	awaitBank(d)
	log := captureLog(t)
	d.bank.singles["sfx:coin"], d.bank.singles[slotRide] = make([]byte, 400), make([]byte, 400)
	d.apply(m)
	if len(fs.players) != 2 || fs.players[0].vol != 0.4 || fs.players[1].vol != 0.5 || !fs.players[0].IsPlaying() {
		t.Fatalf("Stimmen: %d, erste %+v", len(fs.players), fs.players[0])
	}
	m.Effects, m.EffectBus = nil, 1
	d.apply(m)
	if fs.players[0].vol != 0.8 || fs.players[1].vol != 1 {
		t.Errorf("Bus nicht nachgezogen: %v, %v", fs.players[0].vol, fs.players[1].vol)
	}

	fs.err = errors.New("oto: initialization failed: WASAPI: device invalidated")
	d.apply(m)
	for i, p := range fs.players {
		if !p.stopped {
			t.Errorf("Stimme %d läuft nach dem Ausfall weiter", i)
		}
	}
	if !d.off || strings.Count(log.String(), "💥 Ton-Gerät ausgefallen") != 1 {
		t.Errorf("off=%v, Log: %q", d.off, log.String())
	}
	m.Effects = []Request{{Slot: "sfx:coin", Gain: 1}}
	d.apply(m)
	s := newSound(reg, d)
	s.Listen(&view.Frame{Settings: view.DefaultSettings()})
	s.Listen(&view.Frame{Settings: view.DefaultSettings()})
	if len(fs.players) != 2 || fs.opens != 1 || s.plan.tick != 2 {
		t.Errorf("nach dem Ausfall: %d Stimmen, %d Kontexte, %d Takte", len(fs.players), fs.opens, s.plan.tick)
	}
	if strings.Count(log.String(), "💥") != 1 {
		t.Errorf("Ausfall mehrfach gemeldet: %q", log.String())
	}
}

// Ein Musikstück mit Lesefehler endet (Datei zu), der Slot bleibt stumm und wird nicht neu geöffnet; Lautstärke wie
// im Mix.
func TestMusicReadErrorDropsTrack(t *testing.T) {
	fs := &fakeSink{}
	reg := &Register{Effects: map[string]string{}, Music: map[string]string{"music:tag": "tag.wav"}}
	d := fakeDevice(t, reg, fs)
	writeWAV(t, filepath.Join(d.dir, "tag.wav"), 256)
	m := &Mix{MusicBus: 0.5, Music: [2]MusicVoice{{Slot: "music:tag", Gain: 0.6}}}
	d.apply(m)
	awaitBank(d)
	if len(d.tracks) != 1 || fs.players[0].vol != 0.3 || !fs.players[0].IsPlaying() {
		t.Fatalf("Stück: %d offen, Spieler %+v", len(d.tracks), fs.players)
	}
	log := captureLog(t)
	f := d.tracks[0].f
	fs.players[0].err = errors.New("read: Datei weg")
	d.apply(m)
	d.apply(m)
	if len(d.tracks) != 0 || len(fs.players) != 1 || !fs.players[0].stopped {
		t.Errorf("nach Lesefehler: %d offen, %d Spieler", len(d.tracks), len(fs.players))
	}
	if !errors.Is(f.Close(), os.ErrClosed) {
		t.Error("Datei des Stücks nicht geschlossen")
	}
	if strings.Count(log.String(), "Ton nicht abspielbar") != 1 || d.off {
		t.Errorf("off=%v, Log: %q", d.off, log.String())
	}
}

// writeWAV schreibt eine stille WAV-Datei (16 Bit Stereo, sampleRate) mit frames Abtastwerten je Kanal.
func writeWAV(t *testing.T, path string, frames int) {
	t.Helper()
	size := uint32(frames * 4)
	le := binary.LittleEndian
	b := []byte("RIFF")
	b = le.AppendUint32(b, 36+size)
	b = append(b, "WAVEfmt "...)
	b = le.AppendUint32(b, 16)
	b = le.AppendUint16(b, 1) // PCM
	b = le.AppendUint16(b, 2)
	b = le.AppendUint32(b, sampleRate)
	b = le.AppendUint32(b, sampleRate*4)
	b = le.AppendUint16(b, 4)
	b = le.AppendUint16(b, 16)
	b = append(b, "data"...)
	b = le.AppendUint32(b, size)
	b = append(b, make([]byte, size)...)
	if err := os.WriteFile(path, b, 0o600); err != nil {
		t.Fatal(err)
	}
}
