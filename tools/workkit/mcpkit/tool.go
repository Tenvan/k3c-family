package mcpkit

import (
	"context"
	"fmt"
	"reflect"
	"sort"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// ReadOnly sind die Annotations eines Tools, das nur liest.
func ReadOnly() *mcp.ToolAnnotations {
	closed := false
	return &mcp.ToolAnnotations{ReadOnlyHint: true, IdempotentHint: true, OpenWorldHint: &closed}
}

// Add registriert ein Tool, dessen Handler Text liefert; ein Fehler wird zum Fehler-Ergebnis.
func Add[In any](s *Server, t *mcp.Tool, h func(context.Context, In) (string, error)) {
	prepare[In](s, t)
	mcp.AddTool(s.mcp, t, func(ctx context.Context, req *mcp.CallToolRequest, in In) (*mcp.CallToolResult, any, error) {
		text, err := h(WithProgress(ctx, req), in)
		if err != nil {
			return nil, nil, err
		}
		return TextResult(text, false), nil, nil
	})
}

// AddImage registriert ein Tool, das ein PNG und Text liefert (wie Add, aber mit ImageContent).
func AddImage[In any](s *Server, t *mcp.Tool, h func(context.Context, In) ([]byte, string, error)) {
	prepare[In](s, t)
	mcp.AddTool(s.mcp, t, func(ctx context.Context, req *mcp.CallToolRequest, in In) (*mcp.CallToolResult, any, error) {
		bild, text, err := h(WithProgress(ctx, req), in)
		if err != nil {
			return nil, nil, err
		}
		return &mcp.CallToolResult{Content: []mcp.Content{&mcp.ImageContent{Data: bild, MIMEType: "image/png"}, &mcp.TextContent{Text: text}}}, nil, nil
	})
}

// prepare merkt die gültigen Parameter (Parameter-Hinweis), ergänzt bei Tools mit checkout das Schema und nimmt das
// Tool in die Zähler auf, damit es auch ohne Aufruf dort steht.
func prepare[In any](s *Server, t *mcp.Tool) {
	s.params[t.Name] = jsonNames(reflect.TypeFor[In]())
	if s.takesCheckout(t.Name) {
		schema, names, err := withCheckout[In](s.params[t.Name])
		if err != nil {
			panic(fmt.Sprintf("Schema von %s: %v", t.Name, err)) // wie mcp.AddTool bei einem ungültigen Typ
		}
		t.InputSchema, s.params[t.Name] = schema, names
		sort.Strings(s.params[t.Name])
	}
	s.stats.Register(t.Name, t.Description)
}
