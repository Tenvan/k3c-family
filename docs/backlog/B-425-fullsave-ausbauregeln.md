# B-425 · Der Voll-Ausbau-Stand hält die Ausbauregeln des Spiels ein

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SV2
- **Projekt:** WRT
- **Erstellt:** 2026-10-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`fullIsland` (`engine/room/fullsave.go`) setzt Plätze selbst auf `built` und Stufen reihum 1–5, ohne die Regeln der Simulation: Mauern und Türme der Stufe n stehen auf Inselstufen mit Hub < n (`nextLevel`, `engine/sim/hub_level.go:60`), Heiler (Hub 3) und Rüstkammer (Hub 4) stehen auch bei niedrigerem Hub, Mauerlinien 2–5 bei Hub 1 und Tore, obwohl `gateHubLevel` 2 ist. `sim.DevBuildAll` / `devBuildSite` (`engine/sim/dev.go:126`) setzt diese Regeln schon um und wurde nicht genutzt. Gefunden im unabhängigen Review von PR #293.

## Ziel

Der Voll-Ausbau-Stand ist ein Zustand, den das Spiel auch regulär erreichen kann. Messläufe auf ihm sind aussagekräftig.

## Beteiligte und Zielgruppen

🧑 bei der Abnahme von B-315; Agenten bei Messläufen.

## Anforderungen

- Erzeugung über `sim.DevBuildAll` (oder dieselben Regeln), nicht über eigene Logik im Raum.
- Weiter gilt B-315: jede Ausbaustufe (Hub, Mauer, Turm) mindestens einmal sichtbar, alle Inselstufen, deterministisch.

## Nicht-Ziele

Neue Regeln für Stufen; Darstellung im Renderer (B-208).

## Regeln und Einschränkungen

SRV nutzt das exportierte `sim.DevBuildAll`. Muss `engine/sim/` dafür geändert werden, wird es eine SIM-Session.

## Beispiele

Inselstufe mit Hub 2 → Mauern und Türme höchstens Stufe 2, kein Heiler, keine Rüstkammer.

## Ausnahme- und Fehlerfälle

Lässt sich „jede Ausbaustufe mindestens einmal“ mit den Regeln nicht erreichen (z. B. zu wenige Plätze), entscheidet 🧑 über die Spec von B-315.

## Akzeptanzkriterien

- **AC-01** Go-Test: Kein Platz im erzeugten Stand hat eine höhere Stufe als die Hub-Stufe seiner Inselstufe erlaubt; Gebäude mit Hub-Voraussetzung und Tore nur, wo die Regeln sie zulassen.
- **AC-02** Die Tests aus SV1.3 (jede Ausbaustufe, deterministisch, Dev-only) bleiben grün.
- **AC-03** `task check:go` grün.

## Offene Fragen

- Reicht `DevBuildAll(isl, level)` mit einer Hub-Stufe je Inselstufe, oder braucht es eine Variante je Stufe (SIM)?

## Notizen

Aus dem unabhängigen Review von PR #293 (2026-10-14); hängt an B-315.
