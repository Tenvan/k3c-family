package mcpsrv

import "net/http"

// forceChunked schickt jede Antwort ohne Event-Stream von Anfang an als chunked (Header sofort geflusht).
//
// Hintergrund: Auf manchen Rechnern schreibt ein Netzwerkfilter (vermutlich eine Schutzsoftware) HTTP-Antworten auf
// MCP-Verkehr um. Antworten mit Content-Length (das 202 auf notifications/initialized, Fehlerantworten) macht er zu
// "chunked" ohne Chunk-Rahmen und ohne Abschluss; Claude Code liest den Body zu Ende und läuft in sein 30-s-Timeout.
// Von Haus aus chunked gesendete Antworten (SSE) lässt der Filter unverändert durch.
func forceChunked(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		next.ServeHTTP(&chunkedWriter{ResponseWriter: w}, r)
	})
}

type chunkedWriter struct {
	http.ResponseWriter
	wrote bool
}

func (w *chunkedWriter) WriteHeader(code int) {
	if w.wrote {
		return
	}
	w.wrote = true
	h := w.Header()
	if h.Get("Content-Type") != "text/event-stream" && code >= 200 && code != http.StatusNoContent && code != http.StatusNotModified {
		h.Del("Content-Length")
		h.Set("Transfer-Encoding", "chunked")
		w.ResponseWriter.WriteHeader(code)
		_ = http.NewResponseController(w.ResponseWriter).Flush()
		return
	}
	w.ResponseWriter.WriteHeader(code)
}

func (w *chunkedWriter) Write(p []byte) (int, error) {
	if !w.wrote {
		w.WriteHeader(http.StatusOK)
	}
	return w.ResponseWriter.Write(p)
}

// Unwrap hält http.ResponseController (Flush, Deadlines) für das SDK erreichbar.
func (w *chunkedWriter) Unwrap() http.ResponseWriter { return w.ResponseWriter }
