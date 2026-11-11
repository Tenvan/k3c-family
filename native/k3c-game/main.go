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
	"time"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/game"
	"k3c/native/k3c-game/local"
)

// shotTime ist die Uhr der Spielstände im Aufnahme-Modus (das Bild hängt nicht von der Wanduhr ab).
var shotTime = time.Date(2026, 1, 1, 12, 0, 0, 0, time.UTC)

func fatal(err error) {
	fmt.Fprintf(os.Stderr, "💥 %v\n", err)
	os.Exit(1)
}

func main() {
	args, err := game.ParseArgs(os.Args[1:], local.DefaultSavesDir())
	if err != nil {
		fatal(err)
	}
	if args.Assets {
		if err := printAssets(); err != nil {
			fatal(err)
		}
		return
	}
	if args.Shots == "" && args.Measure.Seconds == 0 {
		err = run(args)
	} else {
		var tmp string
		if tmp, err = shotSaves(args); err != nil {
			fatal(err)
		}
		args.Saves = tmp
		err = run(args)
		removeTemp(tmp) // vor fatal: os.Exit überspringt defer
	}
	if err != nil {
		fatal(err)
	}
	fmt.Println("🛑 K3C natives Spiel beendet")
}

// removeTemp räumt den Wegwerf-Ordner weg; ein Fehler steht nur im Log.
func removeTemp(dir string) {
	if err := os.RemoveAll(dir); err != nil {
		slog.Warn("🧹 Wegwerf-Ordner nicht entfernt", "ordner", dir, "err", err)
	}
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
	parts.Art = lib
	slog.Info("🚀 K3C natives Spiel", "pakete", parts.Names, "drawer", len(parts.Drawers), "spielstaende", args.Saves, "aufnahme", args.Shots)
	cfg := local.Config{Dir: args.Saves, Log: slog.Default()}
	if args.Shots != "" { // Aufnahme: Speichern im Takt, feste Uhr (Zeitstempel in der Spielstand-Liste, Insel-ID)
		cfg.Recording, cfg.Now = true, func() time.Time { return shotTime }
	}
	sim := local.Open(cfg)
	g, err := game.New(parts, sim, args)
	if err != nil {
		return err
	}
	ebiten.SetWindowSize(args.W, args.H)
	ebiten.SetWindowResizingMode(ebiten.WindowResizingModeEnabled)
	ebiten.SetWindowTitle("K3C")
	ebiten.SetTPS(game.TPS)
	ebiten.SetRunnableOnUnfocused(true)
	if args.Measure.NoVsync {
		ebiten.SetVsyncEnabled(false)
	}
	if err := ebiten.RunGame(g); err != nil && !errors.Is(err, ebiten.Termination) {
		return fmt.Errorf("abgebrochen: %w", err)
	}
	return nil
}

// printAssets gibt die Grafik-Ordner aller Level aus (--assets, je Zeile `gruppe/id` unter public/grafik); ohne Fenster
// und ohne Grafik.
func printAssets() error {
	parts, err := game.Assemble(game.Packages(nil))
	if err != nil {
		return err
	}
	dirs, err := game.AssetDirs(parts.Assets)
	if err != nil {
		return err
	}
	for _, d := range dirs {
		fmt.Println(d)
	}
	return nil
}

// shotSaves legt den Wegwerf-Ordner der Spielstände einer Aufnahme bzw. Messung an; mit --load liegt der Spielstand als
// Kopie darin, damit Autosave den Stand im Ordner --saves nicht verändert.
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
			removeTemp(tmp)
			return "", fmt.Errorf("%s: Spielstand nicht kopiert: %w", name, err)
		}
	}
	return tmp, nil
}
