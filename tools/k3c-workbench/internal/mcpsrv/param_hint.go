package mcpsrv

import (
	"reflect"
	"sort"
	"strings"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// additionalProps ist der Teil der SDK-Meldung, an dem ein unbekanntes Feld erkennbar ist. Die Meldung nennt das
// falsche Feld, aber nie die richtigen; ohne Hinweis rät ein Agent weiter.
const additionalProps = "additional properties"

// jsonNames liest die Parameternamen aus den json-Tags der Eingabe, also aus derselben Quelle wie das Schema.
func jsonNames(t reflect.Type) []string {
	if t.Kind() != reflect.Struct {
		return nil
	}
	var names []string
	for f := range t.Fields() {
		name, _, _ := strings.Cut(f.Tag.Get("json"), ",")
		if name == "" {
			name = f.Name
		}
		if name != "-" && f.IsExported() {
			names = append(names, name)
		}
	}
	sort.Strings(names)
	return names
}

// addParamHint hängt an eine Ablehnung wegen unbekannter Felder die gültigen Parameter des Tools an.
func (s *Server) addParamHint(tool string, res mcp.Result) {
	r, ok := res.(*mcp.CallToolResult)
	if !ok || r == nil || !r.IsError || !strings.Contains(resultText(r), additionalProps) {
		return
	}
	names, known := s.params[tool]
	if !known {
		return
	}
	hint := "gültige Parameter: keine"
	if len(names) > 0 {
		hint = "gültige Parameter: " + strings.Join(names, ", ")
	}
	r.Content = append(r.Content, &mcp.TextContent{Text: hint})
}
