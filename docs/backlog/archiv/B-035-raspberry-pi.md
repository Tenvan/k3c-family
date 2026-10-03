# B-035 · Server läuft auf dem Raspberry Pi im Docker

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** SP11
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Server läuft nur auf dem Windows-PC, der dafür immer an sein muss.

## Ziel

Server läuft auf dem Raspberry Pi im Docker. Nutzen: Der Windows-PC soll nicht immer laufen müssen.

## Beteiligte und Zielgruppen

🧑 richtet den Pi ein und betreibt ihn; Spieler verbinden sich.

## Anforderungen

- Betrieb auf einem Raspberry Pi (arm64) im Docker.
- Autostart nach einem Neustart.
- Update per `docker compose pull`.

## Nicht-Ziele

Server-Suche (B-040).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`.

## Beispiele

Pi wird neu gestartet → nach dem Hochfahren ist das Spiel im Heimnetz erreichbar.

## Ausnahme- und Fehlerfälle

Update-Image startet nicht → das vorige Image bleibt nutzbar.

## Akzeptanzkriterien

- **AC-01** Nach einem Neustart des Pi ist der Server erreichbar.
- **AC-02** Ein Update per `docker compose pull` bringt die neue Version.

## Offene Fragen

Pi-Modell und Leistungsziel (B-042).

## Notizen

Pi-Modell und Leistungsziel: B-042.
