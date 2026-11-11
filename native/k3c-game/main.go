// K3C natives Spiel (Projekt NAT, Sprint NG1, B-792): Go + Ebitengine, Simulation im selben Prozess. Die Schleife
// liegt in game, der Vertrag zwischen Zustand und Zeichnen in view.
//
// Aufruf (über `task native:game` bzw. `task native:game:shots`): siehe game.Usage. Ohne --seed und --load startet das
// Hauptmenü; --shots schreibt eine Bildfolge und beendet sich (Spielstände dabei nur in einem Wegwerf-Ordner).
package main

import (
	"errors"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/game"
	"k3c/native/k3c-game/local"
)

func fatal(err error) {
	fmt.Fprintf(os.Stderr, "💥 %v\n", err)
	os.Exit(1)
}

func main() {
	args, err := game.ParseArgs(os.Args[1:], local.DefaultSavesDir())
	if err != nil {
		fatal(err)
	}
	if args.Shots != "" {
		tmp, err := shotSaves(args)
		if err != nil {
			fatal(err)
		}
		defer os.RemoveAll(tmp)
		args.Saves = tmp
	}
	if err := run(args); err != nil {
		fatal(err)
	}
	fmt.Println("🛑 K3C natives Spiel beendet")
}

// run baut Grafik, Pakete, Simulation und Schleife und läuft bis zum Ende.
func run(args game.Args) error {
	lib, err := art.New(art.DefaultConfig())
	if err != nil {
		return err
	}
	parts, err := game.Assemble(game.Packages(lib))
	if err != nil {
		return err
	}
	slog.Info("🚀 K3C natives Spiel", "pakete", parts.Names, "drawer", len(parts.Drawers), "spielstaende", args.Saves, "aufnahme", args.Shots)
	sim := local.Open(local.Config{Dir: args.Saves, Log: slog.Default()})
	g, err := game.New(parts, sim, args)
	if err != nil {
		return err
	}
	ebiten.SetWindowSize(args.W, args.H)
	ebiten.SetWindowResizingMode(ebiten.WindowResizingModeEnabled)
	ebiten.SetWindowTitle("K3C")
	ebiten.SetTPS(game.TPS)
	ebiten.SetRunnableOnUnfocused(true)
	if err := ebiten.RunGame(g); err != nil && !errors.Is(err, ebiten.Termination) {
		return fmt.Errorf("Abbruch: %w", err)
	}
	return nil
}

// shotSaves legt den Wegwerf-Ordner der Spielstände einer Aufnahme an; mit --load liegt der Spielstand als Kopie darin,
// damit Autosave den Stand im Ordner --saves nicht verändert.
func shotSaves(args game.Args) (string, error) {
	tmp, err := os.MkdirTemp("", "k3c-shots-")
	if err != nil {
		return "", err
	}
	if name := args.Scene.Load; name != "" {
		data, err := os.ReadFile(filepath.Join(args.Saves, name+".json"))
		if err == nil {
			err = os.WriteFile(filepath.Join(tmp, name+".json"), data, 0o644)
		}
		if err != nil {
			os.RemoveAll(tmp)
			return "", fmt.Errorf("Spielstand %s: %w", name, err)
		}
	}
	return tmp, nil
}
