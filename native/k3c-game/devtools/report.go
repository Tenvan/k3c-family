package devtools

// Sitzungszusammenfassung (B-889): am Ende eines Spiels (Spielende, Verlassen, Programmende) eine JSON-Datei
// native-session-<Zeit>.json in reports/ (lesbar über report_read der Workbench). Ein Schreibfehler steht im Log.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
)

// Report ist die Sitzungszusammenfassung.
type Report struct {
	Kind     string  `json:"kind"` // immer "native-session"
	Seed     string  `json:"seed"`
	Start    string  `json:"start"`
	End      string  `json:"end"`
	Seconds  float64 `json:"dauerSekunden"`
	Ticks    uint64  `json:"takte"`
	FirstDay int     `json:"ersterTag"`
	LastDay  int     `json:"letzterTag"`
	Days     int     `json:"tage"`
	Cause    string  `json:"endeUrsache"` // gewonnen, verloren, verlassen, beendet
}

// report baut die Zusammenfassung der laufenden Sitzung.
func (c *chronicle) report(cause string) Report {
	end := c.now()
	return Report{Kind: "native-session", Seed: c.seed, Start: c.start.Format("2006-01-02T15:04:05"),
		End: end.Format("2006-01-02T15:04:05"), Seconds: round1(end.Sub(c.start).Seconds()), Ticks: c.tick - c.startTick,
		FirstDay: c.firstDay, LastDay: c.day, Days: c.day - c.firstDay + 1, Cause: cause}
}

// finish schreibt die Zusammenfassung, einmal je Sitzung.
func (c *chronicle) finish(cause string) {
	if c.written || c.dir == "" {
		return
	}
	c.written = true
	r := c.report(cause)
	if path, err := writeReport(c.dir, r); err != nil {
		c.log.Warn("💥 Sitzungsbericht nicht geschrieben", "err", err)
	} else {
		c.log.Info("💾 Sitzungsbericht", "datei", path, "seed", r.Seed, "tage", r.Days, "ursache", r.Cause)
	}
}

// Finish beendet die Sitzung beim Programmende (view.Finisher).
func (c *chronicle) Finish() {
	if c.active {
		c.finish("beendet")
	}
	c.active = false
}

// writeReport schreibt r als JSON nach dir und liefert den Pfad.
func writeReport(dir string, r Report) (string, error) {
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return "", err
	}
	raw, err := json.MarshalIndent(r, "", "  ")
	if err != nil {
		return "", err
	}
	name := fmt.Sprintf("native-session-%s.json", r.End[:len("2006-01-02T15:04:05")])
	name = filepath.Join(dir, sanitize(name))
	return name, os.WriteFile(name, raw, 0o644)
}

// sanitize ersetzt die Doppelpunkte der Uhrzeit (unter Windows nicht im Dateinamen erlaubt).
func sanitize(name string) string {
	out := []byte(name)
	for i, b := range out {
		if b == ':' {
			out[i] = '-'
		}
	}
	return string(out)
}
