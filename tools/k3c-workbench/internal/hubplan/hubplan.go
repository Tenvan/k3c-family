package hubplan

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"time"

	"k3c/engine/simcli"
)

// Gebäude-Planer (B-923, Port aus dem früheren Asset-Studio, B-462, WZ9.2): liest data/hub.json und zeigt die Freischaltung als Matrix Hub-Stufe × Bauplatz. Nur
// lesen; Matrix und Prüfung rechnet `simcli hub` (engine/sim, B-924: im Prozess). Bearbeiten und Schreiben:
// app_planer_edit.go, app_planer_save.go (WZ9.3).

// engineTimeout begrenzt einen Aufruf von simcli hub (Prüfung braucht Millisekunden).
const engineTimeout = 30 * time.Second

// Engine ist die Engine-Seite des Planers: input ist das JSON von `k3c-sim hub` ({"op":"plan","raw":…}), die Antwort
// sein report. Ein Fehler ist die Meldung von k3c-sim oder ein Startfehler.
type Engine func(ctx context.Context, input []byte) (json.RawMessage, error)

// Direct ist die Engine im Prozess (engine/simcli, B-924) mit den Daten des Checkouts root; eine Panik der Engine
// wird zum Fehler (recover in simcli.Exec).
func Direct(root string) Engine {
	return func(ctx context.Context, input []byte) (json.RawMessage, error) {
		ctx, cancel := context.WithTimeout(ctx, engineTimeout)
		defer cancel()
		ev, err := simcli.Call(ctx, []string{"hub"}, input, simcli.DirFiles(filepath.Join(root, "data")))
		return ev.Report, err
	}
}

// Editor ist der Gebäude-Planer eines Checkouts: Root ist die Repo-Wurzel, Engine die Engine-Seite (simcli hub).
// Methoden: Plan, Check, Rules, Save.
type Editor struct {
	Root   string
	Engine Engine
}

// hubFile ist die Quelle des Planers, relativ zur Repo-Wurzel.
const hubFile = "data/hub.json"

// HubCell ist eine Zelle der Matrix (Hub-Stufe × Bauplatz): verfügbar ab der Stufe der Zeile, dann mit Platz und Tiefe.
type HubCell struct {
	Available   bool    `json:"available"`
	OffsetUnits float64 `json:"offsetUnits"` // Platz zur Hub-Mitte; bei Mauerlinie und Tor je Seite (±)
	FromDepth   int     `json:"fromDepth"`
	NeedsDeeper bool    `json:"needsDeeper"`
	OnlyDepth   int     `json:"onlyDepth"` // nur in dieser Tiefe (Händler); -1 = jede Tiefe
}

// HubRow ist ein Bauplatz der Matrix: Hub-Platz, Insel-Platz, Mauerlinie (mit Turm), Tor oder Händler.
type HubRow struct {
	Key         string    `json:"key"`         // kind@offset wie room.PreviewSiteKey, Baugrund der Zone 1 zone1
	Kind        string    `json:"kind"`        // Art aus hub.json; wall (Baugrund der Zone 1), merchantBuy, merchantSell
	Source      string    `json:"source"`      // sites, islandSites, sitePlacement, merchant
	Index       int       `json:"index"`       // Eintrag in seiner Liste (sites, islandSites, wallUnits); Händler 0
	HubLevel    int       `json:"hubLevel"`    // ab dieser Hub-Stufe verfügbar
	Line        int       `json:"line"`        // Mauerlinie bzw. Tor: Linie k; sonst 0
	BothSides   bool      `json:"bothSides"`   // je Seite (±): Mauerlinie und Tor
	TowerUnits  float64   `json:"towerUnits"`  // Mauerlinie: Turm innen; sonst 0
	JitterUnits float64   `json:"jitterUnits"` // Streuung nach außen je Seed; 0 = fest
	Cells       []HubCell `json:"cells"`       // je Hub-Stufe 1 … Levels
}

// HubMatrix ist das Matrix-Modell: Spalten = Hub-Stufen aus levels (beliebig viele), Zeilen = Bauplätze in der
// Reihenfolge sites, islandSites, Mauerlinien und Tore, Händler.
type HubMatrix struct {
	Levels int      `json:"levels"`
	Rows   []HubRow `json:"rows"`
}

// HubFindingView ist ein Befund der Prüfung (sim.HubFinding) mit den JSON-Namen der Oberfläche.
type HubFindingView struct {
	Rule   string `json:"rule"`
	Site   string `json:"site"`
	Detail string `json:"detail"`
}

// HubPlanView ist der geladene Stand des Planers: Pfad, Inhalt (Kandidat der Live-Vorschau), Matrix und Prüfung; Error
// nennt eine nicht lesbare oder ungültige Datei mit Pfad, dann ist die Matrix leer. Stand ist der Inhalt beider
// Dateien beim Laden (Schreibregel), Kinds die Arten, die als Platz hinzukommen dürfen.
type HubPlanView struct {
	Path     string           `json:"path"`
	Raw      string           `json:"raw"`
	Matrix   HubMatrix        `json:"matrix"`
	Findings []HubFindingView `json:"findings"`
	Notes    []string         `json:"notes"`
	Error    string           `json:"error"`
	Stand    HubStand         `json:"stand"`
	Kinds    []string         `json:"kinds"`
}

// Plan lädt data/hub.json des Checkouts und liefert Matrix und Befunde.
func (e Editor) Plan(ctx context.Context) HubPlanView { return e.planFrom(ctx, hubFile) }

// planFrom liest rel unter Root (Schrägstriche), prüft über die Engine und merkt den Stand von rel und der Regeltabelle.
func (e Editor) planFrom(ctx context.Context, rel string) HubPlanView {
	raw, err := os.ReadFile(filepath.Join(e.Root, filepath.FromSlash(rel)))
	if err != nil {
		view := emptyPlan(rel)
		view.Error = fmt.Sprintf("%s nicht lesbar: %v", rel, err)
		return view
	}
	view := e.planOf(ctx, rel, raw)
	view.Stand = HubStand{Hub: stamp(raw), Rules: fileStamp(e.Root, rulesFile)}
	return view
}

func emptyPlan(rel string) HubPlanView {
	return HubPlanView{Path: rel, Matrix: HubMatrix{Rows: []HubRow{}}, Findings: []HubFindingView{}, Notes: []string{}, Kinds: []string{}}
}

// engineReport ist die Antwort von `k3c-sim hub`.
type engineReport struct {
	Matrix     HubMatrix        `json:"matrix"`
	Findings   []HubFindingView `json:"findings"`
	Notes      []string         `json:"notes"`
	Kinds      []string         `json:"kinds"`
	ParseError string           `json:"parseError"`
}

// planOf prüft den Inhalt raw (Datei oder Kandidat) über die Engine und baut die Matrix; ohne Stand.
func (e Editor) planOf(ctx context.Context, rel string, raw []byte) HubPlanView {
	view := emptyPlan(rel)
	view.Raw = string(raw) // auch bei Fehler: das Frontend ordnet die Antwort seinem Kandidaten zu
	rep, err := e.ask(ctx, raw)
	if err != nil {
		view.Error = fmt.Sprintf("%s nicht geprüft: %v", rel, err)
		return view
	}
	view.Kinds = rep.Kinds
	if rep.ParseError != "" {
		view.Error = fmt.Sprintf("%s nicht lesbar als hub.json: %s", rel, rep.ParseError)
		return view
	}
	view.Matrix, view.Findings, view.Notes = rep.Matrix, rep.Findings, rep.Notes
	return view
}

// ask schickt raw an die Engine; ohne Engine (nil) ist das ein Fehler, kein Absturz.
func (e Editor) ask(ctx context.Context, raw []byte) (engineReport, error) {
	var rep engineReport
	if e.Engine == nil {
		return rep, fmt.Errorf("keine Engine")
	}
	input, err := json.Marshal(map[string]string{"op": "plan", "raw": string(raw)})
	if err != nil {
		return rep, err
	}
	report, err := e.Engine(ctx, input)
	if err != nil {
		return rep, err
	}
	if err := json.Unmarshal(report, &rep); err != nil {
		return rep, err
	}
	if rep.Findings == nil {
		rep.Findings = []HubFindingView{}
	}
	if rep.Notes == nil {
		rep.Notes = []string{}
	}
	if rep.Kinds == nil {
		rep.Kinds = []string{}
	}
	if rep.Matrix.Rows == nil {
		rep.Matrix.Rows = []HubRow{}
	}
	return rep, nil
}
