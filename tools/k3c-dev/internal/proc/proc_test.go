package proc

import (
	"context"
	"net"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// TestHelperProcess ist kein Test: als "parent" startet das Test-Binary einen Enkel und endet sofort (der Enkel ist
// dann verwaist), als "grandchild" öffnet es einen Port, schreibt ihn in K3C_PORTFILE und wartet.
func TestHelperProcess(t *testing.T) {
	switch os.Getenv("K3C_HELPER") {
	case "parent":
		child := exec.Command(os.Args[0], "-test.run=^TestHelperProcess$")
		child.Env = append(os.Environ(), "K3C_HELPER=grandchild")
		if err := child.Start(); err != nil {
			os.Exit(2)
		}
		os.Exit(0)
	case "grandchild":
		ln, err := net.Listen("tcp", "127.0.0.1:0")
		if err != nil {
			os.Exit(2)
		}
		_ = os.WriteFile(os.Getenv("K3C_PORTFILE"), []byte(ln.Addr().String()), 0o644)
		time.Sleep(time.Minute)
	}
}

func waitForAddr(t *testing.T, file string) string {
	t.Helper()
	for range 100 {
		if b, err := os.ReadFile(file); err == nil && len(b) > 0 {
			return strings.TrimSpace(string(b))
		}
		time.Sleep(50 * time.Millisecond)
	}
	t.Fatal("Enkel hat keinen Port gemeldet")
	return ""
}

func TestKillTrifftVerwaistenEnkel(t *testing.T) {
	portFile := filepath.Join(t.TempDir(), "port")
	t.Setenv("K3C_HELPER", "parent")
	t.Setenv("K3C_PORTFILE", portFile)
	cmd := Command(context.Background(), []string{os.Args[0], "-test.run=^TestHelperProcess$"})
	if err := cmd.Start(); err != nil {
		t.Fatal(err)
	}
	addr := waitForAddr(t, portFile)
	if err := cmd.Wait(); err != nil { // das Kind endet von selbst; Wait räumt den Job ab
		t.Fatalf("Kind: %v", err)
	}
	for range 50 {
		conn, err := net.DialTimeout("tcp", addr, 200*time.Millisecond)
		if err != nil {
			return // Enkel beendet
		}
		_ = conn.Close()
		time.Sleep(100 * time.Millisecond)
	}
	t.Errorf("verwaister Enkel an %s läuft noch", addr)
}

func TestKillOhneStartUndExecutable(t *testing.T) {
	cmd := Command(context.Background(), []string{"go", "version"})
	if err := cmd.Kill(); err != nil {
		t.Errorf("Kill vor Start: %v", err)
	}
	if got := Executable("go"); got != "go" {
		t.Errorf("Executable(go) = %q", got)
	}
	if out, err := Command(context.Background(), []string{"go", "version"}).Output(); err != nil || !strings.HasPrefix(string(out), "go version") {
		t.Errorf("einfacher Lauf: %q, %v", out, err)
	}
}
