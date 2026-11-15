package planning

import (
	"fmt"
	"strings"
)

// maxItems begrenzt einen Aufruf von Items; mehr IDs sind ein Fehler der Oberfläche (sie lädt dann die Liste neu).
const maxItems = 100

// Item ist ein einzelnes Planungs-Dokument mit Fließtext (B-909): Ticket, Sprint-README, Session oder Projekt. Die
// Listen von LoadList tragen keinen Fließtext; die Oberfläche holt ihn je ID mit Items.
type Item struct {
	ID    string `json:"id"`
	Kind  string `json:"kind"`            // ticket | sprint | session | projekt
	Path  string `json:"path"`            // Datei relativ zum Repo (docs/…)
	Text  string `json:"text"`            // Markdown der Datei mit `\n`, beim Ticket um die Rückverweise ergänzt wie Get
	Error string `json:"error,omitempty"` // unbekannte oder ungültige ID; Text und Path dann leer
}

// LoadList liest die Planung wie Load, aber ohne die Fließtexte der Session-Dateien (Session.Text, den Löwenanteil der
// Daten, B-909). Titel, Umgebung, Domäne und Abhängigkeiten bleiben: die Listen brauchen sie.
func LoadList(root string) (Data, error) {
	d, err := Load(root)
	for i := range d.Sprints {
		for j := range d.Sprints[i].Sessions {
			d.Sprints[i].Sessions[j].Text = ""
		}
	}
	return d, err
}

// Items liefert Pfad und Fließtext zu den IDs (B-123, WZ61, WZ61.1, GRA) in der Reihenfolge der Anfrage; Doppelte
// zählen einmal. Eine unbekannte ID ergibt einen Eintrag mit Error, keinen Fehler des ganzen Aufrufs.
func Items(root string, ids []string) ([]Item, error) {
	if len(ids) > maxItems {
		return nil, fmt.Errorf("höchstens %d IDs je Aufruf, gefragt %d", maxItems, len(ids))
	}
	seen := map[string]bool{}
	out := make([]Item, 0, len(ids))
	for _, id := range ids {
		if seen[id] {
			continue
		}
		seen[id] = true
		r, err := resolve(root, id)
		if err != nil {
			out = append(out, Item{ID: id, Error: err.Error()})
			continue
		}
		get, err := getRef(root, r)
		if err != nil {
			out = append(out, Item{ID: id, Kind: r.kind, Error: err.Error()})
			continue
		}
		_, text, _ := strings.Cut(get, "\n\n") // Get setzt den Pfad und eine Leerzeile davor
		out = append(out, Item{ID: id, Kind: r.kind, Path: "docs/" + r.rel, Text: text})
	}
	return out, nil
}
