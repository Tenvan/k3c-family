package review

// Gemeinsame Anbindung der Render-CLI für review und paperassets (B-706): Skript-Wahl, Zeitlimit, Ablehnung (Exit 1),
// Abbruch und der Hinweis, wenn bun fehlt.

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"k3c/tools/k3c-workbench/internal/proc"
)

// Zeitlimit begrenzt jeden Aufruf: eine hängende CLI hielte sonst eine Asset-Sperre oder einen Review-Lauf für immer.
const Zeitlimit = 2 * time.Minute

// Skript ist die Render-CLI des Checkouts root: K3C_PAPER_CLI, sonst tools/paper/bin/paper.ts, solange die Datei
// fehlt die Attrappe tools/paper/fixtures/cli-stub.ts.
func Skript(root string) string {
	if s := os.Getenv("K3C_PAPER_CLI"); s != "" {
		return s
	}
	echt := filepath.Join(root, "tools", "paper", "bin", "paper.ts")
	if _, err := os.Stat(echt); err == nil {
		return echt
	}
	return filepath.Join(root, "tools", "paper", "fixtures", "cli-stub.ts")
}

// Aufruf führt `bun <skript> <args…>` im Ordner root aus (höchstens zeit lang): Exit 0 → stdout, Exit 1 mit
// {fehler} → *Abgelehnt, ein abgebrochener Kontext → ctx.Err(), sonst ein Fehler mit stderr.
func Aufruf(eltern context.Context, root, skript string, zeit time.Duration, args ...string) ([]byte, error) {
	ctx, cancel := context.WithTimeout(eltern, zeit)
	defer cancel()
	cmd := proc.Command(ctx, append([]string{"bun", skript}, args...))
	cmd.Dir = root
	var out, errOut bytes.Buffer
	cmd.Stdout, cmd.Stderr = &out, &errOut
	err := cmd.Run()
	switch {
	case eltern.Err() != nil:
		return nil, eltern.Err()
	case ctx.Err() != nil:
		return nil, fmt.Errorf("Render-CLI %s: Zeitlimit %s überschritten", args[0], zeit) //nolint:staticcheck // Satz für die Antwort
	case errors.Is(err, exec.ErrNotFound):
		return nil, fmt.Errorf("Render-CLI %s: bun fehlt im PATH (Installation: requirements.md)", args[0]) //nolint:staticcheck // Satz für die Antwort
	case err == nil:
		return out.Bytes(), nil
	}
	code := -1
	if cmd.ProcessState != nil {
		code = cmd.ProcessState.ExitCode()
	}
	return Auswerten(args[0], code, out.Bytes(), errOut.String(), err.Error())
}

// Auswerten deutet einen beendeten Befehl wie Aufruf (auch je Schritt eines Sammelaufrufs `batch`, B-856): Exit 0 →
// stdout, Exit 1 mit {fehler} → *Abgelehnt, sonst ein Fehler mit stderr (leer: ersatz).
func Auswerten(befehl string, code int, stdout []byte, stderr, ersatz string) ([]byte, error) {
	if code == 0 {
		return stdout, nil
	}
	if code == 1 {
		var ab struct {
			Fehler []Fehler `json:"fehler"`
		}
		// Exit 1 ohne Antwort ist ein Absturz von bun (z. B. fehlende node_modules), keine Ablehnung.
		if json.Unmarshal(stdout, &ab) == nil && len(ab.Fehler) > 0 {
			return nil, &Abgelehnt{Fehler: ab.Fehler}
		}
	}
	text := strings.TrimSpace(stderr)
	if text == "" {
		text = ersatz
	}
	return nil, fmt.Errorf("Render-CLI %s (Exit %d): %s", befehl, code, text) //nolint:staticcheck // Satz für die Antwort
}
