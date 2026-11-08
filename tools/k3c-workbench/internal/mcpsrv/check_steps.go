package mcpsrv

import (
	"context"
	"log/slog"
	"strconv"
	"strings"
	"sync"
	"time"
)

// B-739: Schritt-Protokoll und Sperre für schwere Läufe.
//
// task check:diff meldet seine Schritte als Markerzeilen `⏱▶ <name>` (Start) und `⏱■ <name> exit=<n>` (Ende). Das Zeitmessen
// übernimmt check_run, denn die Shell des Tasks hat nicht überall `date`. Jeder Schritt wird ein Log-Eintrag
// (ns=check, step, ms, exit), damit `logs_query` zeigt, wo ein Lauf Zeit braucht.

const (
	stepStart = "⏱▶ "
	stepEnd   = "⏱■ "
)

// stepLog verfolgt die Schritte eines Laufs. Schritte der parallelen Ketten überlappen, daher je Name.
type stepLog struct {
	log    *slog.Logger
	target string
	now    func() time.Time

	mu      sync.Mutex
	started map[string]time.Time
}

func newStepLog(log *slog.Logger, target string) *stepLog {
	return &stepLog{log: log, target: target, now: time.Now, started: map[string]time.Time{}}
}

// line wertet eine Ausgabezeile aus; Zeilen ohne Marker bleiben unberührt.
func (l *stepLog) line(text string) {
	text = strings.TrimSpace(text)
	switch {
	case strings.HasPrefix(text, stepStart):
		name := strings.TrimSpace(strings.TrimPrefix(text, stepStart))
		l.mu.Lock()
		l.started[name] = l.now()
		l.mu.Unlock()
		l.log.Info("🚦 schritt beginnt", "ns", "check", "target", l.target, "step", name)
	case strings.HasPrefix(text, stepEnd):
		name, exit := parseStepEnd(strings.TrimPrefix(text, stepEnd))
		l.mu.Lock()
		at, ok := l.started[name]
		delete(l.started, name)
		l.mu.Unlock()
		attrs := []any{"ns", "check", "target", l.target, "step", name, "exit", exit}
		if ok {
			attrs = append(attrs, "ms", l.now().Sub(at).Milliseconds())
		}
		if exit != 0 {
			l.log.Warn("❌ schritt beendet", attrs...)
			return
		}
		l.log.Info("✅ schritt beendet", attrs...)
	}
}

// parseStepEnd zerlegt `<name> exit=<n>`; ohne Zahl gilt exit -1.
func parseStepEnd(s string) (name string, exit int) {
	name, rest, _ := strings.Cut(strings.TrimSpace(s), " ")
	exit = -1
	if v, ok := strings.CutPrefix(strings.TrimSpace(rest), "exit="); ok {
		if n, err := strconv.Atoi(v); err == nil {
			exit = n
		}
	}
	return name, exit
}

// heavySlot lässt höchstens einen schweren Lauf (check:diff, check) gleichzeitig laufen, auch über Worktrees hinweg:
// Mehrere Sessions auf einem Rechner verdoppeln sonst die Laufzeit und lösen Zeitlimits aus (B-739).
var heavySlot = make(chan struct{}, 1)

func isHeavyTarget(name string) bool { return name == "task:check:diff" || name == "task:check" }

// acquireHeavy wartet auf den Slot (nur bei schweren Zielen) und gibt eine Freigabe zurück. Das Zeitlimit des Laufs
// beginnt erst danach.
func (s *Server) acquireHeavy(ctx context.Context, target, key string) (release func()) {
	if !isHeavyTarget(target) {
		return func() {}
	}
	waited := time.Now()
	select {
	case heavySlot <- struct{}{}:
	default:
		s.log.Info("⏳ wartet auf freien Prüf-Slot", "ns", "check", "target", key)
		select {
		case heavySlot <- struct{}{}:
			s.log.Info("🚦 Prüf-Slot frei", "ns", "check", "target", key, "ms", time.Since(waited).Milliseconds())
		case <-ctx.Done():
			return func() {}
		}
	}
	return func() { <-heavySlot }
}
