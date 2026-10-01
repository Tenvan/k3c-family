# D1 · SRV · Diagnose-Schnittstelle des Servers

- **Status:** aktiv
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-066, B-088
- **Start-Commit:** 7c4288c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat („D1 freigegeben“), Revision 1

## Ausgangslage

Der Go-Server loggt nur Text nach stderr; `logs/k3c-server.jsonl` entsteht nie (B-066 offen), und der Dienst „Heimnetz“ in
`tools/k3c-dev/services.json` hat keine Log-Quelle, deshalb sieht k3c-dev keine Server-Logs. `GET /api/status` kennt weder Speicher
noch Geräte noch Log noch Aktionen; die Diagnose-TUI (SP10, B-002) braucht das.

## Ziel

Der Server schreibt ein JSON-Log, das k3c-dev liest, und bietet Speicher, Geräte, Log und zwei Aktionen hinter dem Token an.
Am Ende sichtbar: `logs_query Heimnetz`-Einträge in k3c-dev und `curl`-Aufrufe der neuen Endpunkte mit Token.

## Beteiligte und Zielgruppen

🧑 als Betreiber; Entwickler und Agenten über k3c-dev; die TUI (SP10) als Client.

## Anforderungen

B-066 › Anforderungen (Ordner `K3C_LOG_DIR` oder vorhandenes `logs/`, sonst stderr; `services.json` mit `"log": "k3c-server"`),
B-088 › Anforderungen. Sprint-eigen: Log und Aktionen schreiben keine Tokens und keine vollen Geräte-IDs.

## Nicht-Ziele

Die TUI (SP10); neue MCP-Tools in k3c-dev für die Endpunkte (später aus M6 heraus erweiterbar); Log-Rotation (B-066 › Nicht-Ziele);
Änderungen am Protokoll v2 und am Spielverhalten.

## Regeln und Einschränkungen

Standardbibliothek; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. `engine/net` importiert nichts aus `cmd/`.
Der Dienst „Heimnetz“ in `tools/k3c-dev/services.json` gehört zur Domäne SRV (Entwickler-Werkzeug).

## Beispiele

- `k3c-server` aus dem Repo gestartet → `logs/k3c-server.jsonl` wächst, `logs_errors Heimnetz` in k3c-dev zeigt Fehler.
- `POST /api/status/save?room=FAMILIE` mit Token → `{"ok":true,…}`, Spielstand liegt in `saves/`.

## Ausnahme- und Fehlerfälle

- Ordner nicht schreibbar → Log bleibt auf stderr, eine Warnung.
- Kein Token am Server → alle Diagnose-Wege antworten 404; falsches Token 401; falsche Methode 405.
- Unbekannter Raum oder unbekanntes Gerät → 404, mehrdeutige Gerätekennung → 409.

## Akzeptanzkriterien

- **AC-01** Der Server schreibt sein Log zusätzlich als JSON nach `<Ordner>/k3c-server.jsonl`; ohne Ordner bleibt es bei stderr; k3c-dev führt „Heimnetz“ mit dieser Log-Quelle (B-066/AC-01, B-066/AC-02, B-066/AC-03).
- **AC-02** `/api/status` zeigt `memory`, `?room=` die `devices`, `/api/status/log` die Log-Zeilen ab Cursor (B-088/AC-01, B-088/AC-02).
- **AC-03** `disconnect` und `save` wirken, alle Wege sind durch das Token geschützt (B-088/AC-03, B-088/AC-04).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| D1.1 | `D1.1-json-log.md` | Umsetzung | autonom | fertig |
| D1.2 | `D1.2-status-log-endpunkt.md` | Umsetzung | autonom | offen |
| D1.3 | `D1.3-aktionen.md` | Umsetzung | autonom | offen |
| D1.4 | `D1.4-review.md` | Review | autonom | offen |

## Abnahme

–
