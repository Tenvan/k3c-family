package mcpsrv

import (
	"bytes"
	"context"
	"log/slog"
	"strings"
	"sync"
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

// lockedBuf: Log-Puffer, den Test und wartende Goroutine gleichzeitig benutzen.
type lockedBuf struct {
	mu  sync.Mutex
	buf bytes.Buffer
}

func (b *lockedBuf) Write(p []byte) (int, error) { b.mu.Lock(); defer b.mu.Unlock(); return b.buf.Write(p) }
func (b *lockedBuf) String() string               { b.mu.Lock(); defer b.mu.Unlock(); return b.buf.String() }

// B-739/AC-03: Zwei schwere Läufe aus verschiedenen Checkouts laufen nacheinander, der zweite loggt das Warten.
func TestAcquireHeavyNacheinander(t *testing.T) {
	var buf lockedBuf
	s := &Server{log: slog.New(slog.NewTextHandler(&buf, nil))}
	ctx := context.Background()
	release := s.acquireHeavy(ctx, "task:check:diff", "wurzel")
	got := make(chan func(), 1)
	go func() { got <- s.acquireHeavy(ctx, "task:check:diff", "worktree") }()
	for deadline := time.Now().Add(5 * time.Second); !strings.Contains(buf.String(), "wartet auf freien Prüf-Slot"); {
		if time.Now().After(deadline) {
			t.Fatalf("zweiter Lauf wartet nicht:\n%s", buf.String())
		}
		time.Sleep(5 * time.Millisecond)
	}
	select {
	case <-got:
		t.Fatal("zweiter Lauf bekam den Slot, obwohl der erste ihn hält")
	default:
	}
	s.acquireHeavy(ctx, "task:test", "leicht")() // leichte Ziele warten nie
	release()
	select {
	case r := <-got:
		r()
	case <-time.After(5 * time.Second):
		t.Fatal("zweiter Lauf bekommt den Slot nach der Freigabe nicht")
	}
	if out := buf.String(); !strings.Contains(out, "target=worktree") || !strings.Contains(out, "Prüf-Slot frei") {
		t.Errorf("Log unvollständig:\n%s", out)
	}
}

func TestAcquireHeavyAbbruch(t *testing.T) {
	s := &Server{log: slog.New(slog.NewTextHandler(&bytes.Buffer{}, nil))}
	release := s.acquireHeavy(context.Background(), "task:check", "erster")
	defer release()
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	s.acquireHeavy(ctx, "task:check", "abgebrochen")() // kehrt ohne Slot zurück und gibt nichts frei
	select {
	case heavySlot <- struct{}{}:
		t.Fatal("Slot frei, obwohl der erste Lauf ihn hält")
	default:
	}
}
