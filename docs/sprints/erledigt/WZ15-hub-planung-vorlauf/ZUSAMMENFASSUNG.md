# WZ15 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Hub-Planung Vorlauf: Regeltabelle erzeugt, Prüfung ohne globalen Zustand
- **Projekt:** WZG

## Ziel

Die Regeltabelle „Hub-Stufe → Gebäude“ in § 3 wird aus `data/hub.json` erzeugt (Go-Generator, Task `rules:update`, Test Block = JSON), und `engine/sim` bietet eine exportierte Prüfung einer Kandidaten-`hub.json` ohne globalen Zustand, mit Linien als Parameter in `engine/level`; keine feste 5 im Code. Am Ende sichtbar: im Terminal `task rules:update` ohne Diff, grüne Go-Tests (Block = JSON, Prüfung …

## Ergebnis

2026-10-18, Review WZ15.4 (autonom, getrennte Reviewer): AC-01 bis AC-04 geprüft (`task:check`, `task:check:go`, `go:lint` grün, Golden-Diff leer). Behoben: Befunde der Reviews von WZ15.1 bis WZ15.3 (u. a. Tor Linie 1, float64/hubLevel im Generator, Kandidaten-Kopie, Linienzahl ≤ Hub-Stufen). Neue Tickets: B-479 (geplante Chunks nach WZ8 prüfen, `CreateOptions` vereinen). Version: v– vorgeschlagen (Vorlauf für WZ9, kein Spielinhalt).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-462/AC-07` Regeltabelle in § 3 aus `data/hub.json` erzeugt (Go-Generator, Task `rules:u … |
| AC-02 | geprüft | `B-462/AC-02` (Prüfung) Eine Funktion in `engine/sim` prüft eine Kandidaten-`hub.json` ohn … |
| AC-03 | geprüft | `B-462/AC-08` (Prüfung, Generator) Mehr Hub-Stufen ohne Umbau: Eine Fixture mit 7 `levels` … |
| AC-04 | geprüft | Der Sprint ändert weder `data/hub.json` noch `testdata/golden/`: `git diff --stat origin/d … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-462 | verworfen | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft | Ein Gebäude-Planer bearbeitet, welches Gebäude ab welcher Hub-Stufe verfügbar is … |
