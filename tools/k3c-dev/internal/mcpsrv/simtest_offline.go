package mcpsrv

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
	"sync"
	"time"
)

// Modus offline (B-348/AC-02): Balancing-Tester im Checkout bauen (bin/k3c-balance, fester Pfad, kein go run) und mit
// --targets ausführen. So rechnet die Engine des Checkouts, nicht die der Workbench. Bericht des Testers und eigener
// Bericht liegen in reports/simtest-<id>/.

const (
	buildTimeout   = 5 * time.Minute
	offlineTimeout = 60 * time.Minute
)

// balanceSummary ist der Teil von reports/balance-*.json, den sim_test liest (Format: internal/balance/summary.go).
type balanceSummary struct {
	Seeds   int  `json:"seeds"`
	Pass    bool `json:"pass"`
	Targets []struct {
		Name   string   `json:"name"`
		Kind   string   `json:"kind"`
		Value  *float64 `json:"value"`
		Lower  *float64 `json:"lower"`
		Upper  *float64 `json:"upper"`
		Status string   `json:"status"`
		Pass   bool     `json:"pass"`
	} `json:"targets"`
}

func (s *Server) runOffline(ctx context.Context, run *simRun, spec simSpec) {
	dir := filepath.Join("reports", "simtest-"+run.id)
	exe := filepath.Join("bin", "k3c-balance")
	if runtime.GOOS == "windows" {
		exe += ".exe"
	}
	steps := []struct {
		phase string
		spec  runSpec
	}{
		{"bauen", runSpec{dir: filepath.Join(run.root, "tools", "k3c-dev"), timeout: buildTimeout,
			args: []string{"go", "build", "-o", filepath.Join("..", "..", exe), "./internal/balance/cmd"}}},
		{"rechnen", runSpec{dir: run.root, timeout: offlineTimeout, args: offlineArgs(exe, dir, spec.seeds)}},
	}
	var mu sync.Mutex // stdout und stderr schreiben nebenläufig
	var tail []string
	for _, st := range steps {
		s.sims.setPhase(run, st.phase)
		st.spec.out = func(_, text string) {
			mu.Lock()
			defer mu.Unlock()
			tail = lastLines(append(tail, text), 3)
		}
		res := s.run(ctx, st.spec)
		if failed := stepFailed(ctx, res); failed != "" {
			mu.Lock()
			lines := append([]string{st.phase + ": " + failed}, tail...)
			mu.Unlock()
			s.finishOffline(run, "fehler", "", lines, dir)
			return
		}
	}
	sum, err := readBalanceSummary(filepath.Join(run.root, dir))
	if err != nil {
		s.finishOffline(run, "fehler", "", []string{err.Error()}, dir)
		return
	}
	verdict := "Fail"
	if sum.Pass {
		verdict = "Pass"
	}
	s.finishOffline(run, "fertig", verdict, sum.lines(), dir)
}

func offlineArgs(exe, dir string, seeds int) []string {
	args := []string{exe, "--targets", "--report-dir", dir}
	if seeds > 0 {
		args = append(args, "--seeds", strconv.Itoa(seeds))
	}
	return args
}

// stepFailed beschreibt einen gescheiterten Schritt; leer heißt geschafft. Ein Abbruch über stop zählt als Grund.
func stepFailed(ctx context.Context, res runResult) string {
	switch {
	case errors.Is(ctx.Err(), context.Canceled):
		return "abgebrochen"
	case res.err != nil:
		return "nicht gestartet: " + res.err.Error()
	case res.timedOut:
		return "Zeitlimit erreicht"
	case res.exit != 0:
		return fmt.Sprintf("Exit %d", res.exit)
	}
	return ""
}

// finishOffline schreibt den eigenen Bericht (Kopf und Kennzahlen) und setzt das Ende im Register.
func (s *Server) finishOffline(run *simRun, state, verdict string, lines []string, dir string) {
	report := filepath.ToSlash(filepath.Join(dir, "simtest.md"))
	s.sims.finish(run, state, verdict, lines, report)
	s.sims.mu.Lock()
	text := "# Testlauf " + run.id + "\n\n" + run.head(time.Now()) + "\n\n- " + strings.Join(run.lines, "\n- ") + "\n"
	s.sims.mu.Unlock()
	full := filepath.Join(run.root, dir)
	err := os.MkdirAll(full, 0o755)
	if err == nil {
		err = os.WriteFile(filepath.Join(full, "simtest.md"), []byte(text), 0o644)
	}
	if err != nil {
		s.log.Error("❌ testlauf-bericht nicht geschrieben", "ns", "simtest", "id", run.id, "error", err.Error())
	}
}

// readBalanceSummary liest den neuesten reports/…/balance-*.json des Laufs.
func readBalanceSummary(dir string) (balanceSummary, error) {
	files, _ := filepath.Glob(filepath.Join(dir, "balance-*.json"))
	if len(files) == 0 {
		return balanceSummary{}, fmt.Errorf("kein Bericht balance-*.json in %s", filepath.ToSlash(dir))
	}
	raw, err := os.ReadFile(files[len(files)-1])
	if err != nil {
		return balanceSummary{}, err
	}
	var sum balanceSummary
	if err := json.Unmarshal(raw, &sum); err != nil {
		return balanceSummary{}, fmt.Errorf("bericht unlesbar: %w", err)
	}
	return sum, nil
}

// lines: eine Zeile je Ziel, z. B. „Fail Burg hält Nacht 1–5: 4 % (75–90 %)“.
func (b balanceSummary) lines() []string {
	out := []string{fmt.Sprintf("%d Seeds", b.Seeds)}
	for _, t := range b.Targets {
		unit := ""
		if t.Kind == "share" {
			unit = " %"
		}
		mark := "Fail"
		if t.Pass {
			mark = "Pass"
		}
		out = append(out, fmt.Sprintf("%s %s: %s%s (%s)", mark, t.Name, num(t.Value), unit, bounds(t.Lower, t.Upper, unit)))
	}
	return out
}

// bounds: „75–90 %“, „≤ 1“ oder „≥ 20“.
func bounds(lower, upper *float64, unit string) string {
	switch {
	case lower == nil:
		return "≤ " + num(upper) + unit
	case upper == nil:
		return "≥ " + num(lower) + unit
	}
	return num(lower) + "–" + num(upper) + unit
}

func num(v *float64) string {
	if v == nil {
		return "–"
	}
	return strconv.FormatFloat(*v, 'f', -1, 64)
}

func lastLines(lines []string, n int) []string {
	if len(lines) > n {
		return lines[len(lines)-n:]
	}
	return lines
}
