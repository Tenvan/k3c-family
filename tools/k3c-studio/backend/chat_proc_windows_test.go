//go:build windows

package backend

import (
	"context"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"syscall"
	"testing"
	"time"
)

const stillActive = 259 // STILL_ACTIVE

func processAlive(pid int) bool {
	h, err := syscall.OpenProcess(0x1000, false, uint32(pid)) // PROCESS_QUERY_LIMITED_INFORMATION
	if err != nil {
		return false
	}
	defer func() { _ = syscall.CloseHandle(h) }()
	var code uint32
	return syscall.GetExitCodeProcess(h, &code) == nil && code == stillActive
}

// B-514/AC-01: Nach einem Abbruch lebt weder die CLI noch ihr Kindprozess.
func TestChatAbbruchBeendetProzessbaum(t *testing.T) {
	cli, _ := fakeCLI(t, "tree", time.Minute)
	pids := filepath.Join(t.TempDir(), "pids")
	cli.env = append(cli.env, fakePids+"="+pids)
	ctx, cancel := context.WithCancel(context.Background())
	done := make(chan chatOutcome, 1)
	go func() { done <- cli.run(ctx, "Hallo", chatOptions{}, func(string) {}) }()
	var child, grand int
	for range 500 {
		if raw, err := os.ReadFile(pids); err == nil {
			f := strings.Fields(string(raw))
			if len(f) == 2 {
				child, _ = strconv.Atoi(f[0])
				grand, _ = strconv.Atoi(f[1])
				break
			}
		}
		time.Sleep(20 * time.Millisecond)
	}
	if child == 0 || !processAlive(child) || !processAlive(grand) {
		t.Fatalf("Baum läuft nicht: Kind %d, Enkel %d", child, grand)
	}
	cancel()
	select {
	case out := <-done:
		if out.Status != chatAborted {
			t.Errorf("Status %q", out.Status)
		}
	case <-time.After(15 * time.Second):
		t.Fatal("run endet nicht nach Abbruch")
	}
	for range 100 {
		if !processAlive(child) && !processAlive(grand) {
			return
		}
		time.Sleep(50 * time.Millisecond)
	}
	t.Errorf("Prozess lebt nach Abbruch: Kind %v, Enkel %v", processAlive(child), processAlive(grand))
}
