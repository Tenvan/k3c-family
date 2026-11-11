// Package audio spielt Klang-Effekte, Menü-Klänge und Musik zu Ereignissen, Cues und Lage des Frames wie der
// Browser-Client (src/audio/, B-801). Zwei Schichten: planner (mix.go) entscheidet als reine, getestete Logik, welcher
// Klang wann und wie laut klingt und welche Musik läuft; device (output.go) setzt das über Ebitengine um. Ohne
// Ton-Gerät, ohne Ton-Dateien, in Tests und im Aufnahme-Modus bleibt das Spiel stumm, nie ein Absturz.
package audio

import (
	"log/slog"
	"testing"

	"k3c/data"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// Sound ist der Ton-Listener: je Frame genau einmal Listen (view.Listener).
type Sound struct {
	plan *planner
	out  output
}

// New liefert den Ton-Listener mit den Ton-Dateien unter public/ (Pfad relativ zur Repo-Wurzel). lib braucht der Ton
// nicht. In Tests bleibt er stumm; im Aufnahme-Modus (Frame.Recording) rechnet er den Mix, gibt ihn aber nicht aus, das
// Ton-Gerät entsteht dann nie.
func New(_ *art.Library) view.Listener {
	reg, err := LoadRegister(data.Files)
	if err != nil {
		slog.Warn("❌ Ton-Register nicht ladbar, Spiel läuft stumm", "err", err)
	}
	var out output = silent{}
	if err == nil && !testing.Testing() {
		out = newDevice(DefaultPublicDir, &reg)
	}
	return newSound(&reg, out)
}

func newSound(reg *Register, out output) *Sound {
	return &Sound{plan: newPlanner(reg), out: out}
}

// Listen rechnet den Mix des Frames und setzt ihn um (nicht im Aufnahme-Modus).
func (s *Sound) Listen(f *view.Frame) {
	m := s.plan.plan(f)
	if !f.Recording {
		s.out.apply(m)
	}
}
