package main

// Lauf: Schritte einer Stufe ausführen, Bahnen nebeneinander, Schritte einer Bahn nacheinander bis zum ersten Fehler.
// Die Marker `⏱▶ name` und `⏱■ name exit=n` zeitstempelt check_run der Workbench (B-739).

import (
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"sync"
	"time"
)

var outMu sync.Mutex

func say(format string, args ...any) {
	outMu.Lock()
	defer outMu.Unlock()
	fmt.Printf(format+"\n", args...)
}

// runStep führt einen Schritt aus; die Ausgabe erscheint nur bei einem Fehler, am Stück.
func runStep(s step) bool {
	say("⏱▶ %s", s.Name)
	start := time.Now()
	cmd := exec.Command(s.Cmd[0], s.Cmd[1:]...)
	cmd.Dir = filepath.FromSlash(s.Dir)
	out, err := cmd.CombinedOutput()
	code := 0
	var exit *exec.ExitError
	switch {
	case errors.As(err, &exit):
		code = exit.ExitCode()
	case err != nil:
		code = 127
		out = append(out, []byte(err.Error()+"\n")...)
	}
	outMu.Lock()
	defer outMu.Unlock()
	if code != 0 {
		fmt.Printf("❌ %s (%.1f s)\n", describe(s), time.Since(start).Seconds())
		_, _ = os.Stdout.Write(out)
	}
	fmt.Printf("⏱■ %s exit=%d\n", s.Name, code)
	return code == 0
}

// runStage führt die Schritte einer Stufe aus und liefert die Namen der gescheiterten Schritte.
func runStage(steps []step, stage int) []string {
	lanes := map[string][]step{}
	for _, s := range steps {
		if s.Stage == stage {
			lanes[s.Lane] = append(lanes[s.Lane], s)
		}
	}
	var (
		wg     sync.WaitGroup
		mu     sync.Mutex
		failed []string
	)
	for _, lane := range lanes {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for _, s := range lane {
				if !runStep(s) {
					mu.Lock()
					failed = append(failed, s.Name)
					mu.Unlock()
					return
				}
			}
		}()
	}
	wg.Wait()
	return failed
}
