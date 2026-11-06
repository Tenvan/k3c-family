package mcpsrv

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"slices"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/assetsui"
)

// Prompt-Vorlagen als MCP-Prompts (paper-mcp.md › MCP-Prompts): asset-<id>, Argumente = Platzhalter im Text.

// vorlagenPfad ist die Datei der Prompt-Vorlagen relativ zum Checkout.
const vorlagenPfad = "data/asset-prompts.json"

var platzhalter = regexp.MustCompile(`\{(asset|rig|teil|pfad|zustand|basis|eingabe)\}`)

// optional dürfen leer bleiben; alle anderen Platzhalter sind Pflicht.
var optional = map[string]bool{"zustand": true, "basis": true}

func leseVorlagen(root string) ([]assetsui.Vorlage, error) {
	b, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(vorlagenPfad)))
	if err != nil {
		return nil, err
	}
	var v []assetsui.Vorlage
	return v, json.Unmarshal(b, &v)
}

// platzhalterIn sind die Platzhalter eines Textes in Reihenfolge des ersten Auftretens.
func platzhalterIn(text string) []string {
	var out []string
	for _, m := range platzhalter.FindAllStringSubmatch(text, -1) {
		if !slices.Contains(out, m[1]) {
			out = append(out, m[1])
		}
	}
	return out
}

// registerAssetPrompts meldet jede Vorlage der Repo-Wurzel als Prompt an; neue oder gelöschte Vorlagen gelten nach
// einem Neustart, geänderte Texte sofort (der Handler liest die Datei bei jedem Abruf).
func registerAssetPrompts(s *Server) {
	vorlagen, err := leseVorlagen(s.cfg.Root)
	if err != nil {
		return // ohne Datei (z. B. in Tests ohne Repo) keine Prompts
	}
	for _, v := range vorlagen {
		p := &mcp.Prompt{Name: "asset-" + v.ID, Title: v.Titel, Description: fmt.Sprintf("Asset-Prompt (%s): %s", v.Ort, v.Titel)}
		for _, name := range platzhalterIn(v.Text) {
			p.Arguments = append(p.Arguments, &mcp.PromptArgument{Name: name, Required: !optional[name]})
		}
		id := v.ID
		s.mcp.AddPrompt(p, func(_ context.Context, req *mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
			return promptText(s.cfg.Root, id, req.Params.Arguments)
		})
	}
}

func promptText(root, id string, args map[string]string) (*mcp.GetPromptResult, error) {
	vorlagen, err := leseVorlagen(root)
	if err != nil {
		return nil, err
	}
	i := slices.IndexFunc(vorlagen, func(v assetsui.Vorlage) bool { return v.ID == id })
	if i < 0 {
		return nil, fmt.Errorf("Vorlage „%s“ fehlt in %s", id, vorlagenPfad) //nolint:staticcheck // Satz für die Antwort
	}
	text := vorlagen[i].Text
	for _, name := range platzhalterIn(text) {
		if args[name] == "" && !optional[name] {
			return nil, fmt.Errorf("Argument „%s“ fehlt", name) //nolint:staticcheck // Satz für die Antwort
		}
	}
	text = platzhalter.ReplaceAllStringFunc(text, func(m string) string { return args[m[1:len(m)-1]] })
	return &mcp.GetPromptResult{Description: vorlagen[i].Titel,
		Messages: []*mcp.PromptMessage{{Role: "user", Content: &mcp.TextContent{Text: text}}}}, nil
}
