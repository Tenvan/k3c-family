// Package gameexe startet die Spiel-EXE (`k3c-game sim …`, docs/standards/k3c-game-laeufe.md) als Kindprozess der
// Workbench oder des Studios: EXE des Checkouts suchen und auf Frische prüfen (find.go), JSON-Zeilen lesen (event.go),
// Lauf starten und abbrechen (run.go).
package gameexe

import "encoding/json"

// Event ist eine JSON-Zeile von stdout der EXE; die Felder entsprechen den Zeilen der Schnittstelle (progress, result,
// report, error, aborted).
type Event struct {
	Event   string          `json:"event"`
	Done    int             `json:"done,omitempty"`
	Total   int             `json:"total,omitempty"`
	Text    string          `json:"text,omitempty"`
	Message string          `json:"message,omitempty"`
	Files   []string        `json:"files,omitempty"`
	Report  json.RawMessage `json:"report,omitempty"`
}

// ParseEvent liest eine stdout-Zeile; false, wenn sie keine JSON-Zeile mit "event" ist.
func ParseEvent(line string) (Event, bool) {
	var ev Event
	if json.Unmarshal([]byte(line), &ev) != nil || ev.Event == "" {
		return Event{}, false
	}
	return ev, true
}
