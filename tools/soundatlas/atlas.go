package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
)

const (
	sampleRate = 44100
	// gapSeconds Stille zwischen zwei Effekten: fängt die Encoder-Verzögerung von MP3 (~25 ms) und Rundung ab.
	gapSeconds = 0.15
)

// loopSlots spielen als Schleife je Spieler und gehören nicht in den Atlas.
var loopSlots = map[string]bool{"sfx:ride": true}

type slotFile struct {
	Datei  string `json:"datei"`
	Sprite string `json:"sprite,omitempty"`
}

type slot struct {
	ID   string     `json:"id"`
	Art  string     `json:"art"`
	Bus  string     `json:"bus"`
	Wahl []slotFile `json:"wahl"`
}

// entry ist ein Effekt im Atlas: Slot und Datei unter public/.
type entry struct {
	Slot, File string
}

// sprite ist Start und Dauer in Sekunden (auf Millisekunden gerundet).
type sprite struct {
	Start    float64 `json:"start"`
	Duration float64 `json:"duration"`
}

type atlasJSON struct {
	Comment string            `json:"_comment"`
	Files   map[string]string `json:"files"`
	Sprites map[string]sprite `json:"sprites"`
}

// pickEntries wählt rein die erste Präferenz jedes Effekt-Slots ohne Atlas-Sprite, sortiert nach Slot-ID.
func pickEntries(slots []slot) []entry {
	var out []entry
	for _, s := range slots {
		if s.Art != "sound" || s.Bus != "sfx" || loopSlots[s.ID] || len(s.Wahl) == 0 || s.Wahl[0].Sprite != "" {
			continue
		}
		out = append(out, entry{Slot: s.ID, File: s.Wahl[0].Datei})
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Slot < out[j].Slot })
	return out
}

// layout rechnet rein aus den Samples je Effekt Start und Dauer; zwischen zwei Effekten liegt gapSeconds Stille.
// Ergebnis: Sprites je Slot und die Zahl der Stille-Samples vor jedem Effekt.
func layout(entries []entry, samples []int) (map[string]sprite, []int) {
	gap := int(math.Round(gapSeconds * sampleRate))
	sprites := make(map[string]sprite, len(entries))
	pads := make([]int, len(entries))
	pos := 0
	for i, e := range entries {
		if i > 0 {
			pads[i] = gap
			pos += gap
		}
		sprites[e.Slot] = sprite{Start: ms(pos), Duration: ms(samples[i])}
		pos += samples[i]
	}
	return sprites, pads
}

func ms(samples int) float64 { return math.Round(float64(samples)/sampleRate*1000) / 1000 }

func run(root, ffmpeg string) (int, error) {
	raw, err := os.ReadFile(filepath.Join(root, "data", "assets.json"))
	if err != nil {
		return 0, err
	}
	var reg struct {
		Slots []slot `json:"slots"`
	}
	if err := json.Unmarshal(raw, &reg); err != nil {
		return 0, fmt.Errorf("data/assets.json: %w", err)
	}
	entries := pickEntries(reg.Slots)
	if len(entries) == 0 {
		return 0, fmt.Errorf("keine Effekt-Slots")
	}
	pcm := make([][]byte, len(entries))
	samples := make([]int, len(entries))
	for i, e := range entries {
		if !filepath.IsLocal(filepath.FromSlash(e.File)) {
			return 0, fmt.Errorf("%s: Pfad %q liegt nicht unter public/", e.Slot, e.File)
		}
		b, err := decode(ffmpeg, filepath.Join(root, "public", filepath.FromSlash(e.File)))
		if err != nil {
			return 0, fmt.Errorf("%s (%s): %w", e.Slot, e.File, err)
		}
		pcm[i], samples[i] = b, len(b)/2
	}
	sprites, pads := layout(entries, samples)
	var all bytes.Buffer
	for i := range entries {
		all.Write(make([]byte, pads[i]*2))
		all.Write(pcm[i])
	}
	return len(entries), write(root, ffmpeg, all.Bytes(), sprites)
}

// decode liefert die Datei als PCM s16le mono 44,1 kHz.
func decode(ffmpeg, path string) ([]byte, error) {
	if _, err := os.Stat(path); err != nil {
		return nil, err
	}
	var out, errb bytes.Buffer
	cmd := exec.Command(ffmpeg, "-v", "error", "-i", path, "-ac", "1", "-ar", fmt.Sprint(sampleRate), "-f", "s16le", "-")
	cmd.Stdout, cmd.Stderr = &out, &errb
	if err := cmd.Run(); err != nil {
		return nil, fmt.Errorf("ffmpeg: %v %s", err, strings.TrimSpace(errb.String()))
	}
	if out.Len() == 0 {
		return nil, fmt.Errorf("leer")
	}
	return out.Bytes(), nil
}

// write kodiert erst in temporäre Dateien und benennt danach um: ein Fehler hinterlässt keinen halben Atlas.
func write(root, ffmpeg string, pcm []byte, sprites map[string]sprite) error {
	dir := filepath.Join(root, "public", "audio")
	tmp, err := os.MkdirTemp("", "soundatlas-")
	if err != nil {
		return err
	}
	defer func() { _ = os.RemoveAll(tmp) }()
	rawPath := filepath.Join(tmp, "atlas.raw")
	if err := os.WriteFile(rawPath, pcm, 0o644); err != nil {
		return err
	}
	in := []string{"-v", "error", "-y", "-f", "s16le", "-ar", fmt.Sprint(sampleRate), "-ac", "1", "-i", rawPath, "-map_metadata", "-1", "-fflags", "+bitexact", "-flags:a", "+bitexact"}
	encodings := map[string][]string{
		"atlas.ogg": {"-c:a", "libvorbis", "-q:a", "3", "-serial_offset", "1"},
		"atlas.mp3": {"-c:a", "libmp3lame", "-b:a", "64k"},
	}
	for _, name := range []string{"atlas.ogg", "atlas.mp3"} {
		args := append(append(append([]string{}, in...), encodings[name]...), filepath.Join(tmp, name))
		if out, err := exec.Command(ffmpeg, args...).CombinedOutput(); err != nil {
			return fmt.Errorf("%s: %v %s", name, err, strings.TrimSpace(string(out)))
		}
	}
	js, err := json.MarshalIndent(atlasJSON{
		Comment: "Erzeugt von task gen:soundatlas (tools/soundatlas) aus data/assets.json; nicht von Hand ändern.",
		Files:   map[string]string{"ogg": "atlas.ogg", "mp3": "atlas.mp3"},
		Sprites: sprites,
	}, "", "  ")
	if err != nil {
		return err
	}
	if err := os.WriteFile(filepath.Join(tmp, "atlas.json"), append(js, '\n'), 0o644); err != nil {
		return err
	}
	// erst alle drei neben die Ziele kopieren, dann umbenennen: die drei Dateien passen immer zusammen
	names := []string{"atlas.ogg", "atlas.mp3", "atlas.json"}
	for _, name := range names {
		b, err := os.ReadFile(filepath.Join(tmp, name))
		if err != nil {
			return err
		}
		if err := os.WriteFile(filepath.Join(dir, name+".tmp"), b, 0o644); err != nil {
			return err
		}
	}
	for _, name := range names {
		if err := os.Rename(filepath.Join(dir, name+".tmp"), filepath.Join(dir, name)); err != nil {
			return err
		}
	}
	return nil
}
