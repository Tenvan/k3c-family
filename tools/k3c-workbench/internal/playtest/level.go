package playtest

import (
	"io"
	"math"
	"os"
)

const (
	wavHeader   = 44        // Kopf einer PCM-WAV von ffmpeg
	levelWindow = 16000 * 2 // letzte Sekunde (16 kHz, 16 Bit, mono) in Bytes
	levelFloor  = -60.0     // dBFS, darunter gilt der Pegel als 0
)

// wavLevel ist der Pegel der letzten Sekunde einer laufend geschriebenen voice.wav in Prozent (0 bis 100): der Effektivwert
// in dBFS von −60 bis 0 auf die Skala gelegt. Das ist der „Ausschlag“ des Mikros, unabhängig von der Datenrate (PCM ist
// konstant). Nicht lesbar oder noch leer: 0.
func wavLevel(path string) int {
	f, err := os.Open(path)
	if err != nil {
		return 0
	}
	defer func() { _ = f.Close() }()
	st, err := f.Stat()
	if err != nil || st.Size() <= wavHeader+2 {
		return 0
	}
	n := st.Size() - wavHeader
	if n > levelWindow {
		n = levelWindow
	}
	n -= n % 2
	buf := make([]byte, n)
	if _, err := f.ReadAt(buf, st.Size()-n); err != nil && err != io.EOF {
		return 0
	}
	return levelPercent(buf)
}

// levelPercent rechnet little-endian int16-Proben in 0 bis 100.
func levelPercent(pcm []byte) int {
	if len(pcm) < 2 {
		return 0
	}
	var sum float64
	for i := 0; i+1 < len(pcm); i += 2 {
		v := float64(int16(uint16(pcm[i]) | uint16(pcm[i+1])<<8))
		sum += v * v
	}
	rms := math.Sqrt(sum / float64(len(pcm)/2))
	if rms < 1 {
		return 0
	}
	db := 20 * math.Log10(rms/32768)
	p := (db - levelFloor) / -levelFloor * 100
	return int(math.Max(0, math.Min(100, math.Round(p))))
}
