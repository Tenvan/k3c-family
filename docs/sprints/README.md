# Fahrplan

Alle Sprints mit Ordner und Status. Arbeitsweise: [`../arbeitsweise.md`](../arbeitsweise.md).
**Lesen:** `aktiv/` immer, `geplant/` beim Planen, `erledigt/` nur auf Nachfrage.
Jede Sprint-README ist eine Spec (SDD); aktiv wird ein Sprint erst mit `Spec: freigegeben` durch 🧑.

## Aktiv

Kein aktiver Sprint. Nächster Schritt: SP02 mit 🧑 bereit machen (`Reife: Entwurf`).

## Geplant (in dieser Reihenfolge)

Der Weg zur Go-Engine ([Entscheidung 001](../decisions/001-server-engine-go.md)). Nach SP08 spielt man wieder am TV,
dann über den Go-Server mit mehreren Räumen und gemischten Spielern.

| Sprint | Domäne | Thema | Am Ende sichtbar | Reife | Ordner |
|---|---|---|---|---|---|
| SP02 | SRV 🧑 | Protokoll v2 & Raummodell (Entwurf) | `docs/protocol.md`, Entscheidung 002 | Entwurf | `geplant/SP02-protokoll/` |
| SP03 | SRV | Go-Server Basis (ersetzt `server/*.mjs`) | EXE und Docker-Image liefern das Spiel aus | Entwurf | `geplant/SP03-go-server/` |
| SP04 | SIM | Golden-Tests, RNG, Level-Generator in Go | gleiche Level in TS und Go | Entwurf | `geplant/SP04-golden-level/` |
| SP05 | SIM | Port I: Welt, Zyklus, Wirtschaft | Golden-Läufe ohne Gegner grün | Entwurf | `geplant/SP05-port-welt/` |
| SP06 | SIM | Port II: Einheiten, Gegner, Wellen, Reisen, Kampagne | alle Golden-Läufe grün | Entwurf | `geplant/SP06-port-einheiten/` |
| SP07 | SRV | Räume & WebSocket in Go, MCP-Tools für Räume und Simulation | 3 Räume parallel im Test | Entwurf | `geplant/SP07-raeume/` |
| SP08 | CLI | Browser als reiner Client | Xbox (2 Controller) + Handy im selben Raum | Entwurf | `geplant/SP08-client/` |
| SP09 | INF | Aufräumen: TS-Sim und Node-Server löschen | Release `v0.2.0` | Entwurf | `geplant/SP09-aufraeumen/` |
| SP10 | SRV | Diagnose-TUI (Bubble Tea) | `k3c-tui` zeigt Räume live | Entwurf | `geplant/SP10-tui/` |
| SP11 | SRV 🧑 | Raspberry Pi | 2er- und 3er-Spiel parallel auf dem Pi | Entwurf | `geplant/SP11-raspberry-pi/` |

**Einschiebbar** (unabhängig vom Engine-Fortschritt, jeweils zwischen zwei Sprints):

| Sprint | Domäne | Thema | Reife | Ordner |
|---|---|---|---|---|
| R1 | REG 🧑 | Regelwerk I – Fundament | Entwurf | `geplant/R1-regelwerk-1/` |
| X1 | PLAT 🧑 | Xbox-Machbarkeit | Entwurf | `geplant/X1-xbox/` |
| M1 | SRV | Entwickler-MCP-Server `k3c-dev` (frühestens nach SP01, Muster ErpApi) | Entwurf | `geplant/M1-dev-mcp/` |

Nach SP11: Regelwerk II (Skills) → SIM Skills in Go → CLI Skills → Spieleabend → Grafik/Sound → …

## Erledigt

| Sprint | Thema | Ordner |
|---|---|---|
| ALT | Vorgeschichte vor der Sprint-Einteilung | `erledigt/ALT-vorgeschichte/` |
| SP00 | Arbeitsweise einführen | `erledigt/SP00-arbeitsweise/` |
| SP01 | Leitplanken + Go-Gerüst | `erledigt/SP01-leitplanken/` |
| L1 | Go-Verschachtelung als Tiefe prüfen (B-054) | `erledigt/L1-go-verschachtelung/` |
| L2 | Go-Tiefe wie TypeScript zählen (B-057) | `erledigt/L2-go-tiefe/` |
