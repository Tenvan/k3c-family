package main

// „Asset-Studio öffnen“ (B-459): das Studio ist eine Seite des Go-Servers (engine/studio, /studio/index.html) und öffnet
// sich im Browser. Die MCP-Tools assets_* bleiben hier.

import (
	"errors"
	"fmt"

	"github.com/wailsapp/wails/v2/pkg/runtime"
)

// studioPage ist die Adresse der Studio-Oberfläche auf dem Spielserver.
const studioPage = "/studio/index.html"

// errStudioNoServer ist der Hinweis, wenn kein Spielserver konfiguriert ist; der Text geht so an die Oberfläche.
var errStudioNoServer = errors.New("Spielserver nicht konfiguriert (services.json)")

// studioURL ist die Adresse des Studios auf dem Spielserver mit dem Port aus services.json (Dienst „Spielserver“,
// im Worktree mit Versatz); das Studio läuft nur im Dev-Mode des Servers.
func (a *App) studioURL() (string, error) {
	ctl, err := a.controller()
	if err != nil {
		return "", err
	}
	for _, svc := range ctl.Configs() {
		if svc.Name == "Spielserver" {
			return fmt.Sprintf("http://localhost:%d%s", svc.Port, studioPage), nil
		}
	}
	return "", errStudioNoServer
}

// StudioOpen öffnet das Asset-Studio im Browser (Binding). Keine Pfadteile oder Argumente aus der Oberfläche; der
// Spielserver muss laufen (`svc_start`), sonst zeigt der Browser seinen Verbindungsfehler.
func (a *App) StudioOpen() error {
	url, err := a.studioURL()
	if err != nil {
		return err
	}
	a.openBrowser(url)
	a.log.Info("🚀 Asset-Studio geöffnet", "ns", "main", "url", url)
	return nil
}

// openBrowser öffnet eine Adresse im Standard-Browser (Test-Naht `open`).
func (a *App) openBrowser(url string) {
	if a.open != nil {
		a.open(url)
		return
	}
	runtime.BrowserOpenURL(a.ctx, url)
}
