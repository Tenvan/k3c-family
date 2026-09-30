# B-002 · Diagnose-TUI zeigt den laufenden Server

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SP10
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Im Betrieb (PC oder Pi im Docker) sieht man den Zustand des Servers nur in Logs. `/api/status` mit Token entsteht in SP03.2, Räume in SP07.

## Ziel

Diagnose-TUI zeigt den laufenden Server. Nutzen: Im Betrieb (PC oder Pi im Docker) Probleme sehen und beheben, ohne Logs zu durchsuchen.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Spieler verbinden sich mit Xbox und Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Eigenes Programm `cmd/k3c-tui` (Bubble Tea), das nur mit `/api/status` spricht, lokal oder über das Netz.
- Zeigt Räume, Geräte, Tick-Dauer, Speicher, letzte Fehler und Spielstände live.
- Aktionen: Raum ansehen, Gerät trennen, Spielstand sichern, Log folgen.

## Nicht-Ziele

Kein Zugriff am Server vorbei (Dateien, Prozess); kein Desktop-Fenster (B-041).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. Zugriff nur mit Token (B-027); die TUI nutzt nur `/api/status` (Entscheidung 001).

## Beispiele

Server mit 2 Räumen läuft im Docker → `docker exec -it k3c k3c-tui` zeigt beide Räume mit Geräten und Tick-Dauer.

## Ausnahme- und Fehlerfälle

Server nicht erreichbar oder Token falsch → klare Meldung und erneuter Versuch, kein Absturz.

## Akzeptanzkriterien

- **AC-01** `k3c-tui` zeigt gegen einen lokal laufenden Server die Räume live.
- **AC-02** Im Docker-Container zeigt `docker exec -it k3c k3c-tui` dasselbe.
- **AC-03** Die Aktionen Raum ansehen, Gerät trennen, Spielstand sichern und Log folgen wirken (Test oder Beobachtung).

## Offene Fragen

keine

## Notizen

Voraussetzung: `/api/status` mit Token (SP03.2), Räume ab SP07.
