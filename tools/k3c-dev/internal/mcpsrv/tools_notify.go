package mcpsrv

import (
	"context"
	"errors"
	"fmt"
	"slices"
	"strings"
)

// noticeLevels sind die gültigen Stufen eines Hinweises.
var noticeLevels = []string{"info", "success", "warn", "error"}

// Notice ist ein Hinweis an den Nutzer (notify_ui); die Oberfläche zeigt ihn als Meldung.
type Notice struct {
	Level string `json:"level" jsonschema:"info, success, warn oder error"`
	Title string `json:"title" jsonschema:"kurze Überschrift"`
	Text  string `json:"text,omitempty" jsonschema:"optionaler Text darunter"`
}

// notifyUI ist das Tool notify_ui.
func (s *Server) notifyUI(_ context.Context, in Notice) (string, error) {
	if !slices.Contains(noticeLevels, in.Level) {
		return "", fmt.Errorf("level %q ungültig: erlaubt %s", in.Level, strings.Join(noticeLevels, ", "))
	}
	if strings.TrimSpace(in.Title) == "" {
		return "", errors.New("title fehlt")
	}
	if s.cfg.OnNotify == nil {
		return "", errors.New("keine Oberfläche: k3c-dev läuft ohne Fenster")
	}
	s.cfg.OnNotify(in)
	return "angezeigt: " + in.Title, nil
}
