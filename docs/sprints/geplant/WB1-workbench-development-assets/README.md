# WB1 · DEV · Workbench mit Bereichen Development und Assets, Studio-Programm abgebaut

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

Welle 1, Lane **WB1** (DEV, Lane `ui`, parallel zu RD1, MC1, RV1). Die Workbench (`tools/k3c-workbench/frontend/src`) hat heute die Seiten Dienste, Logs, Planung, Balancing, Git, Monitoring, MCP und einen Studio-Link; das Studio (`tools/k3c-studio/`) ist ein eigenes Programm mit Chat. Die RPC-Methoden der Assets (`AssetsList`, `AssetsGet`, `AssetsStyle`, `AssetsPrompts`, `AssetsFrame` …) und der Platzhalter `ReviewPage` liegen seit PV1.8 als Stubs auf `develop`; RV1 füllt sie.

## Ziel

Die Workbench hat die Bereiche **Development** (alle heutigen Seiten) und **Assets** (Review, Katalog, Styleguide, Sound, Quellen) ohne schreibende Controls, mit Prompt-Buttons; das Studio-Programm samt Chat ist abgebaut.

Am Ende sichtbar: `task k3c-workbench:web` zeigt beide Bereiche; `bin/k3c-studio*.exe` entstehen nicht mehr.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Entscheidung 005 › 5; `docs/arbeitsweise.md` › Oberflächen der Wails-Programme (Atomic Design über `@webkit`).

## Nicht-Ziele

Review-Seite selbst (RV1, eigener Ordner `frontend/src/assets/review/`), MCP (MC1).

## Regeln und Einschränkungen

DEV-Sessions nur `tools/k3c-workbench/frontend/` (außer `src/assets/review/`), `tools/k3c-workbench/internal/studioview/` (neu, lesende Studio-Teile inkl. „Im Spiel ansehen“), `tools/k3c-workbench/app_studio.go`, `tools/k3c-studio/` (löschen). Die INF-Session WB1.5 ändert nur `.github/workflows/ci.yml`, `Taskfile.yml`, `tools/k3c-studio/Taskfile.yml`, `vite.config.ts`, `tests/atomicDesign.test.ts`, `tests/offlineTools.test.ts`, `tests/projectRules.test.ts`, `requirements.md`, `docs/glossar.md` (Eintrag Asset-Studio), `docs/arbeitsweise.md` (DEV-Zeile), `CLAUDE.md`. **Schreibendes Control** = Control, das Dateien, Specs oder Zustände ändert; Filter, Auswahl, Schieberegler zum Ansehen sind erlaubt. „In Claude öffnen“ bleibt (B-701 D-12).

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Kopfleiste mit Development und Assets; alle bisherigen Seiten unter Development erreichbar (Vitest) (005/AC-07).
- **AC-02** Assets: Katalog (Animations-Vorschau, Abzeichen „im Diff“, **roter Eintrag** für ungültige Specs mit Fehlertext), Styleguide (Materialtafel), Sound, Quellen; kein schreibendes Control (Test) (005/AC-07, §7).
- **AC-03** Prompt-Buttons am Asset und am angeklickten Teil kopieren den gefüllten Prompt aus `AssetsPrompts` (Test der Platzhalter-Ersetzung) (005/AC-09).
- **AC-04** `tools/k3c-studio/` samt Chat, EXEs, Ports 8091/8182 und Studio-Tasks entfernt; `task buildall` grün (005/AC-10).

## Offene Fragen

keine

## Sessions

Reife Entwurf, Lane `ui`:

- WB1.1 Sonnet: Navigation Development/Assets, Routen inkl. `ReviewPage` (AC-01)
- WB1.2 Sonnet: Katalog und Styleguide gegen die Stubs (AC-02)
- WB1.3 Sonnet: Prompt-Buttons, Teil-Auswahl per Klick (AC-03)
- WB1.4 Sonnet: lesende Studio-Teile nach `internal/studioview`, `tools/k3c-studio` löschen (AC-04)
- WB1.5 INF · Sonnet: Studio aus CI, Taskfile, Repo-Regeln und Doku (AC-04)
- WB1.6 Review · Opus

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
