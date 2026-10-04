package services

import (
	"encoding/json"
	"log/slog"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"sync"
	"time"

	"k3c/tools/k3c-dev/internal/applog"
)

// ansi sind Escape-Sequenzen (Farben, Cursor), die nicht ins Log gehören.
var ansi = regexp.MustCompile(`\x1b\[[0-9;?]*[ -/]*[@-~]`)

// outLevel schätzt das Level einer Ausgabezeile: error/fehler/fatal -> ERROR, warn -> WARN, sonst INFO.
func outLevel(text string) string {
	low := strings.ToLower(text)
	switch {
	case strings.Contains(low, "error") || strings.Contains(low, "fehler") || strings.Contains(low, "fatal"):
		return "ERROR"
	case strings.Contains(low, "warn"):
		return "WARN"
	}
	return "INFO"
}

// outSink schreibt die Ausgabe eines Dienstes als JSON-Zeilen nach logs/<name>.jsonl (Format von internal/logs).
// Fehler sind nie fatal: der erste Schreibfehler geht einmal als Warn ins eigene Log, danach schweigt die Datei.
type outSink struct {
	mu     sync.Mutex
	f      *os.File
	name   string
	log    *slog.Logger
	failed bool
	now    func() time.Time
}

// openOutSink öffnet logs/<dienstname klein>.jsonl unterhalb von root (nach Rotation ab 10 MB). Ohne root oder bei
// einem Fehler gibt es keine Datei: write und close tun dann nichts.
func openOutSink(root, service string, log *slog.Logger, now func() time.Time) *outSink {
	s := &outSink{name: service, log: log, now: now}
	if root == "" {
		return s
	}
	dir := filepath.Join(root, "logs")
	path := filepath.Join(dir, strings.ToLower(service)+".jsonl")
	err := os.MkdirAll(dir, 0o755)
	if err == nil {
		err = applog.Rotate(path, applog.MaxFileSize)
	}
	if err == nil {
		s.f, err = os.OpenFile(path, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	}
	if err != nil {
		log.Warn("📄 dienst "+service+": Ausgabe-Log nicht geöffnet: "+err.Error(), "ns", "svc", "datei", path)
	}
	return s
}

func (s *outSink) write(stream, text string) {
	if s == nil || s.f == nil {
		return
	}
	text = ansi.ReplaceAllString(text, "")
	line, _ := json.Marshal(map[string]string{"time": s.now().Format(time.RFC3339Nano), "level": outLevel(text),
		"msg": text, "ns": "out", "stream": stream})
	s.mu.Lock()
	defer s.mu.Unlock()
	if _, err := s.f.Write(append(line, '\n')); err != nil && !s.failed {
		s.failed = true
		s.log.Warn("📄 dienst "+s.name+": Ausgabe-Log nicht beschreibbar: "+err.Error(), "ns", "svc")
	}
}

func (s *outSink) close() {
	if s == nil || s.f == nil {
		return
	}
	s.mu.Lock()
	defer s.mu.Unlock()
	_ = s.f.Close()
	s.f = nil
}
