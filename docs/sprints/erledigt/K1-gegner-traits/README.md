# K1 · SIM · Gegner-Traits, neue Gegner und Elite-KI

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** B-128, B-129, B-013
- **Start-Commit:** e26de644
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, Revision 1; mit Änderungen aus der Spec-Prüfung

## Ausgangslage

Traits aoe, swarm, phases und Kiting wirken nicht, die Angriffsrate steht nicht je Gegner in den Daten, Eisenstollen und Kristallhöhle haben keine Gegner (B-128, B-129, B-013).

## Ziel

Traits, Angriffsrate, Tor-Blockade, neue Gegnerpools und die restlichen Gegner samt Elite-KI sind in der Simulation spielbar.

Am Ende sichtbar: Tests je Trait und Gegner grün, aktualisierte Golden-Daten.

## Beteiligte und Zielgruppen

Spieler und Gegner-Balancing (REG); 🧑 gibt die Spec frei.

## Anforderungen

B-128 › Anforderungen, B-129 › Anforderungen, B-013 › Anforderungen.

## Nicht-Ziele

Bosse (K2), Events (K3), Anzeige (K5), Balancing (BR2).

## Regeln und Einschränkungen

Werte nur in `data/`, Logik und Tests in `engine/sim/`; deterministisch (`engine/rng`), mit 2+ Spielern gleichzeitig. Golden-Daten nach dem Golden-Ablauf (B-137) aktualisieren, Spielstand-Änderungen nach der Migrationsregel (B-137). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`. Der Sprint bleibt in der Domäne SIM. Quelle: `docs/rules/gegner.md` §§ 1–4.

## Beispiele

Gegner mit Flächenschlag greift an → Schaden im Radius laut Daten, Warnzeit vorher.

## Ausnahme- und Fehlerfälle

Gegner ohne Eintrag für die Angriffsrate → 1 Angriff je Sekunde.

## Akzeptanzkriterien

- **AC-01** Je Trait (aoe, swarm, phases, Kiting) wirkt er laut Daten und deterministisch (Test) (B-128/AC-01).
- **AC-02** Die Angriffsrate steht je Gegner in den Daten, ohne Eintrag 1/s (Test) (B-128/AC-02).
- **AC-03** Das Tor blockiert Gegner, eigene Bürger und Spieler passieren, übrige Gebäude werden nach Regel angegriffen (Test) (B-128/AC-03).
- **AC-04** `enemyKilled` wird je Tod gemeldet (Test) (B-128/AC-04).
- **AC-05** Beide neuen Biome laden gültige Pools (Standard und Elite), Wellen planen aus dem Pool, Skalierung nach Insel-Tabelle (Test) (B-129/AC-01, B-129/AC-02).
- **AC-06** Alle Gegner aus `enemies.json` verhalten sich wie beschrieben, jedes Verhalten hat einen Test (B-013/AC-01, B-013/AC-02).
- **AC-07** Golden-Daten aktualisiert, `task check:go` grün (B-128/AC-05, B-129/AC-03).

## Offene Fragen

- Warnzeit vor dem Flächenschlag (`aoe`): Wie lange vorher erscheint die Warnung? Klärt K1.1 als Startwert in `data/enemies.json`, 🧑 bestätigt.
- Namen und Aussehen der neuen Gegner sind vorläufig (B-129 › Offene Fragen, B-010).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| K1.1 | `K1.1-traits-angriffsrate.md` | Umsetzung | autonom | fertig |
| K1.2 | `K1.2-tor-enemykilled.md` | Umsetzung | autonom | fertig |
| K1.3 | `K1.3-neue-gegner-pools-golden.md` | Umsetzung | autonom | fertig |
| K1.4 | `K1.4-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

- 2026-10-06 (K1.4, autonom): AC-01 und AC-02 (K1.1: `enemies_traits_test.go`), AC-03 und AC-04 (K1.2: `enemies_targets_test.go`, `TestGegnerKillJeTod`; `kill` = `enemyKilled` laut Spec-Freigabe), AC-05 und AC-06 (K1.3: `enemies_pools_test.go`, `enemies_behaviour_test.go`), AC-07 (Golden je Regel erklärt, `rng.json` unverändert; `task check` und `task check:go` grün) mit Nachweis.
- Review des Diffs: keine schweren Befunde, keine behoben; Hinweis: Warnzeit vor dem Flächenschlag offen (K5/BR2).
- Neue Tickets: keine im Review (aus K1.3: B-327 Golden tiefe Stufen, B-328 Feuergeist-Fläche, B-329 Figuren).
- Version: v0.13.0 vorgeschlagen (Minor: Traits, Angriffsrate, Gebäudeziele und sechs neue Gegner wirken im Spiel; letzter Tag v0.12.0).
