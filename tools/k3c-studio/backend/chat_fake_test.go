package backend

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"strings"
	"testing"
	"time"
)

// Test-Ersatz für die Claude Code CLI: Mit K3C_CHAT_FAKE startet das Test-Binary nicht die Tests, sondern spielt
// „claude -p --output-format stream-json“ nach. So läuft ein echter Prozess (Zeitlimit, Abbruch), nie ein Modell.
const (
	fakeMode = "K3C_CHAT_FAKE"      // echo, hang, login, oldcli
	fakeArgs = "K3C_CHAT_FAKE_ARGS" // Datei: Argumente und Arbeitsordner, wie der Prozess sie bekam
	fakePids = "K3C_CHAT_FAKE_PIDS" // Datei: PIDs von Kind und Enkel (Modus tree)
	// fakeScript ist die Datei mit den Antworten (JSON-Liste) im Modus script (B-686).
	fakeScript = "K3C_CHAT_FAKE_SCRIPT"
)

func TestMain(m *testing.M) {
	if mode := os.Getenv(fakeMode); mode != "" {
		os.Exit(fakeClaude(mode))
	}
	chatProgram = "k3c-claude-im-test-gesperrt" // Riegel: defaultCLI() findet nie das echte claude (B-514)
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
	case "tree": // startet einen Enkel, meldet beide PIDs und hängt
		grand := exec.Command(os.Args[0])
		grand.Env = append(os.Environ(), fakeMode+"=hang")
		if err := grand.Start(); err != nil {
			return 1
		}
		_ = os.WriteFile(os.Getenv(fakePids), []byte(fmt.Sprintf("%d %d", os.Getpid(), grand.Process.Pid)), 0o644)
		time.Sleep(time.Minute)
	case "script": // B-686: Antworten der Reihe nach aus fakeScript, jeder Aufruf steht als Zeile in fakeScript+".log"
		text := fakeScriptAnswer(string(prompt), fakeSystem(os.Args[1:]))
		fakeDelta(text)
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

// fakeScriptAnswer liefert die n-te Antwort des Skripts (n = Zahl der bisherigen Aufrufe, danach die letzte) und hängt
// stdin und Instruktion an das Protokoll.
func fakeScriptAnswer(stdin, system string) string {
	path := os.Getenv(fakeScript)
	var answers []string
	raw, _ := os.ReadFile(path)
	_ = json.Unmarshal(raw, &answers)
	logRaw, _ := os.ReadFile(path + ".log")
	n := strings.Count(string(logRaw), "\n")
	line, _ := json.Marshal(map[string]string{"stdin": stdin, "system": system})
	_ = os.WriteFile(path+".log", append(logRaw, append(line, '\n')...), 0o644)
	if len(answers) == 0 {
		return ""
	}
	return answers[min(n, len(answers)-1)]
}

// scriptCLI startet den Ersatz im Modus script mit den Antworten answers; calls liest danach alle Aufrufe.
func scriptCLI(t *testing.T, answers ...string) (cli chatCLI, calls func() []struct{ Stdin, System string }) {
	t.Helper()
	cli, _ = fakeCLI(t, "script", 10*time.Second)
	path := t.TempDir() + string(os.PathSeparator) + "script.json"
	raw, _ := json.Marshal(answers)
	if err := os.WriteFile(path, raw, 0o644); err != nil {
		t.Fatal(err)
	}
	cli.env = append(cli.env, fakeScript+"="+path)
	return cli, func() []struct{ Stdin, System string } {
		raw, _ := os.ReadFile(path + ".log")
		var out []struct{ Stdin, System string }
		for _, line := range strings.Split(strings.TrimSpace(string(raw)), "\n") {
			var c struct{ Stdin, System string }
			if json.Unmarshal([]byte(line), &c) == nil {
				out = append(out, c)
			}
		}
		return out
	}
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
