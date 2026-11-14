# WZ6 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Asset-Studio als eigene EXE mit Seiten für Grafik und Sound
- **Projekt:** WZG

## Ziel

Ein eigenes Asset-Studio (EXE mit festem Pfad, eigener Task) mit je einer Seite für Grafik und Sound; die Workbench ist davon entkoppelt. Am Ende sichtbar: beide Seiten im neuen Fenster, Workbench ohne Assets-Seite, von 🧑 abgenommen.

## Ergebnis

- 2026-10-17 (Review WZ6.7): AC-01–AC-07 angenommen laut Ergebnissen WZ6.1–WZ6.6; Diff ohne schwere Befunde, Checks grün (`task check`, `check:go`, `dev:test`, `build`). Behobene Befunde: keine. - 2026-10-17 (WZ6.8): AC-08 von Ralf am PC abgenommen (alle 13 Punkte bestanden, Chat); AC-01 und AC-03 damit validiert. B-421 `erledigt` (B-298, B-299 ebenfalls), B-459 `erledigt`. - Neue Tickets: B-473 (CI auf `develop` rot, k3c-dev `internal/planning`), B-474 (`TestDownloadHit` lokal an Norton-TLS). - Version: v0.19.0 vorgeschlagen (neues Werkzeug Asset-Studio, Minor); nicht gesetzt, wartet auf grün …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | teilweise | `B-459/AC-01` eigene EXE `bin/k3c-studio.exe` mit Task, nur Loopback. |
| AC-02 | – | `B-459/AC-02` Seiten Grafik und Sound mit heutigem Umfang. |
| AC-03 | – | `B-459/AC-03` Workbench ohne Assets-Seite, Verweis startet das Studio, Neubau stört sie ni … |
| AC-04 | – | `B-459/AC-04` Tests in `task check:all` und CI. |
| AC-05 | – | `B-459/AC-05` Domänentabelle und Glossar. |
| AC-06 | geprüft | `B-459/AC-06` gesamter Inhalt von `assetvorschau.html` (Grafik- und Sound-Szenen) im Studi … |
| AC-07 | – | `B-459/AC-07` MCP bleibt in k3c-dev; Neuladen ohne Verlust, Speichern auf veraltetem Stand … |
| AC-08 | erfüllt | 🧑 nimmt Studio, umgezogene Vorschau und ruhige Workbench ab und entscheidet über B-421 (Ab … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-459 | erledigt | AC-01 teilweise, AC-02 –, AC-03 –, AC-04 –, AC-05 –, AC-06 geprüft, AC-07 – | Die Asset-Seiten für Grafik und Sound laufen in einer eigenen EXE getrennt von d … |
