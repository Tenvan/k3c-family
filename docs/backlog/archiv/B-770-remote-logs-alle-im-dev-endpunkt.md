# B-770 · Alle Logs eines Servers sind über den Dev-Endpunkt abrufbar

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** verworfen
- **Sprint:** LG1
- **Projekt:** LST
- **Erstellt:** 2026-11-10
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (Nutzer: „LST ganz abarbeiten“, 2026-11-10)

## Ausgangslage

`GET /api/dev/log` (B-769) liest nur `<LogDir>/k3c-server.jsonl`. Die Browser-Meldungen der Clients stehen in `k3c-client.jsonl` und sind darüber nicht erreichbar. Auf dem Pi (`pi-gaming:8080`, v0.23.2) antwortet der Endpunkt am 2026-11-10 mit `{"error":"Log aus"}`, weil kein Log-Ordner gesetzt ist (`K3C_LOG_DIR` leer, kein `logs/` im Arbeitsverzeichnis). Berichte (`/api/dev/reports`) und Status gehen. Logs vom Pi kamen bisher nur über SSH.

## Ziel

Der Entwickler ruft **alle** Logs eines laufenden Servers (Pi oder ein anderer) über Dev-Endpunkte ab, ohne SSH, ohne Token, nur lesend.

## Beteiligte und Zielgruppen

Entwickler (🧑) und Claude lesen die Logs des Pi aus der Ferne; Betreiber des Pi setzt den Log-Ordner.

## Anforderungen

- Der Endpunkt liefert jede Log-Datei des Servers, nicht nur `k3c-server.jsonl`: mindestens `k3c-server.jsonl` und `k3c-client.jsonl`, dazu rotierte Dateien, falls es sie gibt. Eine Auswahl über feste Namen (`file=server|client|…`), nie über einen Pfad aus der URL.
- Es gibt eine Liste der abrufbaren Logs (Name, Größe, Änderungszeit), z. B. `GET /api/dev/logs`.
- Filter und Fenster wie bisher: `pattern`, `limit`, `mib`; dazu Cursor (`since`) wie bei `/api/status/log`, damit man nur Neues holt.
- Der Server läuft auf dem Pi mit gesetztem Log-Ordner (Standard so, dass „Log aus“ nicht mehr vorkommt) und meldet im Status (`/api/dev/status`), ob und wo er loggt.
- Das Log-Verhalten bleibt Dev-Mode-gebunden (`K3C_DEV`, sonst 403), nur GET.
- `docs/protocol.md` (Dev-Lese-Endpunkte) und README werden nachgezogen.

## Nicht-Ziele

Kein Schreiben, keine Löschfunktion, kein beliebiger Dateizugriff, kein Streaming (SSE), keine Rotation neu erfinden (B-142 ist erledigt).

## Regeln und Einschränkungen

Nur Dateien aus `LogDir` mit festen Namen; Pfade und `..` werden abgelehnt. Antwortgröße begrenzt (`mib`, `limit`). Der Server importiert nichts aus `tools/`. Emoji-Präfix für neue Log-Zeilen laut CLAUDE.md.

## Beispiele

- `GET /api/dev/logs` → Liste `k3c-server.jsonl 12 MiB`, `k3c-client.jsonl 3 MiB`.
- `GET /api/dev/log?file=client&pattern=WebGL&limit=50` → die letzten 50 Client-Zeilen mit „WebGL“.
- `GET /api/dev/log?file=server&since=<Cursor>` → nur neue Zeilen und neuer Cursor.

## Ausnahme- und Fehlerfälle

- Kein Dev-Mode → 403; andere Methode → 405.
- Unbekannter Name oder Pfad im Parameter → 400.
- Datei fehlt oder Log-Ordner leer → 404 mit Grund, der den fehlenden Ordner nennt (`K3C_LOG_DIR`).
- Cursor hinter dem Dateiende (Rotation) → von vorn, `truncated` gemeldet.

## Akzeptanzkriterien

- **AC-01** Test: `GET /api/dev/logs` listet alle vorhandenen Log-Dateien; `/api/dev/log?file=` liest Server- und Client-Log, filtert, nutzt den Cursor und lehnt Pfade ab (`engine/net/dev_read_test.go`).
- **AC-02** Test: Ohne Dev-Mode 403, falsche Methode 405, ohne Log-Ordner 404 mit Hinweis auf `K3C_LOG_DIR`.
- **AC-03** Test: `/api/dev/status` nennt Log-Ordner und Dateien.
- **AC-04** Abnahme (🧑): Auf dem Pi ist der Log-Ordner gesetzt, `pi-gaming:8080/api/dev/log?file=client` liefert Zeilen eines Xbox-Laufs.

## Offene Fragen

- Wie wird der Log-Ordner auf dem Pi gesetzt (Dienst-Datei, Container)? Default: `K3C_LOG_DIR` im Start des Pi, Betrieb liegt bei 🧑.
- Sind noch weitere Logs gemeint (z. B. Supervisor-Konsole, Berichte)? Default: nur die Dateien in `LogDir`; Berichte laufen schon über `/api/dev/reports`.

## Notizen

Baut auf B-769 auf. Verbraucher ist B-771 (MCP-Log-Tools). Ursache „Log aus“ am 2026-11-10 über `engine/net/dev_read.go:67` bestätigt.

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009; Projekt LST gesammelt verworfen). Umsetzung aus LG1.1–LG1.5 liegt mit #541 auf `develop`, die Abnahme LG1.6 entfällt.
