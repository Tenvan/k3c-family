// Package browser öffnet eine Adresse im Standard-Browser: der Web-Start von k3c-workbench und Studio (`-open`, B-614).
package browser

import (
	"errors"
	"os/exec"
	"runtime"
	"strings"
)

// Open startet den Standard-Browser mit url und wartet nicht auf ihn. Nur http(s), damit nie eine Datei oder ein
// fremdes Protokoll gestartet wird.
func Open(url string) error {
	if !strings.HasPrefix(url, "http://") && !strings.HasPrefix(url, "https://") {
		return errors.New("nur http(s)-Adressen: " + url)
	}
	var cmd *exec.Cmd
	switch runtime.GOOS {
	case "windows":
		cmd = exec.Command("rundll32", "url.dll,FileProtocolHandler", url)
	case "darwin":
		cmd = exec.Command("open", url)
	default:
		cmd = exec.Command("xdg-open", url)
	}
	if err := cmd.Start(); err != nil {
		return err
	}
	go func() { _ = cmd.Wait() }()
	return nil
}
