# WZ49 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Planungs-Werkzeuge und check_run: Restbefunde, Fahrplan aus Dateien, Im Spiel ansehen nativ
- **Projekt:** WZG

## Ziel

Die Planungs- und Prüf-Werkzeuge der Workbench rechnen richtig und schreiben den Fahrplan vollständig; „Im Spiel ansehen“ startet das native Spiel.

## Ergebnis

2026-11-14 (Claude, autonom): AC-01 bis AC-05 geprüft (WZ49.1: AC-02, AC-03; WZ49.2: AC-01, AC-04, AC-05). Review WZ49.3: `task check`, `task check:go` (ohne Sim), `go:test`, `task check:dev` grün; zwei leichte Befunde des k3c-reviewers behoben (Fehler beim Fahrplan-Heilen, Mensch-Sessions belegen keinen Platz). Neue Wirkung erst nach Neubau der Workbench-EXE (Sperre exe); der erste schreibende `plan_*`-Aufruf danach heilt den Fahrplan. Kein Abnahme-Ticket (kein Sim-, Balance- oder Gen-Bezug).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-831` Kriterien grün. |
| AC-02 | geprüft | `B-839` Kriterien grün. |
| AC-03 | geprüft | `B-842` Kriterien grün. |
| AC-04 | geprüft | `B-847` Kriterien grün. |
| AC-05 | geprüft | `B-855` Kriterien grün (Konsolenpuffer je Checkout). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-831 | eingeplant | AC-01 geprüft | check_run dev:test baut das Frontend vorher und reicht für die volle Workbench-S … |
| B-839 | eingeplant | AC-02 geprüft | plan_next und plan_waves zählen Plätze, Ordner-Pfade und laufende Sessions ohne … |
| B-842 | eingeplant | AC-03 geprüft | Fahrplan-Abschnitt „Offen am Gerät“ und verwaiste Zeilen pflegt das Werkzeug |
| B-847 | eingeplant | AC-04 geprüft | „Im Spiel ansehen“ der Workbench startet das native Spiel statt game.html |
| B-855 | eingeplant | AC-05 geprüft | console_tail und check_run trennen die Konsolenpuffer je Checkout |
