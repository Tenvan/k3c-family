package net

import (
	"encoding/json"
	"log/slog"
	"net/http"
	"strings"
	"sync"
	"time"
	"unicode/utf8"
)

// Zugriffslog für HTTP und der Eingang für die Browser-Meldungen (POST /api/clientlog).

// Grenzen von POST /api/clientlog: je Anfrage 64 KB und 50 Meldungen, insgesamt höchstens clientRate Meldungen je clientWindow.
const (
	clientBody    = 64 << 10
	clientBatch   = 50
	clientRate    = 200
	clientWindow  = 10 * time.Second
	clientMaxText = 2000
	clientMaxLong = 4000
)

// statusWriter merkt sich den Status der Antwort.
type statusWriter struct {
	http.ResponseWriter
	status int
}

func (w *statusWriter) WriteHeader(code int) {
	w.status = code
	w.ResponseWriter.WriteHeader(code)
}

// Unwrap lässt http.ResponseController an den echten Writer (Flush, Hijack).
func (w *statusWriter) Unwrap() http.ResponseWriter { return w.ResponseWriter }

// accessLog schreibt jede HTTP-Anfrage mit Status und Dauer. Die WebSocket-Verbindung loggt ws.go selbst, die Query bleibt
// aus dem Log (sie kann einen Token enthalten). Gesund- und Dateiabrufe sind Debug, API-Aufrufe Info, 4xx Warn, 5xx Error.
func (s *server) accessLog(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/ws" {
			next.ServeHTTP(w, r)
			return
		}
		start := time.Now()
		sw := &statusWriter{ResponseWriter: w, status: http.StatusOK}
		next.ServeHTTP(sw, r)
		level := slog.LevelDebug
		switch {
		case sw.status >= 500:
			level = slog.LevelError
		case sw.status >= 400:
			level = slog.LevelWarn
		case strings.HasPrefix(r.URL.Path, "/api/") && r.URL.Path != "/api/health":
			level = slog.LevelInfo
		}
		s.log.Log(r.Context(), level, "HTTP "+r.Method+" "+r.URL.Path, "ns", "http", "status", sw.status,
			"ms", float64(time.Since(start).Microseconds())/1000, "remote", r.RemoteAddr)
	})
}

// clientEntry ist eine Meldung des Browsers.
type clientEntry struct {
	Level string `json:"level"`
	Msg   string `json:"msg"`
	URL   string `json:"url"`
	Stack string `json:"stack"`
	Ctx   string `json:"ctx"`
	Dev   string `json:"device"`
}

// clientGate begrenzt die Zahl der Meldungen, damit ein Browser in einer Fehlerschleife den Server nicht füllt.
type clientGate struct {
	mu    sync.Mutex
	from  time.Time
	count int
}

func (g *clientGate) allow(n int, now time.Time) bool {
	g.mu.Lock()
	defer g.mu.Unlock()
	if now.Sub(g.from) >= clientWindow {
		g.from, g.count = now, 0
	}
	if g.count+n > clientRate {
		return false
	}
	g.count += n
	return true
}

func clip(s string, max int) string {
	if len(s) <= max {
		return s
	}
	return strings.ToValidUTF8(s[:max], "") + "…"
}

func levelOf(name string) slog.Level {
	switch strings.ToLower(name) {
	case "debug":
		return slog.LevelDebug
	case "warn", "warning":
		return slog.LevelWarn
	case "error":
		return slog.LevelError
	}
	return slog.LevelInfo
}

// clientLog ist POST /api/clientlog: {"entries":[{level,msg,url,stack,ctx,device}]}. Die Meldungen landen im Log des
// Clients (logs/k3c-client.jsonl und Konsole). Antwort 204; bei Überlauf 429.
func (s *server) clientLog(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		fail(w, http.StatusMethodNotAllowed, "Nur POST")
		return
	}
	data, err := readBody(r, clientBody)
	var body struct {
		Entries []clientEntry `json:"entries"`
	}
	if err != nil || json.Unmarshal(data, &body) != nil || len(body.Entries) == 0 || len(body.Entries) > clientBatch {
		fail(w, http.StatusBadRequest, "entries: 1 bis 50 Meldungen, Body höchstens 64 KB")
		return
	}
	if !s.gate.allow(len(body.Entries), time.Now()) {
		fail(w, http.StatusTooManyRequests, "zu viele Meldungen")
		return
	}
	log := s.cfg.ClientLog
	if log == nil {
		log = s.log
	}
	for _, e := range body.Entries {
		if !utf8.ValidString(e.Msg) {
			e.Msg = strings.ToValidUTF8(e.Msg, "?")
		}
		attrs := []any{"ns", "client", "remote", r.RemoteAddr, "device", short(e.Dev), "url", clip(e.URL, 300)}
		if e.Stack != "" {
			attrs = append(attrs, "stack", clip(e.Stack, clientMaxLong))
		}
		if e.Ctx != "" {
			attrs = append(attrs, "ctx", clip(e.Ctx, clientMaxText))
		}
		log.Log(r.Context(), levelOf(e.Level), clip(e.Msg, clientMaxText), attrs...)
	}
	w.WriteHeader(http.StatusNoContent)
}
