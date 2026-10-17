package main

// „Asset-Studio öffnen“ (B-459): startet das Asset-Studio als eigene EXE mit festem Pfad, losgelöst von der Workbench.
// Die Assets-Seite steckt seit WZ6 im Asset-Studio (tools/k3c-studio); die MCP-Tools assets_* bleiben hier.

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"
)

// errStudioMissing ist der Hinweis, wenn bin/k3c-studio.exe fehlt (B-459 › Ausnahme- und Fehlerfälle); der Text geht so
// an die Oberfläche.
var errStudioMissing = errors.New("Asset-Studio nicht gebaut (`task k3c-studio:build`)")

// studioPath ist die feste EXE des Asset-Studios unter der Repo-Wurzel (Firewall: immer derselbe Pfad).
func studioPath(root string) string {
	return filepath.Join(root, "bin", "k3c-studio.exe")
}

// startDetached startet path im Ordner dir ohne Argumente. Bewusst ohne proc.Command: dessen Job Object würde das
// Studio beim Beenden der Workbench mitreißen. Test-Naht.
var startDetached = func(path, dir string) error {
	cmd := exec.Command(path)
	cmd.Dir = dir
	if err := cmd.Start(); err != nil {
		return err
	}
	go func() { _ = cmd.Wait() }()
	return nil
}

// openStudio prüft die EXE und startet sie mit der Repo-Wurzel als Arbeitsordner (das Studio sucht dort seine Wurzel).
func openStudio(root string, start func(path, dir string) error) error {
	path := studioPath(root)
	if st, err := os.Stat(path); err != nil || st.IsDir() {
		return errStudioMissing
	}
	return start(path, root)
}

// StudioOpen startet das Asset-Studio (Binding). Keine Pfadteile oder Argumente aus der Oberfläche.
func (a *App) StudioOpen() error {
	a.wait()
	if err := openStudio(a.root, startDetached); err != nil {
		return err
	}
	a.log.Info("🚀 Asset-Studio gestartet", "ns", "main", "exe", studioPath(a.root))
	return nil
}
