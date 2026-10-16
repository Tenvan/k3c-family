# B-436 · plan_create scheitert an Platzhalter-Zeilen „– (Entwurf)“ in der Session-Tabelle

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ein Sprint-Entwurf (GEB1) führte seine Sessions in der Tabelle mit Datei `– (Entwurf)`. `plan_create` für GEB1.1 brach ab mit `open …\GEB1-grundregeln-werkstatt\– (Entwurf): The system cannot find the file specified.` Das Tool hält die Platzhalter-Zeile für eine bestehende Session und will die Datei öffnen. Workaround 2026-10-16: Zeilen von Hand entfernt, dann lief `plan_create`.

## Ziel

`plan_create` ersetzt eine Platzhalter-Zeile ohne Datei (`–`, `– (Entwurf)`) durch die neue Session-Zeile, statt abzubrechen.

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
