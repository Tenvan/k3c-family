package audio

import (
	"io"

	"github.com/ebitengine/oto/v3"
)

// sink ist das Ton-Gerät unter der Wiedergabe: oto im Spiel, ein Fake in Tests (Treiberfehler nachstellen, B-805).
// Kein ebiten/audio: dessen Kontext gibt Treiberfehler über einen Hook vor jedem Update zurück und beendet RunGame.
type sink interface {
	NewPlayer(r io.Reader) player
	Err() error // Fehler des Treibers; danach bleibt der Ton aus bis zum Neustart
}

// player ist eine Stimme am Gerät (Methoden von *oto.Player). oto liest r in seinem eigenen Thread (mux); alle
// Methoden sind dort per Mutex geschützt und dürfen vom Spiel-Thread aus gerufen werden. Ein Player muss erreichbar
// bleiben, solange er klingt (oto schließt unerreichbare), darum halten ihn die Listen des device.
type player interface {
	Play()
	IsPlaying() bool
	SetVolume(v float64)
	PauseAndStopReading() // wartet auf einen laufenden Lesevorgang; danach liest oto die Quelle nicht mehr
	Err() error           // Lesefehler der Quelle; der Player ist dann geschlossen
}

// otoSink reicht oto durch; nur das Ergebnis von NewPlayer wird zur Schnittstelle.
type otoSink struct{ c *oto.Context }

func (s otoSink) NewPlayer(r io.Reader) player { return s.c.NewPlayer(r) }

func (s otoSink) Err() error { return s.c.Err() }

// openOto legt den oto-Kontext an: 16 Bit Stereo mit sampleRate wie die Bank, Puffer nach Vorgabe des Treibers
// (wie Ebitengine). oto erlaubt einen Kontext je Prozess. Unter Windows startet der Treiber im Hintergrund, seine
// Fehler meldet erst Err; Player dürfen vorher entstehen (die Mischung läuft unabhängig vom Treiber).
func openOto() (sink, error) {
	c, _, err := oto.NewContext(&oto.NewContextOptions{
		SampleRate:   sampleRate,
		ChannelCount: 2,
		Format:       oto.FormatSignedInt16LE,
	})
	if err != nil {
		return nil, err
	}
	return otoSink{c}, nil
}
