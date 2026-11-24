package playtest

import (
	"bytes"
	"context"
	"fmt"
	"regexp"
	"strconv"
	"strings"
	"time"

	"k3c/tools/workkit/proc"
)

// Monitor ist ein aufnehmbarer Bildschirm (ddagrab `output_idx`), geprüft mit einem echten Probebild.
type Monitor struct {
	Index  int    `json:"index"`
	Width  int    `json:"width"`
	Height int    `json:"height"`
	Label  string `json:"label"`
}

const (
	maxMonitors   = 8
	probeTimeout  = 8 * time.Second
	micProbeSecs  = "0.4"
	monitorSizeRe = `Video:[^\n]*?(\d{3,5})x(\d{3,5})`
)

var monitorSize = regexp.MustCompile(monitorSizeRe)

// probe führt einen kurzen ffmpeg-Aufruf aus (höchstens probeTimeout) und liefert stdout und stderr.
type probe func(args []string) (string, error)

func runProbe(args []string) (string, error) {
	ctx, cancel := context.WithTimeout(context.Background(), probeTimeout)
	defer cancel()
	var buf bytes.Buffer
	c := proc.Command(ctx, args)
	c.Stdout, c.Stderr = &buf, &buf
	err := c.Run()
	return buf.String(), err
}

// MonitorProbeArgs ist der Aufruf, der ein Bild von Monitor idx holt (nichts wird geschrieben).
func MonitorProbeArgs(ff string, idx int) []string {
	return []string{ff, "-hide_banner", "-f", "lavfi", "-i", "ddagrab=output_idx=" + strconv.Itoa(idx) + ":framerate=1",
		"-frames:v", "1", "-f", "null", "-"}
}

// MicProbeArgs ist der Aufruf, der kurz vom DirectShow-Gerät mic liest (nichts wird geschrieben).
func MicProbeArgs(ff, mic string) []string {
	return []string{ff, "-hide_banner", "-f", "dshow", "-i", "audio=" + mic, "-t", micProbeSecs, "-f", "null", "-"}
}

// ParseMonitorSize liest die Bildgröße aus der Ausgabe eines Probelaufs; 0, 0 wenn sie fehlt.
func ParseMonitorSize(out string) (w, h int) {
	m := monitorSize.FindStringSubmatch(out)
	if m == nil {
		return 0, 0
	}
	w, _ = strconv.Atoi(m[1])
	h, _ = strconv.Atoi(m[2])
	return w, h
}

// listMonitors probiert die Monitore 0, 1, … bis zum ersten, der sich nicht aufnehmen lässt (die Indizes laufen
// lückenlos), und liefert nur echte Bildschirme. Leer, wenn schon Monitor 0 scheitert (z. B. kein Desktop).
func listMonitors(run probe, ff string) []Monitor {
	var out []Monitor
	for i := 0; i < maxMonitors; i++ {
		text, err := run(MonitorProbeArgs(ff, i))
		if err != nil {
			break
		}
		w, h := ParseMonitorSize(text)
		label := "Monitor " + strconv.Itoa(i+1)
		if w > 0 {
			label += fmt.Sprintf(" (%d×%d)", w, h)
		}
		out = append(out, Monitor{Index: i, Width: w, Height: h, Label: label})
	}
	return out
}

// lastError ist die letzte nichtleere Zeile der Ausgabe, wo ffmpeg den Grund nennt.
func lastError(out string) string {
	lines := strings.Split(strings.TrimSpace(out), "\n")
	for i := len(lines) - 1; i >= 0; i-- {
		if l := strings.TrimSpace(lines[i]); l != "" {
			return l
		}
	}
	return ""
}

// CheckDevices prüft vor dem Start gegen die echten Geräte: Das Mikro lässt sich öffnen und der Monitor liefert ein
// Bild. Der Fehler nennt das Gerät und den Grund von ffmpeg, damit ein Tippfehler, ein abgezogenes USB-Mikro oder
// ein Monitor-Index ohne Bildschirm vor dem Start auffallen und nicht erst als leere Aufnahme.
func CheckDevices(run probe, ff, mic string, monitor int) error {
	if out, err := run(MicProbeArgs(ff, mic)); err != nil {
		return fmt.Errorf("Mikrofon „%s“ lässt sich nicht öffnen (abgezogen, belegt oder umbenannt?): %s", mic, lastError(out)) //nolint:staticcheck // Grund für die Oberfläche
	}
	if monitor < 0 { // Mikro-Session: kein Bild
		return nil
	}
	if out, err := run(MonitorProbeArgs(ff, monitor)); err != nil {
		return fmt.Errorf("Monitor %d lässt sich nicht aufnehmen (nicht angeschlossen?): %s", monitor+1, lastError(out))
	}
	return nil
}
