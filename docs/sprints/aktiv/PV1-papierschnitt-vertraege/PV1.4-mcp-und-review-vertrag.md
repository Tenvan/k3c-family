# PV1.4 · MCP-Vertrag, Review-API und Prompt-Vorlagen festschreiben

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** pv1/4-mcp-und-review-vertrag
- **Abhängig von:** PV1.3
- **Tickets:** B-699
- **Kriterien:** AC-06

## Ziel

Auf `develop` liegen `docs/standards/paper-mcp.md` (alle Asset-Tools, RPC-Methoden der Oberfläche, Prompt-Vorlagen), `data/asset-prompts.json` sowie Attrappen und Beispielantworten unter `tools/paper/fixtures/`, gegen die MC1, WB1 und RV1 ohne Renderer arbeiten.

## Kontext

- Entscheidung 005 › 4–7. CLI-Vertrag, Hash- und Ausgabe-Regeln: `docs/standards/paper-spec.md` (PV1.3).
- MCP-Server: `tools/k3c-workbench/internal/mcpsrv/` (Registrierung `tools.go`, Anleitung `instructions.md`, heutige `tools_assets.go` für das alte Register `data/assets.json`). Regeln: `<bereich>_<verb>`, Beschreibung endet mit „Nutze es bei: … Statt: …“, `checkout` bei jedem Tool, das im Worktree einen anderen Stand sehen kann. `task_*` bedient nur die Repo-Wurzel.
- **Oberfläche der Workbench spricht kein REST:** nur Wails-Bindings bzw. `POST /api/dev/rpc/{Method}` mit der Allowlist `uiMethods` (`uiapi.go`), gleich gehalten mit `frontend/src/api/wails.ts`. Bilder deshalb als Data-URL (PNG, base64).

## Erlaubte Dateien

- `docs/standards/paper-mcp.md`
- `data/asset-prompts.json`
- `tools/paper/fixtures/` (neu: `mcp/`, `ui/`, `review-repo/`, `cli-stub.ts`)
- `tools/paper/schema/` (nur `prompts.schema.json` und Test)

## Nicht-Ziele

Keine Tool-Implementierung (MC1), keine API im Go-Backend (RV1), keine Oberfläche (WB1).

## Schritte

1. Test `tools/paper/schema/prompts.test.ts`: `data/asset-prompts.json` gültig gegen `prompts.schema.json`; Platzhalter nur `{asset} {rig} {teil} {pfad} {zustand} {basis} {eingabe}`; `ort` ist `asset|teil|review`; IDs eindeutig. Rot.
2. `data/asset-prompts.json` = `[{id, titel, ort, text}]` mit mindestens: `material-aendern`, `proportion-anpassen`, `detail-ergaenzen`, `variante-anlegen`, `animation-aendern`, `neues-wie-dieses` (asset); `teil-material`, `teil-form` (teil); `aenderung-zuruecknehmen`, `nachbessern` (review, mit `{eingabe}`). Jeder Text beginnt mit „Rufe zuerst `assets_guide {rig: {rig}}` auf“ und endet mit „Prüfe das Ergebnis mit `assets_preview` und rufe zum Schluss `assets_review` auf.“
3. `docs/standards/paper-mcp.md` › **MCP-Tools** (jedes mit `checkout`):
   - `assets_guide {rig?}` → Text: Stil-Regeln, Papier-Materialien, Pflicht-Teile, Pivots, Beispiel-Spec.
   - `assets_list {rig?, gruppe?, geaendert?}` → je Asset `id · rig · pfad · gültig|ungültig[ · geändert]`.
   - `assets_get {id, felder?}` → Spec als JSON.
   - `assets_create {id, rig, beschreibung, gruppe}` → Pfad der neuen Spec aus der Rig-Vorlage, Ergebnis von `validate`.
   - `assets_set {id, patch}` → JSON-Merge-Patch (RFC 7396); validate → render in Tempordner → Spec per Temp-Datei und Umbenennen ersetzen, dann Ausgaben; Antwort: geänderte Felder, Anzahl PNGs, Pixel-Anteil; Ablehnung: `feld · regel · text`, nichts geändert.
   - `assets_style_get {}`, `assets_style_set {patch}` (nur Sessions mit Lane `stil`; startet ein Render aller betroffenen Assets als Job).
   - `assets_render {id}` synchron; `assets_render {alle: true}` oder Stil-Render → sofort `{job}`; `assets_render {job, wait?}` → `laeuft n/m` bzw. Ergebnis (eigener Job-Mechanismus, nicht `task_*`).
   - `assets_delete {id, confirm}`.
   - `assets_preview {id, animation?, frame?, variante?, basis?}` → MCP-ImageContent `image/png` plus Text mit Teil-Boxen.
   - `assets_review {basis?}` → je geändertem Asset `status`, Felder, Teile, Pixel-Anteil; Stiländerungen melden alle betroffenen Assets mit `stil: true`.
   - `basis`: `HEAD` (Standard: Arbeitsstand gegen letzten Commit), `HEAD~1`, Commit-SHA, `origin/develop`; ohne vorheriges Fetch Antwortzeile `Basis-Stand: lokal`.
   - Ersetzt werden die alten Register-Tools `assets_list/get/set/catalog` (Register `data/assets.json` ab MC1 eingefroren, Entfernung B-700). `assets_research`, `assets_hits`, `assets_hit_add`, `assets_hit_drop`, `assets_import` bleiben.
   - **MCP-Prompts:** jede Vorlage als Prompt `asset-<id>`, Argumente = ihre Platzhalter.
4. `docs/standards/paper-mcp.md` › **Oberfläche (RPC)**, Besitzer RV1, Stubs PV1.8: `AssetsList() []AssetEintrag`, `AssetsGet(id) Spec`, `AssetsStyle() Stil`, `AssetsPrompts() []Vorlage`, `AssetsReview(basis) Review` = `{basis, basisCommit, basisStand: aktuell|lokal, assets: [{id, rig, pfad, status: neu|geaendert|geloescht|ungueltig, fehler?, felder: [{pfad, alt, neu}], teile: [], stil: bool, pixelAnteil}]}`, `AssetsFrame(id, animation, frame, variante, basis) {bild (Data-URL), teile}`, `AssetsPixelDiff(id, animation, frame, basis) {anteil, boxen, overlay (Data-URL)}`. Fehler: ungültige Basis → Fehlertext mit erlaubten Werten. Cache `.cache/paper/<renderKey>/`.
5. **Attrappen:** `fixtures/cli-stub.ts` beantwortet die fünf CLI-Befehle deterministisch: bekannte Fixtures aus `fixtures/`, **unbekannte, aber gültige Specs** mit einem Platzhalter-PNG (damit `assets_create` testbar ist), ungültige mit Exit 1. `fixtures/ui/*.json` je RPC-Methode; Review-Fälle `neu`, `geaendert` (Umhang purpur, Bild aus `monarch-galopp-0-purpur.svg`), `geloescht`, `ungueltig`, `stil`. `fixtures/mcp/*.txt` Beispielantworten aller Tools. `fixtures/review-repo/` Skript, das ein Test-Git-Repo mit diesen Fällen erzeugt (für RV1).
6. Test `stub.test.ts`: `validate` monarch → `{ok:true}`; ungültige Fixture → Exit 1 mit Regel; neue gültige Spec → `render` ok.
7. Tests grün, Commit `docs(pap): MCP-, RPC- und Prompt-Vertrag mit Attrappen`.

## Fertig, wenn

- [ ] AC-06: Standard vollständig (Schritte 3, 4, Prompt-Format); `task test -- paper` grün inkl. `prompts.test.ts` und `stub.test.ts`.
- [ ] Jede Beispielantwort verweist auf existierende Fixture-PNGs.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
