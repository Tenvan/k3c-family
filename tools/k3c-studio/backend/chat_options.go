package backend

import "errors"

// Wahlmöglichkeiten des Chats (Anbieter, Modell, Denkstufe): eine Tabelle je Wert, aus ihr kommen die Prüfung der
// Anfrage und die Liste für die Oberfläche (ChatOptions). Leer heißt immer: Standard der CLI, kein Argument.

const chatProviderCLI = "claude-cli"

// ChatChoice ist ein Eintrag einer Auswahl; die id geht an die CLI, das label zeigt die Oberfläche.
type ChatChoice struct {
	ID    string `json:"id"`
	Label string `json:"label"`
}

// ChatOptionsView sind die erlaubten Werte für die Auswahl im Chat.
type ChatOptionsView struct {
	Providers []ChatChoice `json:"providers"`
	Models    []ChatChoice `json:"models"`
	Efforts   []ChatChoice `json:"efforts"`
}

var (
	chatProviders = []ChatChoice{{"", "Standard"}, {chatProviderCLI, "Claude Code CLI"}}
	chatModels    = []ChatChoice{{"", "Standard"}, {"fable", "Fable"}, {"opus", "Opus"}, {"sonnet", "Sonnet"},
		{"haiku", "Haiku"}}
	chatEfforts = []ChatChoice{{"", "Standard"}, {"low", "Niedrig"}, {"medium", "Mittel"}, {"high", "Hoch"},
		{"xhigh", "Sehr hoch"}, {"max", "Maximum"}}
)

// chatOptions ist die Wahl für eine Antwort; Felder bleiben leer, wenn nichts gewählt ist.
type chatOptions struct {
	Provider string
	Model    string // Alias der CLI (--model)
	Effort   string // Denkstufe (--effort)
	// instruction ist die System-Instruktion des Auftrags (Welt-Auftrag: Grundgerüst, B-516); geht als Datei an die
	// CLI (--append-system-prompt-file), nicht in den Verlauf. Leer = freier Chat.
	instruction string
}

func inChoices(list []ChatChoice, id string) bool {
	for _, c := range list {
		if c.ID == id {
			return true
		}
	}
	return false
}

// validate lehnt Werte außerhalb der Tabellen ab.
func (o chatOptions) validate() error {
	switch {
	case !inChoices(chatProviders, o.Provider):
		return errors.New("unbekannter Provider")
	case !inChoices(chatModels, o.Model):
		return errors.New("unbekanntes Modell")
	case !inChoices(chatEfforts, o.Effort):
		return errors.New("unbekannte Denkstufe")
	}
	return nil
}

// args sind die zusätzlichen Argumente der CLI für die Wahl.
func (o chatOptions) args() []string {
	var a []string
	if o.Model != "" {
		a = append(a, "--model", o.Model)
	}
	if o.Effort != "" {
		a = append(a, "--effort", o.Effort)
	}
	return a
}

// ChatOptions nennt die erlaubten Anbieter, Modelle und Denkstufen.
func (a *App) ChatOptions() ChatOptionsView {
	return ChatOptionsView{Providers: chatProviders, Models: chatModels, Efforts: chatEfforts}
}
