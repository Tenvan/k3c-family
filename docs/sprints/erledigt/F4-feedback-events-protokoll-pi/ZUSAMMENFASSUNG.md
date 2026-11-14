# F4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Feedback-Ereignisse im Protokoll und Pi-Betrieb
- **Projekt:** –

## Ziel

Ereignisse laufen im Protokoll zum Client mit gemessener Bandbreite, der Pi füllt seine SD-Karte nicht, und Restore ist abgesichert. Am Ende sichtbar: Benchmark mit Bytes je Tick, README-Abschnitt zum Backup, Test „Restore ohne Berechtigung abgelehnt“.

## Ergebnis

2026-10-03, Review F4.4 (Sonnet): `task check` und `task check:go` grün, keine schweren Befunde, keine neuen Tickets. AC-01 bis AC-04 geprüft in F4.1, AC-05 und AC-06 in F4.2, AC-07 in F4.3 (Ergebnisse der Sessions). Restore-Probe am Pi (AC-06, Q18, 🧑): angenommen, Validierung offen (Session); Backup lokal getestet, der Pi-Lauf folgt mit dem Gerät. Version: v0.6.0 vorgeschlagen (Minor: Wirkung im Server — Rotation, Backup-Skript, Restore nur mit Token, Ereignis-Budget geprüft; aktuell v0.5.0).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Ein Ereignis erscheint im Snapshot nur der betroffenen Stufe (B-140/AC-01). |
| AC-02 | geprüft | Protokolldokument, Client-Typ und Beispiele unter `testdata/protocol/` kennen die Ereignis … |
| AC-03 | geprüft | Der Benchmark nennt Bytes je Tick, der Ist-Wert steht in `docs/protocol.md` (B-140/AC-03). |
| AC-04 | geprüft | Das Budget KB/s je Client steht in `docs/protocol.md` und wird geprüft (B-140/AC-04, B-142 … |
| AC-05 | geprüft | Reports und Client-Log rotieren, `saves/` bleibt unberührt (B-142/AC-01, B-142/AC-02). |
| AC-06 | geprüft | Der Backup-Befehl auf ein zweites Gerät ist in der README beschrieben und lokal getestet ( … |
| AC-07 | – | Restore ist geschützt, das Spiel läuft ohne Geheimnis weiter, abgelehnte Zugriffe stehen i … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-140 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft | Feedback-Ereignisse laufen im Protokoll mit gemessener Bandbreite zum Client |
| B-142 | erledigt | AC-04 geprüft, AC-05 geprüft, AC-06 geprüft | Spielstände werden außerhalb des Pi gesichert, Berichte und Logs rotieren |
| B-143 | erledigt | AC-07 – | Restore-, Save- und Report-Endpunkte sind im Heimnetz abgesichert |
