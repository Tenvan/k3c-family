# B-1244 · Das Schiff kostet deutlich mehr und wird in mehreren Stufen bezahlt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** ECO1
- **Projekt:** WIR
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-24
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Aus dem Spieltest 2026-11-22-11, Befund f6 (Zeitpunkt 1795364856455 ms Unix).

> Das sollte natürlich deutlich teurer sein, wenn man das bauen will … in mehreren Stufen sollte man das Schiff bezahlen können.

- **Spielkontext:** Insel 1 Stufe 0, Welt standard, Tag, Tag 1, Bau des Schiffs

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **Bezahlung in Stufen:** Jede Bezahlstufe kostet 2 Gold. Eine Stufe verlangt ein Stück Holz, das ein oder mehrere Handwerker zur Baustelle des Schiffs bringen. Erst danach verarbeiten Handwerker das Holz in das Schiff.
- **Anzahl der Bezahlstufen (geklärt):** 5, eine je Insel der Standard-Welt.
- **Holzmenge je Insel (geklärt):** 8, 12, 16, 20, 24 Hölzer, von der ersten zur letzten Insel steigend. Summe 80, wie der heutige Kiel.
- **Einstellbarkeit (geklärt):** Stufen und Holz je Insel sind nicht einstellbar. Die Werte stehen fest in `data/ship.json`; kein Override je Welt.
- **Stapellauf (geklärt):** Nach dem Bau kostet der Stapellauf 5 Gold. Der Betrag steht als Zahl in `data/ship.json`.
- **Animation (geklärt):** Das Schiff gleitet nach dem Bezahlen vom Steg ins Wasser, rund 4 Sekunden.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `data/ship.json`, `rust/crates/k3c-sim/src/world/ship.rs`
- **Abhängig von:** –
- **Braucht 🧑:** Sichtprüfung der Stapellauf-Animation (Anzeige nicht im Bereich dieses Tickets, eigener Punkt in der Ralf-Queue)
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
