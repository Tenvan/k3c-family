package view

import (
	"os"
	"strings"
	"testing"
)

// Die Kronen-Zustände stehen in docs/protocol.md (Zeile players[].crown); weicht die Engine ab, ändert sich die Doku mit.
func TestCrownStatesMatchProtocol(t *testing.T) {
	raw, err := os.ReadFile("../../../docs/protocol.md")
	if err != nil {
		t.Skipf("Protokoll fehlt: %v", err)
	}
	var row string
	for _, l := range strings.Split(string(raw), "\n") {
		if strings.HasPrefix(l, "| `players[].crown`") {
			row = l
		}
	}
	if row == "" {
		t.Fatal("Zeile players[].crown fehlt im Protokoll")
	}
	for _, s := range []string{CrownFlying, CrownGround, CrownEnemy, CrownPlayer} {
		if !strings.Contains(row, "`"+s+"`") {
			t.Errorf("Zustand %q steht nicht im Protokoll", s)
		}
	}
}
