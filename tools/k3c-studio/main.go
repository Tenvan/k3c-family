// Command k3c-studio ist das Asset-Studio von K3C (B-459): je eine Seite für Grafik und Sound auf dem Slot-Register
// data/assets.json, getrennt von der Workbench k3c-dev. Kein eigener Port, kein MCP; die MCP-Tools assets_* bleiben
// in k3c-dev.
package main

import (
	"crypto/sha256"
	"embed"
	"encoding/hex"
	"fmt"
	"os"
	"path/filepath"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"

	assets "k3c/tools/assetkit"
)

// instancePrefix plus Hash der Wurzel verbindet einen zweiten Start für denselben Checkout mit dem laufenden Fenster;
// ein Start aus einem anderen Worktree öffnet ein eigenes Fenster für diesen Worktree.
const instancePrefix = "k3c-studio-8f2c4a61-"

//go:embed all:frontend/dist
var dist embed.FS

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, "k3c-studio:", err)
		os.Exit(1)
	}
}

func run() error {
	root, err := findRoot()
	if err != nil {
		return err
	}
	app := newApp(root)
	return wails.Run(&options.App{
		Title: "Asset-Studio", Width: 1280, Height: 800, MinWidth: 900, MinHeight: 600,
		BackgroundColour: &options.RGBA{R: 0x0b, G: 0x10, B: 0x26, A: 0xff},
		AssetServer:      &assetserver.Options{Assets: dist, Handler: publicHandler(root)},
		OnStartup:        app.startup,
		SingleInstanceLock: &options.SingleInstanceLock{UniqueId: instanceID(root),
			OnSecondInstanceLaunch: func(options.SecondInstanceData) { app.secondInstance() }},
		Bind: []any{app},
	})
}

// instanceID ist die Kennung des Single-Instance-Locks je Checkout.
func instanceID(root string) string {
	sum := sha256.Sum256([]byte(filepath.Clean(root)))
	return instancePrefix + hex.EncodeToString(sum[:8])
}

// findRoot sucht die Repo-Wurzel ab dem Arbeitsverzeichnis (Start aus einem Worktree bedient diesen), ersatzweise ab
// dem Ordner der EXE (Doppelklick auf bin/k3c-studio.exe).
func findRoot() (string, error) {
	wd, err := os.Getwd()
	if err == nil {
		if root, err := assets.FindRoot(wd); err == nil {
			return root, nil
		}
	}
	exe, err := os.Executable()
	if err != nil {
		return "", err
	}
	return assets.FindRoot(filepath.Dir(exe))
}
