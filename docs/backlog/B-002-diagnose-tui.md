# B-002 · Diagnose-TUI zeigt den laufenden Server

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SP10
- **Erstellt:** 2026-09-29

## Beschreibung

Eigenes Programm `cmd/k3c-tui` (Bubble Tea), das nur mit `/api/status` spricht: Räume, Geräte, Tick-Dauer, Speicher, letzte Fehler, Spielstände. Aktionen: Raum ansehen, Gerät trennen, Spielstand sichern, Log folgen.

## Warum

Im Betrieb (PC oder Pi im Docker) Probleme sehen und beheben, ohne Logs zu durchsuchen.

## Akzeptanz

`k3c-tui` zeigt gegen einen laufenden Server (lokal und im Docker per `docker exec -it k3c k3c-tui`) die Räume live; Aktionen wirken.

## Notizen

Voraussetzung: `/api/status` mit Token (SP03.2), Räume ab SP07.
