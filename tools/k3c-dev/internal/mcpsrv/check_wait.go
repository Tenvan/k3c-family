package mcpsrv

import (
	"context"
	"fmt"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// B-520: check_run wartet bis zum Ende des Laufs. Trägt der Aufruf ein progressToken, meldet es dabei alle
// progressEvery Fortschritt (zählt beim Client als Aktivität gegen dessen Leerlauf-Frist) und wartet bis zum Ende;
// ohne Token wartet es höchstens checkWait, knapp unter der Leerlauf-Frist von HTTP-MCP-Clients (5 min), und nennt
// danach die Lauf-ID. check_run {run} wartet genauso. Die Frist je Aufruf (60 s bei HTTP) hebt `timeout` in .mcp.json.
const (
	defaultCheckWait     = 4 * time.Minute
	defaultProgressEvery = 20 * time.Second
)

type progressKey struct{}

// progressFunc meldet dem Client, dass der Aufruf noch arbeitet; elapsed steigt monoton.
type progressFunc func(msg string, elapsed time.Duration)

// withProgress hängt einen Melder an ctx, wenn der Aufruf ein progressToken trägt.
func withProgress(ctx context.Context, req *mcp.CallToolRequest) context.Context {
	if req == nil || req.Session == nil || req.Params == nil {
		return ctx
	}
	token := req.Params.GetProgressToken()
	if token == nil {
		return ctx
	}
	session := req.Session
	return context.WithValue(ctx, progressKey{}, progressFunc(func(msg string, elapsed time.Duration) {
		// Fortschritt ist nur ein Lebenszeichen: Ein Fehler beim Senden ändert nichts am Lauf.
		_ = session.NotifyProgress(ctx, &mcp.ProgressNotificationParams{
			ProgressToken: token, Message: msg, Progress: elapsed.Seconds()})
	}))
}

func progressOf(ctx context.Context) progressFunc {
	p, _ := ctx.Value(progressKey{}).(progressFunc)
	return p
}

// awaitJob wartet auf das Ende des Laufs: mit Fortschritts-Meldungen bis zum Ende, ohne höchstens checkWait.
func (s *Server) awaitJob(ctx context.Context, j *checkJob) (string, error) {
	progress := progressOf(ctx)
	wait := s.checkWait
	var tick <-chan time.Time
	if progress != nil {
		wait = j.t.timeout + time.Minute // der Lauf endet spätestens mit seinem Zeitlimit
		ticker := time.NewTicker(s.progressEvery)
		defer ticker.Stop()
		tick = ticker.C
	}
	s.log.Info("⏳ check_run wartet auf Lauf", "ns", "check", "id", j.id, "target", j.t.name,
		"progress", progress != nil, "wait_s", int(wait.Seconds()))
	deadline := time.NewTimer(wait)
	defer deadline.Stop()
	for {
		select {
		case <-j.done:
			return j.answer()
		case <-tick:
			elapsed := time.Since(j.start)
			progress(fmt.Sprintf("%s läuft seit %s", j.t.name, formatMs(float64(elapsed.Milliseconds()))), elapsed)
		case <-deadline.C:
			return fmt.Sprintf("%s läuft noch seit %s · Lauf-ID %s · weiter warten mit check_run {run: %q}, Ausgabe mit console_tail check:%s",
				j.t.name, formatMs(float64(time.Since(j.start).Milliseconds())), j.id, j.id, j.t.name), nil
		case <-ctx.Done():
			return "", ctx.Err()
		}
	}
}
