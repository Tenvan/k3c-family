# B-509 · Im Studio geänderte Grafiken und Sounds erreichen die Clients ohne neuen Build

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ25
- **Projekt:** WZG
- **Erstellt:** 2026-10-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit TS1.4 (B-507) beobachtet der Server `public/` und startet bei Änderungen neu. Ausgeliefert wird aber `dist/` (Kopie von `public/` beim Build, Grafiken mit `Cache-Control: immutable`). Eine im Studio geänderte Grafik oder ein Sound erreicht die Clients deshalb erst nach `task build`. Zu klären: Auslieferung aus `public/` vor `dist/` (mit passendem Caching) oder Kopie beim Start.

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

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
