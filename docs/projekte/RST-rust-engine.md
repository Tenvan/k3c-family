# RST · Rust-Engine

- **Status:** ruht
- **Rang:** –
- **Ziel-Tickets:** –

## Ziel

Der Server läuft komplett in Rust (Entscheidung 006, Variante 2, **Etappe 2**: Sim, `room`, `net`, `store`, `watch` und Server-`cmd` in Rust, Brücke `engine/core` und Go-Server entfallen; Entscheidung 001 wird im Wortlaut angepasst). Ralf hat diese Stufe am 2026-11-07 ausdrücklich gewählt, obwohl RS1 zeigte: Sim ~2 % der Raum-Tick-Zeit, 97,7 % im JSON-Roundtrip `stateOf` (B-731). Treiber bleibt der Pi 3 (Tick p99 < 10 ms, B-042) und schnelleres Balancing. Spec: B-736 (Spec-Entwurf, Gates G1–G4); Vorarbeit und Alternative: B-726, B-731. Umfang 57–72 Sessions (Skizze RS2–RS12 in B-736).

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| RS1 | Messen, Go-Fixes, Rust-Spike für den Simulationskern | aktiv |
| RS2 | Etappe 2 vorbereiten: Workbench entkoppeln, Zustand ohne JSON-Roundtrip, Inventar | aktiv |

## Nicht-Ziele

Lasttest, TUI, Workbench, `assetkit` und Starter in Rust (006, Varianten 3–4). Client-Leistung gehört zu LST. Protokoll-, Regel- oder Balance-Änderungen im Port.

## Notizen

2026-11-07: angelegt; 006 von Ralf angenommen („erstmal“), Pi bleibt Zielplattform und ist der Hauptgrund. Timing-Messung auf Ralfs Wunsch in die Umsetzung verschoben (RS1.1/RS1.2). Workbench-Importe von `engine/sim` dürfen wegfallen (B-726), damit ist Variante 2 (Rust-Server) mögliche Etappe 2. Generatoren entfallen mit PAP und zählen nicht.

2026-11-07 (RS1 gemerged, PR #510): RS1-Empfehlung lautete „kein Vollumbau, zuerst B-731“. Ralf hat trotzdem Etappe 2 gewählt; Planung als B-736 und Sprint RS2 (Entwurf, nicht freigegeben). B-731 bleibt Vorarbeit (Referenzweg, Gate G1: erfüllt der Go-Server nach B-731 das Pi-Ziel mit Reserve, entscheidet Ralf, ob es weitergeht). Folgesprints RS3–RS12 sind nur in B-736 skizziert und werden einzeln bereit gemacht.

2026-11-11: Projekt ruht. Entscheidung 008: natives Spiel in Go + Ebitengine, Rust entfällt komplett; 006 und 007 sind überholt. Aufräumen und Schließen: B-779 (Projekt NAT).
