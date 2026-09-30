# B-035 · Server läuft auf dem Raspberry Pi im Docker

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** SP11
- **Erstellt:** 2026-09-30

## Beschreibung

Betrieb auf einem Raspberry Pi (arm64) im Docker, mit Autostart und Updates.

## Warum

Der Windows-PC soll nicht immer laufen müssen.

## Akzeptanz

Nach einem Neustart des Pi ist der Server erreichbar; Update per `docker compose pull`.

## Notizen

Pi-Modell und Leistungsziel: B-042.
