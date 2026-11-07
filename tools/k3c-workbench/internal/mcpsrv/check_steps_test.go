package mcpsrv

import (
	"bytes"
	"log/slog"
	"strings"
	"testing"
	"time"
)

func TestStepLogZeitUndExit(t *testing.T) {
	var buf bytes.Buffer
	log := slog.New(slog.NewTextHandler(&buf, nil))
	l := newStepLog(log, "task:check:diff")
	now := time.Unix(1000, 0)
	l.now = func() time.Time { return now }
	l.line("⏱▶ go-test")
	now = now.Add(2500 * time.Millisecond)
	l.line("irgendeine Ausgabe")
	l.line("⏱■ go-test exit=0")
	l.line("⏱▶ lint")
	now = now.Add(time.Second)
	l.line("⏱■ lint exit=3")
	out := buf.String()
	for _, w := range []string{"step=go-test", "ms=2500", "exit=0", "step=lint", "ms=1000", "exit=3", "level=WARN"} {
		if !strings.Contains(out, w) {
			t.Errorf("Log ohne %q:\n%s", w, out)
		}
	}
}

func TestStepLogEndeOhneStart(t *testing.T) {
	var buf bytes.Buffer
	l := newStepLog(slog.New(slog.NewTextHandler(&buf, nil)), "x")
	l.line("⏱■ ohne-start exit=1")
	if strings.Contains(buf.String(), "ms=") || !strings.Contains(buf.String(), "step=ohne-start") {
		t.Errorf("unerwartet: %s", buf.String())
	}
}

func TestParseStepEnd(t *testing.T) {
	if n, e := parseStepEnd("grafik-golden exit=7"); n != "grafik-golden" || e != 7 {
		t.Errorf("%q %d", n, e)
	}
	if _, e := parseStepEnd("x"); e != -1 {
		t.Errorf("ohne exit: %d", e)
	}
}

func TestIsHeavyTarget(t *testing.T) {
	if !isHeavyTarget("task:check:diff") || isHeavyTarget("task:test") {
		t.Error("schwere Ziele falsch erkannt")
	}
}
