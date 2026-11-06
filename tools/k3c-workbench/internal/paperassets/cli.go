// Package paperassets ist die Asset-Schicht der MCP-Tools (Vertrag docs/standards/paper-mcp.md, paper-spec.md):
// Aufruf der Render-CLI, JSON-Merge-Patch, atomares Schreiben einer Spec, Git-Basis und Render-Jobs. Bis RD1 die
// echte CLI liefert, zeigt K3C_PAPER_CLI auf die Attrappe tools/paper/fixtures/cli-stub.ts.
package paperassets

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"k3c/tools/k3c-workbench/internal/assetsui"
	"k3c/tools/k3c-workbench/internal/proc"
)

// Fehler ist eine verletzte Regel {feld, regel, text} (paper-spec.md › Regeln).
type Fehler = assetsui.Fehler

// Abgelehnt ist ein fachlich abgelehnter Aufruf (Exit 1 der CLI); er ändert nichts.
type Abgelehnt struct{ Fehler []Fehler }

func (a *Abgelehnt) Error() string {
	teile := make([]string, len(a.Fehler))
	for i, f := range a.Fehler {
		teile[i] = f.Feld + " · " + f.Regel + " · " + f.Text
	}
	return "abgelehnt: " + strings.Join(teile, "; ")
}

// AlsAbgelehnt holt die Ablehnung aus err.
func AlsAbgelehnt(err error) (*Abgelehnt, bool) {
	var a *Abgelehnt
	ok := errors.As(err, &a)
	return a, ok
}

// CLI ruft `bun <Skript> <befehl> …` im Checkout Root.
type CLI struct{ Root, Skript string }

// NewCLI nimmt das Skript aus K3C_PAPER_CLI, sonst tools/paper/bin/paper.ts im Checkout.
func NewCLI(root string) CLI {
	if s := os.Getenv("K3C_PAPER_CLI"); s != "" {
		return CLI{Root: root, Skript: s}
	}
	return CLI{Root: root, Skript: filepath.Join(root, "tools", "paper", "bin", "paper.ts")}
}

// run führt einen Befehl aus: Exit 0 → stdout, Exit 1 → *Abgelehnt, sonst Fehler mit stderr.
func (c CLI) run(ctx context.Context, args ...string) (json.RawMessage, error) {
	cmd := proc.Command(ctx, append([]string{"bun", c.Skript}, args...))
	cmd.Dir = c.Root
	var out, errOut bytes.Buffer
	cmd.Stdout, cmd.Stderr = &out, &errOut
	err := cmd.Run()
	if ctx.Err() != nil {
		return nil, ctx.Err()
	}
	code := 0
	if err != nil {
		code = -1
		if cmd.ProcessState != nil {
			code = cmd.ProcessState.ExitCode()
		}
	}
	if code == 1 {
		var ab struct {
			Fehler []Fehler `json:"fehler"`
		}
		if json.Unmarshal(out.Bytes(), &ab) == nil && len(ab.Fehler) > 0 {
			return nil, &Abgelehnt{Fehler: ab.Fehler}
		}
	}
	if code != 0 {
		text := strings.TrimSpace(errOut.String())
		if text == "" && err != nil {
			text = err.Error()
		}
		return nil, fmt.Errorf("Render-CLI %s (Exit %d): %s", args[0], code, text) //nolint:staticcheck // Satz für die Antwort
	}
	return out.Bytes(), nil
}

// rel macht einen Pfad im Checkout relativ mit /; außerhalb bleibt er (mit /) unverändert.
func (c CLI) rel(p string) string {
	if !filepath.IsAbs(p) {
		return filepath.ToSlash(p)
	}
	r, err := filepath.Rel(c.Root, p)
	if err != nil || r == ".." || strings.HasPrefix(r, ".."+string(filepath.Separator)) {
		return filepath.ToSlash(p)
	}
	return filepath.ToSlash(r)
}

// abs macht eine Pfadangabe der CLI (relativ zum Checkout oder absolut) absolut.
func (c CLI) abs(p string) string {
	p = filepath.FromSlash(p)
	if filepath.IsAbs(p) {
		return p
	}
	return filepath.Join(c.Root, p)
}

// Validate prüft eine Spec; nil = gültig, sonst *Abgelehnt oder ein Aufruffehler.
func (c CLI) Validate(ctx context.Context, spec string) error {
	_, err := c.run(ctx, "validate", spec)
	return err
}

// RenderErgebnis sind die geschriebenen Manifeste und Dateien (Pfade wie rel).
type RenderErgebnis struct {
	Manifeste []string `json:"manifeste"`
	Dateien   []string `json:"dateien"`
}

// Render rendert eine Spec oder einen Gruppen-Ordner.
func (c CLI) Render(ctx context.Context, ziel string) (RenderErgebnis, error) {
	var r RenderErgebnis
	if err := c.decode(ctx, &r, "render", ziel); err != nil {
		return r, err
	}
	for i := range r.Manifeste {
		r.Manifeste[i] = c.rel(c.abs(r.Manifeste[i]))
	}
	for i := range r.Dateien {
		r.Dateien[i] = c.rel(c.abs(r.Dateien[i]))
	}
	return r, nil
}

// FrameArgs wählt einen Frame; Variante leer und Stufe 0 = keine Angabe.
type FrameArgs struct {
	Animation string
	Frame     int
	Variante  string
	Stufe     int
}

// FrameErgebnis sind Bilddatei und Teil-Boxen [x, y, b, h] in Pixeln.
type FrameErgebnis struct {
	Datei string           `json:"datei"`
	Teile map[string][]int `json:"teile"`
}

// Frame schreibt einen Frame nach out.
func (c CLI) Frame(ctx context.Context, spec string, f FrameArgs, out string) (FrameErgebnis, error) {
	args := []string{"frame", spec, "--animation", f.Animation, "--frame", strconv.Itoa(f.Frame), "--out", out}
	if f.Variante != "" {
		args = append(args, "--variante", f.Variante)
	}
	if f.Stufe > 0 {
		args = append(args, "--stufe", strconv.Itoa(f.Stufe))
	}
	var r FrameErgebnis
	err := c.decode(ctx, &r, args...)
	return r, err
}

// DiffErgebnis sind die geänderten Felder, Teile und ob sich der Stil geändert hat.
type DiffErgebnis struct {
	Felder []assetsui.FeldAenderung `json:"felder"`
	Teile  []string                 `json:"teile"`
	Stil   bool                     `json:"stil"`
}

// Diff vergleicht zwei Specs.
func (c CLI) Diff(ctx context.Context, alt, neu string) (DiffErgebnis, error) {
	var r DiffErgebnis
	err := c.decode(ctx, &r, "diff", alt, neu)
	return r, err
}

// PixelErgebnis ist der Anteil abweichender Pixel, ihre Boxen und das Overlay.
type PixelErgebnis struct {
	Anteil float64 `json:"anteil"`
	Boxen  [][]int `json:"boxen"`
	Datei  string  `json:"datei"`
}

// PixelDiff vergleicht zwei PNGs und schreibt das Overlay nach out.
func (c CLI) PixelDiff(ctx context.Context, a, b, out string) (PixelErgebnis, error) {
	var r PixelErgebnis
	err := c.decode(ctx, &r, "pixeldiff", a, b, "--out", out)
	return r, err
}

func (c CLI) decode(ctx context.Context, ziel any, args ...string) error {
	raw, err := c.run(ctx, args...)
	if err != nil {
		return err
	}
	if err := json.Unmarshal(raw, ziel); err != nil {
		return fmt.Errorf("Render-CLI %s: Antwort kein JSON: %w", args[0], err) //nolint:staticcheck // Satz für die Antwort
	}
	return nil
}
