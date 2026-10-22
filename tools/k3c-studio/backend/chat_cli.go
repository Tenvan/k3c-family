package backend

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"slices"
	"strings"
	"sync"
	"time"
)

// Anbindung der Claude Code CLI für den Chat des Studios (B-488, WZ19.1): ein Prozess je Nachricht, ohne Shell, ohne
// Werkzeuge und Schreibrechte, in einem festen leeren Ordner außerhalb des Checkouts; die Antwort kommt fortlaufend.

// chatProgram wird über exec.LookPath gefunden (unter Windows auch claude.exe oder claude.cmd); Variable, damit der
// Test-Riegel in TestMain sie sperren kann (B-514): kein Test startet das echte claude.
var chatProgram = "claude"

const (
	chatTimeout = 3 * time.Minute // Zeitlimit einer Antwort
	chatWorkDir = "k3c-studio-chat"
	// chatSystemFlag hängt die Instruktion eines Auftrags aus einer Datei an (belegt mit claude --help 2.1.304); als
	// Datei, weil Windows die Kommandozeile auf 32 KiB begrenzt.
	chatSystemFlag = "--append-system-prompt-file"
)

// Zustand einer Antwort.
const (
	chatDone    = "fertig"
	chatAborted = "abgebrochen" // Zeitlimit oder Abbruch auf Wunsch; der Text bis dahin bleibt
	chatFailed  = "fehler"
)

// chatArgs ist der Aufruf (belegt mit claude --help, Version 2.1.304; --model: Alias fable, opus, sonnet, haiku; --effort: low, medium, high,
// xhigh, max): -p nicht-interaktiv mit dem Prompt über stdin,
// stream-json mit Teil-Nachrichten für die fortlaufende Antwort (--verbose verlangt die CLI dazu), --tools "" ohne
// Werkzeuge, dontAsk und keine Rückfragen (alles nicht Erlaubte wird abgelehnt), keine MCP-Server, --safe-mode ohne
// CLAUDE.md, Hooks, Plugins und Skills, kein gespeicherter Verlauf der CLI. Modell und Denkstufe kommen nur dazu,
// wenn gewählt (--model, --effort); die Sicherheits-Argumente bleiben gleich.
func chatArgs(o chatOptions) []string {
	return append([]string{"-p", "--output-format", "stream-json", "--verbose", "--include-partial-messages",
		"--tools", "", "--permission-mode", "dontAsk", "--permission-prompts", "none",
		"--strict-mcp-config", "--safe-mode", "--no-session-persistence"}, o.args()...)
}

// chatKeepEnv sind Einstellungen von 🧑 mit dem Präfix CLAUDE_CODE_, die an die CLI durchgehen (Anmeldung, Anbieter,
// Pfad der Git Bash, Ausgabegrenze); sie gehören nicht zu einer äußeren Sitzung.
var chatKeepEnv = []string{"CLAUDE_CODE_GIT_BASH_PATH", "CLAUDE_CODE_USE_BEDROCK", "CLAUDE_CODE_USE_VERTEX",
	"CLAUDE_CODE_USE_FOUNDRY", "CLAUDE_CODE_MAX_OUTPUT_TOKENS", "CLAUDE_CODE_OAUTH_TOKEN"}

// chatEnviron ist die Umgebung der CLI ohne geerbte Einstellungen einer äußeren Claude-Code-Sitzung (CLAUDE_CODE_*
// außer chatKeepEnv, CLAUDECODE, ANTHROPIC_BASE_URL): sonst fragt die CLI nicht mit der Anmeldung von 🧑, sondern mit
// der der Sitzung (B-515).
func chatEnviron(env []string) []string {
	out := make([]string, 0, len(env))
	for _, kv := range env {
		name, _, _ := strings.Cut(kv, "=")
		name = strings.ToUpper(name) // Windows unterscheidet die Schreibweise nicht
		if (strings.HasPrefix(name, "CLAUDE_CODE_") && !slices.Contains(chatKeepEnv, name)) || name == "CLAUDECODE" ||
			name == "ANTHROPIC_BASE_URL" {
			continue
		}
		out = append(out, kv)
	}
	return out
}

// chatCLI startet die CLI; die Felder sind die Naht für den Test-Ersatz.
type chatCLI struct {
	program string
	timeout time.Duration
	workDir string   // fester leerer Arbeitsordner (kein CLAUDE.md, keine Dateien als Kontext)
	env     []string // zusätzliche Umgebung (Test-Ersatz)
	log     *slog.Logger
}

func defaultCLI() chatCLI {
	return chatCLI{program: chatProgram, timeout: chatTimeout, workDir: filepath.Join(os.TempDir(), chatWorkDir)}
}

// chatOutcome ist das Ergebnis einer Antwort.
type chatOutcome struct {
	Text   string
	Status string
	Error  string
}

// run sendet prompt mit der Wahl opts an die CLI und meldet jedes Stück der Antwort an emit. Fehler enden als Outcome, nie als Panik.
func (c chatCLI) run(ctx context.Context, prompt string, opts chatOptions, emit func(string)) chatOutcome {
	log := c.logger()
	path, err := exec.LookPath(c.program)
	if err != nil {
		log.Warn("❌ Chat: Claude Code CLI nicht gefunden", "ns", "main", "program", c.program)
		return chatOutcome{Status: chatFailed, Error: "Claude Code CLI nicht gefunden: „" + c.program +
			"“ ist nicht im PATH. Claude Code installieren und anmelden, dann den Server neu starten."}
	}
	if err := os.MkdirAll(c.workDir, 0o755); err != nil {
		return chatOutcome{Status: chatFailed, Error: "Arbeitsordner des Chats fehlt: " + err.Error()}
	}
	args := chatArgs(opts)
	if opts.instruction != "" {
		file, err := c.instructionFile(opts.instruction)
		if err != nil {
			return chatOutcome{Status: chatFailed, Error: "Grundgerüst nicht geschrieben: " + err.Error()}
		}
		defer func() { _ = os.Remove(file) }()
		args = append(args, chatSystemFlag, file)
	}
	runCtx, cancel := context.WithTimeout(ctx, c.timeout)
	defer cancel()
	stream := &chatStream{emit: emit}
	var stderr limitedBuffer
	cmd := exec.CommandContext(runCtx, path, args...)
	cmd.Dir = c.workDir
	cmd.Env = append(chatEnviron(os.Environ()), c.env...)
	cmd.Stdin = strings.NewReader(prompt)
	lines := &lineWriter{line: stream.handle}
	cmd.Stdout = lines
	cmd.Stderr = &stderr
	cmd.WaitDelay = 2 * time.Second // hält ein Enkelprozess die Ausgabe offen, endet Wait trotzdem
	start := time.Now()
	log.Info("🚀 Chat: Claude Code gestartet", "ns", "main", "zeichen", len(prompt), "modell", opts.Model,
		"denkstufe", opts.Effort)
	werr := c.runTree(cmd, log)
	lines.flush()
	out := c.outcome(ctx, runCtx, stream, werr, stderr.String())
	c.logOutcome(out, time.Since(start))
	return out
}

// runTree startet den Prozess und wartet; ein Abbruch (Kontext oder Zeitlimit) beendet den ganzen Prozessbaum (B-514).
func (c chatCLI) runTree(cmd *exec.Cmd, log *slog.Logger) error {
	prepareTree(cmd)
	var mu sync.Mutex
	var job chatTree
	cmd.Cancel = func() error {
		mu.Lock()
		defer mu.Unlock()
		return killTree(cmd, job)
	}
	if err := cmd.Start(); err != nil {
		return err
	}
	mu.Lock()
	var berr error
	job, berr = bindTree(cmd)
	mu.Unlock()
	if berr != nil {
		log.Warn("❌ Chat: Prozessbaum nicht gebunden, ein Abbruch trifft nur die CLI", "ns", "main", "fehler", berr)
	}
	defer closeTree(job)
	return cmd.Wait()
}

// instructionFile schreibt die Instruktion in eine eigene Datei neben dem Arbeitsordner (der bleibt leer); der Name ist je
// Lauf eindeutig, weil mehrere Chats zugleich antworten dürfen. Der Aufrufer löscht sie.
func (c chatCLI) instructionFile(text string) (string, error) {
	f, err := os.CreateTemp(filepath.Dir(c.workDir), "k3c-chat-instruktion-*.md")
	if err != nil {
		return "", err
	}
	_, werr := f.WriteString(text)
	if err := errors.Join(werr, f.Close()); err != nil {
		_ = os.Remove(f.Name())
		return "", err
	}
	return f.Name(), nil
}

func (c chatCLI) logger() *slog.Logger {
	if c.log == nil {
		return slog.Default()
	}
	return c.log
}

// outcome ordnet das Ende des Prozesses: Abbruch vor Zeitlimit vor Fehler der CLI.
func (c chatCLI) outcome(ctx, runCtx context.Context, s *chatStream, werr error, stderr string) chatOutcome {
	out := chatOutcome{Text: s.text(), Status: chatDone}
	switch {
	case ctx.Err() != nil:
		out.Status, out.Error = chatAborted, "Abgebrochen"
	case errors.Is(runCtx.Err(), context.DeadlineExceeded):
		out.Status, out.Error = chatAborted, fmt.Sprintf("Zeitlimit von %v erreicht", c.timeout)
	case s.failure() != "" || werr != nil:
		out.Status, out.Error = chatFailed, explainCLI(s.failure()+"\n"+stderr, werr)
	}
	return out
}

func (c chatCLI) logOutcome(out chatOutcome, d time.Duration) {
	log := c.logger()
	switch {
	case out.Status == chatDone:
		log.Info("✅ Chat: Antwort fertig", "ns", "main", "zeichen", len(out.Text), "dauer", d.Round(time.Millisecond))
	case strings.HasPrefix(out.Error, "Zeitlimit"):
		log.Warn("⏳ Chat: Zeitlimit erreicht, Claude Code beendet", "ns", "main", "dauer", d.Round(time.Millisecond))
	case out.Status == chatAborted:
		log.Info("🛑 Chat: auf Wunsch abgebrochen", "ns", "main", "dauer", d.Round(time.Millisecond))
	default:
		log.Warn("❌ Chat: Claude Code meldet einen Fehler", "ns", "main", "fehler", out.Error)
	}
}

// explainCLI macht aus der Meldung der CLI einen verständlichen Satz; fehlende Anmeldung bekommt einen Hinweis.
func explainCLI(text string, werr error) string {
	first := ""
	for _, line := range strings.Split(text, "\n") {
		if line = strings.TrimSpace(line); line != "" {
			first = line
			break
		}
	}
	if first == "" && werr != nil {
		first = werr.Error()
	}
	if r := []rune(first); len(r) > 300 {
		first = string(r[:300]) + " …"
	}
	low := strings.ToLower(text)
	if strings.Contains(low, "unknown option") && strings.Contains(low, chatSystemFlag) {
		return "Diese Claude-Code-Version kennt " + chatSystemFlag + " nicht, bitte Claude Code aktualisieren („claude update“)"
	}
	for _, hint := range []string{"login", "log in", "logged in", "authenticat", "api key", "oauth"} {
		if strings.Contains(low, hint) {
			return "Claude Code ist nicht angemeldet: im Terminal einmal „claude“ starten und anmelden (" + first + ")"
		}
	}
	return "Claude Code meldet einen Fehler: " + first
}

// chatStream liest die Zeilen von --output-format stream-json.
type chatStream struct {
	emit     func(string)
	mu       sync.Mutex
	buf      strings.Builder
	sawDelta bool
	failMsg  string
}

type streamLine struct {
	Type  string `json:"type"`
	Event *struct {
		Type  string `json:"type"`
		Delta *struct {
			Type string `json:"type"`
			Text string `json:"text"`
		} `json:"delta"`
	} `json:"event"`
	Message *struct {
		Content []struct {
			Type string `json:"type"`
			Text string `json:"text"`
		} `json:"content"`
	} `json:"message"`
	IsError bool   `json:"is_error"`
	Result  string `json:"result"`
}

// handle verarbeitet eine Zeile: Text-Stücke (stream_event), sonst die ganze Nachricht (assistant), am Ende result.
func (s *chatStream) handle(raw []byte) {
	var l streamLine
	if json.Unmarshal(raw, &l) != nil {
		return // keine JSON-Zeile (z. B. Hinweis der CLI): gehört nicht zur Antwort
	}
	if text, ok := l.delta(); ok {
		s.add(text, true)
		return
	}
	switch {
	case l.Type == "assistant" && l.Message != nil && !s.sawDelta:
		for _, c := range l.Message.Content {
			if c.Type == "text" {
				s.add(c.Text, false)
			}
		}
	case l.Type == "result" && l.IsError:
		s.mu.Lock()
		s.failMsg = l.Result
		s.mu.Unlock()
	case l.Type == "result" && s.text() == "":
		s.add(l.Result, false)
	}
}

// delta ist ein Text-Stück einer Teil-Nachricht (--include-partial-messages).
func (l streamLine) delta() (string, bool) {
	if l.Type != "stream_event" || l.Event == nil || l.Event.Type != "content_block_delta" || l.Event.Delta == nil {
		return "", false
	}
	return l.Event.Delta.Text, l.Event.Delta.Type == "text_delta"
}

func (s *chatStream) add(text string, delta bool) {
	if text == "" {
		return
	}
	s.mu.Lock()
	s.buf.WriteString(text)
	s.sawDelta = s.sawDelta || delta
	s.mu.Unlock()
	s.emit(text)
}

func (s *chatStream) text() string {
	s.mu.Lock()
	defer s.mu.Unlock()
	return s.buf.String()
}

func (s *chatStream) failure() string {
	s.mu.Lock()
	defer s.mu.Unlock()
	return s.failMsg
}

// lineWriter teilt die Ausgabe des Prozesses in Zeilen.
type lineWriter struct {
	line func([]byte)
	rest []byte
}

func (w *lineWriter) Write(p []byte) (int, error) {
	w.rest = append(w.rest, p...)
	for {
		i := bytes.IndexByte(w.rest, '\n')
		if i < 0 {
			break
		}
		w.line(bytes.TrimRight(w.rest[:i], "\r"))
		w.rest = w.rest[i+1:]
	}
	return len(p), nil
}

// flush gibt die letzte Zeile ohne Zeilenende weiter.
func (w *lineWriter) flush() {
	if len(w.rest) > 0 {
		w.line(bytes.TrimRight(w.rest, "\r"))
		w.rest = nil
	}
}

// limitedBuffer hält höchstens 8 KiB der Fehlerausgabe.
type limitedBuffer struct{ bytes.Buffer }

func (b *limitedBuffer) Write(p []byte) (int, error) {
	if room := 8<<10 - b.Len(); room > 0 {
		b.Buffer.Write(p[:min(len(p), room)])
	}
	return len(p), nil
}
