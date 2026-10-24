# B-614 · Workbench und Studio starten als Server, gearbeitet wird im Browser

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** WZ31
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-10-24

## Ausgangslage

k3c-workbench und k3c-studio sind Wails-Fenster mit eigenem Go-Backend. Je App gibt es `build` (EXE), `run`, `dev` (Supervisor `engine/devrun` plus Vite mit Hot Reload unter 5191/5192) und `window` (`wails dev`). Ohne Vite und ohne Fenster gibt es keinen Start: Der Workbench-Modus `-headless` liefert nur die API `/api/dev/` (`tools/k3c-workbench/headless.go`, `uiapi.go`), keine Seiten. Das Studio liefert im `-headless` die gebaute Oberfläche `tools/k3c-studio/frontend/dist` schon unter `/studio/` aus (`tools/k3c-studio/server.go`), es fehlt nur ein Task, der baut und startet.

## Ziel

Ralf startet je App nur den Server und arbeitet im Browser („Web-Client first“). Kein Fenster, kein Vite-Prozess, eine feste Adresse zum Lesezeichen. Die EXE-Builds bleiben unverändert, bis sich der Weg bewährt hat.

## Beteiligte und Zielgruppen

🧑 Ralf als Entwickler und Nutzer der Werkzeuge; Claude-Sessions, die über den MCP-Server der Workbench arbeiten.

## Anforderungen

- Task `k3c-workbench:web`: baut die Oberfläche (nur bei geänderten Quellen), baut die EXE `bin/k3c-workbench-web` per `go build` (fester Pfad, Konsole) und startet sie `-headless`; das Backend liefert dabei die eingebettete Oberfläche unter `/` aus. MCP auf 5180 (wie die EXE), Oberfläche auf `http://127.0.0.1:8181/`.
- Task `k3c-studio:web`: baut die Oberfläche (nur bei geänderten Quellen), baut `bin/k3c-studio-web` per `go build` und startet sie `-headless`; Oberfläche auf `http://127.0.0.1:8182/studio/`.
- Beide Tasks geben die Adresse aus und öffnen sie im Standard-Browser (Schalter `-open`); Ports über Task-Variablen änderbar.
- Nur Loopback, die bestehenden Schutzregeln (`uiGuard`, `hostGuard`) gelten auch für die Seiten.
- `task --list` zeigt beide Tasks mit `desc`; `CLAUDE.md` › Befehle nennt sie mit Ports.

## Nicht-Ziele

- EXE-Builds (`k3c-workbench:build`, `k3c-studio:build`, `buildall`) und `window` bleiben unverändert.
- Kein Hot Reload im `web` (dafür bleibt `dev`), kein Neustart bei Go-Änderungen.
- Kein Zugriff aus dem LAN.
- Starter (`k3c-starter`) und die Dienste-Liste der Workbench starten die Web-Server nicht (später eigenes Ticket, falls gewünscht).

## Regeln und Einschränkungen

Domäne DEV. Befehle nur über `task`, neue Tasks im Taskfile ihres Projekts. Feste EXE-Pfade unter `bin/` statt `go run` (keine Firewall-Abfrage), Proben nur auf Loopback. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Log-Meldungen mit Emoji.

## Beispiele

- Ralf ruft `task k3c-workbench:web` → Browser öffnet `http://127.0.0.1:8181/`, die Workbench zeigt Dienste, Tasks, Planung wie im Fenster; Claude erreicht den MCP-Server auf 5180.
- Ralf ruft `task k3c-studio:web` → Browser öffnet `http://127.0.0.1:8182/studio/`, Grafik- und Sound-Seite laden mit echten Daten.
- Zweiter Aufruf ohne geänderte Oberfläche → kein Frontend-Neubau.

## Ausnahme- und Fehlerfälle

- Workbench-EXE läuft schon (MCP 5180 belegt) → der Web-Start bricht mit Hinweis auf den belegten Port ab; die laufende Instanz bleibt unberührt.
- Port der Oberfläche belegt → Abbruch mit Meldung, welche Variable den Port ändert.
- Anfrage mit fremdem Host oder von außerhalb des Rechners → 403 wie bisher.

## Akzeptanzkriterien

- **AC-01** `task k3c-workbench:web` startet das Backend ohne Fenster; `GET /` auf dem Oberflächen-Port liefert die gebaute Oberfläche, die Seite lädt im Browser-Pane ohne Konsolenfehler und zeigt echte Daten (Go-Test für die Auslieferung).
- **AC-02** `task k3c-studio:web` startet das Studio ohne Fenster; `/studio/` lädt im Browser-Pane mit echten Daten.
- **AC-03** Beide Tasks bauen über feste EXE-Pfade unter `bin/` (`go build`), nicht `go run`, und bauen die Oberfläche nur bei geänderten Quellen.
- **AC-04** `task --list` zeigt `k3c-workbench:web` und `k3c-studio:web` mit Beschreibung; `CLAUDE.md` › Befehle nennt beide mit Ports.
- **AC-05** `build`, `window`, `dev` und `buildall` sind unverändert.

## Offene Fragen

keine (Ports 8181/8182 und MCP 5180 als Default gesetzt; änderbar über Task-Variablen).

## Notizen

Auftrag aus dem Chat mit Ralf am 2026-10-24 (Session „Web-Client first“).
