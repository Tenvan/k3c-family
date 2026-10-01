# Fahrplan

Alle Sprints mit Ordner und Status. Arbeitsweise: [`../arbeitsweise.md`](../arbeitsweise.md).
**Lesen:** `aktiv/` immer, `geplant/` beim Planen, `erledigt/` nur auf Nachfrage.
Jede Sprint-README ist eine Spec (SDD); aktiv wird ein Sprint erst mit `Spec: freigegeben` durch 🧑.

## Aktiv

| Sprint | Domäne | Thema | Am Ende sichtbar | Ordner |
|---|---|---|---|---|
| L3 | INF | Race-Detector für die nebenläufigen Go-Pakete (B-077, eingeschoben) | CI-Schritt `go test -race ./engine/...` | `aktiv/L3-race/` |

Danach: SP08 bereit machen (Client auf Protokoll v2); M6 (MCP-Tools) ist einschiebbar.

## Geplant (in dieser Reihenfolge)

Der Weg zur Go-Engine ([Entscheidung 001](../decisions/001-server-engine-go.md)). Nach SP08 spielt man wieder am TV,
dann über den Go-Server mit mehreren Räumen und gemischten Spielern.

| Sprint | Domäne | Thema | Am Ende sichtbar | Reife | Ordner |
|---|---|---|---|---|---|
| SP08 | CLI | Browser als reiner Client | Xbox (2 Controller) + Handy im selben Raum | Entwurf | `geplant/SP08-client/` |
| SP09 | INF | Aufräumen: TS-Sim und Node-Server löschen | Release `v0.2.0` | Entwurf | `geplant/SP09-aufraeumen/` |
| SP10 | SRV | Diagnose-TUI (Bubble Tea) | `k3c-tui` zeigt Räume live | Entwurf | `geplant/SP10-tui/` |
| SP11 | SRV 🧑 | Raspberry Pi | 2er- und 3er-Spiel parallel auf dem Pi | Entwurf | `geplant/SP11-raspberry-pi/` |

**Einschiebbar** (unabhängig vom Engine-Fortschritt, jeweils zwischen zwei Sprints):

| Sprint | Domäne | Thema | Reife | Ordner |
|---|---|---|---|---|
| R1 | REG 🧑 | Regelwerk I – Fundament | Entwurf | `geplant/R1-regelwerk-1/` |
| X1 | PLAT 🧑 | Xbox-Machbarkeit | Entwurf | `geplant/X1-xbox/` |
| M6 | SRV | k3c-dev VI: MCP-Tools für Räume und Simulation (B-047, nach SP07) | Entwurf | `geplant/M6-dev-raeume/` |

Nach SP11: Regelwerk II (Skills) → SIM Skills in Go → CLI Skills → Spieleabend → Grafik/Sound → …

## Erledigt

| Sprint | Thema | Ordner |
|---|---|---|
| ALT | Vorgeschichte vor der Sprint-Einteilung | `erledigt/ALT-vorgeschichte/` |
| SP00 | Arbeitsweise einführen | `erledigt/SP00-arbeitsweise/` |
| SP01 | Leitplanken + Go-Gerüst | `erledigt/SP01-leitplanken/` |
| L1 | Go-Verschachtelung als Tiefe prüfen (B-054) | `erledigt/L1-go-verschachtelung/` |
| L2 | Go-Tiefe wie TypeScript zählen (B-057) | `erledigt/L2-go-tiefe/` |
| SP02 | Protokoll v2 & Raummodell, Entscheidung 002 | `erledigt/SP02-protokoll/` |
| M1 | k3c-dev I: MCP-Kern über HTTP | `erledigt/M1-dev-mcp/` |
| M2 | k3c-dev II: Nutzungsstatistik, Berichte, Spielstände | `erledigt/M2-dev-statistik/` |
| M3 | k3c-dev III: Dienste führen, `svc_*`-Tools | `erledigt/M3-dev-dienste/` |
| M4 | k3c-dev IV: Oberfläche (Wails) mit Dienste- und Logs-Seite | `erledigt/M4-dev-oberflaeche/` |
| SP03 | Go-Server Basis: Auslieferung, Spielstände mit Sicherungen, Status, Docker | `erledigt/SP03-go-server/` |
| M5 | k3c-dev V: MCP-Seite mit Monitoren und Statistik | `erledigt/M5-dev-mcp-seite/` |
| SP04 | Golden-Tests, RNG und Level-Generator in Go | `erledigt/SP04-golden-level/` |
| SP05 | Port I: Welt, Zyklus, Truppen, Wirtschaft in Go | `erledigt/SP05-port-welt/` |
| SP06 | Port II: Gegner, Wellen, Reisen, Kampagne, Spielstand in Go | `erledigt/SP06-port-einheiten/` |
| SP07 | Räume & WebSocket (Protokoll v2) in Go | `erledigt/SP07-raeume/` |
