# B-002 · Diagnose-TUI zeigt den laufenden Server

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** SP10
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-10-01 🧑 Chat („SP10 freigegeben“), Revision 3

## Ausgangslage

Im Betrieb (PC oder Pi im Docker) sieht man den Zustand des Servers nur in Logs. `/api/status` mit Token entsteht in SP03.2, Räume in SP07.

## Ziel

Diagnose-TUI zeigt den laufenden Server. Nutzen: Im Betrieb (PC oder Pi im Docker) Probleme sehen und beheben, ohne Logs zu durchsuchen.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Spieler verbinden sich mit Xbox und Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Eigenes Programm `cmd/k3c-tui` (Bubble Tea), das nur mit der Diagnose-Schnittstelle des Servers (`/api/status…`, B-027, B-088) spricht, lokal oder über das Netz.
- Zeigt Räume, Geräte, Tick-Dauer, Speicher, letzte Fehler und Spielstände live.
- Aktionen: Raum ansehen, Gerät trennen, Spielstand sichern, Log folgen.
- `k3c-tui -once` druckt den Zustand einmal als Text (für Skripte, CI und `docker exec` ohne Terminal); das Docker-Image enthält `k3c-tui`.

## Nicht-Ziele

Kein Zugriff am Server vorbei (Dateien, Prozess); kein Desktop-Fenster (B-041).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. Zugriff nur mit Token (B-027); die TUI nutzt nur `/api/status…` (Entscheidung 001). Neue Abhängigkeit Bubble Tea (`charmbracelet/bubbletea`, `lipgloss`) ist mit der Freigabe dieser Spec zugestimmt; das Dockerfile muss dafür `go.sum` kopieren und `k3c-tui` ins Image legen.

## Beispiele

Server mit 2 Räumen läuft im Docker → `docker exec -it k3c k3c-tui` zeigt beide Räume mit Geräten und Tick-Dauer.

## Ausnahme- und Fehlerfälle

Server nicht erreichbar oder Token falsch → klare Meldung und erneuter Versuch, kein Absturz.

## Akzeptanzkriterien

- **AC-01** `k3c-tui` zeigt gegen einen lokal laufenden Server die Räume live.
- **AC-02** Im Docker-Container zeigt `docker exec -it k3c k3c-tui` dasselbe; `docker exec k3c k3c-tui -once` druckt es und die CI führt das aus.
- **AC-03** Die Aktionen Raum ansehen, Gerät trennen, Spielstand sichern und Log folgen wirken (Test oder Beobachtung).

## Offene Fragen

keine

## Notizen

Voraussetzung: `/api/status` mit Token (SP03.2), Räume ab SP07, Speicher, Geräte, Log und Aktionen aus Sprint D1 (B-088). Entscheidung 🧑 2026-10-01: Server erweitern statt TUI nur lesend; Bubble Tea.
