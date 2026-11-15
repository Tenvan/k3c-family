package main

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"os"
	"os/exec"
	"path/filepath"
	"sync/atomic"
	"testing"
	"time"

	"k3c/engine/simcli"
)

// childEnv startet die Test-EXE als k3c-sim (Abbruch-Test mit echtem Prozess); Argumente als JSON-Feld.
const childEnv = "K3C_SIM_CHILD_ARGS"

func TestMain(m *testing.M) {
	if raw := os.Getenv(childEnv); raw != "" {
		var args []string
		if err := json.Unmarshal([]byte(raw), &args); err != nil {
			os.Exit(simcli.ExitUsage)
		}
		os.Exit(simcli.Exec(context.Background(), args, os.Stdin, os.Stderr, simcli.NewEmitter(os.Stdout), nil))
	}
	os.Exit(m.Run())
}

// AC-01 (RS2.1): Abbruch per Prozess-Ende. Der Lauf meldet Fortschritt als JSON-Zeilen; nach der ersten Zeile endet
// der Prozess (TerminateProcess wie beim Job-Objekt der Workbench), ein Bericht entsteht nicht.
func TestAbbruchPerProzessEnde(t *testing.T) {
	exe, err := os.Executable()
	if err != nil {
		t.Fatal(err)
	}
	dir := t.TempDir()
	args, _ := json.Marshal([]string{"balance", "--targets", "--seeds", "20", "--days", "1", "--progress", "--report-dir", dir})
	cmd := exec.Command(exe, "-test.run=^$")
	cmd.Env = append(os.Environ(), childEnv+"="+string(args))
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		t.Fatal(err)
	}
	if err := cmd.Start(); err != nil {
		t.Fatal(err)
	}
	line := firstLine(t, stdout)
	var ev struct {
		Event       string
		Done, Total int
	}
	if err := json.Unmarshal([]byte(line), &ev); err != nil || ev.Event != "progress" || ev.Done < 1 || ev.Done >= ev.Total {
		_ = cmd.Process.Kill()
		t.Fatalf("erste Zeile %q: %v", line, err)
	}
	if err := cmd.Process.Kill(); err != nil {
		t.Fatal(err)
	}
	if err := cmd.Wait(); err == nil {
		t.Fatal("Prozess endete nach Kill mit Exit 0")
	}
	if files, _ := filepath.Glob(filepath.Join(dir, "balance-*")); len(files) > 0 {
		t.Fatalf("Bericht trotz Abbruch: %v", files)
	}
}

// Signal-Abbruch: Zeile aborted und Exit 130; ist der Lauf schon fertig, bleibt es bei dessen Ausgabe und Exit-Code.
func TestSignalAbbruch(t *testing.T) {
	var out, errOut bytes.Buffer
	var finished atomic.Bool
	code := -1
	onAbort(simcli.NewEmitter(&out), &errOut, &finished, func(c int) { code = c })
	if code != simcli.ExitAborted || out.String() != `{"event":"aborted"}`+"\n" {
		t.Fatalf("Exit %d, stdout %q", code, out.String())
	}
	out.Reset()
	code = -1
	finished.Store(true)
	onAbort(simcli.NewEmitter(&out), &errOut, &finished, func(c int) { code = c })
	if code != -1 || out.Len() != 0 {
		t.Fatalf("nach Laufende: Exit %d, stdout %q", code, out.String())
	}
}

// firstLine liest die erste Zeile von r, höchstens eine Minute lang.
func firstLine(t *testing.T, r interface{ Read([]byte) (int, error) }) string {
	t.Helper()
	got := make(chan string, 1)
	go func() {
		sc := bufio.NewScanner(r)
		sc.Scan()
		got <- sc.Text()
	}()
	select {
	case line := <-got:
		return line
	case <-time.After(time.Minute):
		t.Fatal("keine Fortschrittszeile nach einer Minute")
	}
	return ""
}
