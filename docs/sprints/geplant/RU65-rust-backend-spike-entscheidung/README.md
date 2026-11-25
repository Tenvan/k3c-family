# RU65 · DEV · Rust-Backend für Workbench und Studio: Spike (Tauri) und Entscheidung 014

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entwurf (2026-11-25, Ralf: Plan für einen schrittweisen Umstieg von Workbench und Studio auf Rust, bei Bedarf abzuarbeiten). Beide Apps sind Wails (Go-Backend, React/TypeScript-Frontend, `tools/webkit`). Nach RU41 entfällt die Go-Engine (rund 59.000 Zeilen samt Tests); übrig bleiben Workbench rund 19.000, Studio rund 19.500, workkit 2.900, assetkit 4.000 Zeilen Go ohne Tests (Messung 2026-11-25).

## Ziel

Ein Spike zeigt, ob Tauri 2 (React-Frontend bleibt, Backend Rust) Wails ersetzen kann: Fenster, Commands/Events, MCP-Server über ein Rust-SDK (z. B. `rmcp`), Dev-Betrieb mit Hot Reload. Ergebnis ist die Entscheidung 014 (Annahme nur durch Ralf) mit Aufwand, Risiken und Reihenfolge der Folge-Sprints RU66 bis RU71. Fällt sie gegen Rust aus, werden RU66 bis RU71 verworfen.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `pfad/**` (Vereinigung der Erlaubten Dateien der Sessions ohne Sammeldateien; ein Sprint = eine Dateimenge)
- **Abhängig von:** – (nur IDs: SP00, B-000, SP00.1; erst starten, wenn erledigt und gemergt)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; Abnahmen am Gerät sind keine Abhängigkeit)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert, mit Herkunft, falls aus einem Ticket (`B-009/AC-01`).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`. Blockierende Fragen verhindern die Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
