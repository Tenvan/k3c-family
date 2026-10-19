package main

// „Asset-Studio öffnen“ (B-459): startet die eigenständige EXE bin/k3c-studio (tools/k3c-studio) wie ein Doppelklick;
// sie bedient das Studio selbst und öffnet ein eigenes Fenster, ein zweiter Start holt nur ein Fenster auf das laufende
// Studio. Die MCP-Tools assets_* bleiben hier.

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
)

// errStudioNotBuilt ist der Hinweis, wenn die EXE fehlt; der Text geht so an die Oberfläche.
var errStudioNotBuilt = errors.New("bin/k3c-studio fehlt – erst bauen: task k3c-studio:build")

// studioExe ist der Pfad der Studio-EXE im Checkout.
func (a *App) studioExe() string {
	name := "k3c-studio"
	if runtime.GOOS == "windows" {
		name += ".exe"
	}
	return filepath.Join(a.root, "bin", name)
}

// StudioOpen startet das Asset-Studio als eigenes Fenster (Binding). Keine Pfadteile oder Argumente aus der Oberfläche.
func (a *App) StudioOpen() error {
	a.wait()
	exe := a.studioExe()
	if _, err := os.Stat(exe); err != nil {
		return errStudioNotBuilt
	}
	if err := a.startProgram(exe, a.root); err != nil {
		return err
	}
	a.log.Info("🚀 Asset-Studio gestartet", "ns", "main", "exe", exe)
	return nil
}

// startProgram startet exe im Ordner dir und wartet nicht auf das Ende (Test-Naht `launch`).
func (a *App) startProgram(exe, dir string) error {
	if a.launch != nil {
		return a.launch(exe, dir)
	}
	cmd := exec.Command(exe)
	cmd.Dir = dir
	if err := cmd.Start(); err != nil {
		return err
	}
	return cmd.Process.Release()
}
