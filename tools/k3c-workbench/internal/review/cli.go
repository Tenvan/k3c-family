package review

import (
	"context"
	"encoding/json"
	"errors"
	"path/filepath"
	"strconv"
	"strings"
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

// CLI ruft die Render-CLI (Skript: K3C_PAPER_CLI, tools/paper/bin/paper.ts oder die Attrappe) im Ordner Root auf.
type CLI struct{ Root string }

// Kennung ist die Renderer-Version für den Cache-Schlüssel; die Attrappe bekommt einen Zusatz.
func (c CLI) Kennung() string {
	if strings.HasSuffix(filepath.ToSlash(c.skript()), "cli-stub.ts") {
		return RendererVersion + "+attrappe"
	}
	return RendererVersion
}

func (c CLI) skript() string { return Skript(c.Root) }

// run führt einen Befehl über Aufruf aus und dekodiert die Antwort nach ziel.
func (c CLI) run(ctx context.Context, ziel any, args ...string) error {
	out, err := Aufruf(ctx, c.Root, c.skript(), Zeitlimit, args...)
	if err != nil {
		return err
	}
	return json.Unmarshal(out, ziel)
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

// FrameOhne rendert einen Frame nach out, die Teile in ohne ohne ihre Lagen (B-734: Teil in der Vorschau ausblenden).
func (c CLI) FrameOhne(ctx context.Context, spec, animation string, frame int, ohne []string, out string) (FrameAntwort, error) {
	args := []string{"frame", spec, "--animation", animation, "--frame", strconv.Itoa(frame), "--out", out}
	if len(ohne) > 0 {
		args = append(args, "--ohne", strings.Join(ohne, ","))
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
