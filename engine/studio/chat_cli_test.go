package studio

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
	args := chatArgs()
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
	out := cli.run(context.Background(), "Baue eine Welt", func(s string) { chunks = append(chunks, s) })
	if out.Status != chatDone || out.Error != "" {
		t.Fatalf("Status %q, Fehler %q", out.Status, out.Error)
	}
	if len(chunks) < 2 || strings.Join(chunks, "") != "Echo: Baue eine Welt" || out.Text != "Echo: Baue eine Welt" {
		t.Errorf("Antwort nicht fortlaufend: %q → %q", chunks, out.Text)
	}
	args, dir := fakeRecord(t, argsFile)
	if !slices.Equal(args, chatArgs()) {
		t.Errorf("Prozess bekam %v, erwartet %v (Prompt nur über stdin)", args, chatArgs())
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
	out := cli.run(context.Background(), "Hallo", func(string) {})
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
	out := cli.run(ctx, "Hallo", func(string) { cancel() }) // Abbruch nach dem ersten Stück
	if d := time.Since(start); d > 10*time.Second {
		t.Errorf("Abbruch griff nicht: %v", d)
	}
	if out.Status != chatAborted || !strings.Contains(out.Error, "Abgebrochen") {
		t.Errorf("Abbruch: %+v", out)
	}
}

func TestChatCLIFehlt(t *testing.T) {
	cli := chatCLI{program: "k3c-claude-gibt-es-nicht", timeout: time.Second, workDir: t.TempDir()}
	out := cli.run(context.Background(), "Hallo", func(string) {})
	if out.Status != chatFailed || !strings.Contains(out.Error, "nicht gefunden") {
		t.Errorf("fehlende CLI: %+v", out)
	}
}

func TestChatNichtAngemeldet(t *testing.T) {
	cli, _ := fakeCLI(t, "login", 10*time.Second)
	out := cli.run(context.Background(), "Hallo", func(string) {})
	if out.Status != chatFailed || !strings.Contains(out.Error, "nicht angemeldet") {
		t.Errorf("nicht angemeldet: %+v", out)
	}
}

// Lange Meldungen werden nach Zeichen gekürzt, nie mitten in einem Umlaut.
func TestChatMeldungGekuerzt(t *testing.T) {
	msg := explainCLI(strings.Repeat("ä", 400), nil)
	if !utf8.ValidString(msg) || !strings.HasSuffix(msg, "ä …") || utf8.RuneCountInString(msg) > 400 {
		t.Errorf("gekürzte Meldung: %q", msg)
	}
}
