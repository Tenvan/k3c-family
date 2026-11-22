package playtest

import (
	"encoding/binary"
	"os"
	"path/filepath"
	"testing"
)

func pcm(amp int16, samples int) []byte {
	b := make([]byte, samples*2)
	for i := 0; i < samples; i++ {
		v := amp
		if i%2 == 1 {
			v = -amp
		}
		binary.LittleEndian.PutUint16(b[i*2:], uint16(v))
	}
	return b
}

func TestLevelPercent(t *testing.T) {
	if got := levelPercent(pcm(0, 100)); got != 0 {
		t.Errorf("Stille: %d", got)
	}
	if got := levelPercent(pcm(32767, 100)); got < 99 {
		t.Errorf("Vollaussteuerung: %d", got)
	}
	quiet, loud := levelPercent(pcm(100, 1000)), levelPercent(pcm(8000, 1000))
	if quiet <= 0 || loud <= quiet || loud >= 100 {
		t.Errorf("leise %d, laut %d", quiet, loud)
	}
}

func TestWavLevelLiestDasEndeDerDatei(t *testing.T) {
	p := filepath.Join(t.TempDir(), "voice.wav")
	data := append(make([]byte, wavHeader), pcm(0, 40000)...) // lange Stille …
	data = append(data, pcm(8000, 16000)...)                  // … dann die letzte Sekunde laut
	if err := os.WriteFile(p, data, 0o644); err != nil {
		t.Fatal(err)
	}
	if got := wavLevel(p); got < 50 {
		t.Errorf("Pegel der letzten Sekunde: %d", got)
	}
	if got := wavLevel(filepath.Join(t.TempDir(), "fehlt.wav")); got != 0 {
		t.Errorf("fehlende Datei: %d", got)
	}
}
