# Fahrplan

Alle Sprints mit Ordner und Status. Arbeitsweise: [`../arbeitsweise.md`](../arbeitsweise.md).
**Lesen:** `aktiv/` immer, `geplant/` beim Planen, `erledigt/` nur auf Nachfrage.
Jede Sprint-README ist eine Spec (SDD); aktiv wird ein Sprint erst mit `Spec: freigegeben` durch 🧑.

## Aktiv

| Sprint | Domäne | Thema | Am Ende sichtbar | Ordner |
|---|---|---|---|---|
| SP11 | SRV 🧑 | Raspberry Pi | 2er- und 3er-Spiel parallel auf dem Pi | `aktiv/SP11-raspberry-pi/` |

## Geplant (in dieser Reihenfolge)

Der Weg zur Go-Engine ([Entscheidung 001](../decisions/001-server-engine-go.md)). Nach SP08 spielt man wieder am TV,
dann über den Go-Server mit mehreren Räumen und gemischten Spielern.

| Sprint | Domäne | Thema | Am Ende sichtbar | Reife | Ordner |
|---|---|---|---|---|---|

**Einschiebbar** (unabhängig vom Engine-Fortschritt, jeweils zwischen zwei Sprints):

| Sprint | Domäne | Thema | Reife | Ordner |
|---|---|---|---|---|
| R3 | REG 🧑 | Regelwerk II – Monarch, Bürger, Klassen, Level, Skills (B-110, B-017) | bereit | `geplant/R3-monarch-buerger/` |
| X1 | PLAT 🧑 | Xbox-Machbarkeit | Entwurf | `geplant/X1-xbox/` |

Nach SP11: Regelwerk II (Monarch und Bürger: Skillung, Klassen, Level, B-110) → SIM Skills in Go → CLI Skills → Spieleabend → Grafik/Sound → …

## Erledigt

| Sprint | Thema | Ordner |
|---|---|---|
| ALT | Vorgeschichte vor der Sprint-Einteilung | `erledigt/ALT-vorgeschichte/` |
| SP00 | Arbeitsweise einführen | `erledigt/SP00-arbeitsweise/` |
| SP01 | Leitplanken + Go-Gerüst | `erledigt/SP01-leitplanken/` |
| L1 | Go-Verschachtelung als Tiefe prüfen (B-054) | `erledigt/L1-go-verschachtelung/` |
| L2 | Go-Tiefe wie TypeScript zählen (B-057) | `erledigt/L2-go-tiefe/` |
| L3 | Race-Detector für die nebenläufigen Go-Pakete (B-077) | `erledigt/L3-race/` |
| T1 | Testseite mit Szenarien und Mock-Spielern (B-081) | `erledigt/T1-testseite/` |
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
| SP08 | Browser als reiner Client (Protokoll v2, Lobby, lokale Spieler, Layout 1–4) | `erledigt/SP08-client/` |
| SP10 | Diagnose-TUI (Bubble Tea): `k3c-tui` zeigt Räume live, Aktionen, Log (B-002) | `erledigt/SP10-tui/` |
| D1 | Diagnose-Schnittstelle des Servers: JSON-Log, Speicher, Geräte, Log, Aktionen (B-066, B-088) | `erledigt/D1-diagnose-server/` |
| M6 | k3c-dev VI: MCP-Tools für Räume und Simulation (B-047) | `erledigt/M6-dev-raeume/` |
| SP09 | Aufräumen: TS-Sim und Node-Server gelöscht, Release `v0.2.0` | `erledigt/SP09-aufraeumen/` |
| G1 | Referenzseite für die gewählten Grafik-Packs, Auswahl auf den Referenzseiten (B-087) | `erledigt/G1-grafiken/` |
| I1 | Ein Weg für alle Befehle: `task`, `package.json` ohne Skripte, CI und k3c-dev rufen `task` (B-073, B-070, B-072, B-051) | `erledigt/I1-ein-weg/` |
| U1 | Radar-Leiste im HUD (B-090) | `erledigt/U1-radar/` |
| U2 | Level-Abfrage per HTTP: GET /api/level (B-091) | `erledigt/U2-level-abfrage/` |
| U3 | Level-Betrachter: leveltest.html, Einstieg von der Testseite (B-092) | `erledigt/U3-level-betrachter/` |
| U4 | Debug-Overlay: Verbindung, Takt und Entitäten per Ö oder Stick-Klick (B-093) | `erledigt/U4-debug-overlay/` |
| R1 | Regelwerk I: Wirtschaft, Stufen, Schwierigkeitsgrade, Spielstruktur Inseln/Stufen (B-004, B-005, B-021, B-025) | `erledigt/R1-regelwerk-1/` |
| R2 | Regelwerk I b: Materialien, Hub-Ausbau, Gebäude, Stufenbreite, Plantage und Adern (B-109, B-111) | `erledigt/R2-materialien-gebaeude/` |
