// K3C natives Spiel (Projekt NAT, Sprint NG1, B-792): Go + Ebitengine, Simulation im selben Prozess. Die Schleife
// liegt in game, der Vertrag zwischen Zustand und Zeichnen in view.
//
// Aufruf (über `task native:game`): k3c-game
package main

import (
	"errors"
	"fmt"
	"os"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/game"
)

func main() {
	parts, err := game.Assemble(game.Packages())
	if err != nil {
		fmt.Fprintf(os.Stderr, "💥 %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("🚀 K3C natives Spiel, Pakete %v, %d Drawer\n", parts.Names, len(parts.Drawers))
	ebiten.SetWindowSize(1280, 720)
	ebiten.SetWindowResizingMode(ebiten.WindowResizingModeEnabled)
	ebiten.SetWindowTitle("K3C")
	if err := ebiten.RunGame(game.New(parts, game.Offline{})); err != nil && !errors.Is(err, ebiten.Termination) {
		fmt.Fprintf(os.Stderr, "💥 Abbruch: %v\n", err)
		os.Exit(1)
	}
	fmt.Println("🛑 K3C natives Spiel beendet")
}
