package mcpsrv

import (
	"context"
	"errors"
	"net"
	"net/http"
	"net/http/pprof"

)

// workbenchResources ist das Tool workbench_resources: der Bericht des Messgeräts.
func (s *Server) workbenchResources(context.Context, struct{}) (string, error) {
	if s.cfg.Resources == nil {
		return "", errors.New("das Ressourcen-Monitoring ist in dieser Instanz nicht eingeschaltet")
	}
	return s.cfg.Resources.Report(), nil
}

// registerGauges meldet die Puffer des Servers an: gehaltene Prüflaufzeilen, Konsolenpuffer, Testläufe, Sessions.
func (s *Server) registerGauges() {
	r := s.cfg.Resources
	if r == nil {
		return
	}
	r.Gauge("Konsole-Quellen", "", func() int64 { n, _, _ := s.console.Stats(); return int64(n) })
	r.Gauge("Konsole-Zeilen", "", func() int64 { _, n, _ := s.console.Stats(); return int64(n) })
	r.Gauge("Konsole-Text", "B", func() int64 { _, _, b := s.console.Stats(); return b })
	r.Gauge("Prüfläufe", "", func() int64 { n, _ := s.checks.stats(); return int64(n) })
	r.Gauge("Prüflauf-Text", "B", func() int64 { _, b := s.checks.stats(); return b })
	r.Gauge("Testläufe", "", func() int64 { return int64(s.sims.count()) })
	r.Gauge("MCP-Sessions", "", func() int64 {
		n := 0
		for range s.mcp.Sessions() {
			n++
		}
		return int64(n)
	})
}

// count ist die Zahl der gemerkten Testläufe.
func (r *simRuns) count() int {
	r.mu.Lock()
	defer r.mu.Unlock()
	return len(r.runs)
}

// mountPprof hängt net/http/pprof unter /debug/pprof/ ein (`go tool pprof http://127.0.0.1:<Port>/debug/pprof/heap`).
// Der Server hört nur an 127.0.0.1; zusätzlich lehnt der Wächter fremde Hosts ab (DNS-Rebinding aus dem Browser).
func mountPprof(mux *http.ServeMux) {
	h := http.NewServeMux()
	h.HandleFunc("/debug/pprof/", pprof.Index)
	h.HandleFunc("/debug/pprof/cmdline", pprof.Cmdline)
	h.HandleFunc("/debug/pprof/profile", pprof.Profile)
	h.HandleFunc("/debug/pprof/symbol", pprof.Symbol)
	h.HandleFunc("/debug/pprof/trace", pprof.Trace)
	mux.Handle("/debug/pprof/", loopbackOnly(h))
}

func loopbackOnly(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		host, _, err := net.SplitHostPort(r.Host)
		if err != nil {
			host = r.Host
		}
		if ip := net.ParseIP(host); host != "localhost" && (ip == nil || !ip.IsLoopback()) {
			http.Error(w, "nur Loopback", http.StatusForbidden)
			return
		}
		next.ServeHTTP(w, r)
	})
}
