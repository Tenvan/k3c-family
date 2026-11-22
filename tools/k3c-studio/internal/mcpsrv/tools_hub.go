package mcpsrv

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strings"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-studio/internal/hubplan"
	pa "k3c/tools/k3c-studio/internal/paperassets"
)

// Gebäude-Planer (B-923, Port aus dem früheren Asset-Studio, B-462): data/hub.json als Matrix Hub-Stufe × Bauplatz,
// Prüfung mit sim.ValidateHubData, Abgleich mit der Regeltabelle und geprüftes Speichern. Dieselbe Logik wie die Seite
// Gebäude-Planer (internal/hubplan); die Tools heißen hub_*, damit sie nicht mit den Planungs-Tools plan_* kollidieren.

const (
	hubRowLimit  = 60 // Zeilen der Matrix in einer Antwort
	hubDiffLimit = 8  // abweichende Zeilen der Regeltabelle in einer Antwort
	hubStandLen  = 12 // Stellen des Stands (SHA-256) in der Antwort
)

// hubEngine ist die Engine-Seite des Planers (simcli hub); Tests setzen einen Ersatz.
var hubEngine = hubplan.Direct

func hubEditor(root string) hubplan.Editor {
	return hubplan.Editor{Root: root, Engine: hubEngine(root)}
}

type hubGetIn struct{}

type hubCheckIn struct {
	Patch     map[string]any `json:"patch,omitempty" jsonschema:"JSON-Merge-Patch (RFC 7396) auf data/hub.json: null löscht, Objekte werden gemischt, Listen (sites, islandSites, levels) ganz angeben"`
	Candidate string         `json:"candidate,omitempty" jsonschema:"stattdessen der ganze Text einer hub.json"`
}

type hubSaveIn struct {
	Patch     map[string]any `json:"patch,omitempty" jsonschema:"JSON-Merge-Patch (RFC 7396) auf data/hub.json: null löscht, Objekte werden gemischt, Listen (sites, islandSites, levels) ganz angeben"`
	Candidate string         `json:"candidate,omitempty" jsonschema:"stattdessen der ganze Text einer hub.json"`
	Stand     string         `json:"stand,omitempty" jsonschema:"Pflicht: Stand aus hub_get (Zeile Stand, hub=…); ist die Datei seitdem geändert, wird nichts geschrieben"`
}

func registerHub(s *Server) {
	notDestructive, closed := false, false
	add(s, &mcp.Tool{
		Name: "hub_get",
		Description: "Gebäude-Planer: data/hub.json als Matrix Hub-Stufe × Bauplatz (Platz, ab Stufe, Tiefe), Befunde der Prüfung (sim.ValidateHubData), " +
			"Abweichung der Regeltabelle und Stand für hub_save. Nutze es bei: wann wird welches Gebäude frei, vor jeder Änderung an data/hub.json. " +
			"Statt: data/hub.json und docs/rules/materialien-gebaeude.md lesen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, _ hubGetIn) (string, error) {
		return hubText(ctx, s.ws(ctx).Root, "")
	})
	add(s, &mcp.Tool{
		Name: "hub_check",
		Description: "Gebäude-Planer: prüft eine Änderung an data/hub.json (patch oder ganzer candidate), ohne zu schreiben: Matrix, Befunde, " +
			"Abweichung der Regeltabelle. Nutze es bei: Änderung an Plätzen oder Hub-Stufen vorab prüfen. Statt: hub.json von Hand ändern und go test fahren.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in hubCheckIn) (string, error) {
		root := s.ws(ctx).Root
		cand, err := hubCandidate(ctx, root, in.Patch, in.Candidate, false)
		if err != nil {
			return "", err
		}
		return hubText(ctx, root, cand)
	})
	add(s, &mcp.Tool{
		Name: "hub_save",
		Description: "Gebäude-Planer: schreibt eine geprüfte Änderung in data/hub.json (format-stabil, ein Wert eine Zeile) und den erzeugten Block " +
			"in docs/rules/materialien-gebaeude.md; mit Befunden, bei geänderter Datei (stand) oder ohne Änderung wird nichts geschrieben. " +
			"Eine REG-Änderung: Commit mit Begründung. Nutze es bei: Plätze, Hub-Stufen, Händler oder Baugrund ändern. Statt: data/hub.json von Hand ändern.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &notDestructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in hubSaveIn) (string, error) {
		return hubSaveText(ctx, s.ws(ctx).Root, in)
	})
}

// hubCandidate ist der Text einer hub.json aus patch (auf der Datei des Checkouts) oder candidate; leer = die Datei selbst.
// required verlangt genau eine Angabe.
func hubCandidate(ctx context.Context, root string, patch map[string]any, candidate string, required bool) (string, error) {
	switch {
	case patch != nil && candidate != "":
		return "", errors.New("patch oder candidate, nicht beides")
	case patch == nil && candidate == "":
		if required {
			return "", errors.New("patch oder candidate fehlt")
		}
		return "", nil
	case candidate != "":
		return candidate, nil
	}
	cur := hubEditor(root).Plan(ctx)
	if cur.Error != "" {
		return "", errors.New(cur.Error)
	}
	raw, err := json.Marshal(patch)
	if err != nil {
		return "", err
	}
	out, err := pa.MergePatch(json.RawMessage(cur.Raw), raw)
	if err != nil {
		return "", fmt.Errorf("patch nicht anwendbar: %w", err)
	}
	return string(out), nil
}

// hubText ist die Antwort zu data/hub.json (cand leer) bzw. zu einem Kandidaten: Kopf, Stand, Matrix, Befunde, Regeltabelle.
func hubText(ctx context.Context, root, cand string) (string, error) {
	e := hubEditor(root)
	view := e.Plan(ctx)
	if cand != "" {
		view = e.Check(ctx, cand)
	}
	if view.Error != "" {
		return "", errors.New(view.Error)
	}
	var b strings.Builder
	head := view.Path
	if cand != "" {
		head += " (Kandidat, nichts geschrieben)"
	}
	fmt.Fprintf(&b, "%s · %d Hub-Stufen · %d Bauplätze · Prüfung: %s\n", head, view.Matrix.Levels, len(view.Matrix.Rows), findingsLabel(len(view.Findings)))
	if cand == "" {
		fmt.Fprintf(&b, "Stand: hub=%s rules=%s\n", short(view.Stand.Hub), short(view.Stand.Rules))
	}
	b.WriteString("Bauplätze (Schlüssel · Liste#Index · ab Hub-Stufe · Platz[ · Tiefe]):\n")
	for i, r := range view.Matrix.Rows {
		if i == hubRowLimit {
			fmt.Fprintf(&b, "… %d weitere\n", len(view.Matrix.Rows)-hubRowLimit)
			break
		}
		b.WriteString(hubRowLine(r) + "\n")
	}
	for _, f := range view.Findings {
		fmt.Fprintf(&b, "Befund: %s · %s · %s\n", f.Rule, f.Site, f.Detail)
	}
	for _, n := range view.Notes {
		fmt.Fprintf(&b, "Hinweis: %s\n", n)
	}
	b.WriteString(hubRulesLine(e.Rules(view.Raw)))
	return b.String(), nil
}

func findingsLabel(n int) string {
	if n == 0 {
		return "ok"
	}
	return fmt.Sprintf("%d Befunde", n)
}

func short(stand string) string {
	if stand == "" {
		return "-"
	}
	return stand[:min(hubStandLen, len(stand))]
}

// hubRowLine ist eine Zeile der Matrix: Platz und Tiefe der ersten Stufe, in der sie verfügbar ist.
func hubRowLine(r hubplan.HubRow) string {
	line := fmt.Sprintf("%s · %s#%d · ab %d", r.Key, r.Source, r.Index, r.HubLevel)
	for _, c := range r.Cells {
		if !c.Available {
			continue
		}
		switch {
		case r.BothSides:
			line += fmt.Sprintf(" · ±%g", c.OffsetUnits)
		case c.OffsetUnits < 0:
			line += fmt.Sprintf(" · −%g", -c.OffsetUnits)
		default:
			line += fmt.Sprintf(" · +%g", c.OffsetUnits)
		}
		if c.FromDepth > 0 {
			line += fmt.Sprintf(" · ab Tiefe %d", c.FromDepth)
		}
		if c.NeedsDeeper {
			line += " · nur mit tieferer Stufe"
		}
		if c.OnlyDepth >= 0 {
			line += fmt.Sprintf(" · nur Tiefe %d", c.OnlyDepth)
		}
		break
	}
	return line
}

// hubRulesLine ist die Zeile zur Regeltabelle: stimmt sie, weicht sie ab (mit den ersten Zeilen) oder ist nicht vergleichbar.
func hubRulesLine(v hubplan.HubRulesView) string {
	switch {
	case v.Error != "":
		return "Regeltabelle: " + v.Error + "\n"
	case v.Same:
		return "Regeltabelle: stimmt\n"
	}
	var b strings.Builder
	fmt.Fprintf(&b, "Regeltabelle: weicht ab (%d Zeilen in %s § 3), %s schreibt sie neu\n", len(v.Lines), v.Path, v.Task)
	for i, l := range v.Lines {
		if i == hubDiffLimit {
			fmt.Fprintf(&b, "… %d weitere\n", len(v.Lines)-hubDiffLimit)
			break
		}
		fmt.Fprintf(&b, "  Zeile %d: ist %q, soll %q\n", l.Line, l.Ist, l.Soll)
	}
	return b.String()
}

// hubSaveText speichert den Kandidaten wie die Seite: Prüfung ohne Befund, Stand der Dateien, dann hub.json und Regeltabelle.
func hubSaveText(ctx context.Context, root string, in hubSaveIn) (string, error) {
	cand, err := hubCandidate(ctx, root, in.Patch, in.Candidate, true)
	if err != nil {
		return "", err
	}
	e := hubEditor(root)
	plan := e.Plan(ctx)
	if plan.Error != "" {
		return "", errors.New(plan.Error)
	}
	if in.Stand == "" {
		return "", errors.New("stand fehlt: erst hub_get lesen und dessen Stand (hub=…) mitgeben")
	}
	if !strings.HasPrefix(plan.Stand.Hub, in.Stand) {
		return "", errors.New("nicht gespeichert: data/hub.json seit hub_get geändert, bitte neu lesen")
	}
	res := e.Save(ctx, cand, plan.Stand)
	switch {
	case res.Stale != "":
		return "", errors.New("nicht gespeichert: " + res.Stale)
	case res.Error != "":
		return "", errors.New(hubSaveError(res))
	case res.Unchanged:
		return "Unverändert: der Kandidat bedeutet dasselbe wie data/hub.json, nichts geschrieben.", nil
	}
	out := "Gespeichert: data/hub.json und Regeltabelle. " + res.Notice
	if res.RulesError != "" {
		out += "\nRegeltabelle nicht geschrieben: " + res.RulesError
	}
	return out + "\nStand: hub=" + short(res.Stand.Hub) + " rules=" + short(res.Stand.Rules), nil
}

// hubSaveError nennt den Fehler mit den Befunden der Prüfung, einer je Zeile.
func hubSaveError(res hubplan.HubSaveResult) string {
	lines := []string{res.Error}
	for _, f := range res.Findings {
		lines = append(lines, fmt.Sprintf("Befund: %s · %s · %s", f.Rule, f.Site, f.Detail))
	}
	return strings.Join(lines, "\n")
}
