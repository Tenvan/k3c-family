//go:build windows

package devrun

import (
	"context"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// TestHelperDevLauf ist der Dev-Lauf für TestHartesEndeBeendetKind in einem eigenen Prozess; sonst übersprungen.
func TestHelperDevLauf(t *testing.T) {
	src, state := os.Getenv("DEVRUN_HELPER_SRC"), os.Getenv("DEVRUN_HELPER_STATE")
	if src == "" {
		t.Skip("nur als Hilfsprozess")
	}
	_ = Run(context.Background(), Config{Name: "helper", Dir: src, Package: ".", Child: filepath.Join("bin", "app-dev.exe"), Args: []string{state}})
}

// Wird der Dev-Lauf hart beendet (Absturz, Kill ohne /T), beendet das Job-Objekt auch das Kind: Kein verwaister Prozess
// hält Port oder EXE. Unter Windows lässt sich die EXE eines laufenden Prozesses nicht löschen.
func TestHartesEndeBeendetKind(t *testing.T) {
	src, state := newAppDirs(t)
	helper := exec.Command(os.Args[0], "-test.run=^TestHelperDevLauf$")
	helper.Env = append(os.Environ(), "DEVRUN_HELPER_SRC="+src, "DEVRUN_HELPER_STATE="+state)
	if err := helper.Start(); err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = helper.Process.Kill(); _ = helper.Wait() })
	waitUntil(t, "Kind gestartet", func() bool {
		b, _ := os.ReadFile(filepath.Join(state, "log.txt"))
		return strings.Contains(string(b), "a start 1")
	})
	_ = helper.Process.Kill()
	_ = helper.Wait()
	exe := filepath.Join(src, "bin", "app-dev.exe")
	waitUntil(t, "Kind nach hartem Ende des Dev-Laufs beendet (EXE löschbar)", func() bool { return os.Remove(exe) == nil })
}
