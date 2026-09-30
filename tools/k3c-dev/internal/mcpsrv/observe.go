package mcpsrv

import (
	"context"
	"fmt"
	"strings"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-dev/internal/usage"
)

// panicText steht im Ergebnis, wenn ein Handler in Panik gerät; der Server läuft weiter.
const panicText = "Tool-Aufruf durch Panik abgebrochen"

// observe ist die eine Middleware für alle Tool-Aufrufe: Zähler und Aufruf-Log, Panik als Fehler und der
// Parameter-Hinweis bei unbekannten Feldern. Andere Methoden laufen unverändert durch.
func (s *Server) observe(next mcp.MethodHandler) mcp.MethodHandler {
	return func(ctx context.Context, method string, req mcp.Request) (res mcp.Result, err error) {
		call, ok := req.(*mcp.CallToolRequest)
		if !ok || method != "tools/call" {
			return next(ctx, method, req)
		}
		id := s.stats.begin(call.Params.Name, string(call.Params.Arguments))
		defer func() {
			if p := recover(); p != nil {
				res, err = textResult(fmt.Sprintf("%s: %v", panicText, p), true), nil
			}
			s.finish(call, id, outcomeOf(res, err))
		}()
		res, err = next(ctx, method, req)
		s.addParamHint(call.Params.Name, res)
		return res, err
	}
}

// finish schließt einen Aufruf ab: Zähler und Aufruf-Log, eigenes Log bei Fehlern, Nutzungsstatistik mit den rohen
// Argumenten (das Aufruf-Log kürzt sie, gekürztes JSON ließe sich nicht mehr normieren).
func (s *Server) finish(call *mcp.CallToolRequest, id int64, o outcome) {
	c, found := s.stats.end(id, o)
	if !o.ok {
		// Tool und Fehler in der Meldung, damit logs_errors gleichartige Fehler je Tool gruppiert.
		s.log.Warn(call.Params.Name+": "+clip(o.err), "ns", "mcp", "tool", call.Params.Name)
	}
	if found && s.cfg.Usage != nil {
		s.cfg.Usage.Record(usage.Event{At: time.Now(), Tool: c.Tool, Args: string(call.Params.Arguments),
			DurationMs: c.DurationMs, OK: c.OK, Error: o.err})
	}
}

// outcomeOf liest Erfolg, Fehlertext und erste Antwortzeile aus einem Ergebnis.
func outcomeOf(res mcp.Result, err error) outcome {
	if err != nil {
		return outcome{err: err.Error()}
	}
	r, ok := res.(*mcp.CallToolResult)
	if !ok || r == nil {
		return outcome{ok: true}
	}
	text := resultText(r)
	first, _, _ := strings.Cut(text, "\n")
	if r.IsError {
		return outcome{err: text, summary: first}
	}
	return outcome{ok: true, summary: first}
}

// resultText hängt die Text-Inhalte eines Ergebnisses aneinander.
func resultText(r *mcp.CallToolResult) string {
	var parts []string
	for _, c := range r.Content {
		if t, ok := c.(*mcp.TextContent); ok {
			parts = append(parts, t.Text)
		}
	}
	return strings.Join(parts, "\n")
}

func textResult(text string, isError bool) *mcp.CallToolResult {
	return &mcp.CallToolResult{Content: []mcp.Content{&mcp.TextContent{Text: text}}, IsError: isError}
}

// clip kürzt auf textRunes Zeichen.
func clip(s string) string {
	r := []rune(s)
	if len(r) <= textRunes {
		return s
	}
	return string(r[:textRunes-1]) + "…"
}

// ref ist ein kurzer, im Ring eindeutiger Name je Aufruf: Multiplikation mit einer ungeraden Zahl ist modulo 2^28
// umkehrbar, zwei IDs unter 2^28 bekommen also nie denselben Namen.
func ref(id int64) string {
	return fmt.Sprintf("%07x", (uint64(id)*2654435761)&0xfffffff)
}
