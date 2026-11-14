# WZ55 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** assets_set ruft die Paper-CLI gesammelt auf
- **Projekt:** WZG

## Ziel

Ein Sammelaufruf der Paper-CLI erledigt die Schritte eines `assets_set` in einem Prozess.

## Ergebnis

2026-11-14, Claude (autonom). AC-01 bis AC-03 geprüft (WZ55.1, WZ55.2); `task check`, `task check:go`, `go:test`, `task check:dev` grün; `assets_set` startet bun einmal statt zwölfmal. Behoben im Review: Zeitlimit des Sammelaufrufs, Test ohne Skip, gocyclo in `apply`. Neue Tickets: keine angelegt (Vorschläge im Bericht). Abnahme-Ticket: vorgeschlagen, `gen:paper` samt Grafik-Golden (CLI geändert, Render-Logik nicht); vom Koordinator anzulegen.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-856/AC-01` und `B-856/AC-02` grün. |
| AC-02 | geprüft | `B-867/AC-01` grün (gescheitertes Entfernen zurückgenommen). |
| AC-03 | geprüft | `B-868/AC-01` grün (Mindestversion von Task geprüft). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-856 | eingeplant | AC-01 geprüft | assets_set ruft die Paper-CLI gesammelt auf statt zwölfmal je Änderung |
| B-867 | eingeplant | AC-02 geprüft | Ein gescheitertes Entfernen in changeSet.apply wird zurückgenommen |
| B-868 | eingeplant | AC-03 geprüft | Die Mindestversion von Task wird geprüft, auf die check_run beim Quoten von CLI_ … |
