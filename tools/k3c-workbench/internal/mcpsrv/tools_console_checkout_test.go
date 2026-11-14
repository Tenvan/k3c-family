package mcpsrv

import (
	"context"
	"strings"
	"testing"
	"time"

	"k3c/tools/k3c-workbench/internal/console"
)

// B-855/AC-01: Zwei Checkouts fahren task:check:go; console_tail liefert je Checkout nur den eigenen Lauf, die
// Kopfzeile nennt Checkout und Startzeit, ohne Lauf kommt eine klare Meldung statt fremder Ausgabe.
func TestConsoleTailJeCheckout(t *testing.T) {
	t.Parallel()
	store := console.New(0, nil)
	root := t.TempDir()
	s := New(Config{Version: "test", Root: root, Console: store})
	wt := workspace{root: t.TempDir(), name: "wt-a"}
	ctxRoot := context.Background()
	ctxWt := context.WithValue(ctxRoot, wsKey{}, wt)

	store.Add("check:task:check:go", "stdout", "lauf der Wurzel")
	store.Add("check:wt-a/task:check:go", "stdout", "lauf des Worktrees")
	s.checks.end("wt-a/task:check:go", runResult{at: time.Date(2026, 11, 14, 9, 30, 15, 0, time.Local)})

	own, err := s.consoleTail(ctxWt, tailIn{Source: "check:task:check:go"})
	if err != nil || !strings.Contains(own, "lauf des Worktrees") || strings.Contains(own, "lauf der Wurzel") {
		t.Fatalf("Worktree: %q %v", own, err)
	}
	if head := strings.SplitN(own, "\n", 2)[0]; !strings.Contains(head, "Checkout Worktree wt-a") || !strings.Contains(head, "Lauf ab 09:30:15") {
		t.Errorf("Kopfzeile: %q", head)
	}
	other, err := s.consoleTail(ctxRoot, tailIn{Source: "check:task:check:go"})
	if err != nil || !strings.Contains(other, "lauf der Wurzel") || strings.Contains(other, "lauf des Worktrees") {
		t.Fatalf("Wurzel: %q %v", other, err)
	}
	if _, err := s.consoleTail(ctxWt, tailIn{Source: "check:task:check"}); err == nil || !strings.Contains(err.Error(), "kein Lauf von check:task:check im Checkout Worktree wt-a") {
		t.Errorf("ohne Lauf: %v", err)
	}
	if !takesCheckout("console_tail") {
		t.Error("console_tail nimmt kein checkout")
	}
}
