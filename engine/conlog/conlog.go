// Package conlog ist der gemeinsame Konsolen-Handler für slog (k3c-server, k3c-dev): eine Zeile je Eintrag,
//
//	17:03:11.042 ERROR [svc] Dienst fehlgeschlagen port=8080 err="bereits belegt"
//
// mit ANSI-Farben je Level statt des key=value-Formats von slog.TextHandler. Die Level-Wörter (WARN, ERROR) bleiben
// im Text, damit Leser ohne Farben (k3c-dev markOf, outLevel) sie weiter finden. Die JSON-Dateien bleiben unverändert.
package conlog

import (
	"context"
	"io"
	"log/slog"
	"os"
	"strconv"
	"strings"
	"sync"
	"unicode"
)

const (
	reset  = "\x1b[0m"
	bold   = "\x1b[1m"
	dim    = "\x1b[2m"
	red    = "\x1b[31m"
	yellow = "\x1b[33m"
	blue   = "\x1b[34m"
	purple = "\x1b[35m"
	cyan   = "\x1b[36m"
	gray   = "\x1b[90m"
)

// Topics ordnet jedem Namespace ("ns") ein Emoji zu, das die Konsole vor [ns] setzt: Themen auf einen Blick.
// Nur Emojis aus einem Codepunkt (ohne U+FE0F), sonst verrutscht die Breite in manchen Terminals. Neuer ns → hier eintragen.
var Topics = map[string]string{
	"main": "🚀", "http": "📡", "ws": "🔌", "room": "🏰", "client": "🌐", "save": "💾", "store": "📦",
	"report": "📊", "diag": "🩺", "svc": "🔧", "check": "🧪", "tasks": "📋", "mcp": "🤖", "usage": "📈",
}

// Color sagt, ob die Konsole Farben bekommt: aus, sobald NO_COLOR gesetzt ist (https://no-color.org).
func Color() bool { return os.Getenv("NO_COLOR") == "" }

// Handler schreibt lesbare, farbige Zeilen. WithAttrs/WithGroup teilen sich Writer und Sperre mit dem Ursprung.
type Handler struct {
	mu     *sync.Mutex
	w      io.Writer
	level  slog.Leveler
	color  bool
	ns     string // Attribut "ns" aus WithAttrs, erscheint als [ns] vor der Meldung
	attrs  string // vorformatierte Attribute aus WithAttrs
	prefix string // Gruppenpfad aus WithGroup, z. B. "req."
}

// New liefert den Handler; level nil heißt Info.
func New(w io.Writer, level slog.Leveler, color bool) *Handler {
	if level == nil {
		level = slog.LevelInfo
	}
	return &Handler{mu: &sync.Mutex{}, w: w, level: level, color: color}
}

func (h *Handler) Enabled(_ context.Context, l slog.Level) bool { return l >= h.level.Level() }

func (h *Handler) Handle(_ context.Context, r slog.Record) error {
	var b strings.Builder
	if !r.Time.IsZero() {
		h.paint(&b, gray, r.Time.Format("15:04:05.000"))
		b.WriteByte(' ')
	}
	lc, name := levelStyle(r.Level)
	h.paint(&b, lc, name)
	ns, attrs := h.ns, h.attrs
	var tail strings.Builder
	r.Attrs(func(a slog.Attr) bool {
		if h.prefix == "" && a.Key == "ns" {
			ns = a.Value.String()
		} else {
			h.attr(&tail, h.prefix, a)
		}
		return true
	})
	if ns != "" {
		b.WriteByte(' ')
		if e, ok := Topics[ns]; ok {
			b.WriteString(e + " ")
		}
		h.paint(&b, cyan, "["+ns+"]")
	}
	b.WriteByte(' ')
	msgColor := bold
	if r.Level >= slog.LevelError {
		msgColor = bold + red
	}
	h.paint(&b, msgColor, r.Message)
	b.WriteString(attrs + tail.String()) // jedes Attribut bringt sein Leerzeichen mit
	b.WriteByte('\n')
	h.mu.Lock()
	defer h.mu.Unlock()
	_, err := io.WriteString(h.w, b.String())
	return err
}

func (h *Handler) WithAttrs(as []slog.Attr) slog.Handler {
	c := *h
	var b strings.Builder
	for _, a := range as {
		if h.prefix == "" && a.Key == "ns" {
			c.ns = a.Value.String()
		} else {
			h.attr(&b, h.prefix, a)
		}
	}
	c.attrs += b.String()
	return &c
}

func (h *Handler) WithGroup(name string) slog.Handler {
	if name == "" {
		return h
	}
	c := *h
	c.prefix += name + "."
	return &c
}

// attr schreibt " key=value" (Gruppen aufgelöst zu key.sub=value); leere Attribute fallen weg wie bei slog.
func (h *Handler) attr(b *strings.Builder, prefix string, a slog.Attr) {
	a.Value = a.Value.Resolve()
	if a.Equal(slog.Attr{}) {
		return
	}
	if a.Value.Kind() == slog.KindGroup {
		if a.Key != "" {
			prefix += a.Key + "."
		}
		for _, g := range a.Value.Group() {
			h.attr(b, prefix, g)
		}
		return
	}
	b.WriteByte(' ')
	h.paint(b, gray, prefix+a.Key+"=")
	val := quote(a.Value.String())
	if a.Key == "err" || a.Key == "error" {
		h.paint(b, red, val)
	} else {
		b.WriteString(val)
	}
}

func (h *Handler) paint(b *strings.Builder, code, s string) {
	if h.color {
		b.WriteString(code + s + reset)
		return
	}
	b.WriteString(s)
}

// levelStyle liefert Farbe und auf 5 Zeichen aufgefüllten Namen (DEBUG, INFO, WARN, ERROR, Zwischenstufen wie INFO+2).
func levelStyle(l slog.Level) (string, string) {
	name := l.String()
	if len(name) < 5 {
		name += strings.Repeat(" ", 5-len(name))
	}
	switch {
	case l >= slog.LevelError:
		return bold + red, name
	case l >= slog.LevelWarn:
		return bold + yellow, name
	case l >= slog.LevelInfo:
		return blue, name
	}
	return purple, name
}

// quote setzt Werte mit Leer-, Gleich-, Anführungs- oder Steuerzeichen in Anführungszeichen (wie slog.TextHandler).
func quote(s string) string {
	if s == "" {
		return `""`
	}
	for _, r := range s {
		if r == '=' || r == '"' || unicode.IsSpace(r) || !unicode.IsPrint(r) {
			return strconv.Quote(s)
		}
	}
	return s
}
