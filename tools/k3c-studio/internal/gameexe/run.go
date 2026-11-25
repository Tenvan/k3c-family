package gameexe

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
	"sync"
	"sync/atomic"
	"time"

	"k3c/tools/workkit/proc"
)

// Grace ist die Frist zwischen dem Schließen von stdin (Abbruch) und dem harten Beenden des Prozessbaums.
const Grace = 5 * time.Second

// validSub sind die Unterbefehle von `k3c-game sim`.
var validSub = map[string]bool{"run": true, "level": true, "balance": true, "world": true, "hub": true}

// maxLine ist die längste erwartete stdout-Zeile (Berichte von world und hub).
const maxLine = 16 << 20

// Options beschreibt einen Lauf `k3c-game sim <Sub> …`.
type Options struct {
	Root     string       // Checkout: EXE, data/ und .work/runs/
	Data     string       // absoluter Daten-Ordner statt <Root>/data (Tests: Fixture-Daten, EXE der Repo-Wurzel); leer = <Root>/data
	Sub      string       // run, level, world, hub (balance ab RU40)
	Input    []byte       // Eingabe-JSON; der Starter schreibt sie nach <Root>/.work/runs/ und löscht sie nach dem Ende
	Window   bool         // --window (nur run und balance)
	Speed    int          // --speed N (1 bis 64), nur mit Window
	Args     []string     // weitere Schalter der Befehlszeile (balance: --targets …), stehen nach dem Eingabe-JSON
	OnEvent  func(Event)  // jede JSON-Zeile von stdout; darf nil sein (läuft nebenläufig zu OnStderr)
	OnStderr func(string) // jede Zeile von stderr; darf nil sein
	Grace    time.Duration
	Log      *slog.Logger
}

// Stream startet die EXE des Checkouts im Job-Objekt und reicht Zeilen an die Rückrufe. Ende:
//   - nil: Exit 0
//   - ctx.Err(): Abbruch. stdin wird geschlossen (die EXE schreibt `aborted`, Exit 130); nach Grace beendet das Job-Objekt
//     den ganzen Baum. Ein Ende nach eigenem Stop gilt als Abbruch, auch ohne Zeile `aborted`.
//   - sonst der Text der Zeile `error` oder „Exit <n>“.
func Stream(ctx context.Context, o Options) error {
	if !validSub[o.Sub] {
		return fmt.Errorf("unbekannter Unterbefehl %q", o.Sub)
	}
	exe, dir, err := Find(o.Root)
	if err != nil {
		return err
	}
	runs := filepath.Join(o.Root, ".work", "runs")
	if err := os.MkdirAll(runs, 0o750); err != nil {
		return err
	}
	f, err := os.CreateTemp(runs, time.Now().Format("20060102-150405")+"-*.json") // eindeutig auch gegen andere Prozesse
	if err != nil {
		return err
	}
	input := f.Name()
	defer func() { _ = os.Remove(input) }()
	_, werr := f.Write(o.Input)
	if cerr := f.Close(); werr == nil {
		werr = cerr
	}
	if werr != nil {
		return werr
	}
	return run(ctx, argvFor(exe, input, o), dir, o)
}

// argvFor baut die Befehlszeile `k3c-game sim <Sub> --input <Datei> --data <Data|Root/data> --stop-stdin [--window [--speed N]] [Args…]`.
func argvFor(exe, input string, o Options) []string {
	data := o.Data
	if data == "" {
		data = filepath.Join(o.Root, "data")
	}
	argv := []string{exe, "sim", o.Sub, "--input", input, "--data", data, "--stop-stdin"}
	if o.Window {
		argv = append(argv, "--window")
		if o.Speed > 0 {
			argv = append(argv, "--speed", strconv.Itoa(o.Speed))
		}
	}
	return append(argv, o.Args...)
}

// readLines ruft für jede Zeile von r den Rückruf; ist eine Zeile zu lang, wird der Rest verworfen (die EXE soll nicht
// an einer vollen Pipe blockieren).
func readLines(r io.Reader, line func(string)) {
	sc := bufio.NewScanner(r)
	sc.Buffer(make([]byte, 0, 64<<10), maxLine)
	for sc.Scan() {
		line(sc.Text())
	}
	if err := sc.Err(); err != nil {
		slog.Warn("❌ Zeile der Spiel-EXE nicht lesbar", "ns", "sim", "err", err)
	}
	_, _ = io.Copy(io.Discard, r)
}

// pumpEvents liest die JSON-Zeilen von stdout, merkt sich die Meldung der Zeile error und reicht jedes Event an on.
func pumpEvents(r io.Reader, on func(Event), failure *atomic.Value) {
	readLines(r, func(l string) {
		ev, ok := ParseEvent(strings.TrimSpace(l))
		if !ok {
			return
		}
		if ev.Event == "error" {
			failure.Store(ev.Message)
		}
		if on != nil {
			on(ev)
		}
	})
}

// watch wartet auf das Ende des Prozesses (done) oder des Contexts: dann schließt es stdin (die EXE bricht ab) und
// beendet nach grace den ganzen Baum. Ohne Abbruch gibt es keine Frist: Stream wartet, bis die EXE endet.
func watch(ctx context.Context, stdin io.Closer, cmd *proc.Cmd, grace time.Duration, done <-chan struct{}, stopped *atomic.Bool, log *slog.Logger) {
	select {
	case <-done:
	case <-ctx.Done():
		stopped.Store(true)
		_ = stdin.Close()
		select {
		case <-done:
		case <-time.After(grace):
			log.Warn("⏳ Spiel-EXE nach der Frist hart beendet", "ns", "sim", "grace", grace)
			if err := cmd.Kill(); err != nil {
				log.Error("💥 Spiel-EXE nicht beendet", "ns", "sim", "err", err)
			}
		}
	}
}

// run startet argv (Tests ersetzen die EXE durch das Test-Binary) und wartet auf das Ende.
func run(ctx context.Context, argv []string, dir string, o Options) error {
	log := o.Log
	if log == nil {
		log = slog.Default()
	}
	grace := max(o.Grace, 0)
	if grace == 0 {
		grace = Grace
	}
	// bewusst ohne ctx im Befehl: ctx beendet nicht hart, sondern über stdin (siehe Stream)
	cmd := proc.Command(context.Background(), argv)
	cmd.Dir = dir
	stdin, err := cmd.StdinPipe()
	if err != nil {
		return err
	}
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return err
	}
	stderr, err := cmd.StderrPipe()
	if err != nil {
		return err
	}
	if err := cmd.Start(); err != nil {
		log.Error("❌ Spiel-EXE nicht gestartet", "ns", "sim", "sub", o.Sub, "err", err)
		return fmt.Errorf("Spiel-EXE nicht gestartet: %w", err)
	}
	log.Info("🚀 Spiel-EXE gestartet", "ns", "sim", "exe", argv[0], "sub", o.Sub, "head", o.Window)

	var failure atomic.Value
	var wg sync.WaitGroup
	wg.Add(2)
	go func() { defer wg.Done(); pumpEvents(stdout, o.OnEvent, &failure) }()
	go func() {
		defer wg.Done()
		readLines(stderr, func(l string) {
			if o.OnStderr != nil {
				o.OnStderr(l)
			}
		})
	}()

	var stopped atomic.Bool
	done, watcher := make(chan struct{}), make(chan struct{})
	go func() { defer close(watcher); watch(ctx, stdin, cmd, grace, done, &stopped, log) }()

	wg.Wait() // beide Pipes zu: der Prozess ist beendet; os/exec verlangt Wait erst nach den Lesern
	waitErr := cmd.Wait()
	close(done)
	<-watcher
	_ = stdin.Close()

	switch {
	case waitErr == nil: // ein fertiger Lauf zählt auch dann, wenn der Abbruch gleichzeitig kam
		log.Info("✅ Spiel-EXE beendet", "ns", "sim", "sub", o.Sub)
		return nil
	case stopped.Load():
		log.Info("🛑 Spiel-EXE abgebrochen", "ns", "sim", "sub", o.Sub)
		return ctx.Err()
	}
	msg := failureText(waitErr, &failure)
	log.Error("❌ Spiel-EXE gescheitert", "ns", "sim", "sub", o.Sub, "err", msg)
	return errors.New(msg)
}

// Call führt einen kurzen Lauf (run, level, world, hub) aus und liefert das letzte Ergebnis (result oder report).
func Call(ctx context.Context, o Options) (Event, error) {
	var res *Event
	on := o.OnEvent
	o.OnEvent = func(ev Event) {
		if ev.Event == "result" || ev.Event == "report" {
			res = &ev
		}
		if on != nil {
			on(ev)
		}
	}
	if err := Stream(ctx, o); err != nil {
		return Event{}, err
	}
	if res == nil {
		return Event{}, fmt.Errorf("k3c-game sim %s: ohne Ergebnis beendet", o.Sub)
	}
	return *res, nil
}

// failureText ist die Meldung der Zeile error, sonst „Exit <n>“.
func failureText(waitErr error, failure *atomic.Value) string {
	if m, ok := failure.Load().(string); ok && m != "" {
		return m
	}
	var ee *exec.ExitError
	if errors.As(waitErr, &ee) {
		return fmt.Sprintf("Exit %d", ee.ExitCode())
	}
	return waitErr.Error()
}
