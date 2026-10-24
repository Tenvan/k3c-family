package simrun

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"runtime"
	"strconv"
	"sync"
	"time"

	"k3c/data"
	"k3c/tools/k3c-workbench/internal/console"
	"k3c/tools/k3c-workbench/internal/proc"
)

// Modus offline (B-348/AC-02): Balancing-Tester im Checkout bauen (bin/k3c-balance, fester Pfad, kein go run) und mit
// --targets ausführen. So rechnet die Engine des Checkouts, nicht die der Workbench. Bericht des Testers und eigener
// Bericht liegen in reports/simtest-<id>/.

const (
	buildTimeout   = 5 * time.Minute
	offlineTimeout = 60 * time.Minute
)

// Proc ist ein Prozesslauf; Out bekommt jede Zeile mit ihrem Strom („stdout“, „stderr“).
type Proc struct {
	Dir     string
	Args    []string
	Timeout time.Duration
	Env     []string // zusätzlich zur Umgebung der Workbench, NAME=Wert
	Out     func(stream, text string)
}

// Result ist das Ende eines Prozesses.
type Result struct {
	Exit     int
	TimedOut bool
	Err      error // Start gescheitert
}

var progressLine = regexp.MustCompile(`^PROGRESS (\d+)/(\d+)$`)

// StartOffline prüft die Spec und reiht einen Offline-Lauf ein (wartet, wenn alle Plätze belegt sind).
func (g *Registry) StartOffline(root, where string, spec Spec) (Info, error) {
	if err := spec.Check(); err != nil {
		return Info{}, err
	}
	run, err := g.Start(root, where, spec.Title(), &spec, g.Offline(spec), true)
	if err != nil {
		return Info{}, err
	}
	return g.Get(root, run.id)
}

// Offline ist der Runner des Modus offline: bauen, rechnen, Bericht lesen.
func (g *Registry) Offline(spec Spec) Runner {
	return func(ctx context.Context, run *Run) {
		dir := ReportDir(run)
		// Zwei feste EXE-Pfade, einer je Platz: läuft der erste Lauf noch, sperrt Windows seine EXE, und der zweite
		// Bau schlüge fehl (kein go run, B-348/AC-02).
		exe := balanceExe(run.Slot())
		steps := []struct {
			phase string
			proc  Proc
		}{
			{"bauen", Proc{Dir: filepath.Join(run.root, "tools", "k3c-workbench"), Timeout: buildTimeout,
				Args: []string{"go", "build", "-o", filepath.Join("..", "..", exe), "./internal/balance/cmd"}}},
			{"rechnen", Proc{Dir: run.root, Timeout: offlineTimeout, Args: spec.args(exe, dir),
				Env: []string{data.DirEnv + "=" + filepath.Join(run.root, "data")}}},
		}
		var mu sync.Mutex // stdout und stderr schreiben nebenläufig
		var tail []string
		for _, st := range steps {
			g.SetPhase(run, st.phase)
			st.proc.Out = func(stream, text string) {
				if m := progressLine.FindStringSubmatch(text); m != nil && stream == "stdout" {
					done, _ := strconv.Atoi(m[1])
					total, _ := strconv.Atoi(m[2])
					g.SetProgress(run, done, total)
					return
				}
				mu.Lock()
				defer mu.Unlock()
				tail = lastLines(append(tail, text), 3)
			}
			if failed := stepFailed(ctx, g.Exec(ctx, st.proc)); failed != "" {
				mu.Lock()
				lines := append([]string{st.phase + ": " + failed}, tail...)
				mu.Unlock()
				g.finishFailed(run, lines, dir)
				return
			}
		}
		sum, err := readBalanceSummary(filepath.Join(run.root, dir), run.Started())
		if err != nil {
			g.Finish(run, Failed, "", []string{err.Error()}, dir)
			return
		}
		g.Finish(run, Done, sum.verdict(), sum.lines(), dir)
	}
}

// finishFailed beendet einen gescheiterten oder abgebrochenen Lauf; steht schon ein balance-*.json, gilt er als
// Teilbericht und liefert seine Kennzahlen.
func (g *Registry) finishFailed(run *Run, lines []string, dir string) {
	sum, err := readBalanceSummary(filepath.Join(run.root, dir), run.Started())
	if err != nil {
		g.Finish(run, Failed, "", lines, dir)
		return
	}
	g.Finish(run, Failed, sum.verdict(), append(append(lines, "Teilbericht:"), sum.lines()...), dir)
}

// stepFailed beschreibt einen gescheiterten Schritt; leer heißt geschafft. Ein Abbruch über Stop zählt als Grund.
func stepFailed(ctx context.Context, res Result) string {
	switch {
	case errors.Is(ctx.Err(), context.Canceled):
		return "abgebrochen"
	case res.Err != nil:
		return "nicht gestartet: " + res.Err.Error()
	case res.TimedOut:
		return "Zeitlimit erreicht"
	case res.Exit != 0:
		return fmt.Sprintf("Exit %d", res.Exit)
	}
	return ""
}

// runProcess startet einen Prozess mit Zeitlimit; Zeitlimit und Abbruch beenden den ganzen Prozessbaum (Job-Objekt).
func runProcess(ctx context.Context, p Proc) Result {
	ctx, cancel := context.WithTimeout(ctx, p.Timeout)
	defer cancel()
	cmd := proc.Command(ctx, p.Args)
	cmd.Dir = p.Dir
	if len(p.Env) > 0 {
		cmd.Env = append(os.Environ(), p.Env...)
	}
	out := p.Out
	if out == nil {
		out = func(string, string) {}
	}
	stdout := console.NewLineWriter(func(text string) { out("stdout", text) })
	stderr := console.NewLineWriter(func(text string) { out("stderr", text) })
	cmd.Stdout, cmd.Stderr = stdout, stderr
	err := cmd.Run()
	stdout.Flush()
	stderr.Flush()
	var res Result
	switch {
	case errors.Is(ctx.Err(), context.DeadlineExceeded):
		res.TimedOut, res.Exit = true, -1
	case cmd.ProcessState != nil:
		res.Exit = cmd.ProcessState.ExitCode()
	case err != nil:
		res.Err = err
	}
	return res
}

func lastLines(lines []string, n int) []string {
	if len(lines) > n {
		return lines[len(lines)-n:]
	}
	return lines
}

// balanceExe ist der feste Pfad der Balance-EXE für einen Platz (relativ zum Checkout): Platz 0 bin/k3c-balance,
// weitere bin/k3c-balance-<n+1>.
func balanceExe(slot int) string {
	exe := filepath.Join("bin", "k3c-balance")
	if slot > 0 {
		exe += "-" + strconv.Itoa(slot+1)
	}
	if runtime.GOOS == "windows" {
		exe += ".exe"
	}
	return exe
}
