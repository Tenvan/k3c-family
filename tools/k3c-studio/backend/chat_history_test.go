package backend

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func chatFile(t *testing.T, dir, id string, age time.Duration, now time.Time) string {
	t.Helper()
	p := filepath.Join(dir, id+".json")
	write(t, p, `{"id":"`+id+`","messages":[]}`)
	if err := os.Chtimes(p, now.Add(-age), now.Add(-age)); err != nil {
		t.Fatal(err)
	}
	return p
}

// Einträge älter als eine Woche verschwinden, jüngere und fremde Dateien bleiben (B-488/AC-07).
func TestChatAufraeumen(t *testing.T) {
	now := time.Date(2026, 10, 20, 12, 0, 0, 0, time.UTC)
	dir := filepath.Join(t.TempDir(), chatDirName)
	old := chatFile(t, dir, "alt", 8*24*time.Hour, now)
	young := chatFile(t, dir, "jung", 6*24*time.Hour, now)
	other := filepath.Join(dir, "notiz.txt")
	write(t, other, "bleibt")
	if err := os.Chtimes(other, now.Add(-30*24*time.Hour), now.Add(-30*24*time.Hour)); err != nil {
		t.Fatal(err)
	}
	n, err := chatStore{dir: dir, now: func() time.Time { return now }}.cleanup()
	if err != nil || n != 1 {
		t.Fatalf("aufgeräumt %d, %v", n, err)
	}
	if _, err := os.Stat(old); !os.IsNotExist(err) {
		t.Error("alter Chat blieb liegen")
	}
	for _, p := range []string{young, other} {
		if _, err := os.Stat(p); err != nil {
			t.Errorf("%s fehlt: %v", p, err)
		}
	}
}

// Beim Start des Studios (New) wird aufgeräumt.
func TestChatAufraeumenBeimStart(t *testing.T) {
	root := studioRepo(t)
	now := time.Now()
	old := chatFile(t, filepath.Join(root, chatDirName), "alt", 8*24*time.Hour, now)
	young := chatFile(t, filepath.Join(root, chatDirName), "jung", 24*time.Hour, now)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	New(ctx, root)
	if _, err := os.Stat(old); !os.IsNotExist(err) {
		t.Error("alter Chat blieb beim Start liegen")
	}
	if _, err := os.Stat(young); err != nil {
		t.Errorf("junger Chat fehlt: %v", err)
	}
}

// Beim Speichern wird ebenfalls aufgeräumt; der gespeicherte Chat trägt die Zeit der eingespeisten Uhr.
func TestChatSpeichernUndLaden(t *testing.T) {
	now := time.Date(2026, 10, 20, 12, 0, 0, 0, time.UTC)
	dir := filepath.Join(t.TempDir(), chatDirName)
	old := chatFile(t, dir, "alt", 8*24*time.Hour, now)
	s := chatStore{dir: dir, now: func() time.Time { return now }}
	log := ChatLog{ID: "neu", Messages: []ChatMessage{{Role: chatUser, Text: "Hallo"}, {Role: chatModel, Text: "Hi"}}}
	if err := s.save(log); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(old); !os.IsNotExist(err) {
		t.Error("Speichern räumte nicht auf")
	}
	got, err := s.load("neu")
	if err != nil || len(got.Messages) != 2 || got.Messages[0].Text != "Hallo" || !got.Updated.Equal(now) {
		t.Fatalf("geladen %+v, %v", got, err)
	}
	list, err := s.list()
	if err != nil || len(list) != 1 || list[0].ID != "neu" || list[0].Title != "Hallo" || list[0].Count != 2 {
		t.Errorf("Liste %+v, %v", list, err)
	}
	if _, err := s.load("../geheim"); err == nil {
		t.Error("ungültige id wurde gelesen")
	}
}
