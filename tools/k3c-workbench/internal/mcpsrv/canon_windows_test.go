package mcpsrv

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"golang.org/x/sys/windows"
)

// B-628: Ein Kurzname (wie RUNNER~1 auf dem CI-Runner) gleicht dem Langnamen, auch wenn das Ende des Pfads noch fehlt.
func TestSamePathKurznameOhneVorhandenesEnde(t *testing.T) {
	long := filepath.Join(t.TempDir(), "ein langer Ordnername")
	if err := os.Mkdir(long, 0o755); err != nil {
		t.Fatal(err)
	}
	buf := make([]uint16, windows.MAX_PATH)
	n, err := windows.GetShortPathName(windows.StringToUTF16Ptr(long), &buf[0], uint32(len(buf)))
	if err != nil || n == 0 {
		t.Skipf("kein Kurzname (8.3 auf dem Laufwerk aus): %v", err)
	}
	short := windows.UTF16ToString(buf[:n])
	if !strings.Contains(short, "~") {
		t.Skipf("Kurzname ohne ~: %s", short)
	}
	if !samePath(filepath.Join(short, "fehlt", "noch"), filepath.Join(long, "fehlt", "noch")) {
		t.Errorf("%s ≠ %s", short, long)
	}
	if samePath(filepath.Join(short, "a"), filepath.Join(long, "b")) {
		t.Error("verschiedene Enden gelten als gleich")
	}
}
