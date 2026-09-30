// Package applog ist das eigene Log von k3c-dev (B-046): JSON nach logs/k3c-dev.jsonl im Format, das
// internal/logs liest, und dieselben Einträge lesbar als Strom "log" im Konsolenpuffer k3c-dev.
package applog

import (
	"context"
	"errors"
	"io"
	"log/slog"
	"os"
	"path/filepath"

	"k3c/tools/k3c-dev/internal/console"
)

// Source ist der Name der Log-Datei (ohne .jsonl) und der Konsolen-Quelle.
const Source = "k3c-dev"

// Log hält den Logger und die offene Datei.
type Log struct {
	*slog.Logger
	file   *os.File
	mirror *console.LineWriter
}

// Open öffnet logs/k3c-dev.jsonl unter logsDir zum Anhängen. Lässt sich die Datei nicht öffnen, schreibt der Logger
// nur in den Konsolenpuffer, und der Fehler kommt zurück, damit der Aufrufer ihn melden kann.
func Open(logsDir string, store *console.Store) (*Log, error) {
	mirror := console.NewLineWriter(func(text string) { store.Add(Source, "log", text) })
	opts := &slog.HandlerOptions{Level: slog.LevelDebug}
	handlers := []slog.Handler{slog.NewTextHandler(mirror, opts)}
	l := &Log{mirror: mirror}
	err := os.MkdirAll(logsDir, 0o755)
	if err == nil {
		l.file, err = os.OpenFile(filepath.Join(logsDir, Source+".jsonl"), os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	}
	if err == nil {
		handlers = append(handlers, slog.NewJSONHandler(l.file, opts))
	}
	l.Logger = slog.New(fanout(handlers))
	return l, err
}

// Close schließt die Datei.
func (l *Log) Close() error {
	l.mirror.Flush()
	if l.file == nil {
		return nil
	}
	return l.file.Close()
}

// Discard ist ein Logger, der nichts schreibt (Tests, Server ohne Log).
func Discard() *slog.Logger { return slog.New(slog.NewTextHandler(io.Discard, nil)) }

// fanout gibt jeden Eintrag an alle Handler weiter.
type fanout []slog.Handler

func (f fanout) Enabled(ctx context.Context, level slog.Level) bool {
	for _, h := range f {
		if h.Enabled(ctx, level) {
			return true
		}
	}
	return false
}

func (f fanout) Handle(ctx context.Context, r slog.Record) error {
	var errs []error
	for _, h := range f {
		if h.Enabled(ctx, r.Level) {
			errs = append(errs, h.Handle(ctx, r.Clone()))
		}
	}
	return errors.Join(errs...)
}

func (f fanout) WithAttrs(attrs []slog.Attr) slog.Handler {
	out := make(fanout, len(f))
	for i, h := range f {
		out[i] = h.WithAttrs(attrs)
	}
	return out
}

func (f fanout) WithGroup(name string) slog.Handler {
	out := make(fanout, len(f))
	for i, h := range f {
		out[i] = h.WithGroup(name)
	}
	return out
}
