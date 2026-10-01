# B-088 · Der Server zeigt Speicher, Geräte und Log und führt Diagnose-Aktionen aus

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** D1
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat („D1 freigegeben“), Revision 1

## Ausgangslage

`GET /api/status` (B-027, `engine/net/status.go`) liefert Räume, Tick-Dauer, Zähler und Abstürze; `?room=CODE` einen verdichteten Raumzustand. Es fehlen Speicherverbrauch, die Geräte eines Raums, ein Log und jede Aktion. Die Diagnose-TUI (B-002) soll nur mit dieser Schnittstelle sprechen und braucht sie deshalb.

## Ziel

Der Server liefert Speicher, Geräte und Log und führt zwei Aktionen aus, alle hinter dem Token aus B-027. Nutzen: Die TUI (B-002) und k3c-dev sehen und beheben Probleme im Betrieb, ohne am Server vorbei auf Dateien oder Prozess zuzugreifen.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz; die TUI (B-002) und k3c-dev (M6) sind die Clients; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- `GET /api/status` enthält `memory` (belegter Heap und vom System gehaltener Speicher in MB, Zahl der GC-Läufe).
- `GET /api/status?room=CODE` enthält `devices`: je Gerät eine kurze Kennung (nicht die volle Geräte-ID), verbunden ja/nein und die Slots.
- `GET /api/status/log?since=<Cursor>&limit=<n>`: Zeilen des JSON-Logs (B-066) ab dem Byte-Cursor, älteste zuerst, mit neuem Cursor; ohne Log-Datei eine klare Meldung.
- `POST /api/status/disconnect?room=CODE&device=KENNUNG`: trennt das Gerät; seine Monarchen werden `waiting` wie bei einem Verbindungsabbruch.
- `POST /api/status/save?room=CODE`: sichert den Spielstand des Raums sofort (mit Sicherung des alten Stands wie sonst).
- Alle diese Wege sind durch dasselbe Token geschützt wie `/api/status`.

## Nicht-Ziele

Neue Aktionen (Raum schließen, Server beenden); die TUI selbst (B-002); Änderungen am Protokoll v2 (`docs/protocol.md`) und am Spielverhalten.

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. Token wie B-027: ohne gesetztes Token 404, falsches 401, falsche Methode 405. Das Token und volle Geräte-IDs stehen nie in Antworten oder im Log. Aktionen schreiben in das Log (`ns` `diag`).

## Beispiele

- `POST /api/status/disconnect?room=FAMILIE&device=b1f4c2` → Gerät getrennt, Monarchen `waiting`, Antwort `{"ok":true}`.
- `GET /api/status/log?since=0&limit=50` → bis zu 50 Zeilen und ein Cursor für den nächsten Aufruf.

## Ausnahme- und Fehlerfälle

- Unbekannter Raum oder unbekanntes Gerät → 404 mit Meldung; mehrdeutige Kennung → 409.
- Log nicht eingeschaltet → 404 „Log aus“.
- Cursor hinter dem Dateiende (Datei kleiner geworden) → Anfang der Datei, Antwort mit `truncated: true`.

## Akzeptanzkriterien

- **AC-01** `/api/status` enthält `memory`, `?room=` die `devices` (Test).
- **AC-02** `/api/status/log` liefert Zeilen ab Cursor und den neuen Cursor; ohne Log-Datei „Log aus“ (Test).
- **AC-03** `disconnect` trennt das Gerät, `save` sichert den Spielstand (Tests, danach ist der Stand in `saves/` bzw. das Gerät `waiting`).
- **AC-04** Alle Wege verlangen das Token: ohne Token 404, falsches 401, falsche Methode 405 (Test).

## Offene Fragen

keine

## Notizen

Entstanden beim Planen der TUI (B-002), die ursprünglich nur den lesenden `GET /api/status` nutzen sollte. Entscheidung 🧑 2026-10-01: Server um geschützte Endpunkte erweitern.
