# K1 · SIM · Gegner-Traits, neue Gegner und Elite-KI

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-128, B-129, B-013
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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
- **AC-03** Das Tor blockiert Gegner, eigene Truppen passieren, übrige Gebäude werden nach Regel angegriffen (Test) (B-128/AC-03).
- **AC-04** `enemyKilled` wird je Tod gemeldet (Test) (B-128/AC-04).
- **AC-05** Beide neuen Biome laden gültige Pools (Standard und Elite), Wellen planen aus dem Pool, Skalierung nach Insel-Tabelle (Test) (B-129/AC-01, B-129/AC-02).
- **AC-06** Alle Gegner aus `enemies.json` verhalten sich wie beschrieben, jedes Verhalten hat einen Test (B-013/AC-01, B-013/AC-02).
- **AC-07** Golden-Daten aktualisiert, `task check:go` grün (B-128/AC-05, B-129/AC-03).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| K1.1 | `K1.1-traits-angriffsrate.md` | Umsetzung | autonom | offen |
| K1.2 | `K1.2-tor-enemykilled.md` | Umsetzung | autonom | offen |
| K1.3 | `K1.3-neue-gegner-pools-golden.md` | Umsetzung | autonom | offen |
| K1.4 | `K1.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
