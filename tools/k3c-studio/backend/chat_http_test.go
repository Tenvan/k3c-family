package backend

import (
	"context"
	"encoding/json"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// chatApp ist ein Studio mit dem Test-Ersatz als CLI.
func chatApp(t *testing.T, root string, cli chatCLI) *App {
	t.Helper()
	app := &App{root: root, bus: newBus()}
	app.chat = newChatHub(root)
	app.chat.cli = cli
	return app
}

// sse zerlegt den Strom in Ereignisse (Name → Daten in Reihenfolge).
func sse(body string) (names []string, data []string) {
	for _, block := range strings.Split(body, "\n\n") {
		var name, d string
		for _, line := range strings.Split(block, "\n") {
			if v, ok := strings.CutPrefix(line, "event: "); ok {
				name = v
			} else if v, ok := strings.CutPrefix(line, "data: "); ok {
				d = v
			}
		}
		if name != "" {
			names, data = append(names, name), append(data, d)
		}
	}
	return names, data
}

func chatDoneOf(t *testing.T, body string) (chatEnd, string) {
	t.Helper()
	names, data := sse(body)
	if len(names) < 2 || names[0] != "start" || names[len(names)-1] != "done" {
		t.Fatalf("Strom unvollständig: %v\n%s", names, body)
	}
	var start struct {
		Chat string `json:"chat"`
	}
	_ = json.Unmarshal([]byte(data[0]), &start)
	var end chatEnd
	if err := json.Unmarshal([]byte(data[len(data)-1]), &end); err != nil {
		t.Fatal(err)
	}
	return end, start.Chat
}

func TestChatStromUndVerlauf(t *testing.T) {
	root := t.TempDir()
	cli, _ := fakeCLI(t, "echo", 10*time.Second)
	h := chatApp(t, root, cli).Handler(false)
	rec, _ := call(t, h, "POST", Prefix+"chat", `{"text":"Erste Frage"}`, "127.0.0.1:5000", true)
	if rec.Code != 200 || !strings.HasPrefix(rec.Header().Get("Content-Type"), "text/event-stream") {
		t.Fatalf("Strom: %d %s", rec.Code, rec.Body)
	}
	if n := strings.Count(rec.Body.String(), "event: chunk"); n < 2 {
		t.Errorf("Antwort kam nicht in Stücken (%d)", n)
	}
	end, id := chatDoneOf(t, rec.Body.String())
	if end.Status != chatDone || end.Notice != "" || id == "" {
		t.Fatalf("Ende: %+v, Chat %q", end, id)
	}
	body := `{"chat":"` + id + `","text":"Zweite Frage"}`
	rec, _ = call(t, h, "POST", Prefix+"chat", body, "[::1]:5000", true)
	if !strings.Contains(rec.Body.String(), "Erste Frage") { // der Verlauf geht mit in den Prompt
		t.Errorf("Verlauf fehlt im Prompt: %s", rec.Body)
	}
	_, a := call(t, h, "POST", Prefix+"rpc/ChatGet", `["`+id+`"]`, "127.0.0.1:5000", true)
	var log ChatLog
	if err := json.Unmarshal(a.Result, &log); err != nil || len(log.Messages) != 4 || log.Messages[3].Status != chatDone {
		t.Fatalf("Verlauf: %v %+v (%s)", err, log, a.Error)
	}
	if _, err := os.Stat(filepath.Join(root, chatDirName, id+".json")); err != nil {
		t.Errorf("Verlauf liegt nicht in .chats: %v", err)
	}
	_, a = call(t, h, "POST", Prefix+"rpc/ChatList", `[]`, "127.0.0.1:5000", true)
	if !strings.Contains(string(a.Result), id) {
		t.Errorf("Liste ohne Chat: %s %s", a.Result, a.Error)
	}
}

// Auch mit freigegebenem LAN (remote) bedient der Chat nur Loopback (B-488 › Sicherheit).
func TestChatNurLoopback(t *testing.T) {
	cli, argsFile := fakeCLI(t, "echo", 10*time.Second)
	h := chatApp(t, t.TempDir(), cli).Handler(true)
	for _, path := range []string{Prefix + "chat", Prefix + "rpc/ChatList", Prefix + "rpc/ChatGet", Prefix + "rpc/ChatCancel"} {
		rec, a := call(t, h, "POST", path, `{"text":"Hallo"}`, "192.168.1.20:5000", true)
		if rec.Code != http.StatusForbidden || !strings.Contains(a.Error, "Chat") {
			t.Errorf("%s aus dem LAN: %d %q", path, rec.Code, a.Error)
		}
	}
	if rec, _ := call(t, h, "POST", Prefix+"rpc/AssetsCatalog", `[]`, "192.168.1.20:5000", true); rec.Code == http.StatusForbidden {
		t.Error("andere Methoden bleiben im LAN erreichbar")
	}
	if _, err := os.Stat(argsFile); !os.IsNotExist(err) {
		t.Error("CLI wurde aus dem LAN gestartet")
	}
}

func TestChatAnfrageUngueltig(t *testing.T) {
	cli, _ := fakeCLI(t, "echo", 10*time.Second)
	h := chatApp(t, t.TempDir(), cli).Handler(false)
	for name, body := range map[string]string{"leer": `{"text":"  "}`, "Pfad als id": `{"chat":"../x","text":"a"}`, "kein JSON": `x`} {
		if rec, _ := call(t, h, "POST", Prefix+"chat", body, "127.0.0.1:5000", true); rec.Code != http.StatusBadRequest {
			t.Errorf("%s: %d", name, rec.Code)
		}
	}
	if rec, _ := call(t, h, "POST", Prefix+"chat", `{"text":"a"}`, "127.0.0.1:5000", false); rec.Code != http.StatusForbidden {
		t.Errorf("ohne Kopfzeile: %d", rec.Code)
	}
}

// ChatCancel bricht eine laufende Antwort ab; sie bleibt als abgebrochen im Verlauf.
func TestChatCancel(t *testing.T) {
	cli, _ := fakeCLI(t, "hang", time.Minute)
	app := chatApp(t, t.TempDir(), cli)
	h := app.Handler(false)
	done := make(chan string)
	go func() {
		rec, _ := call(t, h, "POST", Prefix+"chat", `{"chat":"warten","text":"Hallo"}`, "127.0.0.1:5000", true)
		done <- rec.Body.String()
	}()
	deadline := time.Now().Add(10 * time.Second)
	for !app.chat.isRunning("warten") && time.Now().Before(deadline) {
		time.Sleep(10 * time.Millisecond)
	}
	if rec, _ := call(t, h, "POST", Prefix+"chat", `{"chat":"warten","text":"Noch eine"}`, "127.0.0.1:5000", true); rec.Code != http.StatusConflict {
		t.Errorf("zweite Antwort im selben Chat: %d, erwartet 409", rec.Code)
	}
	_, a := call(t, h, "POST", Prefix+"rpc/ChatCancel", `["warten"]`, "127.0.0.1:5000", true)
	if string(a.Result) != "true" {
		t.Fatalf("Abbruch: %s %s", a.Result, a.Error)
	}
	select {
	case body := <-done:
		if end, _ := chatDoneOf(t, body); end.Status != chatAborted {
			t.Errorf("Ende: %+v", end)
		}
	case <-time.After(10 * time.Second):
		t.Fatal("Antwort lief nach dem Abbruch weiter")
	}
	log, err := app.chat.store.load("warten")
	if err != nil || len(log.Messages) != 2 || log.Messages[1].Status != chatAborted {
		t.Errorf("Verlauf nach Abbruch: %+v %v", log, err)
	}
}

// Ist .chats nicht beschreibbar, läuft der Chat ohne Verlauf mit Hinweis.
func TestChatVerlaufNichtBeschreibbar(t *testing.T) {
	root := t.TempDir()
	write(t, filepath.Join(root, chatDirName), "eine Datei statt eines Ordners")
	cli, _ := fakeCLI(t, "echo", 10*time.Second)
	h := chatApp(t, root, cli).Handler(false)
	rec, _ := call(t, h, "POST", Prefix+"chat", `{"text":"Hallo"}`, "127.0.0.1:5000", true)
	end, _ := chatDoneOf(t, rec.Body.String())
	if end.Status != chatDone || !strings.Contains(end.Notice, "Verlauf nicht gespeichert") {
		t.Errorf("Ende: %+v", end)
	}
}

// Endet der Server (Kontext von New), endet auch eine laufende Antwort.
func TestChatEndetMitServer(t *testing.T) {
	cli, _ := fakeCLI(t, "hang", time.Minute)
	app := chatApp(t, t.TempDir(), cli)
	ctx, stop := context.WithCancel(context.Background())
	defer stop()
	app.ctx = ctx
	h := app.Handler(false)
	done := make(chan string)
	go func() {
		rec, _ := call(t, h, "POST", Prefix+"chat", `{"chat":"server","text":"Hallo"}`, "127.0.0.1:5000", true)
		done <- rec.Body.String()
	}()
	deadline := time.Now().Add(10 * time.Second)
	for !app.chat.isRunning("server") && time.Now().Before(deadline) {
		time.Sleep(10 * time.Millisecond)
	}
	stop()
	select {
	case body := <-done:
		if end, _ := chatDoneOf(t, body); end.Status != chatAborted {
			t.Errorf("Ende: %+v", end)
		}
	case <-time.After(10 * time.Second):
		t.Fatal("Antwort lief nach dem Ende des Servers weiter")
	}
}
