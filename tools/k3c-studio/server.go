package main

import (
	"context"
	"net"
	"net/http"
	"path/filepath"
	"strings"

	"k3c/engine/studio"
)

// studioPage ist die Startseite der Oberfläche (http.FileServer liefert dort index.html).
const studioPage = "/studio/"

// newHandler bedient das Studio auf einem eigenen Port: die API aus engine/studio, die Oberfläche aus dist/studio und
// die Grafiken und Töne des Spiels zuerst aus public/ (Änderungen sofort sichtbar), dann aus dist/. Nur Loopback.
func newHandler(ctx context.Context, root string) http.Handler {
	mux := http.NewServeMux()
	mux.Handle(studio.Prefix, studio.New(ctx, root).Handler(false))
	mux.Handle("/", siteHandler(root))
	return hostGuard(mux)
}

// hostGuard lässt nur Loopback-Anfragen mit localhost oder einer IP als Host durch (DNS-Rebinding); die API hat
// dieselbe Prüfung, die statischen Dateien brauchen sie ebenso.
func hostGuard(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		host, _, err := net.SplitHostPort(r.Host)
		if err != nil {
			host = strings.Trim(r.Host, "[]")
		}
		ip := net.ParseIP(host)
		if host != "localhost" && ip == nil {
			http.Error(w, "nur localhost oder IP", http.StatusForbidden)
			return
		}
		next.ServeHTTP(w, r)
	})
}

// siteHandler liefert Dateien erst aus public/, dann aus dist/; die Wurzel führt zum Studio.
func siteHandler(root string) http.Handler {
	files := http.FileServer(overlay{http.Dir(filepath.Join(root, "public")), http.Dir(filepath.Join(root, "dist"))})
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/" {
			http.Redirect(w, r, studioPage, http.StatusFound)
			return
		}
		if r.Method != http.MethodGet && r.Method != http.MethodHead {
			http.Error(w, "nur lesen", http.StatusMethodNotAllowed)
			return
		}
		if strings.HasSuffix(r.URL.Path, "/") && r.URL.Path != studioPage { // keine Verzeichnislisten
			http.NotFound(w, r)
			return
		}
		files.ServeHTTP(w, r)
	})
}

// overlay öffnet einen Namen im ersten Dateisystem, das ihn kennt.
type overlay []http.FileSystem

func (o overlay) Open(name string) (http.File, error) {
	var firstErr error
	for _, fs := range o {
		f, err := fs.Open(name)
		if err == nil {
			return f, nil
		}
		if firstErr == nil {
			firstErr = err
		}
	}
	return nil, firstErr
}

// listen öffnet den Port auf Loopback; ist er belegt, nimmt es einen freien.
func listen(port string) (net.Listener, error) {
	if ln, err := net.Listen("tcp", net.JoinHostPort("127.0.0.1", port)); err == nil {
		return ln, nil
	}
	return net.Listen("tcp", "127.0.0.1:0")
}
