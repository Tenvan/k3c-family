package backend

import (
	"context"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"
	"unicode/utf8"
)

// Die Argumente geben dem Modell nie Werkzeuge, Schreibrechte, Ordner oder fremde Konfiguration (B-488 › Sicherheit).
func TestChatArgsOhneRechte(t *testing.T) {
	args := chatArgs(chatOptions{})
	forbidden := []string{"--dangerously-skip-permissions", "--allow-dangerously-skip-permissions", "--allowedTools",
		"--allowed-tools", "--add-dir", "--mcp-config", "--plugin-dir", "--plugin-url", "--agents", "--settings",
		"--chrome", "--ide", "bypassPermissions", "acceptEdits", "auto", "default", "--continue", "--resume"}
	for _, f := range forbidden {
		if slices.Contains(args, f) {
			t.Errorf("verbotenes Argument %q in %v", f, args)
		}
	}
	i := slices.Index(args, "--tools")
	if i < 0 || i+1 >= len(args) || args[i+1] != "" {
		t.Errorf("--tools \"\" (keine Werkzeuge) fehlt: %v", args)
	}
	for _, want := range []string{"-p", "stream-json", "--include-partial-messages", "--strict-mcp-config", "--safe-mode",
		"--no-session-persistence"} {
		if !slices.Contains(args, want) {
			t.Errorf("Argument %q fehlt: %v", want, args)
		}
	}
	if m := slices.Index(args, "--permission-mode"); m < 0 || args[m+1] != "dontAsk" {
		t.Errorf("--permission-mode dontAsk fehlt: %v", args)
	}
}

func TestChatSendetPromptUndStreamt(t *testing.T) {
	cli, argsFile := fakeCLI(t, "echo", 10*time.Second)
	var chunks []string
	out := cli.run(context.Background(), "Baue eine Welt", chatOptions{}, func(s string) { chunks = append(chunks, s) })
	if out.Status != chatDone || out.Error != "" {
		t.Fatalf("Status %q, Fehler %q", out.Status, out.Error)
	}
	if len(chunks) < 2 || strings.Join(chunks, "") != "Echo: Baue eine Welt" || out.Text != "Echo: Baue eine Welt" {
		t.Errorf("Antwort nicht fortlaufend: %q → %q", chunks, out.Text)
	}
	args, dir := fakeRecord(t, argsFile)
	if !slices.Equal(args, chatArgs(chatOptions{})) {
		t.Errorf("Prozess bekam %v, erwartet %v (Prompt nur über stdin)", args, chatArgs(chatOptions{}))
	}
	if !strings.EqualFold(evalDir(dir), evalDir(cli.workDir)) {
		t.Errorf("Arbeitsordner %q, erwartet %q", dir, cli.workDir)
	}
}

func evalDir(dir string) string {
	if d, err := filepath.EvalSymlinks(dir); err == nil {
		return d
	}
	return dir
}

func TestChatZeitlimit(t *testing.T) {
	cli, _ := fakeCLI(t, "hang", 300*time.Millisecond)
	start := time.Now()
	out := cli.run(context.Background(), "Hallo", chatOptions{}, func(string) {})
	if d := time.Since(start); d > 10*time.Second {
		t.Errorf("Zeitlimit griff nicht: %v", d)
	}
	if out.Status != chatAborted || !strings.Contains(out.Error, "Zeitlimit") || out.Text != "Ich denke " {
		t.Errorf("Zeitlimit: %+v", out)
	}
}

func TestChatAbbruch(t *testing.T) {
	cli, _ := fakeCLI(t, "hang", time.Minute)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	start := time.Now()
	out := cli.run(ctx, "Hallo", chatOptions{}, func(string) { cancel() }) // Abbruch nach dem ersten Stück
	if d := time.Since(start); d > 10*time.Second {
		t.Errorf("Abbruch griff nicht: %v", d)
	}
	if out.Status != chatAborted || !strings.Contains(out.Error, "Abgebrochen") {
		t.Errorf("Abbruch: %+v", out)
	}
}

func TestChatCLIFehlt(t *testing.T) {
	cli := chatCLI{program: "k3c-claude-gibt-es-nicht", timeout: time.Second, workDir: t.TempDir()}
	out := cli.run(context.Background(), "Hallo", chatOptions{}, func(string) {})
	if out.Status != chatFailed || !strings.Contains(out.Error, "nicht gefunden") {
		t.Errorf("fehlende CLI: %+v", out)
	}
}

func TestChatNichtAngemeldet(t *testing.T) {
	cli, _ := fakeCLI(t, "login", 10*time.Second)
	out := cli.run(context.Background(), "Hallo", chatOptions{}, func(string) {})
	if out.Status != chatFailed || !strings.Contains(out.Error, "nicht angemeldet") {
		t.Errorf("nicht angemeldet: %+v", out)
	}
}

// B-516: Kennt die CLI die Instruktions-Datei nicht, endet die Antwort als Fehler mit Update-Hinweis, nie ohne Gerüst.
func TestChatAlteCLIOhneInstruktion(t *testing.T) {
	cli, _ := fakeCLI(t, "oldcli", 10*time.Second)
	out := cli.run(context.Background(), "Erstelle eine Welt Nordsee", chatOptions{instruction: "Gerüst"}, func(string) {})
	if out.Status != chatFailed || !strings.Contains(out.Error, chatSystemFlag) || !strings.Contains(out.Error, "aktualisieren") {
		t.Errorf("alte CLI: %+v", out)
	}
}

// Lange Meldungen werden nach Zeichen gekürzt, nie mitten in einem Umlaut.
func TestChatMeldungGekuerzt(t *testing.T) {
	msg := explainCLI(strings.Repeat("ä", 400), nil)
	if !utf8.ValidString(msg) || !strings.HasSuffix(msg, "ä …") || utf8.RuneCountInString(msg) > 400 {
		t.Errorf("gekürzte Meldung: %q", msg)
	}
}

// --model und --effort kommen nur dazu, wenn gewählt; die Sicherheits-Argumente bleiben dieselben.
func TestChatArgsModellUndDenkstufe(t *testing.T) {
	base := chatArgs(chatOptions{})
	for _, a := range []string{"--model", "--effort"} {
		if slices.Contains(base, a) {
			t.Errorf("%s ohne Wahl: %v", a, base)
		}
	}
	args := chatArgs(chatOptions{Model: "opus", Effort: "xhigh"})
	if !slices.Equal(args[:len(base)], base) || !slices.Equal(args[len(base):], []string{"--model", "opus", "--effort", "xhigh"}) {
		t.Errorf("Argumente mit Wahl: %v", args)
	}
	if only := chatArgs(chatOptions{Effort: "low"}); slices.Contains(only, "--model") || !slices.Contains(only, "low") {
		t.Errorf("nur Denkstufe: %v", only)
	}
	cli, argsFile := fakeCLI(t, "echo", 10*time.Second)
	opts := chatOptions{Provider: chatProviderCLI, Model: "haiku", Effort: "max"}
	if out := cli.run(context.Background(), "Hallo", opts, func(string) {}); out.Status != chatDone {
		t.Fatalf("Antwort: %+v", out)
	}
	if got, _ := fakeRecord(t, argsFile); !slices.Equal(got, chatArgs(opts)) {
		t.Errorf("Prozess bekam %v, erwartet %v", got, chatArgs(opts))
	}
}

// Die CLI erbt keine Einstellungen einer äußeren Claude-Code-Sitzung (B-515).
func TestChatUmgebungOhneSitzung(t *testing.T) {
	in := []string{"PATH=/bin", "CLAUDE_CODE_ENTRYPOINT=cli", "claude_code_session=1", "CLAUDECODE=1",
		"ANTHROPIC_BASE_URL=http://x", "ANTHROPIC_MODEL=opus", "CLAUDEHOME=/h", "K3C_CHAT_FAKE=echo",
		"CLAUDE_CODE_GIT_BASH_PATH=C:/git/bash.exe", "CLAUDE_CODE_USE_BEDROCK=1", "CLAUDE_CODE_USE_VERTEX=1",
		"CLAUDE_CODE_USE_FOUNDRY=1", "CLAUDE_CODE_MAX_OUTPUT_TOKENS=8000", "claude_code_oauth_token=t",
		"CLAUDE_CODE_SSE_PORT=1234"}
	want := []string{"PATH=/bin", "ANTHROPIC_MODEL=opus", "CLAUDEHOME=/h", "K3C_CHAT_FAKE=echo",
		"CLAUDE_CODE_GIT_BASH_PATH=C:/git/bash.exe", "CLAUDE_CODE_USE_BEDROCK=1", "CLAUDE_CODE_USE_VERTEX=1",
		"CLAUDE_CODE_USE_FOUNDRY=1", "CLAUDE_CODE_MAX_OUTPUT_TOKENS=8000", "claude_code_oauth_token=t"}
	if got := chatEnviron(in); !slices.Equal(got, want) {
		t.Errorf("Umgebung %v, erwartet %v", got, want)
	}
}
