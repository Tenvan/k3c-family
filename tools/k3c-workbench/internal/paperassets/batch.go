package paperassets

// Sammelaufruf der Render-CLI (B-856): `batch <schritte.json>` führt die Schritte eines Set in einem bun-Prozess aus
// (vorher bis zu zwölf Starts je Änderung). Jeder Schritt wird wie ein Einzelaufruf gedeutet (review.Auswerten).

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strconv"

	"k3c/tools/k3c-workbench/internal/review"
)

// Schritt ist ein Befehl im Sammelaufruf: Args wie auf der Kommandozeile; Stopp: scheitert er, werden alle folgenden
// übersprungen; Braucht: Indizes früherer Schritte, die gelingen müssen, sonst wird er übersprungen.
type Schritt struct {
	Args    []string `json:"args"`
	Stopp   bool     `json:"stopp,omitempty"`
	Braucht []int    `json:"braucht,omitempty"`
}

// SchrittErgebnis ist die Antwort eines Schritts wie bei run: Raw (Exit 0) oder Err (*Abgelehnt, Aufruffehler,
// errUebersprungen).
type SchrittErgebnis struct {
	Raw json.RawMessage
	Err error
}

// errUebersprungen meldet einen Schritt, den die CLI wegen Stopp oder Braucht nicht ausgeführt hat.
var errUebersprungen = errors.New("Schritt übersprungen") //nolint:staticcheck // Satz für die Antwort

// Batch führt schritte in einem CLI-Aufruf aus. Ein Fehler des Aufrufs selbst (bun fehlt, Zeitlimit, Exit 2) gilt
// für alle Schritte und kommt als err; sonst je Schritt ein Ergebnis.
func (c CLI) Batch(ctx context.Context, schritte []Schritt) ([]SchrittErgebnis, error) {
	tmp, err := TempOrdner(c.Root)
	if err != nil {
		return nil, err
	}
	defer func() { _ = os.RemoveAll(tmp) }()
	datei := filepath.Join(tmp, "schritte.json")
	b, err := json.Marshal(schritte)
	if err != nil {
		return nil, err
	}
	if err := os.WriteFile(datei, b, 0o644); err != nil {
		return nil, err
	}
	// Zeitlimit: drei Einzel-Limits (vorher liefen validate+diff, die Frames parallel und render nacheinander mit je
	// eigenem Limit); begrenzt eine hängende CLI weiter.
	raw, err := review.Aufruf(ctx, c.Root, c.Skript, 3*cliZeit, "batch", datei)
	if err != nil {
		return nil, c.alsAbgelehnt(err)
	}
	var antwort struct {
		Schritte []struct {
			Code          *int            `json:"code"`
			Antwort       json.RawMessage `json:"antwort"`
			Stderr        string          `json:"stderr"`
			Uebersprungen bool            `json:"uebersprungen"`
		} `json:"schritte"`
	}
	if err := json.Unmarshal(raw, &antwort); err != nil || len(antwort.Schritte) != len(schritte) {
		return nil, fmt.Errorf("Render-CLI batch: Antwort passt nicht zu %d Schritten", len(schritte)) //nolint:staticcheck // Satz für die Antwort
	}
	out := make([]SchrittErgebnis, len(schritte))
	for i, s := range antwort.Schritte {
		switch {
		case s.Uebersprungen || s.Code == nil:
			out[i].Err = errUebersprungen
		default:
			raw, err := review.Auswerten(schritte[i].Args[0], *s.Code, s.Antwort, s.Stderr, "Exit "+strconv.Itoa(*s.Code))
			out[i] = SchrittErgebnis{Raw: raw, Err: c.alsAbgelehnt(err)}
		}
	}
	return out, nil
}

// alsAbgelehnt übersetzt review.Abgelehnt in *Abgelehnt wie run.
func (c CLI) alsAbgelehnt(err error) error {
	var ab *review.Abgelehnt
	if errors.As(err, &ab) {
		return &Abgelehnt{Fehler: ab.Fehler}
	}
	return err
}

// in deutet das Ergebnis eines Schritts als JSON in ziel (wie decode).
func (s SchrittErgebnis) in(befehl string, ziel any) error {
	if s.Err != nil {
		return s.Err
	}
	if err := json.Unmarshal(s.Raw, ziel); err != nil {
		return fmt.Errorf("Render-CLI %s: Antwort kein JSON: %w", befehl, err) //nolint:staticcheck // Satz für die Antwort
	}
	return nil
}
