package backend

import (
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"sync/atomic"
	"time"
)

// Verlauf des Chats (B-488/AC-07): eine Datei je Chat in .chats im Repo-Stamm (von git ignoriert), nur lokal; Chats,
// die eine Woche nicht geändert wurden, löscht das Studio beim Start und beim Speichern.

const (
	chatDirName = ".chats"
	chatKeep    = 7 * 24 * time.Hour
	chatUser    = "user"      // Nachricht von 🧑
	chatModel   = "assistant" // Antwort des Modells
)

// chatIDPattern hält die id eines Chats frei von Pfaden.
var chatIDPattern = regexp.MustCompile(`^[a-z0-9-]{1,64}$`)

var chatSeq atomic.Uint32

// ChatMessage ist eine Nachricht im Verlauf; Status und Error nur an Antworten.
type ChatMessage struct {
	Role   string    `json:"role"`
	Text   string    `json:"text"`
	Status string    `json:"status,omitempty"`
	Error  string    `json:"error,omitempty"`
	Time   time.Time `json:"time"`
}

// ChatLog ist ein Chat mit allen Nachrichten (Datei .chats/<id>.json).
type ChatLog struct {
	ID       string        `json:"id"`
	Created  time.Time     `json:"created"`
	Updated  time.Time     `json:"updated"`
	Messages []ChatMessage `json:"messages"`
}

// ChatSummary ist ein Eintrag der Chat-Liste; Title ist der Anfang der ersten Nachricht.
type ChatSummary struct {
	ID      string    `json:"id"`
	Title   string    `json:"title"`
	Updated time.Time `json:"updated"`
	Count   int       `json:"count"`
}

// chatStore liest und schreibt den Verlauf; now ist die Uhr (Test-Naht).
type chatStore struct {
	dir string
	now func() time.Time
}

func newChatID(now time.Time) string {
	return fmt.Sprintf("%s-%04d", now.Format("20060102-150405"), chatSeq.Add(1)%10000)
}

func (s chatStore) path(id string) (string, error) {
	if !chatIDPattern.MatchString(id) {
		return "", errors.New("ungültige Chat-id (nur Kleinbuchstaben, Ziffern, Bindestrich)")
	}
	return filepath.Join(s.dir, id+".json"), nil
}

// cleanup löscht Chats (*.json), die älter als chatKeep sind, und meldet ihre Anzahl; andere Dateien bleiben.
func (s chatStore) cleanup() (int, error) {
	entries, err := os.ReadDir(s.dir)
	if errors.Is(err, os.ErrNotExist) {
		return 0, nil
	}
	if err != nil {
		return 0, err
	}
	n := 0
	for _, e := range entries {
		info, ierr := e.Info()
		if e.IsDir() || filepath.Ext(e.Name()) != ".json" || ierr != nil || s.now().Sub(info.ModTime()) <= chatKeep {
			continue
		}
		if rerr := os.Remove(filepath.Join(s.dir, e.Name())); rerr != nil {
			err = rerr
			continue
		}
		n++
	}
	return n, err
}

// save schreibt den Chat atomar, setzt Updated und Dateizeit auf die Uhr und räumt danach auf.
func (s chatStore) save(c ChatLog) error {
	p, err := s.path(c.ID)
	if err != nil {
		return err
	}
	now := s.now()
	if c.Created.IsZero() {
		c.Created = now
	}
	c.Updated = now
	raw, err := json.MarshalIndent(c, "", "  ")
	if err != nil {
		return err
	}
	if err := os.MkdirAll(s.dir, 0o755); err != nil {
		return err
	}
	if err := writeAtomic(p, append(raw, '\n')); err != nil {
		return err
	}
	if err := os.Chtimes(p, now, now); err != nil {
		return err
	}
	if _, err := s.cleanup(); err != nil { // der Chat ist gespeichert; Aufräumen holt der nächste Lauf nach
		slog.Warn("❌ Chat-Verlauf nicht aufgeräumt", "ns", "main", "fehler", err)
	}
	return nil
}

// load liest einen Chat; fehlt er, kommt os.ErrNotExist.
func (s chatStore) load(id string) (ChatLog, error) {
	p, err := s.path(id)
	if err != nil {
		return ChatLog{}, err
	}
	raw, err := os.ReadFile(p)
	if err != nil {
		return ChatLog{}, err
	}
	var c ChatLog
	if err := json.Unmarshal(raw, &c); err != nil {
		return ChatLog{}, fmt.Errorf("der Chat %s ist nicht lesbar: %w", id, err)
	}
	return c, nil
}

// list nennt die Chats, der jüngste zuerst; unlesbare Dateien fehlen in der Liste.
func (s chatStore) list() ([]ChatSummary, error) {
	entries, err := os.ReadDir(s.dir)
	if errors.Is(err, os.ErrNotExist) {
		return []ChatSummary{}, nil
	}
	if err != nil {
		return nil, err
	}
	out := []ChatSummary{}
	for _, e := range entries {
		id, ok := strings.CutSuffix(e.Name(), ".json")
		c, lerr := s.load(id)
		if !ok || e.IsDir() || lerr != nil {
			continue
		}
		out = append(out, ChatSummary{ID: c.ID, Title: chatTitle(c), Updated: c.Updated, Count: len(c.Messages)})
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Updated.After(out[j].Updated) })
	return out, nil
}

func chatTitle(c ChatLog) string {
	if len(c.Messages) == 0 {
		return c.ID
	}
	t := []rune(strings.TrimSpace(c.Messages[0].Text))
	if len(t) > 60 {
		return string(t[:60]) + " …"
	}
	return string(t)
}
