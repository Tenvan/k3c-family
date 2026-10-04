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
	"strings"

	"k3c/engine/conlog"
	"k3c/tools/k3c-dev/internal/console"
)

// Source ist der Name der Log-Datei (ohne .jsonl) und der Konsolen-Quelle.
const Source = "k3c-dev"

// LevelEnv ist die Umgebungsvariable für die Log-Stufe (debug|info|warn|error); gesetzt gilt sie für Datei und Konsole.
const LevelEnv = "K3C_DEV_LOG_LEVEL"

// MaxFileSize ist die Größe, ab der eine Log-Datei beim Öffnen rotiert wird.
const MaxFileSize = 10 << 20

// ParseLevel liest debug|info|warn|error (Groß-/Kleinschreibung egal); ok ist false bei allem anderen.
func ParseLevel(s string) (slog.Level, bool) {
	switch strings.ToLower(strings.TrimSpace(s)) {
	case "debug":
		return slog.LevelDebug, true
	case "info":
		return slog.LevelInfo, true
	case "warn", "warning":
		return slog.LevelWarn, true
	case "error":
		return slog.LevelError, true
	}
	return 0, false
}

// Levels liefert die Stufen für Datei und Konsolen-Spiegel: Standard debug und info, K3C_DEV_LOG_LEVEL setzt beide.
func Levels() (file, mirror slog.Level) {
	if l, ok := ParseLevel(os.Getenv(LevelEnv)); ok {
		return l, l
	}
	return slog.LevelDebug, slog.LevelInfo
}

// Rotate benennt path nach path.1 um (ein altes .1 wird überschrieben), wenn die Datei größer als max ist.
// Eine fehlende Datei ist kein Fehler.
func Rotate(path string, max int64) error {
	st, err := os.Stat(path)
	if err != nil || st.Size() <= max {
		return nil
	}
	_ = os.Remove(path + ".1")
	return os.Rename(path, path+".1")
}

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
	fileLevel, mirrorLevel := Levels()
	// Farben immer: die Oberfläche zeichnet sie, console_tail entfernt sie.
	handlers := []slog.Handler{conlog.New(mirror, mirrorLevel, true)}
	l := &Log{mirror: mirror}
	err := os.MkdirAll(logsDir, 0o755)
	if err == nil {
		path := filepath.Join(logsDir, Source+".jsonl")
		err = Rotate(path, MaxFileSize)
		if err == nil {
			l.file, err = os.OpenFile(path, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
		}
	}
	if err == nil {
		handlers = append(handlers, slog.NewJSONHandler(l.file, &slog.HandlerOptions{Level: fileLevel}))
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
