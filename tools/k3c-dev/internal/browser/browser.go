// Package browser startet den installierten Browser (Edge oder Chrome) headless für Testläufe mit Clients (B-348,
// TR1.3). Gesucht wird nur an einer festen Liste bekannter Orte; eine neue Abhängigkeit gibt es nicht.
package browser

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"runtime"

	"k3c/tools/k3c-dev/internal/proc"
)

// candidates sind die bekannten Orte je Plattform; Umgebungsvariablen in ${…} werden über getenv aufgelöst.
func candidates(goos string, getenv func(string) string) []string {
	switch goos {
	case "windows":
		var out []string
		for _, base := range []string{getenv("ProgramFiles(x86)"), getenv("ProgramFiles"), getenv("LOCALAPPDATA")} {
			if base == "" {
				continue
			}
			out = append(out, filepath.Join(base, "Microsoft", "Edge", "Application", "msedge.exe"),
				filepath.Join(base, "Google", "Chrome", "Application", "chrome.exe"))
		}
		return out
	case "darwin":
		return []string{"/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
			"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"}
	}
	return []string{"/usr/bin/microsoft-edge", "/usr/bin/google-chrome", "/usr/bin/chromium", "/usr/bin/chromium-browser"}
}

// ErrNotFound heißt: an keinem bekannten Ort liegt Edge oder Chrome.
var ErrNotFound = errors.New("kein Browser gefunden (Edge oder Chrome an den bekannten Orten)")

// Find liefert den ersten vorhandenen Browser.
func Find() (string, error) {
	return find(runtime.GOOS, os.Getenv, func(p string) bool {
		st, err := os.Stat(p)
		return err == nil && !st.IsDir()
	})
}

func find(goos string, getenv func(string) string, exists func(string) bool) (string, error) {
	for _, p := range candidates(goos, getenv) {
		if exists(p) {
			return p, nil
		}
	}
	return "", ErrNotFound
}

// Args sind die Startparameter: headless, eigenes Profil (keine Sitzung des Nutzers), Ton aus, ohne Erststart-Dialog.
func Args(exe, profile, url string) []string {
	return []string{exe, "--headless=new", "--user-data-dir=" + profile, "--no-first-run", "--no-default-browser-check",
		"--mute-audio", "--autoplay-policy=no-user-gesture-required", url}
}

// Start öffnet url in einem eigenen Profil unter profile; Kill und danach genau ein Wait auf dem Ergebnis beenden den
// ganzen Prozessbaum (der Aufrufer wartet, nicht Start).
func Start(ctx context.Context, exe, profile, url string) (*proc.Cmd, error) {
	if err := os.MkdirAll(profile, 0o755); err != nil {
		return nil, err
	}
	cmd := proc.Command(ctx, Args(exe, profile, url))
	if err := cmd.Start(); err != nil {
		return nil, err
	}
	return cmd, nil
}
