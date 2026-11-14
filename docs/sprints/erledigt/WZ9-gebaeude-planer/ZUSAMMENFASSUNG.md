# WZ9 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Gebäude-Planer für die Freischaltung je Hub-Stufe
- **Projekt:** WZG

## Ziel

Ein Gebäude-Planer im Asset-Studio zeigt und bearbeitet, welcher Bauplatz ab welcher Hub-Stufe in welcher Tiefe verfügbar ist, prüft über das Go-Backend (Funktion aus WZ15, inklusive geplanter Chunks aus B-461, sofern WZ8 erledigt ist), schreibt `data/hub.json` und den erzeugten Block der Regeltabelle format-stabil und zeigt das Ergebnis in der Live-Vorschau, auch ungespeichert und ohne EXE-Neubau …

## Ergebnis

2026-10-18, Review WZ9.4 (autonom, getrennte Reviewer): AC-01 bis AC-05, AC-07, AC-08, AC-10 geprüft (`task:check`, `task:check:go`, `go:lint`, Studio-Tests grün, Golden-Diff leer); AC-09 teilweise (Vorrang geplanter Chunks → B-479); AC-06 angenommen, Validierung offen (WZ9.5). Behoben: Befunde der Reviews von WZ9.1 bis WZ9.3 (u. a. Welt ohne Insel, unbehandelte Promises, nicht lesbarer Kandidat, Sync und Rechte beim Schreiben). Tickets: B-479. Version: v– vorgeschlagen (Entwickler-Werkzeug).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-462/AC-01` Matrix Hub-Stufe × Bauplatz mit Platz und Tiefe. |
| AC-02 | geprüft | `B-462/AC-02` (Planer) Speichern nur nach Prüfung mit der Funktion aus WZ15.3 (auch für hi … |
| AC-03 | geprüft | `B-462/AC-03` Abweichungen zur erzeugten Regeltabelle sichtbar (Berechnung im Go-Backend m … |
| AC-04 | geprüft | `B-462/AC-04` Live-Vorschau zeigt den geprüften Stand des Planers, auch ungespeichert und … |
| AC-05 | geprüft | `B-462/AC-05` Hinweis auf REG-Änderung (`data/hub.json` und Regeltabelle) und Goldens. |
| AC-06 | geprüft | 🧑 nimmt den Gebäude-Planer im Asset-Studio ab und bestätigt die Pflichtplatz-Liste (Abnahm … |
| AC-07 | geprüft | `B-462/AC-06` Format stabil, atomar, Schreibregel bei extern geänderter Datei (`data/hub.j … |
| AC-08 | geprüft | `B-462/AC-08` (Matrix, Snapshot) Mehr Hub-Stufen ohne Umbau: Matrix und Snapshot-Option ze … |
| AC-09 | teilweise | `B-462/AC-09` Die Snapshot-Funktion aus WZ7.1 nimmt die Kandidaten-`hub.json` als Option ( … |
| AC-10 | geprüft | Der Sprint ändert weder `data/hub.json` noch `testdata/golden/`: `git diff --stat origin/d … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-462 | verworfen | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-07 geprüft, AC-08 geprüft, AC-09 teilweise | Ein Gebäude-Planer bearbeitet, welches Gebäude ab welcher Hub-Stufe verfügbar is … |
