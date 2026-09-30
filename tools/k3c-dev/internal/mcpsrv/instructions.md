# k3c-dev

Entwickler-Werkzeug des Spiels K3C. Diese Tools ersetzen Shell-Befehle und Dateilesen, weil sie nur das Wichtige
zurückgeben.

## Prüfen

- `check_run` statt `npm run check`, `npm test`, `npm run typecheck`, `npm run lint`, `npm run build`, `go test`
  oder `golangci-lint` in der Shell. Ziele: `npm:check`, `npm:test`, `npm:typecheck`, `npm:lint`, `npm:build`,
  `go:test`, `go:lint`, `dev:test`. Ein Testmuster (`pattern`) geht bei `npm:test`, `go:test` und `dev:test`.
- Die Antwort ist bei grünem Lauf eine Zeile, sonst Kopfzeile und nur die Fehlerzeilen.
- Mehr Kontext zu einem Lauf: `console_tail` mit `check:<ziel>`.

## Logs

- `logs_sources` zeigt alle Quellen. Log-Dateien liegen unter `logs/*.jsonl`, `k3c-dev` ist das eigene Log.
- `logs_errors` zuerst: Warnungen und Fehler, gleichartige zu einer Zeile verdichtet.
- `logs_query` für einzelne Einträge (Filter `minLevel`, `ns`, `pattern`, `since`, `limit`).
- `logs_since` zum Mitlesen: den `cursor` aus der letzten Antwort wieder mitgeben.

## Spieldaten

- `reports_list` und `report_read` statt `reports/*.json` zu öffnen: Xbox-Berichte der Gamepad-Testseite.
- `saves_list` statt `saves/` zu durchsuchen: Stufe, Tag, Spieler und Datum je Spielstand.

## Zustand

- `workbench_status`: Adresse, Laufzeit, Aufrufe, Clients, letzte Läufe, Log-Quellen.
