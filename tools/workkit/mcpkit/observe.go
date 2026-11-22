package mcpkit

import (
	"context"
	"fmt"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/workkit/usage"
)

// PanicText steht im Ergebnis, wenn ein Handler in Panik gerät; der Server läuft weiter.
const PanicText = "Tool-Aufruf durch Panik abgebrochen"

// observe ist die eine Middleware für alle Tool-Aufrufe: Zähler und Aufruf-Log, Panik als Fehler und der
// Parameter-Hinweis bei unbekannten Feldern. Andere Methoden laufen unverändert durch.
func (s *Server) observe(next mcp.MethodHandler) mcp.MethodHandler {
	return func(ctx context.Context, method string, req mcp.Request) (res mcp.Result, err error) {
		call, ok := req.(*mcp.CallToolRequest)
		if !ok || method != "tools/call" {
			return next(ctx, method, req)
		}
		// Jeder Aufruf arbeitet im Checkout seines Clients (Repo-Wurzel oder Worktree), nie in einem fremden;
		// das Argument checkout gilt vor dem Header (B-388).
		ws, wsErr := s.resolveWorkspace(call)
		header := headerRoot(call) != ""
		arg := ""
		if s.takesCheckout(call.Params.Name) {
			var err error
			if arg, call.Params.Arguments, err = splitCheckout(call.Params.Arguments); err != nil {
				wsErr = err
			} else if arg != "" {
				ws, wsErr = s.resolveCheckout(arg)
			}
		}
		ctx = WithWorkspace(ctx, ws)
		defer s.track("tool:" + call.Params.Name)()
		id := s.stats.Begin(call.Params.Name, string(call.Params.Arguments))
		s.log.Debug("📨 "+call.Params.Name+": start", "ns", "mcp", "tool", call.Params.Name, "args", ClipArgs(call.Params.Arguments))
		defer func() {
			if p := recover(); p != nil {
				res, err = TextResult(fmt.Sprintf("%s: %v", PanicText, p), true), nil
			}
			s.finish(call, id, OutcomeOf(res, err), ws, header, arg != "")
		}()
		if wsErr != nil {
			res = TextResult(wsErr.Error(), true)
			return res, nil
		}
		res, err = next(ctx, method, req)
		s.addParamHint(call.Params.Name, res)
		s.addCheckout(call.Params.Name, res, ws)
		return res, err
	}
}

// track misst einen Aufruf über Options.Track (Workbench: resmon); ohne Messung nichts.
func (s *Server) track(name string) func() {
	if s.opts.Track == nil {
		return func() {}
	}
	return s.opts.Track(name)
}

// takesCheckout: Tool mit dem Argument checkout im Schema (Options.TakesCheckout).
func (s *Server) takesCheckout(tool string) bool {
	return s.opts.TakesCheckout != nil && s.opts.TakesCheckout(tool)
}

// finish schließt einen Aufruf ab: Zähler und Aufruf-Log, eigenes Log bei Fehlern, Nutzungsstatistik mit den rohen
// Argumenten (das Aufruf-Log kürzt sie, gekürztes JSON ließe sich nicht mehr normieren). header und checkout im Log
// zeigen je Aufruf, ob X-K3C-Root ankam und welcher Checkout galt (B-275), checkout_arg, ob das Argument checkout
// ihn bestimmt hat (B-388).
func (s *Server) finish(call *mcp.CallToolRequest, id int64, o Outcome, ws Workspace, header, arg bool) {
	c, found := s.stats.End(id, o)
	attrs := []any{"ns", "mcp", "tool", call.Params.Name, "ms", c.DurationMs, "ok", o.OK, "header", header,
		"checkout_arg", arg, "checkout", ws.Label()}
	if !o.OK {
		// Tool und Fehler in der Meldung, damit logs_errors gleichartige Fehler je Tool gruppiert.
		s.log.Warn("❌ "+call.Params.Name+": "+Clip(o.Err), attrs...)
	} else {
		s.log.Info("✅ "+call.Params.Name+": ok", attrs...)
	}
	// Nur Tools aus dem Katalog: erfundene Namen eines Clients ließen Statistik und Datei sonst ohne Grenze wachsen.
	if _, known := s.params[c.Tool]; found && known && s.opts.Usage != nil {
		s.opts.Usage.Record(usage.Event{At: time.Now(), Tool: c.Tool, Args: string(call.Params.Arguments),
			DurationMs: c.DurationMs, OK: c.OK, Error: o.Err})
	}
}
