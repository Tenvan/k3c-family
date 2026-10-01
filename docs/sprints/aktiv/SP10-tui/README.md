# SP10 · SRV · Diagnose-TUI

- **Status:** aktiv
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-002
- **Start-Commit:** 5c2559a
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-10-01 🧑 Chat („SP10 freigegeben“), Revision 3

## Ausgangslage

Sprint D1 hat die Diagnose-Schnittstelle des Servers gebaut: `GET /api/status` mit Räumen, Tick-Dauer und `memory`, `?room=CODE` mit `devices`,
`GET /api/status/log` und `POST /api/status/disconnect|save`, alles mit `K3C_STATUS_TOKEN`. Eine Oberfläche dafür gibt es nicht; im Betrieb
(PC, Docker, später Pi) sieht man den Server nur über Logs. Das Image (`Dockerfile`) enthält nur `k3c-server` und kopiert nur `go.mod`, nicht `go.sum`.

## Ziel

`cmd/k3c-tui` (Bubble Tea) zeigt den laufenden Server und führt die Diagnose-Aktionen aus. Am Ende sichtbar: Räume, Geräte, Tick-Dauer und Log live,
auch im Docker-Container (`docker exec -it k3c k3c-tui`).

## Beteiligte und Zielgruppen

🧑 als Betreiber am PC und später am Pi.

## Anforderungen

B-002 › Anforderungen. Sprint-eigen:

- Adresse und Token wie bei k3c-dev (M6): Token `K3C_STATUS_TOKEN`, Adresse `K3C_SERVER_URL`, sonst `http://127.0.0.1:<K3C_HTTP_PORT oder 8080>`. Im Container sind die Variablen die des Servers.
- `k3c-tui -once` druckt den Zustand einmal als Text und endet (Exit 0 bei Erfolg, 1 mit Meldung bei Fehler): für Skripte, CI und `docker exec` ohne Terminal.
- Neue Abhängigkeiten (Zustimmung mit der Freigabe, B-002): Bubble Tea v2 (`charm.land/bubbletea/v2`) und Lip Gloss v2 (`charm.land/lipgloss/v2`), genaue Versionen nach `go get` in `go.mod`/`go.sum`.
- Das Image enthält `k3c-tui` unter `/usr/local/bin/k3c-tui`; das Dockerfile kopiert `go.sum`.

## Nicht-Ziele

Wails-Starter (B-041); Eingriffe am Server vorbei (Dateien, Prozess); weitere Aktionen als Gerät trennen und Spielstand sichern; Änderungen am Server (D1 ist erledigt).

## Regeln und Einschränkungen

- Nur die Diagnose-Endpunkte `/api/status…` mit Token (B-027, B-088); die TUI importiert nichts aus `engine/` außer dem, was für JSON-Typen nötig wäre (keine: die Typen stehen lokal).
- Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md` (Datei ≤ 400 Zeilen, Funktion ≤ 60); `cmd/k3c-tui` importiert nichts aus `tools/`.
- Das Token steht nie in Anzeige, Fehlermeldungen oder Log. Der Docker-Smoke-Schritt in `.github/workflows/ci.yml` (`k3c-tui -once` gegen das gestartete Image) gehört zum Docker-Betrieb (SRV) und ist in dieser Spec erlaubt.
- Bedienung ohne Maus; Tasten: Pfeile/`j` `k` wählen, Enter öffnet, `d` trennt (mit Bestätigung `y`), `s` sichert, `l` Log, `q` oder Esc zurück bzw. beenden. Wie bei jeder Seite keine Taste B-Bezüge (hier nicht relevant, keine Gamepad-Eingabe).

## Beispiele

Server im Docker mit 2 Räumen → `docker exec -it k3c k3c-tui` zeigt beide live (Aktualisierung jede Sekunde); `docker exec k3c k3c-tui -once` druckt die Tabelle und endet.

## Ausnahme- und Fehlerfälle

- Server weg → Meldung „Server nicht erreichbar unter <Adresse>“ und alle 2 Sekunden ein neuer Versuch, kein Absturz.
- Token falsch oder fehlt → „401, Token prüfen (K3C_STATUS_TOKEN)“; Diagnose am Server aus (404) → entsprechende Meldung; jeweils mit Wiederholung.
- Aktion scheitert (unbekanntes Gerät, Raum geschlossen) → Meldung in der Statuszeile, die Anzeige läuft weiter.
- Terminal sehr klein → Anzeige kürzt Spalten, kein Absturz.

## Akzeptanzkriterien

- **AC-01** `k3c-tui` zeigt die Räume live und `-once` druckt sie; das Image enthält `k3c-tui` und die CI prüft es (B-002/AC-01, B-002/AC-02).
- **AC-02** Die Aktionen Raum ansehen, Gerät trennen, Spielstand sichern und Log folgen wirken gegen einen Test-Server (B-002/AC-03).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP10.1 | `SP10.1-anzeige.md` | Umsetzung | autonom | fertig |
| SP10.2 | `SP10.2-aktionen-log.md` | Umsetzung | autonom | fertig |
| SP10.3 | `SP10.3-review.md` | Review | autonom | offen |

## Abnahme

–
