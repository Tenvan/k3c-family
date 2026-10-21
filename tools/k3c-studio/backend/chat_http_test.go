package backend

import (
	"context"
	"encoding/json"
	"net/http"
	"os"
	"path/filepath"
	"slices"
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

// Modell, Denkstufe und Anbieter gehen an die CLI und bleiben im Verlauf und in der Liste; Unbekanntes ist 400.
func TestChatEinstellungen(t *testing.T) {
	cli, argsFile := fakeCLI(t, "echo", 10*time.Second)
	app := chatApp(t, t.TempDir(), cli)
	h := app.Handler(false)
	body := `{"chat":"wahl","text":"Hallo","provider":"claude-cli","model":"sonnet","effort":"high"}`
	rec, _ := call(t, h, "POST", Prefix+"chat", body, "127.0.0.1:5000", true)
	if end, _ := chatDoneOf(t, rec.Body.String()); end.Status != chatDone {
		t.Fatalf("Ende: %+v", end)
	}
	args, _ := fakeRecord(t, argsFile)
	if !slices.Equal(args, chatArgs(chatOptions{Model: "sonnet", Effort: "high"})) {
		t.Errorf("Argumente %v", args)
	}
	_, a := call(t, h, "POST", Prefix+"rpc/ChatList", `[]`, "127.0.0.1:5000", true)
	var list []ChatSummary
	if err := json.Unmarshal(a.Result, &list); err != nil || len(list) != 1 ||
		list[0].Provider != "claude-cli" || list[0].Model != "sonnet" || list[0].Effort != "high" {
		t.Fatalf("Liste %s %v", a.Result, err)
	}
	// ohne Angaben: Standard, gespeichert als leer
	rec, _ = call(t, h, "POST", Prefix+"chat", `{"chat":"wahl","text":"Nochmal"}`, "127.0.0.1:5000", true)
	chatDoneOf(t, rec.Body.String())
	if log, err := app.chat.store.load("wahl"); err != nil || log.Provider != "" || log.Model != "" || log.Effort != "" {
		t.Errorf("Einstellungen nach Standard: %+v %v", log, err)
	}
	if args, _ = fakeRecord(t, argsFile); slices.Contains(args, "--model") || slices.Contains(args, "--effort") {
		t.Errorf("Standard ohne Argumente: %v", args)
	}
}

// B-516: Ein Welt-Auftrag schickt das Grundgerüst als Instruktions-Datei mit; stdin und Verlauf enthalten nur den Wunsch,
// die Datei ist danach weg. Ein freier Chat startet ohne Instruktion.
func TestChatWeltAuftragSchicktGeruestMit(t *testing.T) {
	cli, argsFile := fakeCLI(t, "echo", 10*time.Second)
	app := chatApp(t, t.TempDir(), cli)
	h := app.Handler(false)
	rec, _ := call(t, h, "POST", Prefix+"chat", `{"chat":"welt","text":"Erstelle eine Welt Nordsee","task":"welt"}`, "127.0.0.1:5000", true)
	if end, _ := chatDoneOf(t, rec.Body.String()); end.Status != chatDone {
		t.Fatalf("Ende: %+v", end)
	}
	args, _ := fakeRecord(t, argsFile)
	i := slices.Index(args, chatSystemFlag)
	if i < 0 || i+1 >= len(args) || !slices.Equal(args[:i], chatArgs(chatOptions{})) {
		t.Fatalf("Argumente ohne %s am Ende: %v", chatSystemFlag, args)
	}
	stdin, system := fakeInput(t, argsFile)
	if system != worldInstruction() || strings.Contains(system, "MEINE WÜNSCHE") {
		t.Errorf("Instruktion ist nicht das Grundgerüst: %.80q", system)
	}
	if stdin != "Erstelle eine Welt Nordsee" {
		t.Errorf("stdin %.80q, erwartet nur den Wunsch", stdin)
	}
	if _, err := os.Stat(args[i+1]); !os.IsNotExist(err) {
		t.Errorf("Instruktions-Datei bleibt liegen: %v", err)
	}
	if log, err := app.chat.store.load("welt"); err != nil || strings.Contains(log.Messages[0].Text, "## Format") {
		t.Errorf("Verlauf mit Grundgerüst: %v", err)
	}
	rec, _ = call(t, h, "POST", Prefix+"chat", `{"chat":"frei","text":"Hallo"}`, "127.0.0.1:5000", true)
	chatDoneOf(t, rec.Body.String())
	if args, _ = fakeRecord(t, argsFile); slices.Contains(args, chatSystemFlag) {
		t.Errorf("freier Chat mit Instruktion: %v", args)
	}
}

func TestChatEinstellungenUngueltig(t *testing.T) {
	cli, argsFile := fakeCLI(t, "echo", 10*time.Second)
	h := chatApp(t, t.TempDir(), cli).Handler(false)
	for body, want := range map[string]string{
		`{"text":"a","provider":"openai"}`:      "unbekannter Provider",
		`{"text":"a","model":"gpt-5"}`:          "unbekanntes Modell",
		`{"text":"a","model":"--help"}`:         "unbekanntes Modell",
		`{"text":"a","effort":"ultra"}`:         "unbekannte Denkstufe",
		`{"text":"a","effort":"high extra"}`:    "unbekannte Denkstufe",
		`{"text":"a","model":"Opus"}`:           "unbekanntes Modell",
		`{"text":"a","provider":"claude-cli "}`: "unbekannter Provider",
		`{"text":"a","task":"bild"}`:            "unbekannte Auftragsart",
	} {
		rec, a := call(t, h, "POST", Prefix+"chat", body, "127.0.0.1:5000", true)
		if rec.Code != http.StatusBadRequest || !strings.Contains(a.Error, want) {
			t.Errorf("%s: %d %q", body, rec.Code, a.Error)
		}
	}
	if _, err := os.Stat(argsFile); !os.IsNotExist(err) {
		t.Error("CLI wurde mit ungültiger Wahl gestartet")
	}
}

func TestChatOptionsListe(t *testing.T) {
	cli, _ := fakeCLI(t, "echo", time.Second)
	h := chatApp(t, t.TempDir(), cli).Handler(false)
	_, a := call(t, h, "POST", Prefix+"rpc/ChatOptions", `[]`, "127.0.0.1:5000", true)
	var v ChatOptionsView
	if err := json.Unmarshal(a.Result, &v); err != nil {
		t.Fatalf("%s %v", a.Result, err)
	}
	ids := func(l []ChatChoice) string {
		var s []string
		for _, c := range l {
			s = append(s, c.ID+"="+c.Label)
		}
		return strings.Join(s, ",")
	}
	if got := ids(v.Providers); got != "=Standard,claude-cli=Claude Code CLI" {
		t.Errorf("Provider: %s", got)
	}
	if got := ids(v.Models); got != "=Standard,fable=Fable,opus=Opus,sonnet=Sonnet,haiku=Haiku" {
		t.Errorf("Modelle: %s", got)
	}
	if got := ids(v.Efforts); got != "=Standard,low=Niedrig,medium=Mittel,high=Hoch,xhigh=Sehr hoch,max=Maximum" {
		t.Errorf("Denkstufen: %s", got)
	}
}

func TestChatDelete(t *testing.T) {
	cli, _ := fakeCLI(t, "hang", time.Minute)
	app := chatApp(t, t.TempDir(), cli)
	h := app.Handler(false)
	if err := app.chat.store.save(ChatLog{ID: "alt", Messages: []ChatMessage{{Role: chatUser, Text: "x"}}}); err != nil {
		t.Fatal(err)
	}
	rpc := func(id string) (int, string) {
		rec, a := call(t, h, "POST", Prefix+"rpc/ChatDelete", `["`+id+`"]`, "127.0.0.1:5000", true)
		return rec.Code, a.Error
	}
	if code, e := rpc("alt"); code != http.StatusOK || e != "" {
		t.Fatalf("Löschen: %d %q", code, e)
	}
	if _, err := os.Stat(filepath.Join(app.chat.store.dir, "alt.json")); !os.IsNotExist(err) {
		t.Error("Datei blieb liegen")
	}
	if code, e := rpc("alt"); code != http.StatusUnprocessableEntity || !strings.Contains(e, "kein Chat mit der id alt") {
		t.Errorf("unbekannte id: %d %q", code, e)
	}
	// eine Nachricht an den gelöschten Chat legt ihn nicht neu an
	if rec, _ := call(t, h, "POST", Prefix+"chat", `{"chat":"alt","text":"Spät"}`, "127.0.0.1:5000", true); rec.Code != http.StatusGone {
		t.Errorf("Nachricht an gelöschten Chat: %d", rec.Code)
	}
	if _, err := os.Stat(filepath.Join(app.chat.store.dir, "alt.json")); !os.IsNotExist(err) {
		t.Error("gelöschter Chat wurde neu angelegt")
	}
	if code, e := rpc("../x"); code != http.StatusUnprocessableEntity || !strings.Contains(e, "ungültige Chat-id") {
		t.Errorf("Pfad als id: %d %q", code, e)
	}
	deleteWhileRunning(t, app, h, rpc)
	if code, e := rpc("laeuft"); code != http.StatusOK || e != "" {
		t.Errorf("nach der Antwort: %d %q", code, e)
	}
}

// deleteWhileRunning lässt eine Antwort laufen: Löschen wird abgelehnt, danach (nach dem Abbruch) ist es möglich.
func deleteWhileRunning(t *testing.T, app *App, h http.Handler, rpc func(string) (int, string)) {
	t.Helper()
	done := make(chan string) // die Goroutine meldet nur über den Kanal, geprüft wird im Test
	go func() {
		rec, _ := call(t, h, "POST", Prefix+"chat", `{"chat":"laeuft","text":"Hallo"}`, "127.0.0.1:5000", true)
		done <- rec.Body.String()
	}()
	deadline := time.Now().Add(10 * time.Second)
	for !app.chat.isRunning("laeuft") && time.Now().Before(deadline) {
		time.Sleep(10 * time.Millisecond)
	}
	if code, e := rpc("laeuft"); code != http.StatusUnprocessableEntity || !strings.Contains(e, "läuft noch eine Antwort") {
		t.Errorf("während der Antwort: %d %q", code, e)
	}
	app.chat.cancel("laeuft")
	select {
	case body := <-done:
		if end, _ := chatDoneOf(t, body); end.Status != chatAborted {
			t.Errorf("Ende: %+v", end)
		}
	case <-time.After(10 * time.Second):
		t.Fatal("Antwort lief nach dem Abbruch weiter")
	}
}
