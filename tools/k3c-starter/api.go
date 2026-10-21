package main

import (
	"encoding/json"
	"net"
	"net/http"
	"net/url"
)

// starterHeader ist die Pflichtkopfzeile der Starter-API: Ein fremder Browser-Tab kann sie nicht ohne Preflight setzen,
// und es gibt keine CORS-Freigabe, damit ist die API gegen Aufrufe aus anderen Seiten geschützt.
const (
	starterHeader = "X-K3C-Starter"
	starterValue  = "1"
)

// newAPIHandler ist die kleine HTTP-Schnittstelle des Starters im Entwicklungsmodus (-headless): dieselben drei Aufrufe wie
// die Wails-Bindungen von App. Sie antwortet nur auf Loopback, nur mit Host localhost und nur mit der Kopfzeile X-K3C-Starter.
func newAPIHandler(app *App) http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /api/starter/state", func(w http.ResponseWriter, _ *http.Request) {
		writeState(w, app.State())
	})
	mux.HandleFunc("POST /api/starter/start", func(w http.ResponseWriter, _ *http.Request) {
		app.Start()
		writeState(w, app.State())
	})
	mux.HandleFunc("POST /api/starter/stop", func(w http.ResponseWriter, _ *http.Request) {
		app.Stop()
		writeState(w, app.State())
	})
	return guard(mux)
}

// guard lehnt alles ab, was nicht vom eigenen Rechner mit Loopback-Host, gleicher Herkunft und Pflichtkopfzeile kommt.
func guard(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if !isLoopbackAddr(r.RemoteAddr) || !isLoopbackHost(r.Host) || !sameSite(r) || r.Header.Get(starterHeader) != starterValue {
			http.Error(w, "abgelehnt", http.StatusForbidden)
			return
		}
		next.ServeHTTP(w, r)
	})
}

// sameSite: die Anfrage kommt nicht von einer fremden Seite im Browser (wie gameGuard im Studio). Ein Origin muss zum Host
// passen (die Oberfläche läuft über den Vite-Proxy mit demselben Host), Sec-Fetch-Site darf nur same-origin oder none sein.
// Das gilt auch für GET: der Zustand enthält das Server-Log.
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

// isLoopbackAddr prüft die Gegenstelle der Verbindung ("ip:port").
func isLoopbackAddr(addr string) bool {
	host, _, err := net.SplitHostPort(addr)
	if err != nil {
		return false
	}
	ip := net.ParseIP(host)
	return ip != nil && ip.IsLoopback()
}

// isLoopbackHost prüft die Host-Kopfzeile (Schutz gegen DNS-Rebinding): localhost oder eine Loopback-IP, Port beliebig.
func isLoopbackHost(hostport string) bool {
	host, _, err := net.SplitHostPort(hostport)
	if err != nil {
		host = hostport
	}
	if host == "localhost" {
		return true
	}
	ip := net.ParseIP(host)
	return ip != nil && ip.IsLoopback()
}

func writeState(w http.ResponseWriter, st Status) {
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	w.Header().Set("Cache-Control", "no-store")
	_ = json.NewEncoder(w).Encode(st)
}
