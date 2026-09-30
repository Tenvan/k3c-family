package net

import (
	"net/http"
	"os"
	"path"
	"path/filepath"
	"strings"
)

// mimeTypes wie server/server.mjs; alles andere ist application/octet-stream.
var mimeTypes = map[string]string{
	".html":  "text/html; charset=utf-8",
	".js":    "text/javascript; charset=utf-8",
	".css":   "text/css; charset=utf-8",
	".json":  "application/json; charset=utf-8",
	".png":   "image/png",
	".jpg":   "image/jpeg",
	".svg":   "image/svg+xml",
	".ogg":   "audio/ogg",
	".mp3":   "audio/mpeg",
	".wav":   "audio/wav",
	".woff2": "font/woff2",
}

// resolve macht aus dem URL-Pfad eine Datei unter dist; ok ist false, wenn der Pfad dist verlässt (auch über \ unter
// Windows oder ein NUL-Byte).
func resolve(dist, urlPath string) (string, bool) {
	if strings.ContainsRune(urlPath, 0) {
		return "", false
	}
	file := filepath.Join(dist, filepath.FromSlash(path.Clean("/"+urlPath)))
	rel, err := filepath.Rel(dist, file)
	if err != nil || rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) || filepath.IsAbs(rel) {
		return "", false
	}
	return file, true
}

// static liefert den Build aus: HTML nie gecacht (neue Builds sofort sichtbar), gehashte Assets dauerhaft.
func (s *server) static(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet && r.Method != http.MethodHead {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	file, ok := resolve(s.cfg.Dist, r.URL.Path)
	if !ok {
		w.WriteHeader(http.StatusForbidden)
		return
	}
	if info, err := os.Stat(file); err == nil && info.IsDir() {
		file = filepath.Join(file, "index.html")
	}
	data, err := os.ReadFile(file)
	if err != nil {
		http.Error(w, "Nicht gefunden", http.StatusNotFound)
		return
	}
	ext := strings.ToLower(filepath.Ext(file))
	mime, known := mimeTypes[ext]
	if !known {
		mime = "application/octet-stream"
	}
	w.Header().Set("Content-Type", mime)
	if ext == ".html" {
		w.Header().Set("Cache-Control", "no-cache")
	} else {
		w.Header().Set("Cache-Control", "public, max-age=31536000, immutable")
	}
	if r.Method == http.MethodHead {
		return
	}
	_, _ = w.Write(data)
}
