package mcpsrv

import (
	"context"
	"encoding/json"
	"fmt"
	"slices"
	"strconv"
	"strings"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/world"
)

// Welten-Editor als MCP-Tools (B-954): data/worlds/<id>.json des Checkouts lesen, anlegen, ändern und prüfen. Dieselbe
// Logik wie die Seite Welten (internal/world); geschrieben wird atomar und format-stabil, nach Prüfung wie beim Laden
// im Spiel (sim.ParseWorld, engine/simcli world). Es läuft keine Simulation. Jedes Tool nimmt `checkout`.

// worldEngine ist die Engine-Seite des Editors (simcli world); Tests setzen einen Ersatz.
var worldEngine = world.Direct

func worldEditor(root string) world.Editor { return world.Editor{Root: root, Engine: worldEngine(root)} }

type worldListIn struct{}

type worldGetIn struct {
	ID string `json:"id" jsonschema:"Welt-ID (Dateiname ohne .json, a-z, 0-9, -)"`
}

type worldCreateIn struct {
	ID      string          `json:"id" jsonschema:"neue Welt-ID (a-z, 0-9, -, höchstens 32 Zeichen); die Datei darf noch nicht existieren"`
	Name    string          `json:"name" jsonschema:"Anzeigename der Welt"`
	Extends string          `json:"extends,omitempty" jsonschema:"Eltern-Welt, deren Werte geerbt werden (leer = Basis data/)"`
	Story   *worldStoryArgs `json:"story,omitempty" jsonschema:"Geschichte der neuen Welt wie bei world_story_set (lexicon, start, etappen, end, bottles, islands: {island1: {arrival, boss, done}}); fehlt sie, entsteht die Welt ohne story"`
}

type worldSetIn struct {
	ID    string         `json:"id" jsonschema:"Welt-ID"`
	Patch map[string]any `json:"patch" jsonschema:"Felder: name, notes, extends, combat {mode, groupSize} (null entfernt); islands: Liste von Inseln {id, name?, etappe? (Ganzzahl ≥ 1, lückenlos ab 1, nicht fallend; null entfernt), depths? | biomes?, depthScaling? {hp, damage, speed}, plannedChunks?, chunkCounts?, nature?, natureByDepth?}. Bekannte Insel-id: nur genannte Felder ändern; neue id: anhängen (Standard: Tiefen 0-4, Skalierung der Standard-Welt). biomes = Biom-IDs statt Tiefen (forest, cave, mine, ironhold, crystal)."`
}

type worldOverrideIn struct {
	ID    string   `json:"id" jsonschema:"Welt-ID"`
	File  string   `json:"file" jsonschema:"Basis-Datei ohne .json, z. B. economy (Namen nennt die Fehlermeldung bei einem unbekannten Feld)"`
	Path  []string `json:"path" jsonschema:"Pfad zum Feld in der Datei, z. B. [purse, startGold]"`
	Value any      `json:"value,omitempty" jsonschema:"neuer Wert (JSON); fehlt er oder ist null, erbt das Feld wieder"`
}

type worldChunksIn struct {
	ID     string `json:"id" jsonschema:"Welt-ID"`
	Island string `json:"island" jsonschema:"Insel-ID"`
	Depth  int    `json:"depth" jsonschema:"Tiefe der Stufe (Wert aus depths der Insel, nicht der Index)"`
	Chunks []any  `json:"chunks,omitempty" jsonschema:"geplante Chunks dieser Tiefe (plannedChunks, docs/rules/geplante-chunks.md), z. B. [{slot: -7, kind: harbor}]; [] entfernt, fehlt = unverändert"`
	Count  *int   `json:"count,omitempty" jsonschema:"Chunk-Anzahl dieser Tiefe (chunkCounts: 16, 24, 40, 64, 100, 160); 0 entfernt, fehlt = unverändert"`
}

func registerWorld(s *Server) {
	registerWorldRead(s)
	registerWorldWrite(s)
	registerWorldChunks(s)
	registerWorldStory(s)
	add(s, &mcp.Tool{
		Name: "world_check",
		Description: "Prüft die Datei einer Welt wie das Laden im Spiel (strenge Felder, Inseln, Tiefen, Bosse, Häfen, Chunks, Geschichte) und mit allen Nachfolger-Welten; " +
			"fehlende Einträge der Geschichte kommen als Hinweis, nicht als Fehler; ändert nichts. " +
			"Nutze es bei: nach Änderungen, bevor die Welt gespielt wird. Statt: Spiel starten und auf den Ladefehler warten.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in worldGetIn) (string, error) {
		e := worldEditor(s.ws(ctx).root)
		warnings, err := e.Check(ctx, in.ID)
		if err != nil {
			return "", err
		}
		text, _ := worldDone(fmt.Sprintf("Welt %s ist gültig", in.ID), warnings, nil)
		return text + storyHints(e.Load(in.ID)), nil
	})
}

func registerWorldRead(s *Server) {
	add(s, &mcp.Tool{
		Name: "world_list",
		Description: "Welten unter data/worlds/ mit ID, Name, Eltern-Welt und Zahl der Inseln, die Standard-Welt zuerst. " +
			"Nutze es bei: welche Welten es gibt, vor world_create. Statt: data/worlds/ auflisten und Dateien öffnen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, _ worldListIn) (string, error) {
		e := worldEditor(s.ws(ctx).root)
		var b strings.Builder
		for _, w := range e.List() {
			d := e.Load(w.ID)
			fmt.Fprintf(&b, "%s · %s · extends %s · %d Inseln\n", w.ID, w.Name, orDash(d.Extends), len(d.Islands))
		}
		return strings.TrimRight(b.String(), "\n"), nil
	})
	add(s, &mcp.Tool{
		Name: "world_get",
		Description: "Eine Welt: Kopf, Inseln mit Tiefen und Biomen, geplante Chunks, die Geschichte (Wortschatz und Zeilen je Wendepunkt, fehlende als „fehlt“), dazu der Text der Datei. " +
			"Nutze es bei: vor jeder Änderung an einer Welt. Statt: data/worlds/<id>.json lesen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in worldGetIn) (string, error) { return worldText(worldEditor(s.ws(ctx).root), in.ID) })
}

// worldWrite: schreibt nur die Welt-Datei, überschreibt keine andere Welt.
func worldWrite() *mcp.ToolAnnotations {
	notDestructive, closed := false, false
	return &mcp.ToolAnnotations{DestructiveHint: &notDestructive, IdempotentHint: true, OpenWorldHint: &closed}
}

// worldCreateHints: Anlegen ist nicht idempotent (ein zweiter Aufruf lehnt ab).
func worldCreateHints() *mcp.ToolAnnotations {
	a := worldWrite()
	a.IdempotentHint = false
	return a
}

func registerWorldWrite(s *Server) {
	write := worldWrite()
	add(s, &mcp.Tool{
		Name: "world_create",
		Description: "Legt eine neue Welt an (data/worlds/<id>.json): eine Insel island1 mit Tiefen 0-4, Kampf groups/5, optional von einer Eltern-Welt erbend und mit Geschichte (story); " +
			"eine vorhandene Welt wird nie überschrieben. Danach mit world_set, world_override_set, world_chunks_set und world_story_set ausbauen. " +
			"Nutze es bei: neue Welt entwerfen. Statt: Welt-Datei von Hand schreiben.",
		Annotations: worldCreateHints(),
	}, func(ctx context.Context, in worldCreateIn) (string, error) {
		return worldCreate(ctx, worldEditor(s.ws(ctx).root), in)
	})
	add(s, &mcp.Tool{
		Name: "world_set",
		Description: "Ändert Kopf (name, notes, extends, combat) und Inseln (anhängen oder Felder ändern: Tiefen oder Biome, Skalierung) einer Welt; " +
			"geprüft wie beim Laden im Spiel, bei Fehler bleibt die Datei. Nutze es bei: Inseln, Tiefen, Biome, Kampfvariante einer Welt festlegen. " +
			"Statt: Welt-Datei von Hand ändern.",
		Annotations: write,
	}, func(ctx context.Context, in worldSetIn) (string, error) {
		patch, err := json.Marshal(in.Patch)
		if err != nil {
			return "", err
		}
		warnings, err := worldEditor(s.ws(ctx).root).Set(ctx, in.ID, patch)
		return worldDone(fmt.Sprintf("Welt %s gespeichert", in.ID), warnings, err)
	})
	add(s, &mcp.Tool{
		Name: "world_override_set",
		Description: "Übersteuert ein Basis-Feld (data/<file>.json › path, z. B. economy › purse › startGold) in den `overrides` der Welt; " +
			"null oder der Basiswert setzt es zurück; geprüft mit allen Nachfolger-Welten. " +
			"Nutze es bei: Werte einer Welt anders als die Basis (Gold, Preise, Händler, Handwerker). Statt: data/*.json ändern (gilt für alle Welten).",
		Annotations: write,
	}, func(ctx context.Context, in worldOverrideIn) (string, error) {
		raw, err := json.Marshal(in.Value)
		if err != nil {
			return "", err
		}
		view := worldEditor(s.ws(ctx).root).SetOverride(ctx, in.ID, in.File, in.Path, raw)
		if view.Error != "" {
			return "", fmt.Errorf("%s", view.Error)
		}
		return worldDone(fmt.Sprintf("Override %s › %s der Welt %s gesetzt", in.File, strings.Join(in.Path, "."), in.ID), view.Warnings, nil)
	})
}

func registerWorldChunks(s *Server) {
	write := worldWrite()
	add(s, &mcp.Tool{
		Name: "world_chunks_set",
		Description: "Setzt die geplanten Chunks (plannedChunks) und/oder die Chunk-Anzahl (chunkCounts) einer Tiefe einer Insel; [] bzw. 0 entfernt. " +
			"Jede Insel außer der letzten braucht in Tiefe 0 einen Hafen-Chunk (kind harbor). Nutze es bei: Chunk-Planung einer Insel-Stufe. Statt: plannedChunks in der Datei von Hand bearbeiten.",
		Annotations: write,
	}, func(ctx context.Context, in worldChunksIn) (string, error) {
		var chunks json.RawMessage
		if in.Chunks != nil {
			raw, err := json.Marshal(in.Chunks)
			if err != nil {
				return "", err
			}
			chunks = raw
		}
		warnings, err := worldEditor(s.ws(ctx).root).SetChunks(ctx, in.ID, in.Island, in.Depth, chunks, in.Count)
		return worldDone(fmt.Sprintf("Chunks der Welt %s, Insel %s, Tiefe %d gespeichert", in.ID, in.Island, in.Depth), warnings, err)
	})
}

// worldDone ist die Antwort eines Welt-Tools: der Erfolgssatz mit den Warnungen der Prüfung, sonst der Fehler.
func worldDone(ok string, warnings []string, err error) (string, error) {
	if err != nil {
		return "", err
	}
	if len(warnings) == 0 {
		return ok, nil
	}
	return ok + "\nWarnungen:\n- " + strings.Join(warnings, "\n- "), nil
}

// worldText ist die Antwort von world_get: Kopf, Inseln (Tiefe → Biom, Planung) und der Text der Datei.
func worldText(e world.Editor, id string) (string, error) {
	d := e.Load(id)
	if d.Error != "" {
		return "", fmt.Errorf("%s", d.Error)
	}
	raw, err := e.Raw(id)
	if err != nil {
		return "", err
	}
	var b strings.Builder
	fmt.Fprintf(&b, "%s · %s · extends %s · %d Inseln\n", d.ID, d.Name, orDash(d.Extends), len(d.Islands))
	for _, isl := range d.Islands {
		et := ""
		if isl.Etappe != nil {
			et = " · Etappe " + strconv.Itoa(*isl.Etappe)
		}
		fmt.Fprintf(&b, "- %s · %s · Tiefen %v → %s%s\n", isl.ID, isl.Name, isl.Depths, strings.Join(isl.Biomes, ", "), et)
		for _, depth := range slices.Sorted(mapKeys(isl.Planned)) {
			fmt.Fprintf(&b, "    plannedChunks[%d]: %s\n", depth, strings.TrimSpace(string(isl.Planned[strconv.Itoa(depth)])))
		}
	}
	b.WriteString(storyText(d))
	b.WriteString("Datei:\n" + strings.TrimRight(raw, "\n"))
	return b.String(), nil
}

// mapKeys sind die Schlüssel von m als Zahl (Tiefen einer Planung, Etappen der Geschichte); Nicht-Zahlen entfallen.
func mapKeys[V any](m map[string]V) func(yield func(int) bool) {
	return func(yield func(int) bool) {
		for k := range m {
			if n, err := strconv.Atoi(k); err == nil && !yield(n) {
				return
			}
		}
	}
}
