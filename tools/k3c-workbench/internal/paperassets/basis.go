package paperassets

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"

	"k3c/tools/k3c-workbench/internal/proc"
	"k3c/tools/k3c-workbench/internal/review"
)

// Grafik ist der Ordner der Specs relativ zum Checkout (paper-spec.md › Dateien und Ablage).
const Grafik = "public/grafik"

const (
	endung    = ".paper.json"
	indexName = "index.paper.json"
	// StilPfad ist die Stil-Datei relativ zum Checkout.
	StilPfad = Grafik + "/stil.paper.json"
)

// reBasis: HEAD, HEAD~1, origin/develop oder Commit-SHA mit 7–40 Hex-Zeichen (paper-mcp.md › 1).
var reBasis = regexp.MustCompile(`^(HEAD|HEAD~1|origin/develop|[0-9a-fA-F]{7,40})$`)

// CheckBasis lehnt eine unbekannte Basis mit den erlaubten Werten ab (Vertragstext aus internal/review).
func CheckBasis(basis string) error {
	if !reBasis.MatchString(basis) {
		return review.FehlerBasis(basis)
	}
	return nil
}

// Asset ist eine Spec unter public/grafik/<gruppe>/<id>.paper.json; Pfad relativ zum Checkout mit /.
type Asset struct {
	ID, Rig, Gruppe, Pfad string
}

// Abs ist der absolute Pfad der Spec.
func (a Asset) Abs(root string) string { return filepath.Join(root, filepath.FromSlash(a.Pfad)) }

// Ausgabe ist der absolute Ausgabe-Ordner <gruppe>/<id>/.
func (a Asset) Ausgabe(root string) string {
	return filepath.Join(root, filepath.FromSlash(Grafik), a.Gruppe, a.ID)
}

// Assets listet alle Specs, sortiert nach ID; Gruppen-Indizes und die Stil-Datei zählen nicht.
func Assets(root string) ([]Asset, error) {
	treffer, err := filepath.Glob(filepath.Join(root, filepath.FromSlash(Grafik), "*", "*"+endung))
	if err != nil {
		return nil, err
	}
	var out []Asset
	for _, p := range treffer {
		name := filepath.Base(p)
		if name == indexName {
			continue
		}
		gruppe := filepath.Base(filepath.Dir(p))
		a := Asset{ID: strings.TrimSuffix(name, endung), Gruppe: gruppe, Pfad: Grafik + "/" + gruppe + "/" + name, Rig: "?"}
		var kopf struct {
			Rig string `json:"rig"`
		}
		if b, err := os.ReadFile(p); err == nil && json.Unmarshal(b, &kopf) == nil && kopf.Rig != "" {
			a.Rig = kopf.Rig
		}
		out = append(out, a)
	}
	sort.Slice(out, func(i, j int) bool { return out[i].ID < out[j].ID })
	return out, nil
}

// Finde sucht ein Asset über seine ID.
func Finde(root, id string) (Asset, error) {
	liste, err := Assets(root)
	if err != nil {
		return Asset{}, err
	}
	for _, a := range liste {
		if a.ID == id {
			return a, nil
		}
	}
	return Asset{}, fmt.Errorf("Asset „%s“ fehlt", id) //nolint:staticcheck // Vertragstext (paper-mcp.md)
}

// Geaendert sind die IDs, deren Spec oder Ausgabe gegen HEAD geändert ist (auch unversioniert oder gelöscht).
func Geaendert(ctx context.Context, root string) (map[string]bool, error) {
	cmd := proc.Command(ctx, []string{"git", "-c", "core.quotePath=false", "status", "--porcelain", "-uall", "--", Grafik})
	cmd.Dir = root
	var out, errOut bytes.Buffer
	cmd.Stdout, cmd.Stderr = &out, &errOut
	if err := cmd.Run(); err != nil {
		return nil, fmt.Errorf("git status: %s", strings.TrimSpace(errOut.String()))
	}
	ids := map[string]bool{}
	for zeile := range strings.SplitSeq(out.String(), "\n") {
		if len(zeile) < 4 {
			continue
		}
		for pfad := range strings.SplitSeq(zeile[3:], " -> ") {
			if id := assetVonPfad(strings.Trim(pfad, `"`)); id != "" {
				ids[id] = true
			}
		}
	}
	return ids, nil
}

// assetVonPfad: public/grafik/<gruppe>/<id>.paper.json oder public/grafik/<gruppe>/<id>/… → id.
func assetVonPfad(p string) string {
	rest, ok := strings.CutPrefix(p, Grafik+"/")
	if !ok {
		return ""
	}
	teile := strings.Split(rest, "/")
	if len(teile) < 2 {
		return ""
	}
	if len(teile) == 2 {
		if teile[1] == indexName || !strings.HasSuffix(teile[1], endung) {
			return ""
		}
		return strings.TrimSuffix(teile[1], endung)
	}
	return teile[1]
}
