# WZ45 · DEV · Asset-Seiten: Filter und Prompt-Buttons für Styleguide, Sound und Quellen

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-727, B-728, B-729
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: alle offenen angehen)

## Ausgangslage

Katalog und Review im Bereich Assets haben Filter und Prompt-Buttons; Styleguide, Sound-Seite und Quellen-Seite noch nicht (B-727, B-728, B-729, Ideen aus PAP). Eingereiht am 2026-11-12 bei der Bereinigung des Backlogs (Entscheidung 009), nicht Teil der von 🧑 freigegebenen Welle 5.

## Ziel

Styleguide, Sound- und Quellen-Seite lassen sich filtern und schicken je Eintrag einen kurzen Prompt an Claude.

Am Ende sichtbar: Klick auf ein Material, einen Sound-Slot oder eine Quelle legt einen passenden Prompt in die Zwischenablage bzw. den Chat.

## Beteiligte und Zielgruppen

🧑 Ralf (nutzt die Seiten, gibt die Spec frei); Claude (Umsetzung).

## Anforderungen

`B-727 › Ziel`, `B-728 › Ziel`, `B-729 › Ziel`; Prompt-Füllung über `fillPrompt` (nach B-703).

## Nicht-Ziele

Neue Asset-Arten, Änderungen an Assets ohne MCP (Entscheidung 005).

## Regeln und Einschränkungen

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/**`, `data/asset-prompts.json`, `docs/standards/paper-mcp.md`, `tools/paper/fixtures/ui/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

Styleguide › Material `koenigsrot` › „Alle Nutzer zeigen“ → Prompt mit Material-ID.

## Ausnahme- und Fehlerfälle

Leere Filtertreffer → Hinweis statt leerer Liste.

## Akzeptanzkriterien

- **AC-01** Styleguide hat Prompt-Buttons für Material, Ton und Papier (`B-727/AC-01`, `AC-02`).
- **AC-02** Sound-Seite hat Filter und Prompt-Buttons (`B-728`).
- **AC-03** Quellen-Seite hat Filter und Prompt-Buttons (`B-729`).
- **AC-04** Review ohne offenen schweren Befund.

## Offene Fragen

keine. Entschieden (Default, autonom Claude, 2026-11-14): Aktionen wie in den Tickets B-727 bis B-729 notiert.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ45.1 | `WZ45.1-styleguide-prompts.md` | Umsetzung | autonom | fertig |
| WZ45.2 | `WZ45.2-sound-filter-prompts.md` | Umsetzung | autonom | fertig |
| WZ45.3 | `WZ45.3-quellen-filter-prompts.md` | Umsetzung | autonom | fertig |
| WZ45.4 | `WZ45.4-review.md` | Umsetzung | autonom | fertig |

Entwurf (Stichpunkte, Session-Dateien vor dem Aktivieren):

- WZ45.1 Styleguide-Prompts (AC-01)
- WZ45.2 Sound- und Quellen-Seite: Filter und Prompts (AC-02, AC-03)
- WZ45.3 Review (alle)

## Abnahme

2026-11-14, Claude (autonom). AC-01 bis AC-03 geprüft laut Ergebnis WZ45.1 bis WZ45.3 (Tests je Seite), AC-04 laut WZ45.4 (`task check`, `check:dev`, `check:go` grün).
Behobene Befunde: keine schweren; im Lauf `readonly.test.tsx` auf das Suchfeld des ListFilter erweitert.
Neue Tickets: keine. Abnahme-Ticket: B-882 (Sim/Balance nicht relevant, Gen-Gegenprobe, Race).
