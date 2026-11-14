# SP12 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Insel-Kern
- **Projekt:** –

## Ziel

Die Simulation kann eine **Insel** mit mehreren Stufen als Einheit rechnen: alle Stufen ticken, Spieler sind an eine Stufe gebunden und wechseln einzeln, das Material ist ein Vorrat je Insel, ein Spielstand speichert Insel und Stufe je Spieler. Am Ende sichtbar: `go test ./engine/sim` mit Insel-Tests (2–3 Stufen, 2–4 Spieler), Determinismus-Test und Benchmark der Tick-Dauer; Raum, Protokoll und Cl …

## Ergebnis

- 2026-10-02, Review SP12.4 (Agent, im selben Gespräch wie die Umsetzung): AC-01, AC-03, AC-06, AC-07 belegt (SP12.1 › Ergebnis), AC-02 und AC-05 (SP12.2 › Ergebnis), AC-04 (SP12.3 › Ergebnis). - Behobene Befunde: keine (keine schweren Befunde). Neue Tickets: keine (Folge: B-133 Raum auf Insel, B-104 Protokoll). - Hinweis: Tick-Dauer nur auf dem Entwickler-Rechner gemessen; Pi 3 offen (SP11).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Zwei Stufen ticken gleichzeitig, ein Spieler in jeder; Wellen und Zyklus laufen unabhängig … |
| AC-02 | – | Ein Spieler wechselt einzeln über Eingang und Treppe; Reihenfolge der Spieler bleibt, ande … |
| AC-03 | – | Gleicher Seed und gleiche Eingaben ergeben gleiche Zustände (B-100/AC-03). |
| AC-04 | – | Ein Spielstand speichert Insel und Stufe je Spieler; ein alter Stand wird geladen (B-100/A … |
| AC-05 | – | Die Rechenzeit je Tick mit 3 aktiven Stufen und 4 Spielern ist gemessen und im Ergebnis ge … |
| AC-06 | – | Das Material ist ein Vorrat je Insel (alle Stufen teilen ihn), Gold bleibt je Spieler (B-1 … |
| AC-07 | – | Die `Campaign` bleibt unverändert funktionsfähig: Raum, Protokoll und Golden-Tests laufen, … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-100 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 –, AC-06 – | Eine Insel hat n Stufen, die alle laufen und pro Spieler begehbar sind |
