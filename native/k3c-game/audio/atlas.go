package audio

import (
	"encoding/json"
	"errors"
	"fmt"
	"math"
)

// Sprite ist ein Ausschnitt des Ton-Atlas in Sekunden (public/audio/atlas.json, task gen:soundatlas).
type Sprite struct {
	Start    float64 `json:"start"`
	Duration float64 `json:"duration"`
}

// Atlas beschreibt den Ton-Atlas: eine Datei je Format und Start und Dauer je Effekt-Slot (src/audio/atlas.ts).
type Atlas struct {
	Files   map[string]string `json:"files"`   // Format (ogg, mp3, wav) → Dateiname neben atlas.json
	Sprites map[string]Sprite `json:"sprites"` // Effekt-Slot → Ausschnitt
}

// formatOrder ist die Reihenfolge der Formate wie im Browser (atlas.ts › FORMAT_ORDER); alle drei dekodiert Ebitengine.
var formatOrder = [...]string{"ogg", "mp3", "wav"}

// ParseAtlas prüft die JSON-Beschreibung wie atlas.ts › parseAtlas: ungültige Sprites (negativer Start, Dauer ≤ 0,
// keine Zahl) fallen weg; ohne Dateien oder Sprites ist es ein Fehler.
func ParseAtlas(b []byte) (Atlas, error) {
	var a Atlas
	if err := json.Unmarshal(b, &a); err != nil {
		return Atlas{}, fmt.Errorf("❌ Ton-Atlas: %w", err)
	}
	if len(a.Files) == 0 || a.Sprites == nil {
		return Atlas{}, errors.New("❌ Ton-Atlas ohne Dateien oder Sprites")
	}
	for name, s := range a.Sprites {
		if !finite(s.Start) || !finite(s.Duration) || s.Start < 0 || s.Duration <= 0 {
			delete(a.Sprites, name)
		}
	}
	return a, nil
}

func finite(v float64) bool { return !math.IsNaN(v) && !math.IsInf(v, 0) }

// Pick liefert die Datei des ersten Formats aus formatOrder, die der Atlas nennt und die es gibt (exists).
func (a Atlas) Pick(exists func(name string) bool) (string, bool) {
	for _, f := range formatOrder {
		if name := a.Files[f]; name != "" && exists(name) {
			return name, true
		}
	}
	return "", false
}

// pcmFrame ist die Größe eines Abtastwerts im dekodierten Ton: 16 Bit, Stereo (vorbis/mp3/wav.DecodeWithSampleRate).
const pcmFrame = 4

// spriteRange liefert den Byte-Bereich [lo, hi) eines Sprites in dekodiertem Ton mit rate Hz und total Bytes, auf
// ganze Abtastwerte gerundet und auf den Ton begrenzt. Ein leerer Bereich (lo == hi) heißt: liegt außerhalb.
func spriteRange(s Sprite, rate, total int) (lo, hi int) {
	frames := total / pcmFrame
	from := min(frames, max(0, int(math.Round(s.Start*float64(rate)))))
	to := min(frames, max(from, int(math.Round((s.Start+s.Duration)*float64(rate)))))
	return from * pcmFrame, to * pcmFrame
}
