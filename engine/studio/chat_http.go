package studio

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"
)

// Chat über HTTP (B-488, WZ19.1): POST /api/studio/chat startet die CLI und antwortet als Strom (text/event-stream)
// mit den Ereignissen start {chat}, chunk {text} … und done {status, error, notice}. Der Strom ist die Antwort auf
// den POST, damit die Kopfzeile X-K3C-Studio (guard) auch den Start schützt. Abbruch: Verbindung schließen oder
// ChatCancel(id). Alle Chat-Adressen bedienen nur Loopback, auch mit K3C_STUDIO_LAN=1.

const chatLoopbackMsg = "Der Chat ist nur auf dem Rechner des Servers erreichbar"

// chatHistoryMax begrenzt die Nachrichten des Verlaufs, die mit in den Prompt gehen (die CLI kennt keinen Verlauf).
const chatHistoryMax = 20

// chatHub hält CLI, Verlauf und die laufenden Antworten (je Chat höchstens eine).
type chatHub struct {
	cli     chatCLI
	store   chatStore
	mu      sync.Mutex
	running map[string]context.CancelFunc
}

func newChatHub(root string) *chatHub {
	return &chatHub{cli: defaultCLI(), store: chatStore{dir: filepath.Join(root, chatDirName), now: time.Now},
		running: map[string]context.CancelFunc{}}
}

// chats liefert den Chat des Studios (Tests setzen a.chat vorher).
func (a *App) chats() *chatHub {
	a.chatOnce.Do(func() {
		if a.chat == nil {
			a.chat = newChatHub(a.root)
		}
	})
	return a.chat
}

// chatCleanup räumt beim Start den Verlauf auf; ein Fehler hält das Studio nicht an.
func (a *App) chatCleanup() {
	n, err := a.chats().store.cleanup()
	if err != nil {
		slog.Warn("❌ Chat-Verlauf nicht aufgeräumt", "ns", "main", "fehler", err)
	}
	if n > 0 {
		slog.Info("🧹 Chat-Verlauf aufgeräumt", "ns", "main", "geloescht", n)
	}
}

// begin sperrt den Chat id für eine Antwort; sie endet mit der Anfrage (req) oder dem Server (server).
func (h *chatHub) begin(server, req context.Context, id string) (context.Context, func(), bool) {
	h.mu.Lock()
	defer h.mu.Unlock()
	if _, busy := h.running[id]; busy {
		return nil, nil, false
	}
	ctx, cancel := context.WithCancel(req)
	stop := context.AfterFunc(server, cancel) // Ende des Servers beendet auch die CLI
	h.running[id] = cancel
	return ctx, func() { stop(); h.mu.Lock(); delete(h.running, id); h.mu.Unlock(); cancel() }, true
}

func (h *chatHub) cancel(id string) bool {
	h.mu.Lock()
	defer h.mu.Unlock()
	stop, ok := h.running[id]
	if ok {
		stop()
	}
	return ok
}

func (h *chatHub) isRunning(id string) bool {
	h.mu.Lock()
	defer h.mu.Unlock()
	_, ok := h.running[id]
	return ok
}

// open lädt den Chat id; fehlt er, beginnt er leer. Nur unter der Sperre aus begin aufrufen.
func (h *chatHub) open(id string) (ChatLog, error) {
	c, err := h.store.load(id)
	if errors.Is(err, os.ErrNotExist) {
		return ChatLog{ID: id}, nil
	}
	return c, err
}

// chatEnd ist das Ereignis done.
type chatEnd struct {
	Status string `json:"status"`
	Error  string `json:"error,omitempty"`
	Notice string `json:"notice,omitempty"` // z. B. Verlauf nicht gespeichert
}

type chatRequest struct {
	Chat string `json:"chat"`
	Text string `json:"text"`
}

func readChatRequest(r *http.Request) (chatRequest, error) {
	var req chatRequest
	if err := json.NewDecoder(io.LimitReader(r.Body, maxRPCBytes)).Decode(&req); err != nil {
		return req, errors.New("ungültige Anfrage: " + err.Error())
	}
	if strings.TrimSpace(req.Text) == "" {
		return req, errors.New("die Nachricht ist leer")
	}
	if req.Chat != "" && !chatIDPattern.MatchString(req.Chat) {
		return req, errors.New("ungültige Chat-id (nur Kleinbuchstaben, Ziffern, Bindestrich)")
	}
	return req, nil
}

// chatSend beantwortet POST /api/studio/chat als Strom.
func (a *App) chatSend(w http.ResponseWriter, r *http.Request) {
	if !isLoopback(r.RemoteAddr) {
		fail(w, http.StatusForbidden, chatLoopbackMsg)
		return
	}
	req, err := readChatRequest(r)
	if err != nil {
		fail(w, http.StatusBadRequest, err.Error())
		return
	}
	fl, ok := w.(http.Flusher)
	if !ok {
		fail(w, http.StatusInternalServerError, "kein Strom möglich")
		return
	}
	h := a.chats()
	id := req.Chat // readChatRequest hat die id geprüft
	if id == "" {
		id = newChatID(h.store.now())
	}
	ctx, release, ok := h.begin(a.context(), r.Context(), id) // erst sperren, dann laden: kein Überschreiben
	if !ok {
		fail(w, http.StatusConflict, "Für diesen Chat läuft noch eine Antwort")
		return
	}
	defer release()
	log, err := h.open(id)
	if err != nil {
		fail(w, http.StatusUnprocessableEntity, err.Error())
		return
	}
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-store")
	sendEvent(w, fl, "start", map[string]string{"chat": log.ID})
	asked := h.store.now()
	out := h.cli.run(ctx, chatPrompt(log.Messages, req.Text), func(s string) {
		sendEvent(w, fl, "chunk", map[string]string{"text": s})
	})
	sendEvent(w, fl, "done", h.record(&log, req.Text, asked, out))
}

// record hängt Frage und Antwort an den Verlauf und speichert; scheitert das, läuft der Chat mit Hinweis weiter.
func (h *chatHub) record(c *ChatLog, text string, asked time.Time, out chatOutcome) chatEnd {
	c.Messages = append(c.Messages, ChatMessage{Role: chatUser, Text: text, Time: asked},
		ChatMessage{Role: chatModel, Text: out.Text, Status: out.Status, Error: out.Error, Time: h.store.now()})
	end := chatEnd{Status: out.Status, Error: out.Error}
	if err := h.store.save(*c); err != nil {
		slog.Warn("❌ Chat-Verlauf nicht gespeichert", "ns", "main", "chat", c.ID, "fehler", err)
		end.Notice = "Verlauf nicht gespeichert (" + chatDirName + "): " + err.Error()
		return end
	}
	slog.Info("💾 Chat-Verlauf gespeichert", "ns", "main", "chat", c.ID, "nachrichten", len(c.Messages))
	return end
}

func sendEvent(w io.Writer, fl http.Flusher, name string, data any) {
	raw, _ := json.Marshal(data)
	_, _ = fmt.Fprintf(w, "event: %s\ndata: %s\n\n", name, raw)
	fl.Flush()
}

// chatPrompt setzt die letzten Nachrichten des Verlaufs vor die neue Nachricht (jeder Aufruf der CLI ist neu).
func chatPrompt(history []ChatMessage, text string) string {
	if len(history) == 0 {
		return text
	}
	var b strings.Builder
	b.WriteString("Bisheriger Verlauf dieses Chats:\n\n")
	for _, m := range history[max(0, len(history)-chatHistoryMax):] {
		who := "Ich"
		if m.Role == chatModel {
			who = "Du"
		}
		fmt.Fprintf(&b, "[%s]\n%s\n\n", who, m.Text)
	}
	b.WriteString("Neue Nachricht:\n")
	b.WriteString(text)
	return b.String()
}

// ChatList nennt die gespeicherten Chats, der jüngste zuerst.
func (a *App) ChatList() ([]ChatSummary, error) { return a.chats().store.list() }

// ChatGet liefert einen Chat mit allen Nachrichten.
func (a *App) ChatGet(id string) (ChatLog, error) {
	c, err := a.chats().store.load(id)
	if errors.Is(err, os.ErrNotExist) {
		return c, errors.New("kein Chat mit der id " + id)
	}
	return c, err
}

// ChatCancel bricht die laufende Antwort des Chats id ab; false, wenn keine läuft.
func (a *App) ChatCancel(id string) bool { return a.chats().cancel(id) }
