package review

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	"k3c/tools/k3c-workbench/internal/proc"
)

// Renderer ist die Render-CLI (paper-spec.md › CLI-Vertrag); Tests ersetzen einzelne Befehle.
type Renderer interface {
	Validate(ctx context.Context, spec string) ([]Fehler, error)
	Diff(ctx context.Context, alt, neu, stilAlt, stilNeu string) (DiffAntwort, error)
	Frame(ctx context.Context, spec, animation string, frame int, variante, out string) (FrameAntwort, error)
	PixelDiff(ctx context.Context, a, b, out string) (PixelAntwort, error)
}

// DiffAntwort ist die Antwort von `diff`.
type DiffAntwort struct {
	Felder []FeldAenderung `json:"felder"`
	Teile  []string        `json:"teile"`
	Stil   bool            `json:"stil"`
}

// FrameAntwort ist die Antwort von `frame`: Teil-Boxen [x, y, b, h] in Pixeln des Frames.
type FrameAntwort struct {
	Datei string           `json:"datei"`
	Teile map[string][]int `json:"teile"`
}

// PixelAntwort ist die Antwort von `pixeldiff`.
type PixelAntwort struct {
	Anteil float64 `json:"anteil"`
	Boxen  [][]int `json:"boxen"`
	Datei  string  `json:"datei"`
}

// Abgelehnt ist Exit 1 der CLI: fachlich abgelehnt mit Fehlern je Regel.
type Abgelehnt struct{ Fehler []Fehler }

func (a *Abgelehnt) Error() string {
	texte := make([]string, len(a.Fehler))
	for i, f := range a.Fehler {
		texte[i] = f.Text
	}
	return strings.Join(texte, "; ")
}

const cliZeit = 2 * time.Minute

// CLI ruft `bun tools/paper/bin/paper.ts` (RD1) im Ordner Root auf, solange die Datei fehlt die Attrappe
// tools/paper/fixtures/cli-stub.ts.
type CLI struct{ Root string }

// Kennung ist die Renderer-Version für den Cache-Schlüssel; die Attrappe bekommt einen Zusatz.
func (c CLI) Kennung() string {
	if strings.HasSuffix(c.skript(), "cli-stub.ts") {
		return RendererVersion + "+attrappe"
	}
	return RendererVersion
}

func (c CLI) skript() string {
	echt := filepath.Join(c.Root, "tools", "paper", "bin", "paper.ts")
	if _, err := os.Stat(echt); err == nil {
		return echt
	}
	return filepath.Join(c.Root, "tools", "paper", "fixtures", "cli-stub.ts")
}

// run führt einen Befehl aus und dekodiert die Antwort nach ziel; Exit 1 → *Abgelehnt, Exit 2 → Fehler mit stderr.
func (c CLI) run(ctx context.Context, ziel any, args ...string) error {
	ctx, cancel := context.WithTimeout(ctx, cliZeit)
	defer cancel()
	cmd := proc.Command(ctx, append([]string{"bun", c.skript()}, args...))
	cmd.Dir = c.Root
	var out, errOut bytes.Buffer
	cmd.Stdout, cmd.Stderr = &out, &errOut
	err := cmd.Run()
	var exit *exec.ExitError
	switch {
	case err == nil:
		return json.Unmarshal(out.Bytes(), ziel)
	case errors.As(err, &exit) && exit.ExitCode() == 1:
		var a struct{ Fehler []Fehler }
		if jerr := json.Unmarshal(out.Bytes(), &a); jerr != nil || len(a.Fehler) == 0 {
			// Exit 1 ohne Antwort ist ein Absturz von bun (z. B. fehlende node_modules), keine Ablehnung.
			return fmt.Errorf("paper %s: %w: %s", args[0], err, strings.TrimSpace(errOut.String()))
		}
		return &Abgelehnt{Fehler: a.Fehler}
	default:
		return fmt.Errorf("paper %s: %w: %s", args[0], err, strings.TrimSpace(errOut.String()))
	}
}

// Validate liefert die Regelverstöße der Spec; leer = gültig.
func (c CLI) Validate(ctx context.Context, spec string) ([]Fehler, error) {
	var ok struct{}
	err := c.run(ctx, &ok, "validate", spec)
	var a *Abgelehnt
	if errors.As(err, &a) {
		return a.Fehler, nil
	}
	return nil, err
}

// Diff vergleicht zwei Specs; mit beiden Stil-Dateien meldet `stil`, ob ihr stilHash verschieden ist.
func (c CLI) Diff(ctx context.Context, alt, neu, stilAlt, stilNeu string) (DiffAntwort, error) {
	args := []string{"diff", alt, neu}
	if stilAlt != "" && stilNeu != "" {
		args = append(args, "--stil-alt", stilAlt, "--stil-neu", stilNeu)
	}
	var d DiffAntwort
	err := c.run(ctx, &d, args...)
	return d, err
}

// Frame rendert einen Frame nach out.
func (c CLI) Frame(ctx context.Context, spec, animation string, frame int, variante, out string) (FrameAntwort, error) {
	args := []string{"frame", spec, "--animation", animation, "--frame", strconv.Itoa(frame), "--out", out}
	if variante != "" {
		args = append(args, "--variante", variante)
	}
	var f FrameAntwort
	err := c.run(ctx, &f, args...)
	return f, err
}

// PixelDiff vergleicht zwei PNGs und schreibt das Overlay nach out.
func (c CLI) PixelDiff(ctx context.Context, a, b, out string) (PixelAntwort, error) {
	var p PixelAntwort
	err := c.run(ctx, &p, "pixeldiff", a, b, "--out", out)
	return p, err
}
