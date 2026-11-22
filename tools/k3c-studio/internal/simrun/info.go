package simrun

import (
	"fmt"
	"slices"
	"strings"
	"time"
)

// Info ist der Stand eines Laufs für sim_test und die Seite (JSON der RPC BalanceRuns).
type Info struct {
	ID      string    `json:"id"`
	Label   string    `json:"label"`
	Where   string    `json:"where"` // „Repo-Wurzel“ oder der Name des Worktrees
	State   string    `json:"state"`
	Phase   string    `json:"phase"`
	Done    int       `json:"done"`
	Total   int       `json:"total"`
	Started time.Time `json:"started"`
	Ended   time.Time `json:"ended,omitzero"`
	Verdict string    `json:"verdict"`
	Report  string    `json:"report"` // Pfad relativ zum Checkout, leer bis der Bericht steht
	Lines   []string  `json:"lines"`
	Spec    *Spec     `json:"spec,omitempty"`
}

// info: unter Registry.mu.
func (r *Run) info() Info {
	return Info{ID: r.id, Label: r.label, Where: r.where, State: r.state, Phase: r.phase, Done: r.done, Total: r.total,
		Started: r.started, Ended: r.ended, Verdict: r.verdict, Report: r.report, Lines: slices.Clone(r.lines), Spec: r.spec}
}

// Head ist die Kopfzeile eines Laufs, z. B. „▶ run-1 · offline·headless·balance · läuft · 12s · Repo-Wurzel · rechnen“.
func (i Info) Head(now time.Time) string {
	end := now
	if !i.Ended.IsZero() {
		end = i.Ended
	}
	icon := map[string]string{Waiting: "⏳", Running: "▶", Done: "✅", Canceled: "■", Failed: "❌"}[i.State]
	line := fmt.Sprintf("%s %s · %s · %s · %s", icon, i.ID, i.Label, i.State, end.Sub(i.Started).Round(time.Second))
	line += " · " + i.Where
	if i.State == Running {
		line += " · " + i.Phase
	}
	if i.Verdict != "" {
		line += " · " + i.Verdict
	}
	return line
}

// Status: Kopfzeile, Kennzahlen, Bericht; höchstens maxLines Zeilen.
func (i Info) Status(now time.Time, maxLines int) string {
	out := []string{i.Head(now)}
	room := maxLines - 1
	if i.Report != "" {
		room--
	}
	lines := i.Lines
	if len(lines) > room {
		lines = append(slices.Clone(lines[:room-1]), fmt.Sprintf("… %d weitere im Bericht", len(i.Lines)-room+1))
	}
	out = append(out, lines...)
	if i.Report != "" {
		out = append(out, "Bericht: "+i.Report)
	}
	return strings.Join(out, "\n")
}
