package mcpsrv

import (
	"context"
	"fmt"
	"strings"
)

const (
	defaultTailLines = 50
	maxTailLines     = 500
)

type tailIn struct {
	Source string `json:"source" jsonschema:"Quelle, z. B. check:task:test"`
	Lines  int    `json:"lines,omitempty" jsonschema:"Zahl der letzten Zeilen, Standard 50, höchstens 500"`
}

// consoleTail ist das Tool console_tail: die letzten Zeilen einer Konsolen-Quelle, stderr mit "! " markiert.
func (s *Server) consoleTail(ctx context.Context, in tailIn) (string, error) {
	ws := s.ws(ctx)
	n := in.Lines
	if n <= 0 {
		n = defaultTailLines
	}
	n = min(n, maxTailLines)
	lines, ok := s.console.Tail(ws.consoleSource(in.Source), n)
	if !ok {
		var own []string
		for _, name := range s.console.Sources() {
			if o, mine := ws.ownSource(name); mine {
				own = append(own, o)
			}
		}
		return "", fmt.Errorf("unbekannte Quelle %q; bekannt: %s", in.Source, orNone(own))
	}
	if len(lines) == 0 {
		return in.Source + " · keine Ausgabe", nil
	}
	out := make([]string, 0, len(lines)+1)
	out = append(out, fmt.Sprintf("%s · letzte %d Zeilen (bis Nr. %d)", in.Source, len(lines), lines[len(lines)-1].Seq))
	for _, l := range lines {
		if l.Stream == "stderr" {
			out = append(out, "! "+ansi.ReplaceAllString(l.Text, ""))
		} else {
			out = append(out, ansi.ReplaceAllString(l.Text, "")) // Farben bleiben im Puffer für die Oberfläche
		}
	}
	return strings.Join(out, "\n"), nil
}

func orNone(names []string) string {
	if len(names) == 0 {
		return "noch keine"
	}
	return strings.Join(names, ", ")
}
