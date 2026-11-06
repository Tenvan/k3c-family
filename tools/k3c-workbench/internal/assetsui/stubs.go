// Package assetsui ist die RPC-Schicht der Asset-Oberfläche (Vertrag docs/standards/paper-mcp.md › Oberfläche).
// Bis RV1 liefern die Stubs die Beispielantworten aus tools/paper/fixtures/ui/ (PV1.8); Typen und Basis-Prüfung
// gelten schon jetzt, damit WB1 (Hülle) und RV1 (Review) parallel dagegen bauen.
package assetsui

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
)

// Fehler ist eine abgelehnte Regel einer Spec (paper-spec.md › Regeln).
type Fehler struct {
	Feld  string `json:"feld"`
	Regel string `json:"regel"`
	Text  string `json:"text"`
}

// AssetEintrag ist eine Zeile des Katalogs.
type AssetEintrag struct {
	ID        string   `json:"id"`
	Rig       string   `json:"rig"`
	Gruppe    string   `json:"gruppe"`
	Pfad      string   `json:"pfad"`
	Gueltig   bool     `json:"gueltig"`
	Geaendert bool     `json:"geaendert"`
	Fehler    []Fehler `json:"fehler,omitempty"`
}

// FeldAenderung ist ein geändertes Feld einer Spec gegenüber der Basis.
type FeldAenderung struct {
	Pfad string `json:"pfad"`
	Alt  any    `json:"alt"`
	Neu  any    `json:"neu"`
}

// ReviewAsset ist ein geändertes Asset; Status neu | geaendert | geloescht | ungueltig.
type ReviewAsset struct {
	ID          string          `json:"id"`
	Rig         string          `json:"rig"`
	Pfad        string          `json:"pfad"`
	Status      string          `json:"status"`
	Fehler      []Fehler        `json:"fehler,omitempty"`
	Felder      []FeldAenderung `json:"felder"`
	Teile       []string        `json:"teile"`
	Stil        bool            `json:"stil"`
	PixelAnteil float64         `json:"pixelAnteil"`
}

// Review ist der Vergleich des Arbeitsstands mit einer Basis; BasisStand aktuell | lokal.
type Review struct {
	Basis       string        `json:"basis"`
	BasisCommit string        `json:"basisCommit"`
	BasisStand  string        `json:"basisStand"`
	Assets      []ReviewAsset `json:"assets"`
}

// Frame ist ein gerenderter Frame als Data-URL mit den Teil-Boxen [x, y, b, h] in Pixeln.
type Frame struct {
	Bild  string           `json:"bild"`
	Teile map[string][]int `json:"teile"`
}

// PixelDiff ist der Pixel-Vergleich mit Overlay als Data-URL.
type PixelDiff struct {
	Anteil  float64 `json:"anteil"`
	Boxen   [][]int `json:"boxen"`
	Overlay string  `json:"overlay"`
}

// Vorlage ist eine Prompt-Vorlage aus data/asset-prompts.json.
type Vorlage struct {
	ID    string `json:"id"`
	Titel string `json:"titel"`
	Ort   string `json:"ort"`
	Text  string `json:"text"`
}

// reBasis: HEAD, HEAD~1, origin/develop oder Commit-SHA mit 7–40 Hex-Zeichen.
var reBasis = regexp.MustCompile(`^(HEAD|HEAD~1|origin/develop|[0-9a-fA-F]{7,40})$`)

// fixtureID ist das einzige Asset, für das die Stubs Spec, Frame und Pixel-Diff kennen.
const fixtureID = "monarch"

// CheckBasis lehnt eine unbekannte Basis mit den erlaubten Werten ab; leer ist keine Basis.
func CheckBasis(basis string) error {
	if !reBasis.MatchString(basis) {
		return fmt.Errorf("Basis „%s“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/develop", basis) //nolint:staticcheck // Vertragstext der Oberfläche (paper-mcp.md)
	}
	return nil
}

// checkID meldet den Vertragsfehler für ein Asset, das die Stubs nicht kennen.
func checkID(id string) error {
	if id != fixtureID {
		return fmt.Errorf("Asset „%s“ fehlt", id) //nolint:staticcheck // Vertragstext der Oberfläche (paper-mcp.md)
	}
	return nil
}

// Stubs beantworten die Methoden aus den Fixtures unter Root/tools/paper/fixtures/ui/.
type Stubs struct{ Root string }

func load[T any](s Stubs, method string) (T, error) {
	var v T
	raw, err := os.ReadFile(filepath.Join(s.Root, "tools", "paper", "fixtures", "ui", method+".json"))
	if err != nil {
		return v, fmt.Errorf("%s: Beispielantwort fehlt: %w", method, err)
	}
	err = json.Unmarshal(raw, &v)
	return v, err
}

func (s Stubs) AssetsList() ([]AssetEintrag, error)  { return load[[]AssetEintrag](s, "AssetsList") }
func (s Stubs) AssetsStyle() (map[string]any, error) { return load[map[string]any](s, "AssetsStyle") }
func (s Stubs) AssetsPrompts() ([]Vorlage, error)    { return load[[]Vorlage](s, "AssetsPrompts") }

func (s Stubs) AssetsGet(id string) (map[string]any, error) {
	if err := checkID(id); err != nil {
		return nil, err
	}
	return load[map[string]any](s, "AssetsGet")
}

// AssetsFrame: leere Basis = Arbeitsstand.
func (s Stubs) AssetsFrame(id, _ string, _ int, _, basis string) (Frame, error) {
	if err := checkID(id); err != nil {
		return Frame{}, err
	}
	if basis != "" {
		if err := CheckBasis(basis); err != nil {
			return Frame{}, err
		}
	}
	return load[Frame](s, "AssetsFrame")
}

// AssetsReview: leere Basis = HEAD (Standard laut Vertrag).
func (s Stubs) AssetsReview(basis string) (Review, error) {
	if basis == "" {
		basis = "HEAD"
	}
	if err := CheckBasis(basis); err != nil {
		return Review{}, err
	}
	return load[Review](s, "AssetsReview")
}

// AssetsPixelDiff vergleicht den Arbeitsstand mit einer Basis; leer ist keine Basis.
func (s Stubs) AssetsPixelDiff(id, _ string, _ int, basis string) (PixelDiff, error) {
	if err := checkID(id); err != nil {
		return PixelDiff{}, err
	}
	if err := CheckBasis(basis); err != nil {
		return PixelDiff{}, err
	}
	return load[PixelDiff](s, "AssetsPixelDiff")
}
