package mcpsrv

import (
	"context"
	"fmt"
	"slices"
	"strings"

	"k3c/tools/k3c-workbench/internal/console"
)

const (
	defaultTailLines = 50
	maxTailLines     = 500
)

type tailIn struct {
	Source string `json:"service" jsonschema:"Dienst oder Lauf, z. B. Vite oder check:task:test (Lauf des eigenen Checkouts, im Worktree checkout setzen)"`
	Lines  int    `json:"limit,omitempty" jsonschema:"Zahl der letzten Zeilen, Standard 50, höchstens 500"`
	Since  int64  `json:"since,omitempty" jsonschema:"nur Zeilen nach dieser Nummer (Nr. aus der letzten Antwort)"`
}

// consoleTail ist das Tool console_tail: die letzten Zeilen einer Konsolen-Quelle, stderr mit "! " markiert.
func (s *Server) consoleTail(ctx context.Context, in tailIn) (string, error) {
	ws := s.ws(ctx)
	n := in.Lines
	if n <= 0 {
		n = defaultTailLines
	}
	n = min(n, maxTailLines)
	want := n
	if in.Since > 0 {
		want = 0 // ab dem Cursor: alle holen, danach die ersten n nach dem Cursor
	}
	lines, ok := s.console.Tail(ws.consoleSource(in.Source), want)
	target, isCheck := strings.CutPrefix(in.Source, "check:")
	if !ok && isCheck {
		return "", fmt.Errorf("kein Lauf von %s im Checkout %s (check_run startet ihn dort)", in.Source, ws.label())
	}
	if !ok {
		var own []string
		for _, name := range s.console.Sources() {
			if o, mine := ws.ownSource(name); mine {
				own = append(own, o)
			}
		}
		return "", fmt.Errorf("unbekannte Quelle %q; bekannt: %s", in.Source, orNone(own))
	}
	if in.Since > 0 {
		lines = slices.DeleteFunc(slices.Clone(lines), func(l console.Line) bool { return l.Seq <= in.Since })
		lines = lines[:min(n, len(lines))]
	}
	head := in.Source
	if isCheck { // Läufe nennen Checkout und Startzeit, damit kein fremder Lauf als eigener durchgeht (B-855)
		head += " · Checkout " + ws.label()
		if at, ok := s.checks.startOf(ws.source(target)); ok {
			head += " · Lauf ab " + at.Format("15:04:05")
		}
	}
	if len(lines) == 0 {
		return head + " · keine Ausgabe", nil
	}
	out := make([]string, 0, len(lines)+1)
	out = append(out, fmt.Sprintf("%s · letzte %d Zeilen (bis Nr. %d)", head, len(lines), lines[len(lines)-1].Seq))
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
