package mcpsrv

import (
	"context"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/workkit/mcpkit"
)

// Dünne Anbindung an das Gerüst workkit/mcpkit (WZ63): die Namen, die Katalog und Handler der Workbench seit jeher
// benutzen, zeigen auf das Gerüst.

// Call ist ein Eintrag im Aufruf-Log (mcpkit.Call).
type Call = mcpkit.Call

// Snapshot sind alle Zähler für die Oberfläche (mcpkit.Snapshot).
type Snapshot = mcpkit.Snapshot

// workspace ist der Checkout eines Aufrufs (mcpkit.Workspace).
type workspace = mcpkit.Workspace

// HeaderRoot trägt das Arbeitsverzeichnis des Clients (mcpkit.HeaderRoot).
const HeaderRoot = mcpkit.HeaderRoot

// FindRoot sucht ab dir aufwärts das go.mod des Spiels (module k3c): die Repo-Wurzel bzw. den Worktree.
func FindRoot(dir string) (string, error) { return mcpkit.FindRoot(dir) }

// ws ist der Checkout des laufenden Aufrufs; ohne Zuordnung die Repo-Wurzel.
func (s *Server) ws(ctx context.Context) workspace { return s.kit.Workspace(ctx) }

// add registriert ein Tool, dessen Handler Text liefert (mcpkit.Add).
func add[In any](s *Server, t *mcp.Tool, h func(context.Context, In) (string, error)) {
	mcpkit.Add(s.kit, t, h)
}

// readOnly sind die Annotations eines Tools, das nur liest.
func readOnly() *mcp.ToolAnnotations { return mcpkit.ReadOnly() }

func resultText(r *mcp.CallToolResult) string { return mcpkit.ResultText(r) }

func clip(s string) string { return mcpkit.Clip(s) }
