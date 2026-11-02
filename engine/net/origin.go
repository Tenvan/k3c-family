package net

import (
	"net/http"
	"net/url"
	"strings"
)

// Herkunftsprüfung der schreibenden API (B-613): Ein POST ohne Body ist für den Browser eine einfache Anfrage ohne
// Preflight, jede fremde Webseite im Heimnetz könnte sonst einen Raum schließen oder einen Spielstand verschieben.

// writeMethod meldet, ob die Methode etwas ändert; GET, HEAD und OPTIONS lesen nur.
func writeMethod(m string) bool {
	return m != http.MethodGet && m != http.MethodHead && m != http.MethodOptions
}

// sameOrigin meldet, ob der Origin-Header zur eigenen Seite passt: fehlt er (Werkzeuge, curl), ist die Anfrage erlaubt;
// sonst muss sein Host samt Port gleich dem Host der Anfrage sein. Das Schema zählt nicht, Groß- und Kleinschreibung
// auch nicht. Der Vite-Dev-Proxy (5183) reicht Host und Origin unverändert weiter, beide nennen dann 5183.
func sameOrigin(r *http.Request) bool {
	origin := r.Header.Get("Origin")
	if origin == "" {
		return true
	}
	u, err := url.Parse(origin)
	if err != nil || u.Host == "" {
		return false // auch "null" (Sandbox, file://)
	}
	return strings.EqualFold(u.Host, r.Host)
}

// originGuard lehnt schreibende Anfragen an /api/ mit fremdem Origin ab (403); alles andere geht durch.
func (s *server) originGuard(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if strings.HasPrefix(r.URL.Path, "/api/") && writeMethod(r.Method) && !sameOrigin(r) {
			s.log.Warn("🚫 fremde Herkunft abgelehnt", "ns", "http", "method", r.Method, "path", r.URL.Path,
				"origin", clip(r.Header.Get("Origin"), 200), "host", clip(r.Host, 200), "remote", r.RemoteAddr)
			fail(w, http.StatusForbidden, "Fremde Herkunft")
			return
		}
		next.ServeHTTP(w, r)
	})
}
