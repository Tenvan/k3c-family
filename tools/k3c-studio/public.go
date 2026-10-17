package main

import (
	"errors"
	"io/fs"
	"net/http"
	"os"
	"path"
	"path/filepath"
	"strings"
)

// publicHandler liefert die Dateien unter <root>/public nur lesend aus (Bilder und Sounds der Seiten); Wails ruft ihn
// für alles, was nicht im eingebetteten Frontend liegt. os.Root hält jeden Zugriff unterhalb von public/, auch über
// Links. Ordner und andere Methoden als GET/HEAD gibt es nicht.
func publicHandler(root string) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet && r.Method != http.MethodHead {
			http.Error(w, "nur lesen", http.StatusMethodNotAllowed)
			return
		}
		name, ok := publicName(r.URL.Path)
		if !ok {
			http.NotFound(w, r)
			return
		}
		pub, err := os.OpenRoot(filepath.Join(root, "public"))
		if err != nil {
			http.NotFound(w, r)
			return
		}
		defer func() { _ = pub.Close() }()
		serveFile(w, r, pub, name)
	})
}

// publicName macht aus dem URL-Pfad einen Namen relativ zu public/; ok ist false bei Backslash, ".." oder leerem Pfad.
func publicName(p string) (string, bool) {
	if strings.Contains(p, `\`) {
		return "", false
	}
	for _, part := range strings.Split(p, "/") {
		if part == ".." {
			return "", false
		}
	}
	name := strings.TrimPrefix(path.Clean("/"+p), "/")
	return name, name != "" && name != "."
}

func serveFile(w http.ResponseWriter, r *http.Request, pub *os.Root, name string) {
	f, err := pub.Open(filepath.FromSlash(name))
	if err != nil {
		status := http.StatusNotFound
		if !errors.Is(err, fs.ErrNotExist) {
			status = http.StatusForbidden
		}
		http.Error(w, http.StatusText(status), status)
		return
	}
	defer func() { _ = f.Close() }()
	st, err := f.Stat()
	if err != nil || st.IsDir() {
		http.NotFound(w, r)
		return
	}
	http.ServeContent(w, r, name, st.ModTime(), f)
}
