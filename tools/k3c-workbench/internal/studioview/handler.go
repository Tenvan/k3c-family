package studioview

import (
	"net"
	"net/http"
	"net/url"
	"path/filepath"
	"strings"
)

// Listener von „Im Spiel ansehen“ (aus tools/k3c-studio/server.go): Spiel-API der Engine (/api/…, /ws), Seiten, Grafiken
// und Töne erst aus public/ (Änderungen sofort sichtbar), dann aus dist/. Nur Loopback, nur localhost oder IP als Host.
func handler(root string, api http.Handler) http.Handler {
	mux := http.NewServeMux()
	game := gameGuard(api)
	mux.Handle("/api/", game)
	mux.Handle("/ws", game)
	mux.Handle("/", siteHandler(root))
	return hostGuard(mux)
}

// gameGuard: nur Loopback (RemoteAddr), schreibende Anfragen nur von derselben Seite. GET und der WebSocket-Aufbau
// bleiben frei; den prüft engine/net selbst auf gleiche Herkunft.
func gameGuard(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if !loopbackAddr(r.RemoteAddr) {
			http.Error(w, "nur Loopback", http.StatusForbidden)
			return
		}
		if r.Method != http.MethodGet && r.Method != http.MethodHead && !sameSite(r) {
			http.Error(w, "fremde Herkunft", http.StatusForbidden)
			return
		}
		next.ServeHTTP(w, r)
	})
}

// sameSite: die Anfrage kommt nicht von einer fremden Seite im Browser.
func sameSite(r *http.Request) bool {
	if o := r.Header.Get("Origin"); o != "" {
		u, err := url.Parse(o)
		if err != nil || u.Host != r.Host {
			return false
		}
	}
	site := r.Header.Get("Sec-Fetch-Site")
	return site == "" || site == "same-origin" || site == "none"
}

// loopbackAddr: die Gegenstelle ist 127.0.0.0/8 oder ::1.
func loopbackAddr(remote string) bool {
	host, _, err := net.SplitHostPort(remote)
	if err != nil {
		host = remote
	}
	ip := net.ParseIP(host)
	return ip != nil && ip.IsLoopback()
}

// hostGuard lässt nur Loopback-Gegenstellen mit localhost oder einer Loopback-IP als Host durch (DNS-Rebinding).
func hostGuard(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		host, _, err := net.SplitHostPort(r.Host)
		if err != nil {
			host = strings.Trim(r.Host, "[]")
		}
		ip := net.ParseIP(host)
		if !loopbackAddr(r.RemoteAddr) || (host != "localhost" && (ip == nil || !ip.IsLoopback())) {
			http.Error(w, "nur localhost oder Loopback-IP", http.StatusForbidden)
			return
		}
		next.ServeHTTP(w, r)
	})
}

// siteHandler liefert Dateien erst aus public/, dann aus dist/; nur lesen, keine Verzeichnislisten.
func siteHandler(root string) http.Handler {
	fsys := overlay{http.Dir(filepath.Join(root, "public")), http.Dir(filepath.Join(root, "dist"))}
	files := http.FileServer(fsys)
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet && r.Method != http.MethodHead {
			http.Error(w, "nur lesen", http.StatusMethodNotAllowed)
			return
		}
		if strings.HasSuffix(r.URL.Path, "/") {
			http.NotFound(w, r)
			return
		}
		if r.URL.Path == "/index.html" { // http.FileServer würde auf "/" umleiten
			serveFile(w, r, fsys, "/index.html")
			return
		}
		files.ServeHTTP(w, r)
	})
}

// serveFile liefert name aus fsys ohne die Umleitung von index.html auf das Verzeichnis.
func serveFile(w http.ResponseWriter, r *http.Request, fsys http.FileSystem, name string) {
	f, err := fsys.Open(name)
	if err != nil {
		http.NotFound(w, r)
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
