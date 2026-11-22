package simrun

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"time"

	"k3c/tools/k3c-studio/internal/balance"
)

// Auftrag im Bericht (B-662): Nach dem Rechnen trägt die Workbench Label, Auftrag und Lauf-ID in den Bericht
// (balance-*.json, tuning-*.json) ein. Der Bericht kommt von der CLI des Checkouts, deren Format neuer sein kann als das
// der Workbench; deshalb wird er nicht neu kodiert, sondern vorn ergänzt und nur neu eingerückt (Reihenfolge und
// unbekannte Felder bleiben).

// Stamp sind die Felder, die die Workbench in einen Bericht schreibt; ältere Berichte haben sie nicht.
type Stamp struct {
	Label string `json:"label,omitempty"`
	RunID string `json:"runId,omitempty"`
	Spec  *Spec  `json:"spec,omitempty"`
}

// stampRaw liest die Felder von Stamp aus einem Bericht; spec bleibt roh, damit ein unlesbarer Auftrag nicht den
// ganzen Bericht kostet.
type stampRaw struct {
	Label string          `json:"label"`
	RunID string          `json:"runId"`
	Spec  json.RawMessage `json:"spec"`
}

// spec ist der Auftrag des Berichts; nil, wenn er fehlt oder unlesbar ist.
func (s stampRaw) spec() *Spec {
	var spec Spec
	if len(s.Spec) == 0 || json.Unmarshal(s.Spec, &spec) != nil {
		return nil
	}
	return &spec
}

// readStamped stempelt den neuesten Bericht des Laufs und liest dann Urteil und Zeilen (readResult). Ein Fehler beim
// Stempeln steht im Log; der Lauf zählt trotzdem.
func (g *Registry) readStamped(run *Run, dir string, spec Spec) (string, []string, error) {
	full := filepath.Join(run.root, dir)
	tune := spec.Tune != nil
	pattern := "balance-*.json"
	if tune {
		pattern = "tuning-*.json"
	}
	if file, err := newestFile(full, pattern, run.Started()); err == nil {
		if err := stamp(run.root, file, Stamp{Label: run.label, RunID: run.id, Spec: &spec}); err != nil {
			g.log.Error("❌ auftrag nicht im bericht", "ns", "simtest", "id", run.id, "error", err.Error())
		}
	}
	return readResult(full, run.Started(), tune)
}

// stamp schreibt st an den Anfang des JSON-Objekts in file; ein schon gestempelter Bericht bleibt, wie er ist.
func stamp(root, file string, st Stamp) error {
	raw, err := os.ReadFile(file)
	if err != nil {
		return err
	}
	var have stampRaw
	if err := json.Unmarshal(raw, &have); err != nil {
		return fmt.Errorf("bericht unlesbar: %w", err)
	}
	head, err := json.Marshal(st)
	if err != nil || have.RunID != "" || len(head) <= 2 {
		return err
	}
	body := bytes.TrimSpace(raw)
	if len(body) < 2 || body[0] != '{' {
		return errors.New("bericht ist kein JSON-Objekt")
	}
	rest := bytes.TrimSpace(body[1:]) // Felder des Berichts samt schließender Klammer
	sep := []byte(",")
	if rest[0] == '}' {
		sep = nil
	}
	var out bytes.Buffer
	if err := json.Indent(&out, slices.Concat(head[:len(head)-1], sep, rest), "", "  "); err != nil {
		return err
	}
	out.WriteByte('\n')
	tmp := file + ".tmp"
	if err := balance.WriteFile(root, tmp, out.Bytes()); err != nil {
		return err
	}
	return replaceFile(root, tmp, file)
}

// replaceFile ersetzt file durch tmp. Unter Windows scheitert das Umbenennen, solange ein anderer Leser (z. B. die
// Berichtsliste nach balance:reports) die Datei offen hat; deshalb einige kurze Wiederholungen. Bleibt es dabei,
// fällt tmp weg und der Bericht bleibt unverändert (nur ohne Stempel).
func replaceFile(root, tmp, file string) error {
	var err error
	for range 5 {
		if err = balance.Rename(root, tmp, file); err == nil {
			return nil
		}
		time.Sleep(20 * time.Millisecond)
	}
	_ = balance.Remove(root, tmp)
	return err
}
