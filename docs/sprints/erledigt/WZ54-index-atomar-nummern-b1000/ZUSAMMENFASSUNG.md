# WZ54 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Planungs-Index: Statuswechsel ohne Zeilenverlust, Ticket-Nummern ab B-1000
- **Projekt:** WZG

## Ziel

Kein Statuswechsel verliert eine Index-Zeile; Nummern ab B-1000 funktionieren.

## Ergebnis

2026-11-14, Claude (autonom). AC-01 bis AC-04 geprüft (WZ54.1, WZ54.2); `task check`, `task check:go`, `go:test`, `task check:dev` grün. Behoben im Review: Rollback löscht nur neue Dateien und meldet unvollständiges Zurücknehmen; Test-Regexe ab B-1000. Neue Tickets: keine angelegt (Vorschläge im Bericht: `remove` ohne Rollback, Task-Mindestversion für CLI_ARGS-Quoting). Abnahme-Ticket: keins, kein Sim-, Balance- oder Gen-Bezug.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-862/AC-01` grün. |
| AC-02 | geprüft | `B-863/AC-01` grün. |
| AC-03 | geprüft | `B-865/AC-01` grün (Fahrplan-Heilung nur bei Sprint- oder Session-Änderungen). |
| AC-04 | geprüft | `B-866/AC-01` grün (Testmuster mit Oder und Ankern). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-862 | eingeplant | AC-01 geprüft | Ein Statuswechsel schreibt Haupt- und Archiv-Index so, dass keine Zeile verloren … |
| B-863 | eingeplant | AC-02 geprüft | Die Planungs-Werkzeuge verarbeiten Ticket-Nummern ab B-1000 |
| B-865 | eingeplant | AC-03 geprüft | Die Fahrplan-Heilung schreibt nur bei Sprint- oder Session-Änderungen und erzeug … |
| B-866 | eingeplant | AC-04 geprüft | check_run nimmt Testmuster mit Oder-Strich, Dach und Dollar an |
