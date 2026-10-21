package backend

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"strings"
	"testing"
	"time"
)

// Test-Ersatz für die Claude Code CLI: Mit K3C_CHAT_FAKE startet das Test-Binary nicht die Tests, sondern spielt
// „claude -p --output-format stream-json“ nach. So läuft ein echter Prozess (Zeitlimit, Abbruch), nie ein Modell.
const (
	fakeMode = "K3C_CHAT_FAKE"      // echo, hang, login, oldcli
	fakeArgs = "K3C_CHAT_FAKE_ARGS" // Datei: Argumente und Arbeitsordner, wie der Prozess sie bekam
)

func TestMain(m *testing.M) {
	if mode := os.Getenv(fakeMode); mode != "" {
		os.Exit(fakeClaude(mode))
	}
	os.Exit(m.Run())
}

func fakeClaude(mode string) int {
	prompt, _ := io.ReadAll(os.Stdin)
	if path := os.Getenv(fakeArgs); path != "" {
		wd, _ := os.Getwd()
		rec, _ := json.Marshal(map[string]any{"args": os.Args[1:], "dir": wd, "stdin": string(prompt),
			"system": fakeSystem(os.Args[1:])})
		_ = os.WriteFile(path, rec, 0o644)
	}
	if mode == "oldcli" { // CLI-Version ohne --append-system-prompt-file (B-516)
		fmt.Fprintln(os.Stderr, "error: unknown option '"+chatSystemFlag+"'")
		return 1
	}
	fakeLine(map[string]any{"type": "system", "subtype": "init"})
	switch mode {
	case "echo":
		text := "Echo: " + string(prompt)
		half := len(text) / 2
		fakeDelta(text[:half])
		time.Sleep(50 * time.Millisecond)
		fakeDelta(text[half:])
		fakeLine(map[string]any{"type": "assistant", "message": map[string]any{
			"content": []map[string]string{{"type": "text", "text": text}}}})
		fakeLine(map[string]any{"type": "result", "subtype": "success", "is_error": false, "result": text})
	case "hang":
		fakeDelta("Ich denke ")
		time.Sleep(time.Minute)
	case "login":
		fakeLine(map[string]any{"type": "result", "subtype": "success", "is_error": true,
			"result": "Invalid API key · Please run /login"})
		return 1
	}
	return 0
}

// fakeSystem liest die Instruktions-Datei hinter chatSystemFlag, solange sie während des Laufs noch da ist.
func fakeSystem(args []string) string {
	for i, a := range args {
		if a == chatSystemFlag && i+1 < len(args) {
			raw, _ := os.ReadFile(args[i+1])
			return string(raw)
		}
	}
	return ""
}

func fakeDelta(text string) {
	fakeLine(map[string]any{"type": "stream_event", "event": map[string]any{
		"type": "content_block_delta", "delta": map[string]string{"type": "text_delta", "text": text}}})
}

func fakeLine(v any) {
	b, _ := json.Marshal(v)
	fmt.Println(string(b))
}

// fakeCLI startet das Test-Binary als CLI im Modus mode, Arbeitsordner im Test-Ordner.
func fakeCLI(t *testing.T, mode string, timeout time.Duration) (chatCLI, string) {
	t.Helper()
	exe, err := os.Executable()
	if err != nil {
		t.Fatal(err)
	}
	dir := t.TempDir()
	argsFile := dir + string(os.PathSeparator) + "args.json"
	return chatCLI{program: exe, timeout: timeout, workDir: dir + string(os.PathSeparator) + "leer",
		env: []string{fakeMode + "=" + mode, fakeArgs + "=" + argsFile}}, argsFile
}

// fakeRecord liest, was der Ersatz-Prozess bekam.
func fakeRecord(t *testing.T, path string) (args []string, dir string) {
	t.Helper()
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("Ersatz-Prozess lief nicht: %v", err)
	}
	var rec struct {
		Args []string `json:"args"`
		Dir  string   `json:"dir"`
	}
	if err := json.Unmarshal(raw, &rec); err != nil {
		t.Fatal(err)
	}
	return rec.Args, strings.TrimRight(rec.Dir, `\/`)
}

// fakeInput liest, was der Ersatz-Prozess über stdin und als Instruktions-Datei bekam.
func fakeInput(t *testing.T, path string) (stdin, system string) {
	t.Helper()
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("Ersatz-Prozess lief nicht: %v", err)
	}
	var rec struct{ Stdin, System string }
	if err := json.Unmarshal(raw, &rec); err != nil {
		t.Fatal(err)
	}
	return rec.Stdin, rec.System
}
