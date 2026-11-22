package console

import (
	"bytes"
	"strings"
	"sync"
)

// LineWriter zerlegt einen Ausgabestrom in Zeilen und gibt jede an emit weiter. Als Stdout/Stderr eines exec.Cmd
// begrenzt WaitDelay das Warten auf Pipes, die Enkelprozesse offen halten; lange Zeilen bleiben ganz, weil nichts
// eine feste Zeilenlänge annimmt.
type LineWriter struct {
	mu   sync.Mutex
	buf  []byte
	emit func(string)
}

// NewLineWriter legt einen Writer an, der jede vollständige Zeile an emit gibt.
func NewLineWriter(emit func(string)) *LineWriter { return &LineWriter{emit: emit} }

// Write nimmt beliebige Stücke an; nur vollständige Zeilen gehen weiter.
func (w *LineWriter) Write(p []byte) (int, error) {
	w.mu.Lock()
	defer w.mu.Unlock()
	w.buf = append(w.buf, p...)
	for {
		i := bytes.IndexByte(w.buf, '\n')
		if i < 0 {
			return len(p), nil
		}
		w.emit(clean(w.buf[:i]))
		w.buf = w.buf[i+1:]
	}
}

// Flush gibt eine angefangene letzte Zeile weiter.
func (w *LineWriter) Flush() {
	w.mu.Lock()
	defer w.mu.Unlock()
	if len(w.buf) > 0 {
		w.emit(clean(w.buf))
		w.buf = nil
	}
}

// clean entfernt das Zeilenende und macht den Text zu gültigem UTF-8 (Windows-Werkzeuge schreiben teils Codepage).
func clean(b []byte) string {
	return strings.ToValidUTF8(strings.TrimRight(string(b), "\r"), "�")
}
