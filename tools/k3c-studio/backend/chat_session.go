package backend

import (
	"context"
	"crypto/rand"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// Sitzungen der CLI (B-687): Je Chat merkt der Verlauf des Studios eine Sitzungs-ID (ChatLog.Session). Folgenachrichten
// setzen die Sitzung mit --resume fort und schicken nur die neue Nachricht; der Seitenkontext geht weiter bei jedem Aufruf
// frisch als Instruktion mit (--append-system-prompt-file) und steht damit nie veraltet im Verlauf der Sitzung. Fehlt die
// Sitzungsdatei oder scheitert das Fortsetzen ohne Antwort, geht der Verlauf als Text in einer neuen Sitzung mit, ohne
// Fehler an der Oberfläche; kennt die CLI die Argumente nicht, bleibt es für diesen Lauf beim Verlauf als Text.
// Ablage der Sitzungsdateien: <Profil der CLI>/projects/<Arbeitsordner>/<id>.jsonl (Profil: CLAUDE_CONFIG_DIR, sonst
// ~/.claude); sie werden beim Löschen eines Chats und beim Aufräumen des Verlaufs mit entfernt.

// chatSessionPattern ist eine UUID: die ID aus .chats wird so nie zum Pfad.
var chatSessionPattern = regexp.MustCompile(`^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$`)

// claudeSessionRoot ist der Ordner der Sitzungsdateien der CLI ("" ohne Heimatordner).
func claudeSessionRoot() string {
	dir := os.Getenv("CLAUDE_CONFIG_DIR")
	if dir == "" {
		home, err := os.UserHomeDir()
		if err != nil {
			return ""
		}
		dir = filepath.Join(home, ".claude")
	}
	return filepath.Join(dir, "projects")
}

// newSessionID ist eine zufällige UUID (Version 4).
func newSessionID() string {
	var b [16]byte
	_, _ = rand.Read(b[:])
	b[6] = b[6]&0x0f | 0x40
	b[8] = b[8]&0x3f | 0x80
	return fmt.Sprintf("%x-%x-%x-%x-%x", b[0:4], b[4:6], b[6:8], b[8:10], b[10:16])
}

// sessionFile sucht die Sitzungsdatei id ("" wenn sie fehlt); der Unterordner je Arbeitsordner wird nicht nachgebaut.
func (c chatCLI) sessionFile(id string) string {
	if c.sessions == "" || !chatSessionPattern.MatchString(id) {
		return ""
	}
	m, _ := filepath.Glob(filepath.Join(c.sessions, "*", id+".jsonl"))
	if len(m) == 0 {
		return ""
	}
	return m[0]
}

// removeSession löscht die Sitzungsdatei id, falls es sie gibt.
func (c chatCLI) removeSession(id string) {
	f := c.sessionFile(id)
	if f == "" {
		return
	}
	if err := os.Remove(f); err != nil {
		slog.Warn("❌ Chat: Sitzungsdatei nicht gelöscht", "ns", "main", "datei", f, "fehler", err)
		return
	}
	slog.Info("🧹 Chat: Sitzungsdatei gelöscht", "ns", "main", "sitzung", id)
}

// failedSilently: die CLI scheiterte, bevor ein Wort der Antwort kam (Rückfall ohne doppelten Text möglich).
func failedSilently(out chatOutcome) bool { return out.Status == chatFailed && out.Text == "" }

// unknownOption: die CLI kennt ein Argument nicht (alte Version).
func unknownOption(out chatOutcome) bool { return strings.Contains(strings.ToLower(out.Error), "unknown option") }

// ask schickt text an die CLI und liefert die Antwort und die Sitzung, in der sie jetzt steht ("" ohne Sitzung): mit
// vorhandener Sitzung session nur die neue Nachricht (Resume), sonst den Verlauf history als Text in einer neuen Sitzung.
func (h *chatHub) ask(ctx context.Context, session string, history []ChatMessage, text string, opts chatOptions,
	emit func(string)) (chatOutcome, string) {
	if h.noResume.Load() || h.cli.sessions == "" {
		return h.plain(ctx, history, text, opts, emit), ""
	}
	if h.cli.sessionFile(session) != "" {
		opts.session, opts.resume = session, true
		out := h.cli.run(ctx, text, opts, emit)
		if !failedSilently(out) {
			return out, session
		}
		slog.Info("🩹 Chat: Sitzung nicht fortgesetzt, Verlauf als Text", "ns", "main", "sitzung", session, "fehler", out.Error)
		h.cli.removeSession(session)
		if unknownOption(out) {
			return h.withoutSessions(ctx, history, text, opts, emit), ""
		}
	}
	opts.session, opts.resume = newSessionID(), false
	out := h.cli.run(ctx, chatPrompt(history, text), opts, emit)
	if failedSilently(out) && unknownOption(out) {
		return h.withoutSessions(ctx, history, text, opts, emit), ""
	}
	return out, opts.session
}

// plain ist der Aufruf ohne Sitzung mit dem Verlauf als Text (wie vor B-687).
func (h *chatHub) plain(ctx context.Context, history []ChatMessage, text string, opts chatOptions, emit func(string)) chatOutcome {
	opts.session, opts.resume = "", false
	return h.cli.run(ctx, chatPrompt(history, text), opts, emit)
}

// withoutSessions schaltet die Sitzungen für diesen Lauf ab (CLI zu alt) und fragt ohne Sitzung.
func (h *chatHub) withoutSessions(ctx context.Context, history []ChatMessage, text string, opts chatOptions, emit func(string)) chatOutcome {
	if !h.noResume.Swap(true) {
		slog.Warn("🚫 Chat: Claude Code kennt keine Sitzungen, Verlauf geht dauerhaft als Text mit", "ns", "main")
	}
	return h.plain(ctx, history, text, opts, emit)
}

// keepSession merkt die Sitzung der Antwort im Verlauf c: eine fertige Antwort (oder der Rückfall ohne Sitzung) ersetzt
// die alte Sitzung, deren Datei dann entfernt wird; eine neue Sitzung ohne fertige Antwort wird verworfen.
func (h *chatHub) keepSession(c *ChatLog, session string, out chatOutcome) {
	switch {
	case session == c.Session:
	case out.Status == chatDone || session == "":
		h.cli.removeSession(c.Session)
		c.Session = session
	default:
		h.cli.removeSession(session)
	}
}
