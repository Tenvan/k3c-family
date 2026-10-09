package mcpsrv

import (
	"encoding/json"
	"fmt"
	"slices"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/logs"
)

// Kennzahlen der Clients (B-348/AC-04) aus dem Log k3c-client seit Laufbeginn: Meldungen, deren url den Raumcode
// eines Clients des Laufs trägt. Diagnose-Zeilen haben im ctx die Felder fps und latencyMs (optional bufferMs);
// Fehler-Meldungen zählen für stability.

// clientMetrics sind die Kennzahlen aller Clients eines Laufs.
type clientMetrics struct {
	diag           int // Diagnose-Zeilen
	fpsMin, fpsSum float64
	latMax, bufMax float64
	errors         int
}

// readClientMetrics liest das Log; eine fehlende Datei ergibt leere Kennzahlen.
func readClientMetrics(path string, since time.Time, codes []string) clientMetrics {
	var m clientMetrics
	res, err := logs.Scan(path, logs.Query{Since: since})
	if err != nil {
		return m
	}
	for _, e := range res.Entries {
		url, _ := e.Data["url"].(string)
		if !slices.ContainsFunc(codes, func(c string) bool { return strings.Contains(strings.ToUpper(url), "ROOM="+c) }) {
			continue
		}
		if e.Level == "ERROR" {
			m.errors++
		}
		m.add(e.Data["ctx"])
	}
	return m
}

func (m *clientMetrics) add(raw any) {
	text, _ := raw.(string)
	var ctx map[string]any
	if text == "" || json.Unmarshal([]byte(text), &ctx) != nil {
		return
	}
	fps, ok := ctx["fps"].(float64)
	if !ok {
		return
	}
	if m.diag == 0 || fps < m.fpsMin {
		m.fpsMin = fps
	}
	m.diag++
	m.fpsSum += fps
	if lat, ok := ctx["latencyMs"].(float64); ok {
		m.latMax = max(m.latMax, lat)
	}
	if buf, ok := ctx["bufferMs"].(float64); ok {
		m.bufMax = max(m.bufMax, buf)
	}
}

func (m clientMetrics) line() string {
	if m.diag == 0 {
		return "Client-FPS und Latenz: keine Diagnose-Zeilen im Client-Log (fps, latencyMs im ctx)"
	}
	return fmt.Sprintf("Client-FPS min %.0f, Mittel %.0f; Latenz max %.0f ms, Puffer max %.0f ms (%d Diagnose-Zeilen)",
		m.fpsMin, m.fpsSum/float64(m.diag), m.latMax, m.bufMax, m.diag)
}

// clientLines: Kennzahlen des Servers wie headless, die erste Zeile für Clients, dazu Feed und Client-Log.
func clientLines(spec simSpec, o *onlineRun, seats []*clientSeat, newFailures int, m clientMetrics) []string {
	lines := o.lines(spec, newFailures)
	how := "gestartet"
	if spec.attach != "" {
		how = "angehängt an " + spec.attach
	}
	lines[0] = fmt.Sprintf("%d Clients × %d Spieler (%s), %s, +1 Beobachter-Bot je Raum, %d Proben",
		spec.clients, spec.players, spec.bots, how, o.samples)
	now, never, sent := feedTotals(seats)
	feed := fmt.Sprintf("Feed: verbunden %d/%d, nie verbunden %d, Kommandos %d", now, len(seats), never, sent)
	if spec.attach != "" && len(seats) > 0 {
		feed += " · Client öffnen mit ?botfeed=<k3c-dev>" + "/bot/" + seats[0].feedName
	}
	return append(lines, feed, m.line(), fmt.Sprintf("Client-Fehler %d", m.errors))
}

// clientVerdict wie headless; stability scheitert zusätzlich an einem Client, der den Feed nie geöffnet hat, oder an
// Fehlern im Client-Log. FPS bewertet der Lauf nicht (kein Ziel in der Spec), er zeigt sie nur.
func clientVerdict(spec simSpec, o *onlineRun, seats []*clientSeat, m clientMetrics, newFailures int) string {
	v := o.verdict(spec, newFailures)
	_, never, _ := feedTotals(seats)
	if !slices.Contains(spec.focus, "stability") || never+m.errors == 0 || strings.Contains(v, "stability") {
		return v
	}
	if v == "Pass" {
		return "Fail (stability)"
	}
	return strings.TrimSuffix(v, ")") + ", stability)"
}
