// Package review vergleicht die Assets des Arbeitsstands mit einer Git-Basis (Vertrag docs/standards/paper-mcp.md ›
// Oberfläche): Basis auflösen, Specs und Stil-Datei per git lesen, kanonisch hashen (Port von hash.ts), „Vorher“ über
// die Render-CLI in den Cache .cache/paper/<renderKey>/ rendern und je Asset den Review-Eintrag bilden.
package review

import (
	"context"
	"fmt"
	"regexp"
	"strings"
	"time"
)

// reBasis: HEAD, HEAD~1, origin/develop oder Commit-SHA mit 7–40 Hex-Zeichen.
var reBasis = regexp.MustCompile(`^(HEAD|HEAD~1|origin/develop|[0-9a-fA-F]{7,40})$`)

const fetchZeit = 10 * time.Second

// Stand ist eine aufgelöste Basis; BasisStand aktuell | lokal (origin/develop ohne Fetch).
type Stand struct {
	Basis      string
	Commit     string
	BasisStand string
}

// FehlerBasis ist der Vertragsfehler für eine ungültige Basis.
func FehlerBasis(basis string) error {
	return fmt.Errorf("Basis „%s“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/develop", basis) //nolint:staticcheck // Vertragstext der Oberfläche (paper-mcp.md)
}

// Aufloesen prüft und löst die Basis auf; leer = HEAD. origin/develop versucht einen Fetch, sonst gilt der lokale Stand.
func Aufloesen(ctx context.Context, root, basis string) (Stand, error) {
	if basis == "" {
		basis = "HEAD"
	}
	if !reBasis.MatchString(basis) {
		return Stand{}, FehlerBasis(basis)
	}
	stand := "aktuell"
	if basis == "origin/develop" {
		fctx, cancel := context.WithTimeout(ctx, fetchZeit)
		_, err := git(fctx, root, "fetch", "--quiet", "origin", "develop")
		cancel()
		if err != nil {
			stand = "lokal"
		}
	}
	out, err := git(ctx, root, "rev-parse", "--verify", "--quiet", "--short=7", basis+"^{commit}")
	if err != nil {
		return Stand{}, FehlerBasis(basis)
	}
	return Stand{Basis: basis, Commit: strings.TrimSpace(string(out)), BasisStand: stand}, nil
}
