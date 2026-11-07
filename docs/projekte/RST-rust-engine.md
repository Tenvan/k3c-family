# RST · Rust-Engine

- **Status:** ruht
- **Rang:** –
- **Ziel-Tickets:** –

## Ziel

Simulation, Balancing und Raum-Takt werden messbar schneller und stabiler. Erst Go optimieren und messen, dann ein Rust-Spike für den Simulationskern; Rust nur bei ≥ 2× Durchsatz gegen optimiertes Go. Analyse: `docs/decisions/006-rust-engine.md`, Spec: B-725.

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|

## Nicht-Ziele

Server, Lasttest, TUI und Workbench in Rust (006, Varianten 2–4), bis der Spike entschieden ist. Client-Leistung gehört zu LST.

## Notizen

2026-11-07: angelegt. Timing-Messung auf Ralfs Wunsch in die Umsetzung verschoben (Session RST-M1 in 006). Status ruht bis zur Freigabe von 006; erst dann Sprints anlegen.
