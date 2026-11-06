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

Keine offenen Fragen; Defaults beim Bereitmachen gesetzt (Format wie B-701, Bestätigung durch 🧑 oder in MC1.5):

| D | Thema | Default | Alternative | Sprints | Status |
|---|---|---|---|---|---|
| D-MC1-01 | Checkout für lesende Asset-Tools | `checkout.go` (`takesCheckout`) und `workspace.go` (`writeTools`) gehören zu den erlaubten Dateien | Vertrag „jedes Tool nimmt `checkout`“ nur für schreibende Tools | MC1 | offen |
| D-MC1-02 | Reihenfolge Ersetzen/Entfernen | MC1.2 ersetzt `assets_list/get/set` direkt (gleiche Namen sind nicht doppelt registrierbar); MC1.4 entfernt nur `assets_catalog` und Reste | Register-Tools bis MC1.4 umbenennen | MC1 | offen |
| D-MC1-03 | `assets_review` und `assets_preview {basis}` | über Schnittstelle `assetReview` auf `assetsui.Stubs` (PV1.8); Umstellung auf die echte RV1-Implementierung ist eine Zeile (Prüfpunkt MC1.5) | eigene Review-Logik in MC1 | MC1, RV1 | offen |
| D-MC1-04 | Bilder im Review-Text | Data-URLs der RPC-Methoden als PNG nach `.cache/paper/mcp/`, Pfad im Text; fehlt ein Bild, entfällt der Teil | nur Pixel-Anteil ohne Bild | MC1 | offen |
| D-MC1-05 | Temp-Ordner für atomares Schreiben | `.cache/paper/tmp/` im Checkout (gleiches Laufwerk, Rename atomar) | System-Temp | MC1 | offen |
| D-MC1-06 | Ablehnung als Fehler-Ergebnis | `isError: true` mit Text wie `assets_set-abgelehnt.txt` | normales Ergebnis mit Text „abgelehnt“ | MC1 | offen |
| D-MC1-07 | `assets_style_set` nur Lane `stil` | Hinweis in der Beschreibung, keine technische Sperre | Sperre über Session-Datei | MC1 | offen |
| D-MC1-08 | Prompts registrieren | Liste beim Start aus der Repo-Wurzel, Text bei jedem Abruf neu aus dem Checkout | Liste je Aufruf (go-sdk kennt keine Prompts je Checkout) | MC1 | offen |
| D-MC1-09 | Rig-Pflicht-Teile in Go | Tabelle `paperassets/rigs.go`, gegen die echte Prüfung (`validate` der Attrappe) getestet | aus `check.ts` lesen | MC1 | offen |
| D-MC1-10 | `assets_delete` und Gruppen-Index | `paperassets` entfernt den Eintrag aus `index.paper.json` selbst (Attrappe schreibt keinen Index) | Index über `render <ordner>` neu schreiben lassen (nach RD1) | MC1 | offen |
| D-MC1-11 | bun in Go-Tests | Pflicht, Test bricht ohne bun ab (CI-Job `k3c-workbench` hat bun) | `t.Skip` ohne bun | MC1 | offen |

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MC1.1 | `MC1.1-paperassets.md` | Umsetzung | autonom | fertig |
| MC1.2 | `MC1.2-asset-tools.md` | Umsetzung | autonom | fertig |
| MC1.3 | `MC1.3-preview-review-prompts.md` | Umsetzung | autonom | offen |
| MC1.4 | `MC1.4-register-tools-entfernen.md` | Umsetzung | autonom | offen |
| MC1.5 | `MC1.5-review.md` | Review | autonom | offen |

Lane `mcp`, nacheinander (gleiche Dateien), Kette MC1.1 → MC1.2 → MC1.3 → MC1.4 → MC1.5; MC1.1 wartet auf das Vertrags-Review PV1.5.

- MC1.1 Sonnet: `internal/paperassets` (CLI, Merge-Patch, atomar, Basis, Job, Rig-Vorlagen) (AC-02, AC-05)
- MC1.2 Sonnet: Tools guide, list, get, create, set, style_get, style_set, render, delete (AC-01, AC-02, AC-05)
- MC1.3 Sonnet: preview als Bild, review, MCP-Prompts, `instructions.md` (AC-01, AC-03, AC-04)
- MC1.4 Sonnet: `assets_catalog` und Register-Reste entfernen, Doku (AC-06)
- MC1.5 Review · Opus (alle)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
