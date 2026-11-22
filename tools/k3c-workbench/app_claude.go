package main

import (
	"unicode/utf8"

	"k3c/tools/workkit/claude"
)

// OpenInClaude öffnet eine neue Claude-Code-Session mit dem Prompt im Eingabefeld und der Repo-Wurzel als Ordner
// (Binding; Deep Link in tools/workkit/claude).
func (a *App) OpenInClaude(prompt string) error {
	a.wait()
	if err := claude.Open(prompt, a.root); err != nil {
		return err
	}
	a.log.Info("🚀 Prompt in Claude geöffnet", "ns", "main", "zeichen", utf8.RuneCountInString(prompt))
	return nil
}
