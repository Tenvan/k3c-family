package main

import (
	"context"
	"io"
	"log/slog"
	"os"
	"path/filepath"
)

// logFile ist die JSON-Log-Datei im Log-Ordner (B-066); k3c-dev liest sie als Quelle „k3c-server“.
const logFile = "k3c-server.jsonl"

// logDir bestimmt den Ordner des JSON-Logs: K3C_LOG_DIR, sonst ein schon vorhandener Ordner logs/ im Arbeitsverzeichnis
// (Start aus dem Repo). Leer heißt: kein JSON-Log (Docker, Pi), das Text-Log auf stderr bleibt.
func logDir() string {
	if dir := os.Getenv("K3C_LOG_DIR"); dir != "" {
		return dir
	}
	if info, err := os.Stat("logs"); err == nil && info.IsDir() {
		return "logs"
	}
	return ""
}

// newLogger schreibt Text nach stderr und, wenn ein Ordner bestimmt ist und sich öffnen lässt, JSON nach <Ordner>/k3c-server.jsonl.
// Lässt sich die Datei nicht öffnen, bleibt es bei stderr und eine Warnung sagt warum. Die Funktion schließt die Datei.
func newLogger(dir string, stderr io.Writer) (*slog.Logger, func()) {
	text := slog.NewTextHandler(stderr, nil)
	if dir == "" {
		return slog.New(text), func() {}
	}
	f, err := os.OpenFile(filepath.Join(dir, logFile), os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		log := slog.New(text)
		log.Warn("JSON-Log nicht geschrieben, nur stderr", "dir", dir, "err", err)
		return log, func() {}
	}
	return slog.New(fanout{text, slog.NewJSONHandler(f, nil)}), func() { _ = f.Close() }
}

// fanout gibt jeden Eintrag an beide Handler; ein Handler, der den Level nicht mag, schadet dem anderen nicht.
type fanout [2]slog.Handler

func (f fanout) Enabled(ctx context.Context, l slog.Level) bool {
	return f[0].Enabled(ctx, l) || f[1].Enabled(ctx, l)
}

func (f fanout) Handle(ctx context.Context, r slog.Record) error {
	var first error
	for _, h := range f {
		if h.Enabled(ctx, r.Level) {
			if err := h.Handle(ctx, r.Clone()); err != nil && first == nil {
				first = err
			}
		}
	}
	return first
}

func (f fanout) WithAttrs(attrs []slog.Attr) slog.Handler {
	return fanout{f[0].WithAttrs(attrs), f[1].WithAttrs(attrs)}
}

func (f fanout) WithGroup(name string) slog.Handler {
	return fanout{f[0].WithGroup(name), f[1].WithGroup(name)}
}
