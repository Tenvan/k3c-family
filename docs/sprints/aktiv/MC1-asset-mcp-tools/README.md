# MC1 · DEV · Asset-MCP-Tools auf Spec-Basis, MCP-Prompts, Chat entfernt

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** 8b0e2f18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-06, Chat (🧑: „alles bis zur Abnahme abarbeiten“; Defaults in B-701)

## Ausgangslage

Welle 1, Lane **MC1** (DEV, Lane `mcp`, parallel zu RD1, WB1, RV1). Vertrag: `docs/standards/paper-mcp.md` › MCP-Tools. Bis RD1 fertig ist, ruft der Code die Attrappe `tools/paper/fixtures/cli-stub.ts` auf (Pfad über `K3C_PAPER_CLI`, Standard `tools/paper/bin/paper.ts`, Aufruf mit bun).

## Ziel

Alle Asset-Tools und MCP-Prompts aus dem Vertrag laufen in der Workbench; die alten Register-Tools sind entfernt.

Am Ende sichtbar: `assets_preview {id: monarch}` liefert ein Bild in Claude Code; `asset-material-aendern` erscheint als Prompt.

## Beteiligte und Zielgruppen

- **Claude-Sessions** (Lanes MG*, `stil`, Review): ändern Assets nur über diese Tools, holen Prompts `asset-<id>`.
- **WB1, RV1:** bauen parallel; RV1 liefert die Review-Logik, die `assets_review` und `assets_preview {basis}` nur formatieren.
- **🧑 Ralf:** prüft nach Merge und Neustart der Workbench live (Bild in Claude Code, Prompt sichtbar), bestätigt die D-MC1-Zeilen.

## Anforderungen

Entscheidung 005 › 4, 7; `docs/standards/paper-mcp.md` › Tools, Prompts.

## Nicht-Ziele

Review-API (RV1), Oberfläche (WB1), Renderer (RD1).

## Regeln und Einschränkungen

Nur `tools/k3c-workbench/internal/mcpsrv/tools_assets*.go` (ohne `tools_assets_research*`), `internal/mcpsrv/tools.go` (Registrierung), `internal/mcpsrv/instructions.md` (Abschnitt Assets, Zeile Worktrees), `internal/mcpsrv/checkout.go` (nur `takesCheckout`), `internal/mcpsrv/workspace.go` (nur `writeTools`), `internal/mcpsrv/codemap.md` (nur Zeile Assets, MC1.4), `internal/paperassets/` (neu: CLI-Aufruf, Merge-Patch, atomares Schreiben, Git-Basis, Job für `assets_render {alle}`, Rig-Vorlagen). **Nicht** `app_claude*.go` („In Claude öffnen“ bleibt), **nicht** `internal/assetsui/` (RV1), **nicht** `tools/k3c-studio/` (Abbau samt Chat in WB1), **nicht** `tools/paper/` (Vertrag). Jedes Tool nimmt `checkout`. Neue Tools sind live erst nach Merge und Neustart der Workbench aus der Repo-Wurzel (B-701 D-10); getestet wird in-process gegen den MCP-Server (`connect`, In-Memory-Transport des go-sdk) mit der Attrappe `tools/paper/fixtures/cli-stub.ts` über `K3C_PAPER_CLI` (bun Pflicht, kein Skip). Sessions der Lane `mcp` laufen nacheinander (gleiche Dateien).

## Beispiele

- `assets_set {id: monarch, patch: {umhang: {material: purpur}}}` → `monarch geändert: 1 Feld · umhang.material koenigsrot → purpur`, Render-Zeile, `Checkout: …` (`fixtures/mcp/assets_set.txt`).
- `assets_render {alle: true}` → sofort `Job j-1 gestartet: 3 Assets (alle)`; `assets_render {job: "j-1", wait: true}` → Ergebnis je Asset.
- `assets_preview {id: monarch}` → Bild `image/png` (Frame 0 von `idle`) plus Teil-Boxen.
- Prompt `asset-material-aendern` mit `asset`, `rig`, `pfad`, `zustand` → eine User-Nachricht, die mit `assets_guide {rig: figur}` beginnt.

## Ausnahme- und Fehlerfälle

- Ungültiger Patch oder Render-Fehler → Fehler-Ergebnis `abgelehnt, nichts geändert: …`, Spec und Ausgabe unverändert (AC-02).
- Unbekannte `id` → `Asset „x“ fehlt`; unbekannte Animation → Liste der vorhandenen; ungültige `basis` → Vertragstext mit den erlaubten Werten.
- `assets_create` mit vorhandener `id` → Ablehnung; Figur-Vorlage mit leeren Lagen darf `proportion` melden, die Datei wird trotzdem angelegt.
- `assets_delete` ohne `confirm: true` → nur Hinweis.
- Unbekannter Job → `Job „j-99“ unbekannt`; abgebrochenes `wait` beendet nur das Warten, nicht den Job.
- `bun` fehlt → Fehler mit Hinweis auf `requirements.md`.

## Akzeptanzkriterien

- **AC-01** Jedes Tool aus dem Vertrag hat einen In-Process-Test gegen die Attrappe, Antworten wie `fixtures/mcp/`.
- **AC-02** `assets_set` mit ungültigem Patch lässt Spec und Ausgaben unverändert (Hash vorher/nachher) (005/AC-06).
- **AC-03** `assets_preview` liefert MCP-ImageContent `image/png`.
- **AC-04** Jede Vorlage aus `data/asset-prompts.json` ist als Prompt `asset-<id>` mit ihren Platzhaltern abrufbar.
- **AC-05** `assets_render {alle}` antwortet sofort mit `{job}`, `assets_render {job, wait: true}` liefert das Ergebnis.
- **AC-06** Register-Varianten von `assets_list/get/set` und `assets_catalog` entfernt; `task check:dev` grün.

## Offene Fragen

Keine offenen Fragen. Die Defaults stehen ab MC1.5 als D-MC1-01 bis D-MC1-15 in B-701 › Notizen (Status `offen` bis 🧑 bestätigt). Gegenüber der Planung geändert:

| D | Änderung in der Umsetzung |
|---|---|
| D-MC1-01 | alle `assets_*` nehmen `checkout` (Präfix in `takesCheckout`) |
| D-MC1-03 | Review-Quelle ist `assetsui.Dienst` (RV1 vor MC1.5 gemergt), Tests mit Fixture-Quelle |
| D-MC1-05 | Temp-Ordner spiegelt die Nachbar-Specs (Reittier-Suche der echten CLI), Rollback der Spec |
| D-MC1-09 | Vierbeiner-Vorlage mit Beinteilen, `eltern` und `bein` (echte CLI verlangt sie) |
| D-MC1-10 | Gruppen-Index wird bei create/set/render eingetragen, bei delete ausgetragen |
| D-MC1-12 bis -15 | neu: Gültigkeits-Cache, Hashes in voller Länge, zweite CLI-Anbindung (→ B-706), Asset ohne Animation |

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MC1.1 | `MC1.1-paperassets.md` | Umsetzung | autonom | fertig |
| MC1.2 | `MC1.2-asset-tools.md` | Umsetzung | autonom | fertig |
| MC1.3 | `MC1.3-preview-review-prompts.md` | Umsetzung | autonom | fertig |
| MC1.4 | `MC1.4-register-tools-entfernen.md` | Umsetzung | autonom | fertig |
| MC1.5 | `MC1.5-review.md` | Review | autonom | fertig |

Lane `mcp`, nacheinander (gleiche Dateien), Kette MC1.1 → MC1.2 → MC1.3 → MC1.4 → MC1.5; MC1.1 wartet auf das Vertrags-Review PV1.5.

- MC1.1 Sonnet: `internal/paperassets` (CLI, Merge-Patch, atomar, Basis, Job, Rig-Vorlagen) (AC-02, AC-05)
- MC1.2 Sonnet: Tools guide, list, get, create, set, style_get, style_set, render, delete (AC-01, AC-02, AC-05)
- MC1.3 Sonnet: preview als Bild, review, MCP-Prompts, `instructions.md` (AC-01, AC-03, AC-04)
- MC1.4 Sonnet: `assets_catalog` und Register-Reste entfernen, Doku (AC-06)
- MC1.5 Review · Opus (alle)

## Abnahme

2026-11-06 · AC-01 bis AC-06 erfüllt laut Ergebnissen MC1.1–MC1.5 (In-Process-Tests, dazu zwei Tests gegen die echte CLI aus RD1; `task check:diff` exit 0 nach Merge von `feature/pap`; `task check:race` übersprungen, kein gcc).
Behoben im Review: Gruppen-Index bei create/set/render, Vierbeiner-Vorlage für die echte CLI, CLI-Zeitlimit, Gruppe „alt“, Temp-Ordner mit Nachbar-Specs (Reittier), Review-Quelle auf `assetsui.Dienst` (RV1).
Neue Tickets: B-702 (wackelnder mcpsrv-Test), B-706 (Restbefunde). D-Zeilen D-MC1-01 bis D-MC1-15 in B-701.
Offen bei 🧑: Live-Prüfung nach Merge und Neustart der Workbench (`assets_preview {id: monarch}` als Bild, Prompt `asset-material-aendern`).
