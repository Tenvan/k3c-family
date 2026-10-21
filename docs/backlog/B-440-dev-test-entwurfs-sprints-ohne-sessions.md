# B-440 · k3c-dev-Planungstest lehnt Entwurfs-Sprints ohne Sessions ab

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ20
- **Projekt:** WZG
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`check_run dev:test` scheitert in `tools/k3c-dev/internal/planning` (`TestRepo`, `planning_test.go:75`): „Sprint GEB3 ohne Titel oder Sessions“, ebenso GEB4–GEB7. Die Sprints sind gültige Entwürfe (`Reife: Entwurf`, Sessions als Stichpunkte erlaubt laut `docs/arbeitsweise.md` › Sprint-Lebenslauf), der Go-Test verlangt aber Sessions. Gefunden in GEB2.3 (2026-10-16), besteht schon auf `develop` seit #302.

## Ziel

Der Go-Planungstest akzeptiert Sprints mit `Reife: Entwurf` ohne Session-Dateien; `check_run dev:test` ist wieder grün.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
