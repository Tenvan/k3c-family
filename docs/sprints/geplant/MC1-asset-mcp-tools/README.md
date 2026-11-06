# MC1 · DEV · Asset-MCP-Tools auf Spec-Basis, MCP-Prompts, Chat entfernt

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-699
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Welle 1, Lane **MC1** (DEV, Lane `mcp`, parallel zu RD1, WB1, RV1). Vertrag: `docs/standards/paper-mcp.md` › MCP-Tools. Bis RD1 fertig ist, ruft der Code die Attrappe `tools/paper/fixtures/cli-stub.ts` auf (Pfad über `K3C_PAPER_CLI`, Standard `tools/paper/bin/paper.ts`, Aufruf mit bun).

## Ziel

Alle Asset-Tools und MCP-Prompts aus dem Vertrag laufen in der Workbench; die alten Register-Tools sind entfernt.

Am Ende sichtbar: `assets_preview {id: monarch}` liefert ein Bild in Claude Code; `asset-material-aendern` erscheint als Prompt.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Entscheidung 005 › 4, 7; `docs/standards/paper-mcp.md` › Tools, Prompts.

## Nicht-Ziele

Review-API (RV1), Oberfläche (WB1), Renderer (RD1).

## Regeln und Einschränkungen

Nur `tools/k3c-workbench/internal/mcpsrv/tools_assets*.go` (ohne `tools_assets_research*`), `internal/mcpsrv/tools.go` (Registrierung), `internal/mcpsrv/instructions.md` (Abschnitt Assets), `internal/paperassets/` (neu: CLI-Aufruf, atomares Schreiben, Git-Basis, Job für `assets_render {alle}`). **Nicht** `app_claude*.go` („In Claude öffnen“ bleibt) und **nicht** `tools/k3c-studio/` (Abbau samt Chat in WB1). Jedes Tool nimmt `checkout`. Neue Tools sind live erst nach Merge und Neustart der Workbench aus der Repo-Wurzel (B-701 D-10); getestet wird in-process mit `httptest` gegen den MCP-Server.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Jedes Tool aus dem Vertrag hat einen In-Process-Test gegen die Attrappe, Antworten wie `fixtures/mcp/`.
- **AC-02** `assets_set` mit ungültigem Patch lässt Spec und Ausgaben unverändert (Hash vorher/nachher) (005/AC-06).
- **AC-03** `assets_preview` liefert MCP-ImageContent `image/png`.
- **AC-04** Jede Vorlage aus `data/asset-prompts.json` ist als Prompt `asset-<id>` mit ihren Platzhaltern abrufbar.
- **AC-05** `assets_render {alle}` antwortet sofort mit `{job}`, `assets_render {job, wait: true}` liefert das Ergebnis.
- **AC-06** Register-Varianten von `assets_list/get/set` und `assets_catalog` entfernt; `task check:dev` grün.

## Offene Fragen

keine

## Sessions

Reife Entwurf, Lane `mcp`:

- MC1.1 Sonnet: `internal/paperassets` (CLI, atomar, Basis, Job) (AC-02, AC-05)
- MC1.2 Sonnet: Tools guide, list, get, create, set, style, render, delete (AC-01)
- MC1.3 Sonnet: preview als Bild, review, MCP-Prompts, `instructions.md` (AC-03, AC-04)
- MC1.4 Sonnet: alte Register-Tools entfernen (AC-06)
- MC1.5 Review · Opus

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
