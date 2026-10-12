package mcpsrv

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// B-416: k3c-dev läuft ohne Konsole; git und andere Kindprozesse nur über proc.Command (CREATE_NO_WINDOW).
func TestKeinExecCommandAufGit(t *testing.T) {
	err := filepath.WalkDir("..", func(p string, d os.DirEntry, err error) error {
		if err != nil || d.IsDir() || !strings.HasSuffix(p, ".go") || strings.HasSuffix(p, "_test.go") {
			return err
		}
		b, _ := os.ReadFile(p)
		if strings.Contains(string(b), `exec.Command("git"`) {
			t.Errorf("%s startet git mit exec.Command, nutze proc.Command", p)
		}
		return nil
	})
	if err != nil {
		t.Fatal(err)
	}
}
