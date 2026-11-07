# RST · Rust-Engine

- **Status:** aktiv
- **Rang:** 10
- **Ziel-Tickets:** –

## Ziel

Der Simulationskern läuft in Rust, der Server bleibt Go (Entscheidung 006, Variante 1). Treiber: Pi 3 hält Tick p99 < 10 ms bei wachsenden Welten (B-042), Balancing und `sim_test` laufen messbar schneller. Reihenfolge: messen, Go-Fixes, Spike mit Abbruchkriterium, dann volle Umstellung. Spec: B-725.

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| RS1 | Messen, Go-Fixes, Rust-Spike für den Simulationskern | geplant |

## Nicht-Ziele

Server, Lasttest, TUI und Workbench in Rust (006, Varianten 2–4), bis der Spike entschieden ist. Client-Leistung gehört zu LST.

## Notizen

2026-11-07: angelegt; 006 von Ralf angenommen („erstmal“), Pi bleibt Zielplattform und ist der Hauptgrund. Timing-Messung auf Ralfs Wunsch in die Umsetzung verschoben (RS1.1/RS1.2). Workbench-Importe von `engine/sim` dürfen wegfallen (B-726), damit ist Variante 2 (Rust-Server) mögliche Etappe 2. Generatoren entfallen mit PAP und zählen nicht.
