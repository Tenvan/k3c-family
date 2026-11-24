package hubplan

import (
	"context"
	"strings"
	"testing"

	"k3c/tools/k3c-studio/internal/gameexe"
)

// RU34.3/AC-03: Exe rechnet über die Spiel-EXE des Checkouts; ohne EXE nennt der Fehler den Bau-Befehl, kein Rückfall.
func TestExeOhneSpielExe(t *testing.T) {
	_, err := Exe(t.TempDir())(context.Background(), []byte(`{"op":"plan"}`))
	if err == nil || !strings.Contains(err.Error(), gameexe.BuildCommand) {
		t.Errorf("Exe ohne EXE: %v", err)
	}
}
