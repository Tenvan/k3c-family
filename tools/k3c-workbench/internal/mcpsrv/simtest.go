package mcpsrv

import (
	"context"
	"fmt"
	"strings"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/simrun"
)

// sim_test (B-348, TR1): ein Tool für jeden Testlauf. `start` kehrt sofort mit einer Lauf-ID zurück, der Lauf arbeitet
// im Hintergrund der Workbench im Checkout der Session; `status` zeigt ihn in höchstens 10 Zeilen, `stop` bricht ab,
// `list` zeigt die Läufe des Checkouts. Die Modi stehen in simtest_spec.go, die Runner in simtest_<modus>.go.

// maxStatusLines: status und list höchstens 10 Zeilen (B-348). Register, Warteschlange und Offline-Runner stehen in
// internal/simrun; sim_test reiht nicht ein, sondern lehnt bei belegten Plätzen ab (wie vor WZ28.4).
const maxStatusLines = 10

// simRun ist ein Lauf im gemeinsamen Register.
type simRun = simrun.Run

func registerSimTest(s *Server) {
	add(s, &mcp.Tool{
		Name: "sim_test",
		Description: "Jeder Testlauf (Balancing, Performance, Stabilität) über ein Tool: action start (sofort zurück mit ID), " +
			"status (≤ 10 Zeilen), stop, list. mode offline (Mocks im Prozess) oder online (Spielserver des Checkouts), " +
			"clients 0 = headless, 1–4 = laufende Clients mit Bot-Eingabe; focus balance, perf, stability. Bericht unter reports/simtest-<id>/. " +
			"Offline ohne seeds gilt seedMode aus data/balance-targets.json (dev: nur k3c-dev); bots (z. B. coop2) und seedList gehen auch offline. " +
			"BAS-Standardlauf: {action: start, bots: coop2}. " +
			"Nutze es bei: jedem Balancing-, Performance- oder Stabilitätslauf. Statt: task balance, task k3c-load oder Binaries in der Shell.",
		Annotations: &mcp.ToolAnnotations{OpenWorldHint: new(bool)},
	}, s.simTest)
}

// simTest ist das Tool sim_test.
func (s *Server) simTest(ctx context.Context, in simTestIn) (string, error) {
	ws := s.ws(ctx)
	root := ws.root
	switch in.Action {
	case "start":
		return s.simStart(ctx, ws, in)
	case "status":
		return s.simStatus(root, in.ID)
	case "stop":
		return s.simStop(root, in.ID)
	case "list":
		return s.simList(root), nil
	}
	return "", fmt.Errorf("action %q unbekannt: start, status, stop oder list", in.Action)
}

func (s *Server) simStart(ctx context.Context, ws workspace, in simTestIn) (string, error) {
	root := ws.root
	spec, err := in.spec()
	if err != nil {
		return "", err
	}
	runner, err := s.simRunner(ctx, spec)
	if err != nil {
		return "", err
	}
	run, err := s.sims.Start(root, ws.short(), spec.label(), spec.offline(), runner, false)
	if err != nil {
		return "", fmt.Errorf("schon %d Läufe in diesem Checkout; mit sim_test stop einen beenden", simrun.MaxPerRoot)
	}
	id := run.ID()
	return fmt.Sprintf("▶ %s %s gestartet; Fortschritt mit sim_test status %s", id, spec.label(), id), nil
}

func (s *Server) simStatus(root, id string) (string, error) {
	info, err := s.sims.Get(root, id)
	if err != nil {
		return "", fmt.Errorf("lauf %q unbekannt in diesem Checkout; sim_test list zeigt die Läufe", id)
	}
	return info.Status(time.Now(), maxStatusLines), nil
}

func (s *Server) simStop(root, id string) (string, error) {
	before, err := s.sims.Stop(root, id)
	switch {
	case err != nil:
		return "", fmt.Errorf("lauf %q unbekannt in diesem Checkout", id)
	case before == simrun.Waiting:
		return fmt.Sprintf("■ %s entfernt, er wartete noch auf einen Platz", id), nil
	case before != simrun.Running:
		return fmt.Sprintf("%s ist schon %s", id, before), nil
	}
	return fmt.Sprintf("■ %s abgebrochen; der Bericht folgt, sobald der Runner aufgeräumt hat (sim_test status %s)", id, id), nil
}

func (s *Server) simList(root string) string {
	var out []string
	for _, info := range s.sims.List(root) {
		if len(out) < maxStatusLines {
			out = append(out, info.Head(time.Now()))
		}
	}
	if len(out) == 0 {
		return "keine Testläufe in diesem Checkout; starten mit sim_test {action: start, mode: offline}"
	}
	return strings.Join(out, "\n")
}
