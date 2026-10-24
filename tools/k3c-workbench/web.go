package main

import (
	"io/fs"
	"net/http"

	"k3c/tools/assetkit/browser"
)

// Web-Client (B-614): -headless liefert neben der Oberflächen-API die eingebettete Oberfläche (frontend/dist) selbst aus,
// `task k3c-workbench:web` braucht so weder Fenster noch Vite. Im -dev ist dist nur ein Platzhalter, dort liefert Vite.

// webPages liefert die Dateien aus frontend/dist; index.html ohne Cache, damit ein Neubau sofort gilt.
func webPages() http.Handler {
	dist, err := fs.Sub(assets, "frontend/dist")
	if err != nil {
		panic(err) // fester Pfad im go:embed von main.go
	}
	files := http.FileServerFS(dist)
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/" || r.URL.Path == "/index.html" {
			w.Header().Set("Cache-Control", "no-cache")
		}
		files.ServeHTTP(w, r)
	})
}

// openPage öffnet die Oberfläche im Standard-Browser; ein Fehler ist nur eine Warnung, der Server läuft weiter.
func openPage(app *App, url string) {
	if err := browser.Open(url); err != nil {
		app.log.Warn("🚫 Browser nicht geöffnet", "ns", "main", "url", url, "error", err.Error())
	}
}
