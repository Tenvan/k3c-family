package backend

import (
	"os"
	"slices"
	"strings"
	"testing"
	"time"
)

// sessionChat schickt text an den Chat id (leer = neu) und liefert die id, das Ende und was der Ersatz-Prozess bekam.
func sessionChat(t *testing.T, a *App, argsFile, id, text string) (string, chatEnd, []string, string) {
	t.Helper()
	body := `{"text":"` + text + `"}`
	if id != "" {
		body = `{"chat":"` + id + `","text":"` + text + `"}`
	}
	rec, _ := call(t, a.Handler(false), "POST", Prefix+"chat", body, "127.0.0.1:5000", true)
	end, chat := chatDoneOf(t, rec.Body.String())
	args, _ := fakeRecord(t, argsFile)
	stdin, _ := fakeInput(t, argsFile)
	return chat, end, args, stdin
}

func argAfter(args []string, name string) string {
	if i := slices.Index(args, name); i >= 0 && i+1 < len(args) {
		return args[i+1]
	}
	return ""
}

func sessionOf(t *testing.T, a *App, id string) string {
	t.Helper()
	c, err := a.ChatGet(id)
	if err != nil {
		t.Fatal(err)
	}
	return c.Session
}

// B-687/AC-01: Die erste Nachricht legt die Sitzung an, die Folgenachricht setzt sie per Resume fort und schickt keinen Verlauf.
func TestChatSitzungFortgesetzt(t *testing.T) {
	cli, argsFile := fakeCLI(t, "echo", 10*time.Second)
	a := chatApp(t, t.TempDir(), cli)
	id, end, args, _ := sessionChat(t, a, argsFile, "", "Erste Frage")
	session := argAfter(args, "--session-id")
	if end.Status != chatDone || session == "" || sessionOf(t, a, id) != session || cli.sessionFile(session) == "" {
		t.Fatalf("erste Nachricht: %+v %v, Sitzung %q", end, args, sessionOf(t, a, id))
	}
	_, end, args, stdin := sessionChat(t, a, argsFile, id, "Zweite Frage")
	if end.Status != chatDone || argAfter(args, "--resume") != session || slices.Contains(args, "--session-id") ||
		slices.Contains(args, "--no-session-persistence") {
		t.Fatalf("Folgenachricht: %+v %v", end, args)
	}
	if stdin != "Zweite Frage" {
		t.Errorf("Prompt mit Verlauf: %q", stdin)
	}
	if sessionOf(t, a, id) != session {
		t.Error("Sitzung gewechselt")
	}
}

// B-687/AC-02: Fehlt die Sitzungsdatei, geht der Verlauf als Text in einer neuen Sitzung mit, ohne Fehler.
func TestChatSitzungFehltVerlaufAlsText(t *testing.T) {
	cli, argsFile := fakeCLI(t, "echo", 10*time.Second)
	a := chatApp(t, t.TempDir(), cli)
	id, _, args, _ := sessionChat(t, a, argsFile, "", "Erste Frage")
	old := argAfter(args, "--session-id")
	if err := os.Remove(cli.sessionFile(old)); err != nil {
		t.Fatal(err)
	}
	_, end, args, stdin := sessionChat(t, a, argsFile, id, "Zweite Frage")
	fresh := argAfter(args, "--session-id")
	if end.Status != chatDone || end.Error != "" || slices.Contains(args, "--resume") || fresh == "" || fresh == old {
		t.Fatalf("Rückfall: %+v %v", end, args)
	}
	if !strings.Contains(stdin, "Bisheriger Verlauf") || !strings.Contains(stdin, "Erste Frage") {
		t.Errorf("Verlauf fehlt: %q", stdin)
	}
	if sessionOf(t, a, id) != fresh {
		t.Error("neue Sitzung nicht gemerkt")
	}
}

// B-687/AC-02: Kennt die CLI --resume nicht, fällt die Nachricht ohne Fehler auf den Verlauf als Text zurück, danach
// laufen alle Nachrichten ohne Sitzung.
func TestChatSitzungOhneResumeDauerhaftText(t *testing.T) {
	cli, argsFile := fakeCLI(t, "noresume", 10*time.Second)
	a := chatApp(t, t.TempDir(), cli)
	id, _, args, _ := sessionChat(t, a, argsFile, "", "Erste Frage")
	old := argAfter(args, "--session-id")
	_, end, args, stdin := sessionChat(t, a, argsFile, id, "Zweite Frage")
	if end.Status != chatDone || end.Error != "" || !slices.Contains(args, "--no-session-persistence") ||
		!strings.Contains(stdin, "Erste Frage") {
		t.Fatalf("Rückfall: %+v %v %q", end, args, stdin)
	}
	if sessionOf(t, a, id) != "" || cli.sessionFile(old) != "" || !a.chat.noResume.Load() {
		t.Errorf("Sitzung blieb: %q", sessionOf(t, a, id))
	}
	if _, _, args, _ := sessionChat(t, a, argsFile, "", "Neuer Chat"); slices.Contains(args, "--session-id") {
		t.Errorf("trotz alter CLI mit Sitzung: %v", args)
	}
}

// B-687/AC-03: Chat löschen und das Aufräumen des Verlaufs entfernen die Sitzungsdatei.
func TestChatSitzungGeloescht(t *testing.T) {
	cli, argsFile := fakeCLI(t, "echo", 10*time.Second)
	a := chatApp(t, t.TempDir(), cli)
	id, _, args, _ := sessionChat(t, a, argsFile, "", "Frage")
	if s := argAfter(args, "--session-id"); cli.sessionFile(s) == "" || a.ChatDelete(id) != nil || cli.sessionFile(s) != "" {
		t.Fatalf("Löschen ließ die Sitzungsdatei %q liegen", s)
	}
	id, _, args, _ = sessionChat(t, a, argsFile, "", "Alt")
	s := argAfter(args, "--session-id")
	week := time.Now().Add(-chatKeep - time.Hour)
	if err := os.Chtimes(a.chat.store.dir+string(os.PathSeparator)+id+".json", week, week); err != nil {
		t.Fatal(err)
	}
	a.chatCleanup()
	if cli.sessionFile(s) != "" {
		t.Error("Aufräumen ließ die Sitzungsdatei liegen")
	}
	if cli.sessionFile("../" + s) != "" || (chatCLI{sessions: cli.sessions}).sessionFile("kein-uuid") != "" {
		t.Error("Sitzungs-ID ohne UUID-Form wird zum Pfad")
	}
}
