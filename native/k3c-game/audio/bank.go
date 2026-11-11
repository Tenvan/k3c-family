package audio

import (
	"bytes"
	"fmt"
	"io"
	"log/slog"
	"os"
	"path/filepath"
	"slices"
	"strings"

	"github.com/hajimehoshi/ebiten/v2/audio/mp3"
	"github.com/hajimehoshi/ebiten/v2/audio/vorbis"
	"github.com/hajimehoshi/ebiten/v2/audio/wav"
)

// Pfade relativ zur Repo-Wurzel (Aufruf über `task native:game`, wie art.DefaultConfig).
const (
	DefaultPublicDir = "public"
	atlasJSON        = "audio/atlas.json"
)

// bank hält die Effekte dekodiert im Speicher (16 Bit Stereo, Rate des Kontexts): den Atlas einmal und die Slots
// ohne Atlas-Eintrag als Einzeldatei (Rückfall wie audioCore.ts › preloadEffects). Nach dem Laden nur gelesen.
type bank struct {
	atlas   Atlas
	pcm     []byte            // dekodierter Atlas
	singles map[string][]byte // Slot → dekodierte Einzeldatei
	rate    int
}

// clip liefert den Ton eines Effekt-Slots: Ausschnitt des Atlas, sonst die Einzeldatei; nil = stumm.
func (b *bank) clip(slot string) []byte {
	if b == nil {
		return nil
	}
	if s, ok := b.atlas.Sprites[slot]; ok && b.pcm != nil {
		lo, hi := spriteRange(s, b.rate, len(b.pcm))
		if hi > lo {
			return b.pcm[lo:hi]
		}
	}
	return b.singles[slot]
}

// loadBank lädt Atlas und Einzeldateien aus dir (public/) mit rate Hz. Fehlt etwas, bleibt der Slot stumm; jede
// fehlende oder kaputte Datei steht genau einmal im Log. Die Reihenfolge der Slots ist sortiert (gleiche Logs je Lauf).
func loadBank(dir string, reg *Register, rate int) *bank {
	b := &bank{rate: rate, singles: map[string][]byte{}}
	if err := b.loadAtlas(dir); err != nil {
		slog.Warn("❌ Ton-Atlas nicht ladbar, Effekte als Einzeldateien", "err", err)
	}
	slots := make([]string, 0, len(reg.Effects))
	for slot := range reg.Effects {
		if _, inAtlas := b.atlas.Sprites[slot]; !inAtlas || b.pcm == nil {
			slots = append(slots, slot)
		}
	}
	slices.Sort(slots)
	for _, slot := range slots {
		pcm, err := decodeFile(filepath.Join(dir, reg.Effects[slot]), rate)
		if err != nil {
			slog.Warn("❌ Ton-Datei nicht ladbar, Slot stumm", "slot", slot, "err", err)
			continue
		}
		b.singles[slot] = pcm
	}
	slog.Info("📂 Ton geladen", "atlas", len(b.atlas.Sprites), "einzeln", len(b.singles), "rate", rate)
	return b
}

// loadAtlas liest atlas.json und dekodiert die erste vorhandene Datei in der Format-Reihenfolge.
func (b *bank) loadAtlas(dir string) error {
	path := filepath.Join(dir, atlasJSON)
	raw, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("%s: %w", path, err)
	}
	a, err := ParseAtlas(raw)
	if err != nil {
		return fmt.Errorf("%s: %w", path, err)
	}
	base := filepath.Dir(path)
	name, ok := a.Pick(func(n string) bool { _, e := os.Stat(filepath.Join(base, n)); return e == nil })
	if !ok {
		return fmt.Errorf("%s: keine Ton-Datei des Atlas vorhanden", path)
	}
	pcm, err := decodeFile(filepath.Join(base, name), b.rate)
	if err != nil {
		return err
	}
	b.atlas, b.pcm = a, pcm
	return nil
}

// decodeFile dekodiert eine ganze Datei (ogg, mp3, wav nach Endung) in 16 Bit Stereo mit rate Hz.
func decodeFile(path string, rate int) ([]byte, error) {
	raw, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	s, err := decodeStream(path, bytes.NewReader(raw), rate)
	if err != nil {
		return nil, err
	}
	pcm, err := io.ReadAll(s)
	if err != nil {
		return nil, fmt.Errorf("%s: %w", path, err)
	}
	return pcm, nil
}

// pcmStream ist ein dekodierter Ton mit bekannter Länge in Bytes (für Schleifen).
type pcmStream interface {
	io.ReadSeeker
	Length() int64
}

// decodeStream wählt den Decoder nach der Endung von name (Ton-Pakete aus game/deps.go).
func decodeStream(name string, r io.ReadSeeker, rate int) (pcmStream, error) {
	var (
		s   pcmStream
		err error
	)
	switch strings.ToLower(filepath.Ext(name)) {
	case ".ogg":
		s, err = vorbis.DecodeWithSampleRate(rate, r)
	case ".mp3":
		s, err = mp3.DecodeWithSampleRate(rate, r)
	case ".wav":
		s, err = wav.DecodeWithSampleRate(rate, r)
	default:
		return nil, fmt.Errorf("%s: unbekanntes Ton-Format", name)
	}
	if err != nil {
		return nil, fmt.Errorf("%s: %w", name, err)
	}
	return s, nil
}
