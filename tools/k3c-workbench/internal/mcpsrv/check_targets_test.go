package mcpsrv

import (
	"context"
	"strings"
	"testing"
	"time"
)

// B-788/AC-01: check_run nimmt die Teilaufgaben von check:diff und meldet Exit-Code und Dauer.
func TestCheckRunTeilaufgabenCheckDiff(t *testing.T) {
	t.Parallel()
	cases := []struct {
		target, cmd string
		exit        int
	}{
		{"task:check:diff:plan", "task check:diff:plan", 0},
		{"task:check:diff:js", "task check:diff:js", 1},
		{"task:check:diff:go", "task check:diff:go", 0},
	}
	for _, c := range cases {
		var got []string
		s, _ := fakeServer(t, func(spec runSpec) runResult {
			got = spec.args
			return runResult{exit: c.exit, dur: 2300 * time.Millisecond}
		})
		text, err := s.checkRun(context.Background(), checkIn{Target: c.target})
		if err != nil {
			t.Fatalf("%s: %v", c.target, err)
		}
		if strings.Join(got, " ") != c.cmd {
			t.Errorf("%s: Befehl %v", c.target, got)
		}
		head := strings.SplitN(text, "\n", 2)[0]
		want := c.target + " · exit " + string(rune('0'+c.exit)) + " · 2,3 s"
		if !strings.HasPrefix(head, want) {
			t.Errorf("%s: Antwort %q, will Anfang %q", c.target, head, want)
		}
	}
	for _, s := range []string{"Basis HEAD", "zwei Stufen", "task:check:diff:plan", "task:check:diff:js", "task:check:diff:go"} {
		if !strings.Contains(checkRunDescription, s) {
			t.Errorf("Beschreibung von check_run nennt %q nicht", s)
		}
	}
	if strings.Contains(checkRunDescription, "origin/develop") {
		t.Error("Beschreibung nennt noch origin/develop")
	}
	if _, err := findTarget("task:check:diff:rust"); err == nil || !strings.Contains(err.Error(), "task:check:diff:go") {
		t.Errorf("Ablehnung nennt die Teilaufgaben nicht: %v", err)
	}
}
