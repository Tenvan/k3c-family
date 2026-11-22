// Package claude: „In Claude öffnen“ startet eine neue Code-Session in Claude Desktop mit vorbefülltem Prompt über den
// Deep Link claude://code/new (https://support.claude.com/en/articles/14729294-open-claude-desktop-with-a-link).
// Abgeschickt wird nichts: Claude Desktop füllt nur das Eingabefeld, Enter bleibt beim Nutzer. Gemeinsam für
// k3c-workbench und k3c-studio (WZ63.3).
package claude

import (
	"fmt"
	"net/url"
	"os/exec"
	"strings"
	"unicode/utf8"
)

// MaxPrompt: Claude Desktop kürzt q bei rund 14.000 Zeichen; lieber ablehnen als still kürzen.
const MaxPrompt = 14000

// maxLink hält den Link unter der Windows-Grenze für eine Befehlszeile (32.767).
const maxLink = 32000

// Link baut den Deep Link. Leerzeichen als %20 statt +: Ein `+` im Prompt ist kodiert immer %2B.
func Link(prompt, folder string) (string, error) {
	if strings.TrimSpace(prompt) == "" {
		return "", fmt.Errorf("prompt ist leer")
	}
	if n := utf8.RuneCountInString(prompt); n > MaxPrompt {
		return "", fmt.Errorf("prompt hat %d Zeichen, Claude Desktop übernimmt höchstens %d", n, MaxPrompt)
	}
	q := url.Values{"q": {prompt}}
	if folder != "" {
		q.Set("folder", folder)
	}
	link := "claude://code/new?" + strings.ReplaceAll(q.Encode(), "+", "%20")
	if len(link) > maxLink {
		return "", fmt.Errorf("prompt ist kodiert zu lang für einen Link (%d Zeichen)", len(link))
	}
	return link, nil
}

// Open öffnet eine neue Claude-Code-Session mit dem Prompt im Eingabefeld und folder als Ordner. Bewusst ohne
// proc.Command: dessen Job Object würde ein von rundll32 gestartetes Claude Desktop beim Beenden der App mitreißen.
// ponytail: nur Windows (rundll32), die Apps laufen nur dort.
func Open(prompt, folder string) error {
	link, err := Link(prompt, folder)
	if err != nil {
		return err
	}
	cmd := exec.Command("rundll32", "url.dll,FileProtocolHandler", link)
	if err := cmd.Start(); err != nil {
		return fmt.Errorf("claude desktop nicht startbar: %w", err)
	}
	go func() { _ = cmd.Wait() }()
	return nil
}
