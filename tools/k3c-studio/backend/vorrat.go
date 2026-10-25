package backend

import (
	"net/http"
	"os"
	"path/filepath"

	assets "k3c/tools/assetkit"
)

// vorratURL ist die Adresse des Vorrats für Hörprobe und Vorschau; absolut, weil die Oberfläche unter /studio/ (Server)
// oder bei Vite liegt und beide /api an das Studio reichen.
const vorratURL = Prefix + "vorrat/"

// vorratHandler liefert die Dateien unter <root>/.work/vorrat nur lesend und nur an Loopback aus (B-601), wie
// publicHandler: os.Root hält jeden Zugriff unterhalb des Vorrats, auch über Links.
func vorratHandler(root string) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet && r.Method != http.MethodHead {
			http.Error(w, "nur lesen", http.StatusMethodNotAllowed)
			return
		}
		name, ok := publicName(r.URL.Path)
		if !ok || !isLoopback(r.RemoteAddr) {
			http.NotFound(w, r)
			return
		}
		dir, err := os.OpenRoot(filepath.Join(root, filepath.FromSlash(assets.VorratDir)))
		if err != nil {
			http.NotFound(w, r)
			return
		}
		defer func() { _ = dir.Close() }()
		serveFile(w, r, dir, name)
	})
}
