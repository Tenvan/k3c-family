# B-616 · Lauf-IDs der Balancing-Seite bleiben über einen Neustart der Workbench eindeutig

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Review WZ28.7 (2026-10-24): `tools/k3c-workbench/internal/simrun/registry.go` zählt Lauf-IDs `run-N` nach einem Neustart der Workbench wieder ab 1. `reports/simtest-run-N/` wird dann wiederverwendet; Berichte vermischen sich, Bewertungen in `reports/ratings.json` hängen an neu belegten Pfaden.

## Ziel

Lauf-IDs und ihre Ordner unter `reports/` sind über Neustarts eindeutig.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Startwert aus den vorhandenen Ordnern `reports/simtest-run-*` oder Zeitstempel in der ID.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Go-Test: Nach einem neuen Register im selben Checkout beginnt die nächste ID hinter dem höchsten vorhandenen Ordner.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
