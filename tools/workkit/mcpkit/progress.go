package mcpkit

import (
	"context"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

type progressKey struct{}

// ProgressFunc meldet dem Client, dass der Aufruf noch arbeitet; elapsed steigt monoton (B-520).
type ProgressFunc func(msg string, elapsed time.Duration)

// WithProgress hängt einen Melder an ctx, wenn der Aufruf ein progressToken trägt. Add und AddImage tun das für
// jeden Handler.
func WithProgress(ctx context.Context, req *mcp.CallToolRequest) context.Context {
	if req == nil || req.Session == nil || req.Params == nil {
		return ctx
	}
	token := req.Params.GetProgressToken()
	if token == nil {
		return ctx
	}
	session := req.Session
	return WithProgressFunc(ctx, func(msg string, elapsed time.Duration) {
		// Fortschritt ist nur ein Lebenszeichen: Ein Fehler beim Senden ändert nichts am Lauf.
		_ = session.NotifyProgress(ctx, &mcp.ProgressNotificationParams{
			ProgressToken: token, Message: msg, Progress: elapsed.Seconds()})
	})
}

// WithProgressFunc hängt einen eigenen Melder an ctx (Tests).
func WithProgressFunc(ctx context.Context, p ProgressFunc) context.Context {
	return context.WithValue(ctx, progressKey{}, p)
}

// ProgressOf ist der Melder des Aufrufs; nil, wenn der Client kein progressToken geschickt hat.
func ProgressOf(ctx context.Context) ProgressFunc {
	p, _ := ctx.Value(progressKey{}).(ProgressFunc)
	return p
}
