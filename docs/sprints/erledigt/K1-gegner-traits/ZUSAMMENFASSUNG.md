# K1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Gegner-Traits, neue Gegner und Elite-KI
- **Projekt:** –

## Ziel

Traits, Angriffsrate, Tor-Blockade, neue Gegnerpools und die restlichen Gegner samt Elite-KI sind in der Simulation spielbar. Am Ende sichtbar: Tests je Trait und Gegner grün, aktualisierte Golden-Daten.

## Ergebnis

- 2026-10-06 (K1.4, autonom): AC-01 und AC-02 (K1.1: `enemies_traits_test.go`), AC-03 und AC-04 (K1.2: `enemies_targets_test.go`, `TestGegnerKillJeTod`; `kill` = `enemyKilled` laut Spec-Freigabe), AC-05 und AC-06 (K1.3: `enemies_pools_test.go`, `enemies_behaviour_test.go`), AC-07 (Golden je Regel erklärt, `rng.json` unverändert; `task check` und `task check:go` grün) mit Nachweis. - Review des Diffs: keine schweren Befunde, keine behoben; Hinweis: Warnzeit vor dem Flächenschlag offen (K5/BR2). - Neue Tickets: keine im Review (aus K1.3: B-327 Golden tiefe Stufen, B-328 Feuergeist-Fläche, B-329 …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Je Trait (aoe, swarm, phases, Kiting) wirkt er laut Daten und deterministisch (Test) (B-12 … |
| AC-02 | – | Die Angriffsrate steht je Gegner in den Daten, ohne Eintrag 1/s (Test) (B-128/AC-02). |
| AC-03 | – | Das Tor blockiert Gegner, eigene Bürger und Spieler passieren, übrige Gebäude werden nach … |
| AC-04 | – | `enemyKilled` wird je Tod gemeldet (Test) (B-128/AC-04). |
| AC-05 | – | Beide neuen Biome laden gültige Pools (Standard und Elite), Wellen planen aus dem Pool, Sk … |
| AC-06 | – | Alle Gegner aus `enemies.json` verhalten sich wie beschrieben, jedes Verhalten hat einen T … |
| AC-07 | – | Golden-Daten aktualisiert, `task check:go` grün (B-128/AC-05, B-129/AC-03). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-128 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-07 – | Die Gegner-Traits aoe, swarm, phases und Kiting wirken, die Angriffsrate steht j … |
| B-129 | erledigt | AC-05 –, AC-07 – | Eisenstollen und Kristallhöhle haben ihre Gegner und Pools |
| B-013 | erledigt | AC-06 – | Restliche Gegner und Elite-KI sind umgesetzt |
