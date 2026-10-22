package main

// „In Claude öffnen“: startet eine neue Code-Session in Claude Desktop mit vorbefülltem Prompt über den Deep Link
// claude://code/new (https://support.claude.com/en/articles/14729294-open-claude-desktop-with-a-link). Abgeschickt
// wird nichts: Claude Desktop füllt nur das Eingabefeld, Enter bleibt beim Nutzer.

import (
	"fmt"
	"net/url"
	"os/exec"
	"strings"
	"unicode/utf8"
)

// claudeMaxPrompt: Claude Desktop kürzt q bei rund 14.000 Zeichen; lieber ablehnen als still kürzen.
const claudeMaxPrompt = 14000

// claudeMaxLink hält den Link unter der Windows-Grenze für eine Befehlszeile (32.767).
const claudeMaxLink = 32000

// claudeLink baut den Deep Link. Leerzeichen als %20 statt +: Ein `+` im Prompt ist kodiert immer %2B.
func claudeLink(prompt, folder string) (string, error) {
	if strings.TrimSpace(prompt) == "" {
		return "", fmt.Errorf("prompt ist leer")
	}
	if n := utf8.RuneCountInString(prompt); n > claudeMaxPrompt {
		return "", fmt.Errorf("prompt hat %d Zeichen, Claude Desktop übernimmt höchstens %d", n, claudeMaxPrompt)
	}
	q := url.Values{"q": {prompt}}
	if folder != "" {
		q.Set("folder", folder)
	}
	link := "claude://code/new?" + strings.ReplaceAll(q.Encode(), "+", "%20")
	if len(link) > claudeMaxLink {
		return "", fmt.Errorf("prompt ist kodiert zu lang für einen Link (%d Zeichen)", len(link))
	}
	return link, nil
}

// OpenInClaude öffnet eine neue Claude-Code-Session mit dem Prompt im Eingabefeld und der Repo-Wurzel als Ordner
// (Binding). Bewusst ohne proc.Command: dessen Job Object würde ein von rundll32 gestartetes Claude Desktop beim
// Beenden der Workbench mitreißen.
// ponytail: nur Windows (rundll32), die Workbench läuft nur dort.
func (a *App) OpenInClaude(prompt string) error {
	a.wait()
	link, err := claudeLink(prompt, a.root)
	if err != nil {
		return err
	}
	cmd := exec.Command("rundll32", "url.dll,FileProtocolHandler", link)
	if err := cmd.Start(); err != nil {
		return fmt.Errorf("claude desktop nicht startbar: %w", err)
	}
	a.log.Info("🚀 Prompt in Claude geöffnet", "ns", "main", "zeichen", utf8.RuneCountInString(prompt))
	go func() { _ = cmd.Wait() }()
	return nil
}
