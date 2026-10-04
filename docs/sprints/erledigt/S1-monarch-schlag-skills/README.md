# S1 · SIM · Monarch: Schlag, Fund-Pool und Skills

- **Status:** erledigt
- **Domäne:** SIM
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-118, B-119, B-022, B-152
- **Start-Commit:** 1fa9529
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 2, Tier-Gating B-216 (zuvor 2026-10-03, Chat (Ralf), Revision 1; umfasst B-118, B-119, B-022; B-152 nur AC-02 und AC-03, AC-01 gehört zu F1)

## Ausgangslage

Der Monarch hat keinen Angriff, keine Skills und nur einen gemeinsamen Skill-Punkte-Zähler; `monarch.json` kennt `perLevel` und `maxLevel` ohne Code. Der Spielstand kennt weder Pool noch Verteilung. Voraussetzung: Insel/Stufen aus B-100 und B-103 (Bosse für Punkte) sowie die Migrationsregel aus F2 (B-137).

## Ziel

In der Simulation schlägt der Monarch zu, Skill-Punkte stammen aus einem Fund-Pool je Insel, jeder Spieler verteilt sie für sich, und die Skills von Tank, Zauberer und Heiler wirken. Pool und Verteilung stehen im Spielstand. Am Ende sichtbar: Go-Tests und Golden-Daten grün (Minimum der Phase 1: Schlag plus ein Skill je Klasse).

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen); Werte pflegt REG, Feintuning B-099 und B-155; Umsetzung durch Agent in Go.

## Anforderungen

B-118, B-119 und B-022 › Anforderungen.

## Nicht-Ziele

Dieb-Linie, Wiederbeleben (B-120), Protokoll (S2), Skill-Menü und Tasten (S3).

## Regeln und Einschränkungen

`docs/rules/monarch.md`; Werte nur in `data/`; deterministisch (`engine/rng`), 2+ Spieler; Spielstand-Änderung nur mit Versionssprung und Fixture nach der Migrationsregel (B-137). Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Drei Punkte im Pool, zwei Spieler: jeder verteilt drei Punkte in seinem Baum; der Zauberer feuert Fireball und trifft Gegner in der Fläche.

## Ausnahme- und Fehlerfälle

Respec in der Nacht oder Punkt über das Gating hinaus → abgelehnt; Skill in Abklingzeit → ohne Wirkung; alter Spielstand ohne Pool → lädt mit Startwerten.

## Akzeptanzkriterien

- **AC-01** Der Schlag trifft Gegner in Reichweite mit Abklingzeit (B-118/AC-01).
- **AC-02** Fund-Pool und Verteilung je Spieler, Spätbeitretende, Tier-Gating und Respec nur am Tag (B-118/AC-02, B-118/AC-03).
- **AC-03** Aktive Skills von Tank, Zauberer und Heiler wirken laut Daten, Gating verhindert Skills ohne Punkte (B-119/AC-01, B-119/AC-02).
- **AC-04** Passive wirken in 2-Spieler-Szenen (B-119/AC-03).
- **AC-05** Spielstand speichert und lädt Pool, Verteilung und Skills; ein alter Stand lädt (B-118/AC-04, B-022/AC-01, B-022/AC-02).
- **AC-06** Golden-Daten sind aktualisiert und `task check:go` ist grün (B-118/AC-05, B-119/AC-04, B-152/AC-03).
- **AC-07** Jeder Monarch hat das Standard-Reittier aus den Daten, seine Geschwindigkeit entspricht Basis × Faktor (B-152/AC-02).

## Offene Fragen

Wie Pool-Quelle „jede 3. Truhe“ zählt und die Passiv-Startwerte: siehe B-118 und B-119 › Offene Fragen; Tastenbelegung am Controller: `docs/fragenkatalog.md Q06` (betrifft S3).

## Sessions

S1.2 ist in drei Dateien geteilt (a, b, c), damit jede Session unter dem Richtwert von ca. 400 Code-Zeilen bleibt; die Nummern S1.1, S1.3, S1.4 und S1.5 bleiben wie geplant.

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| S1.1 | `S1.1-schlag-pool.md` | Umsetzung | autonom | fertig |
| S1.2a | `S1.2a-skill-rahmen-tank.md` | Umsetzung | autonom | fertig |
| S1.2b | `S1.2b-skills-zauberer-heiler.md` | Umsetzung | autonom | fertig |
| S1.2c | `S1.2c-passive.md` | Umsetzung | autonom | fertig |
| S1.3 | `S1.3-reittier.md` | Umsetzung | autonom | fertig |
| S1.4 | `S1.4-spielstand-golden.md` | Umsetzung | autonom | fertig |
| S1.5 | `S1.5-review.md` | Review | autonom | fertig |

## Abnahme

- **2026-10-04 (S1.5):** AC-01, AC-02 (S1.1), AC-03 (S1.2a, S1.2b), AC-04 (S1.2c), AC-07 (S1.3), AC-05, AC-06 (S1.4) mit Nachweis; `task check`, `task check:go` grün (`-race` ohne C-Compiler übersprungen).
- **Befunde:** keine schweren. Golden: `sim-forest-monarch.json` neu, `campaign-abstieg.json` nur `skills`/`slots` plus Schlüsselreihenfolge eines Ereignisses, `rng.json` unverändert; Spielstand v3: v1/v2 laden, neuere Version abgelehnt, Quelle unverändert.
- **Tickets:** B-118, B-119, B-022, B-152, B-222 erledigt; B-219 bleibt offen (`game-design.md`, `events.go` nicht erlaubt), B-202 offen bis W1.3; Passiv-Startwerte `provisional` für BAL/BR1 (B-099, B-155).
- **Offen für 🧑:** gleich großer Schild behält den alten (B-220); Heal nur Monarchen; Resurrection hebt Truppen auf 50 % statt wiederzubeleben.
- **Version:** v0.6.0 vorgeschlagen (gemeinsamer Tag nach v0.5.0, Minor: neue Spielmechanik in der Simulation).
