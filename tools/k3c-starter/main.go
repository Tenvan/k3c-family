// Command k3c-starter ist der Windows-Starter für den K3C-Server (B-041): ein Wails-Fenster, das bin/k3c-server startet und
// Status und Adresse im Heimnetz zeigt. Die Server-Logik bleibt in cmd/k3c-server; der Starter ruft die EXE als Kindprozess auf.
package main

import (
	"context"
	"embed"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"
)

// instanceID verbindet einen zweiten Start mit dem laufenden Fenster (ein Starter je Rechner).
const instanceID = "k3c-starter-5d2b7a40"

// Die Oberfläche (React, Radix Themes) entsteht mit `task k3c-starter:frontend` nach frontend/dist und steckt in der EXE.
//
//go:embed all:frontend/dist
var ui embed.FS

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, "k3c-starter:", err)
		os.Exit(1)
	}
}

func run() error {
	assets, err := fs.Sub(ui, "frontend/dist")
	if err != nil {
		return err
	}
	wd, _ := os.Getwd()
	exe, _ := os.Executable()
	lay, layErr := findLayout(wd, filepath.Dir(exe))
	app := newApp(lay, layErr)
	return wails.Run(&options.App{
		Title: "K3C Server", Width: 640, Height: 560, MinWidth: 480, MinHeight: 420,
		BackgroundColour:   &options.RGBA{R: 0x0b, G: 0x10, B: 0x26, A: 0xff},
		AssetServer:        &assetserver.Options{Assets: assets},
		OnStartup:          func(context.Context) { app.start() },
		OnBeforeClose:      func(context.Context) bool { app.stop(); return false },
		SingleInstanceLock: &options.SingleInstanceLock{UniqueId: instanceID},
		Bind:               []any{app},
	})
}
