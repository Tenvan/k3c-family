package mcpkit

import (
	"fmt"
	"strings"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// ArgsRunes begrenzt die Argumente im Log.
const ArgsRunes = 200

// ClipArgs kürzt rohe Tool-Argumente für das Log.
func ClipArgs(raw []byte) string {
	r := []rune(string(raw))
	if len(r) <= ArgsRunes {
		return string(r)
	}
	return string(r[:ArgsRunes-1]) + "…"
}

// OutcomeOf liest Erfolg, Fehlertext und erste Antwortzeile aus einem Ergebnis.
func OutcomeOf(res mcp.Result, err error) Outcome {
	if err != nil {
		return Outcome{Err: err.Error()}
	}
	r, ok := res.(*mcp.CallToolResult)
	if !ok || r == nil {
		return Outcome{OK: true}
	}
	text := ResultText(r)
	first, _, _ := strings.Cut(text, "\n")
	if r.IsError {
		return Outcome{Err: text, Summary: first}
	}
	return Outcome{OK: true, Summary: first}
}

// ResultText hängt die Text-Inhalte eines Ergebnisses aneinander.
func ResultText(r *mcp.CallToolResult) string {
	var parts []string
	for _, c := range r.Content {
		if t, ok := c.(*mcp.TextContent); ok {
			parts = append(parts, t.Text)
		}
	}
	return strings.Join(parts, "\n")
}

// TextResult ist ein Ergebnis aus einem Text, mit isError als Fehler-Ergebnis.
func TextResult(text string, isError bool) *mcp.CallToolResult {
	return &mcp.CallToolResult{Content: []mcp.Content{&mcp.TextContent{Text: text}}, IsError: isError}
}

// Clip kürzt auf TextRunes Zeichen.
func Clip(s string) string {
	r := []rune(s)
	if len(r) <= TextRunes {
		return s
	}
	return string(r[:TextRunes-1]) + "…"
}

// Ref ist ein kurzer, im Ring eindeutiger Name je Aufruf: Multiplikation mit einer ungeraden Zahl ist modulo 2^28
// umkehrbar, zwei IDs unter 2^28 bekommen also nie denselben Namen.
func Ref(id int64) string {
	return fmt.Sprintf("%07x", (uint64(id)*2654435761)&0xfffffff)
}
