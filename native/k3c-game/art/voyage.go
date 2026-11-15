package art

// Werte der Schiffsfahrt beim Inselwechsel (B-977): data/native.json › voyage. Reine Darstellung (Fahrzeit des
// Übergangs, Bogen, Schaukeln, Welle), keine Regel. Fehlt die Datei, der Block oder ein Wert, oder ist er unzulässig,
// gilt sein Standardwert; das Spiel startet immer.

import (
	"encoding/json"
	"log/slog"
	"sync"

	"k3c/data"
)

// VoyageLook sind die Werte der Schiffsfahrt. Längen in Asset-Pixeln des Dioramas.
type VoyageLook struct {
	Seconds float64 `json:"seconds"` // Fahrzeit (B-977/R3)
	Arc     float64 `json:"arcPx"`   // Höhe des Bogens über der geraden Strecke
	Bob     float64 `json:"bobPx"`   // Hub des Schiffs auf den Wellen
	BobHz   float64 `json:"bobHz"`   // Schaukeln je Sekunde
	Wave    float64 `json:"wavePx"`  // Ausschlag der vorderen Welle
	WaveHz  float64 `json:"waveHz"`  // Schwingungen der vorderen Welle je Sekunde
	FlagAt  float64 `json:"flagAt"`  // Anteil der Strecke (0 … 1), ab dem die verlassene Insel ihre Fahne trägt
	Scale   float64 `json:"scale"`   // Maßstab des Dioramas im Vollbild gegenüber dem HUD
}

// DefaultVoyage sind die Standardwerte (wie data/native.json; TestVoyageWieDaten hält beide gleich).
func DefaultVoyage() VoyageLook {
	return VoyageLook{Seconds: 3, Arc: 28, Bob: 2.5, BobHz: 0.7, Wave: 14, WaveHz: 0.22, FlagAt: 0.05, Scale: 1.5}
}

// ParseVoyage liest den Block voyage aus dem Inhalt von native.json. Fehlende Werte behalten den Standard; Fahrzeit und
// Maßstab müssen über 0 liegen, die übrigen dürfen 0 sein (kein Bogen, kein Schaukeln), nichts ist negativ, flagAt
// höchstens 1. Ein unzulässiger Wert fällt auf seinen Standard zurück; bad nennt die Felder.
func ParseVoyage(raw []byte) (v VoyageLook, bad []string, err error) {
	def := DefaultVoyage()
	file := struct {
		Voyage *VoyageLook `json:"voyage"`
	}{Voyage: &v}
	v = def
	if err := json.Unmarshal(raw, &file); err != nil {
		return def, nil, err
	}
	fix := func(name string, got *float64, std float64, ok bool) {
		if !ok {
			*got, bad = std, append(bad, name)
		}
	}
	fix("seconds", &v.Seconds, def.Seconds, v.Seconds > 0)
	fix("scale", &v.Scale, def.Scale, v.Scale > 0)
	fix("arcPx", &v.Arc, def.Arc, v.Arc >= 0)
	fix("bobPx", &v.Bob, def.Bob, v.Bob >= 0)
	fix("bobHz", &v.BobHz, def.BobHz, v.BobHz >= 0)
	fix("wavePx", &v.Wave, def.Wave, v.Wave >= 0)
	fix("waveHz", &v.WaveHz, def.WaveHz, v.WaveHz >= 0)
	fix("flagAt", &v.FlagAt, def.FlagAt, v.FlagAt >= 0 && v.FlagAt <= 1)
	return v, bad, nil
}

// Voyage liefert die Werte der Schiffsfahrt aus data/native.json (einmal gelesen, danach ohne Allokation).
var Voyage = sync.OnceValue(func() VoyageLook {
	raw, err := data.Files.ReadFile("native.json")
	if err != nil {
		slog.Warn("💥 data/native.json nicht gelesen, Standardwerte der Schiffsfahrt", "err", err)
		return DefaultVoyage()
	}
	v, bad, err := ParseVoyage(raw)
	if err != nil {
		slog.Warn("💥 data/native.json unlesbar, Standardwerte der Schiffsfahrt", "err", err)
	} else if len(bad) > 0 {
		slog.Warn("💥 data/native.json › voyage: unzulässige Werte, Standard gilt", "felder", bad)
	}
	return v
})
