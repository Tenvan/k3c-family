package mcpsrv

import (
	"context"
	"fmt"
	"slices"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/gitcommit"
)

// gitCommitIn ist die Eingabe von git_commit: genau diese Dateien werden gestaged und committet.
type gitCommitIn struct {
	Paths   []string `json:"paths" jsonschema:"Dateien relativ zum Repo; nur diese werden gestaged und committet"`
	Type    string   `json:"type" jsonschema:"feat, fix, docs, refactor, test, perf, chore, ci, build oder style"`
	Scope   string   `json:"scope,omitempty" jsonschema:"Domäne der Session kleingeschrieben, z. B. sim, cli, reg, dev, inf; leer = kein Scope"`
	Subject string   `json:"subject" jsonschema:"Betreff, eine Zeile, höchstens 72 Zeichen"`
	Body    string   `json:"body,omitempty" jsonschema:"Rumpf der Nachricht; ggf. Co-Authored-By-Zeile"`
}

// registerGitCommit ist das zentrale Commit-Werkzeug der Workbench: Index nur mit den genannten Dateien, nie auf main.
func registerGitCommit(s *Server) {
	destructive, closed := true, false
	add(s, &mcp.Tool{
		Name: "git_commit",
		Description: "Committet die genannten Dateien des Repos auf dem aktuellen Feature-Branch. Staged wird nur, was in paths steht; " +
			"liegt anderes im Index, lehnt das Werkzeug ab. Auf main wird nie committet. Die Nachricht folgt dem Format " +
			"`typ(domäne): Betreff` (docs/arbeitsweise.md › Commit-Titel). Hooks laufen. " +
			"Nutze es bei: jedem Commit, auch bei Planungsdateien, die die Workbench angelegt hat. Statt: git add und git commit in der Shell.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in gitCommitIn) (string, error) {
		return commitFiles(ctx, s.ws(ctx).Root, in)
	})
}

// commitFiles prüft Branch und Index, stagt die Pfade und committet.
func commitFiles(ctx context.Context, root string, in gitCommitIn) (string, error) {
	st, err := gitcommit.Read(ctx, root)
	if err != nil {
		return "", err
	}
	if st.Branch == "main" {
		return "", fmt.Errorf("auf main wird nicht committet: Feature-Branch anlegen")
	}
	for _, f := range st.Staged {
		if !slices.Contains(in.Paths, f.Path) {
			return "", fmt.Errorf("fremde gestagte Datei im Index: %s (erst entfernen oder mit committen)", f.Path)
		}
	}
	if err := gitcommit.Stage(ctx, root, in.Paths); err != nil {
		return "", err
	}
	msg := gitcommit.Message{Type: in.Type, Scope: in.Scope, Subject: in.Subject, Body: in.Body}
	hash, err := gitcommit.Commit(ctx, root, msg)
	if err != nil {
		return "", err
	}
	m, _ := msg.Normalize()
	return fmt.Sprintf("Commit %s auf %s: %s", hash, st.Branch, m.Format()), nil
}
