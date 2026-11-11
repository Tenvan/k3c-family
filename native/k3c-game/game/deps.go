//go:build tools

package game

// deps.go pinnt die Module für Text, Ton und Rig in go.mod (Sperre deps, B-792): Alles, was die Fach-Sprints NG2–NG8
// von Ebitengine für Text und Ton brauchen, steht damit schon in go.mod und go.sum. Nur für `go mod tidy` (Tag tools),
// nicht im Programm.
import (
	_ "github.com/hajimehoshi/ebiten/v2/audio"
	_ "github.com/hajimehoshi/ebiten/v2/audio/mp3"
	_ "github.com/hajimehoshi/ebiten/v2/audio/vorbis"
	_ "github.com/hajimehoshi/ebiten/v2/audio/wav"
	_ "github.com/hajimehoshi/ebiten/v2/text/v2"

	_ "k3c/native/k3c-rig/paint"
	_ "k3c/native/k3c-rig/rig"
)
