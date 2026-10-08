package mcpsrv

import (
	"context"
	"fmt"
	"slices"
	"strings"
	"sync"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// sim_test (B-348, TR1): ein Tool für jeden Testlauf. `start` kehrt sofort mit einer Lauf-ID zurück, der Lauf arbeitet
// im Hintergrund der Workbench im Checkout der Session; `status` zeigt ihn in höchstens 10 Zeilen, `stop` bricht ab,
// `list` zeigt die Läufe des Checkouts. Die Modi stehen in simtest_spec.go, die Runner in simtest_<modus>.go.

const (
	maxRunsPerRoot = 2  // gleichzeitige Läufe je Checkout
	maxStatusLines = 10 // B-348: status höchstens 10 Zeilen
)

// simRun ist ein Testlauf. Felder nach dem Start nur unter simRuns.mu ändern (setPhase, finish).
type simRun struct {
	id, root, desc string
	started, ended time.Time
	state          string   // läuft, fertig, abgebrochen, fehler
	phase          string   // Schritt des Runners, z. B. „bauen“, „rechnen“
	verdict        string   // Pass, Fail oder leer
	lines          []string // Kennzahlen, je eine Zeile
	report         string   // Pfad relativ zum Checkout
	cancel         context.CancelFunc
}

// simRuns ist das Register aller Läufe der Workbench.
type simRuns struct {
	mu   sync.Mutex
	next int
	runs []*simRun
}

func (r *simRuns) find(root, id string) *simRun {
	for _, run := range r.runs {
		if run.root == root && run.id == id {
			return run
		}
	}
	return nil
}

func (r *simRuns) running(root string) int {
	n := 0
	for _, run := range r.runs {
		if run.root == root && run.state == "läuft" {
			n++
		}
	}
	return n
}

func (r *simRuns) setPhase(run *simRun, phase string) {
	r.mu.Lock()
	defer r.mu.Unlock()
	run.phase = phase
}

// setLines setzt die Kennzahlen eines laufenden Laufs (Zwischenstand im Status).
func (r *simRuns) setLines(run *simRun, lines []string) {
	r.mu.Lock()
	defer r.mu.Unlock()
	run.lines = lines
}

// finish setzt das Ende; ein Abbruch über stop gewinnt gegen ein späteres „fertig“ des Runners.
func (r *simRuns) finish(run *simRun, state, verdict string, lines []string, report string) {
	r.mu.Lock()
	defer r.mu.Unlock()
	if run.state == "läuft" {
		run.state = state
	}
	run.ended, run.verdict, run.lines, run.report = time.Now(), verdict, lines, report
}

// cancelAll beendet alle Läufe (Workbench stoppt).
func (r *simRuns) cancelAll() {
	r.mu.Lock()
	defer r.mu.Unlock()
	for _, run := range r.runs {
		if run.state == "läuft" {
			run.cancel()
		}
	}
}

func registerSimTest(s *Server) {
	add(s, &mcp.Tool{
		Name: "sim_test",
		Description: "Jeder Testlauf (Balancing, Performance, Stabilität) über ein Tool: action start (sofort zurück mit ID), " +
			"status (≤ 10 Zeilen), stop, list. mode offline (Mocks im Prozess) oder online (Spielserver des Checkouts), " +
			"clients 0 = headless, 1–4 = laufende Clients mit Bot-Eingabe; focus balance, perf, stability. Bericht unter reports/simtest-<id>/. " +
			"Nutze es bei: jedem Balancing-, Performance- oder Stabilitätslauf. Statt: task balance, task load oder Binaries in der Shell.",
		Annotations: &mcp.ToolAnnotations{OpenWorldHint: new(bool)},
	}, s.simTest)
}

// simTest ist das Tool sim_test.
func (s *Server) simTest(ctx context.Context, in simTestIn) (string, error) {
	root := s.ws(ctx).root
	switch in.Action {
	case "start":
		return s.simStart(ctx, root, in)
	case "status":
		return s.simStatus(root, in.ID)
	case "stop":
		return s.simStop(root, in.ID)
	case "list":
		return s.simList(root), nil
	}
	return "", fmt.Errorf("action %q unbekannt: start, status, stop oder list", in.Action)
}

func (s *Server) simStart(ctx context.Context, root string, in simTestIn) (string, error) {
	spec, err := in.spec()
	if err != nil {
		return "", err
	}
	runner, err := s.simRunner(ctx, spec)
	if err != nil {
		return "", err
	}
	s.sims.mu.Lock()
	if s.sims.running(root) >= maxRunsPerRoot {
		s.sims.mu.Unlock()
		return "", fmt.Errorf("schon %d Läufe in diesem Checkout; mit sim_test stop einen beenden", maxRunsPerRoot)
	}
	s.sims.next++
	runCtx, cancel := context.WithCancel(context.Background()) // nicht der Kontext des Aufrufs: der Lauf überlebt ihn
	run := &simRun{id: fmt.Sprintf("run-%d", s.sims.next), root: root, desc: spec.label(), started: time.Now(),
		state: "läuft", phase: "start", cancel: cancel}
	s.sims.runs = append(s.sims.runs, run)
	s.sims.mu.Unlock()
	s.log.Info("🚀 testlauf gestartet", "ns", "simtest", "id", run.id, "mode", run.desc)
	go func() {
		defer cancel()
		runner(runCtx, run)
		s.sims.mu.Lock()
		state, verdict := run.state, run.verdict
		s.sims.mu.Unlock()
		s.log.Info(map[bool]string{true: "✅ testlauf beendet", false: "❌ testlauf beendet"}[state == "fertig"],
			"ns", "simtest", "id", run.id, "state", state, "verdict", verdict)
	}()
	return fmt.Sprintf("▶ %s %s gestartet; Fortschritt mit sim_test status %s", run.id, run.desc, run.id), nil
}

func (s *Server) simStatus(root, id string) (string, error) {
	s.sims.mu.Lock()
	defer s.sims.mu.Unlock()
	run := s.sims.find(root, id)
	if run == nil {
		return "", fmt.Errorf("lauf %q unbekannt in diesem Checkout; sim_test list zeigt die Läufe", id)
	}
	return run.status(time.Now()), nil
}

func (s *Server) simStop(root, id string) (string, error) {
	s.sims.mu.Lock()
	run := s.sims.find(root, id)
	if run == nil {
		s.sims.mu.Unlock()
		return "", fmt.Errorf("lauf %q unbekannt in diesem Checkout", id)
	}
	if run.state != "läuft" {
		s.sims.mu.Unlock()
		return fmt.Sprintf("%s ist schon %s", id, run.state), nil
	}
	run.state = "abgebrochen"
	run.cancel()
	s.sims.mu.Unlock()
	s.log.Info("🛑 testlauf abgebrochen", "ns", "simtest", "id", id)
	return fmt.Sprintf("■ %s abgebrochen; der Bericht folgt, sobald der Runner aufgeräumt hat (sim_test status %s)", id, id), nil
}

func (s *Server) simList(root string) string {
	s.sims.mu.Lock()
	defer s.sims.mu.Unlock()
	var out []string
	for _, run := range slices.Backward(s.sims.runs) {
		if run.root == root && len(out) < maxStatusLines {
			out = append(out, run.head(time.Now()))
		}
	}
	if len(out) == 0 {
		return "keine Testläufe in diesem Checkout; starten mit sim_test {action: start, mode: offline}"
	}
	return strings.Join(out, "\n")
}

// head ist die Kopfzeile eines Laufs.
func (r *simRun) head(now time.Time) string {
	end := now
	if !r.ended.IsZero() {
		end = r.ended
	}
	icon := map[string]string{"läuft": "▶", "fertig": "✅", "abgebrochen": "■", "fehler": "❌"}[r.state]
	line := fmt.Sprintf("%s %s · %s · %s · %s", icon, r.id, r.desc, r.state, end.Sub(r.started).Round(time.Second))
	if r.state == "läuft" {
		line += " · " + r.phase
	}
	if r.verdict != "" {
		line += " · " + r.verdict
	}
	return line
}

// status: Kopfzeile, Kennzahlen, Bericht; höchstens maxStatusLines Zeilen.
func (r *simRun) status(now time.Time) string {
	out := []string{r.head(now)}
	room := maxStatusLines - 1
	if r.report != "" {
		room--
	}
	lines := r.lines
	if len(lines) > room {
		lines = append(slices.Clone(lines[:room-1]), fmt.Sprintf("… %d weitere im Bericht", len(r.lines)-room+1))
	}
	out = append(out, lines...)
	if r.report != "" {
		out = append(out, "Bericht: "+r.report)
	}
	return strings.Join(out, "\n")
}
