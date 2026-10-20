package main

import (
	"context"
	"log/slog"
	"net"
	"net/http"
	"net/url"
	"path/filepath"
	"strings"

	"k3c/tools/k3c-studio/backend"
)

// studioPage ist die Startseite der Oberfläche (http.FileServer liefert dort index.html).
const studioPage = "/studio/"

// newHandler bedient das Studio auf einem eigenen Port: die API aus backend/, die Spiel-API seiner eingebundenen Engine
// (/api/…, /ws; Vorschau-Spiele ohne Spielserver, B-507/AC-02), die Oberfläche aus dist/studio und die Grafiken, Töne
// und Seiten des Spiels zuerst aus public/ (Änderungen sofort sichtbar), dann aus dist/. Nur Loopback. Endet ctx,
// speichert die Engine ihre Räume (backend.App.EngineWait).
func newHandler(ctx context.Context, root string, log *slog.Logger) (http.Handler, *backend.App) {
	studio := backend.New(ctx, root, log)
	mux := http.NewServeMux()
	mux.Handle(backend.Prefix, studio.Handler(false))
	game := gameGuard(studio.Game())
	mux.Handle("/api/", game)
	mux.Handle("/ws", game)
	mux.Handle("/", siteHandler(root))
	return hostGuard(mux), studio
}

// gameGuard schützt die Spiel-API der Engine: nur Loopback (RemoteAddr), und schreibende Anfragen nur von derselben
// Seite (Origin gleich Host, Sec-Fetch-Site same-origin oder none). GET und der WebSocket-Aufbau bleiben frei; den
// prüft engine/net selbst auf gleiche Herkunft. Eine Pflicht-Kopfzeile wie bei /api/studio geht nicht: game.html
// sendet sie nicht.
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
