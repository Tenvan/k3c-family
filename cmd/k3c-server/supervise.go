package main

import (
	"errors"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"time"

	"k3c/data"
)

// Selbst-Neustart (B-507): k3c-server startet sich selbst als Kindprozess (K3C_CHILD=1) und wartet. Endet das Kind mit
// restartCode, startet er es neu; die PID des Supervisors bleibt, Starter, k3c-workbench, Docker und task serve merken nichts.
// Das Kind liest data/ von der Platte (K3C_DATA_DIR wird vor dem Start gesetzt, die Pakete laden beim Start); stürzt
// es damit kurz nach dem Start ab, startet der Supervisor einmal mit dem eingebetteten Stand.

const (
	childEnv   = "K3C_CHILD"      // markiert den Kindprozess, der den Server wirklich betreibt
	earlyCrash = 10 * time.Second // ein Ende mit Fehler davor gilt als Absturz beim Laden der Daten
	loopWindow = time.Minute      // Schleifenbremse: mehr als loopMax Neustarts in diesem Fenster …
	loopMax    = 3
	loopPause  = 30 * time.Second // … ergeben diese Pause vor dem nächsten Start
)

// clock ist die Uhr des Supervisors (Test: eingespeist).
type clock struct {
	now   func() time.Time
	after func(time.Duration) <-chan time.Time
}

// proc ist ein laufendes Kind: done liefert seinen Exit-Code (beendet durch ein Signal: negativ), signal reicht ein
// Signal weiter.
type proc struct {
	done   <-chan int
	signal func(os.Signal)
}

// supervise startet das Kind (start, eingebettet oder mit Daten von der Platte) und liefert am Ende seinen Exit-Code.
// Neu gestartet wird nur, wenn das Kind restartCode liefert oder mit Daten von der Platte binnen earlyCrash mit einem
// Fehler endet (dann einmal eingebettet). Nach einem Signal (sigs, für die ganze Laufzeit) startet nichts mehr neu;
// ein Ende durch Signal oder von außen (negativer Code, Kill nach längerer Laufzeit) wird nur weitergereicht.
// embedded: Es gibt keinen Daten-Ordner.
func supervise(start func(embedded bool) (proc, error), sigs <-chan os.Signal, clk clock, embedded bool, log *slog.Logger) int {
	cur := embedded
	var restarts []time.Time
	for {
		began := clk.now()
		p, err := start(cur)
		if err != nil {
			log.Error("💥 Server-Prozess nicht gestartet", "ns", "main", "err", err)
			return 1
		}
		code, stopped := waitChild(p, sigs)
		switch {
		case stopped || pending(sigs): // Ende von außen gewollt: ein Neustart-Wunsch zählt nicht
			if code == restartCode || code < 0 {
				return 0
			}
			return code
		case code == restartCode:
			log.Info("🔁 Server startet neu", "ns", "watch")
			cur = embedded
			if restarts = recent(append(restarts, clk.now()), clk.now()); len(restarts) > loopMax {
				if !brake(sigs, clk, log) {
					return 0
				}
				restarts = nil
			}
		case code > 0 && !cur && clk.now().Sub(began) < earlyCrash:
			log.Error("❌ Server mit Daten von der Platte abgestürzt, starte mit eingebettetem Stand", "ns", "watch", "code", code)
			cur = true
		case code < 0:
			return 1
		default:
			return code
		}
	}
}

// recent sind die Neustarts der letzten loopWindow.
func recent(times []time.Time, now time.Time) []time.Time {
	for len(times) > 0 && now.Sub(times[0]) > loopWindow {
		times = times[1:]
	}
	return times
}

// brake wartet loopPause (Neustart-Schleife, z. B. Dateien ändern sich dauernd); false: Ein Signal kam, nicht neu starten.
func brake(sigs <-chan os.Signal, clk clock, log *slog.Logger) bool {
	log.Error("❌ zu viele Neustarts in kurzer Zeit", "ns", "watch", "anzahl", loopMax+1, "fensterS", loopWindow.Seconds())
	log.Warn("⏳ Pause vor dem nächsten Start", "ns", "watch", "pauseS", loopPause.Seconds())
	select {
	case <-sigs:
		return false
	case <-clk.after(loopPause):
		return true
	}
}

// waitChild wartet auf das Ende des Kinds und reicht Signale weiter; stopped: Es kam ein Signal.
func waitChild(p proc, sigs <-chan os.Signal) (code int, stopped bool) {
	for {
		select {
		case s := <-sigs:
			stopped = true
			p.signal(s)
		case code = <-p.done:
			return code, stopped
		}
	}
}

// pending: Wartet schon ein Signal (zwischen Ende des Kinds und Neustart)?
func pending(sigs <-chan os.Signal) bool {
	select {
	case <-sigs:
		return true
	default:
		return false
	}
}

// consoleLogger ist das Log des Supervisors: nur Text auf stderr, das JSON-Log schreibt das Kind.
func consoleLogger() *slog.Logger {
	log, _ := newLogger("", os.Stderr)
	return log
}

// childEnviron ist die Umgebung des Kinds: K3C_CHILD und der Daten-Ordner (eingebettet: leer). exec.Cmd nimmt bei
// doppelten Schlüsseln den letzten.
func childEnviron(env []string, dataDir string, embedded bool) []string {
	if embedded {
		dataDir = ""
	}
	return append(env, childEnv+"=1", data.DirEnv+"="+dataDir)
}

// dataDir ist der absolute Daten-Ordner (K3C_DATA_DIR, sonst data/ im Arbeitsverzeichnis); leer, wenn es ihn nicht gibt.
func dataDir() string {
	dir := absDir(env(data.DirEnv, "data"))
	if info, err := os.Stat(dir); err != nil || !info.IsDir() {
		return ""
	}
	return dir
}

func absDir(dir string) string {
	if abs, err := filepath.Abs(dir); err == nil {
		return abs
	}
	return dir
}

// startChild startet dieselbe EXE mit denselben Argumenten als Kind. Signale reicht supervise weiter (unter Windows
// nicht möglich: Dort bekommt das Kind Strg+C/Strg+Pause über die gemeinsame Konsole selbst).
func startChild(dir string, log *slog.Logger) func(embedded bool) (proc, error) {
	return func(embedded bool) (proc, error) {
		exe, err := os.Executable()
		if err != nil {
			return proc{}, err
		}
		cmd := exec.Command(exe, os.Args[1:]...)
		cmd.Stdin, cmd.Stdout, cmd.Stderr = os.Stdin, os.Stdout, os.Stderr
		cmd.Env = childEnviron(os.Environ(), dir, embedded)
		if err := cmd.Start(); err != nil {
			return proc{}, err
		}
		done := make(chan int, 1)
		go func() {
			code, err := exitCode(cmd.Wait())
			if err != nil {
				log.Error("💥 Server-Prozess: Ende unbekannt", "ns", "main", "err", err)
				code = 1
			}
			done <- code
		}()
		return proc{done: done, signal: func(s os.Signal) { _ = cmd.Process.Signal(s) }}, nil
	}
}

// exitCode ist der Exit-Code des beendeten Kinds (durch ein Signal beendet: -1); ein anderer Fehler ist ein Fehler.
func exitCode(err error) (int, error) {
	var exit *exec.ExitError
	switch {
	case err == nil:
		return 0, nil
	case errors.As(err, &exit):
		return exit.ExitCode(), nil
	}
	return 0, err
}
