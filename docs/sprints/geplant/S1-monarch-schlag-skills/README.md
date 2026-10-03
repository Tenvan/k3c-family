# S1 · SIM · Monarch: Schlag, Fund-Pool und Skills

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-118, B-119, B-022, B-152
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- S1.1 Schlag, Fund-Pool, Gating, Respec, Presets in `engine/sim/` und `data/monarch.json` (AC-01, AC-02).
- S1.2 Skills und Passive Tank/Zauberer/Heiler in Daten und Sim, Abklingzeiten je Spieler (AC-03, AC-04).
- S1.3 Standard-Reittier aus `data/monarch.json` im Monarchen, Test mit 2 Spielern (AC-07).
- S1.4 Spielstand: Pool, Verteilung, Skills mit Versionssprung und Fixture, Golden-Daten aktualisieren (AC-05, AC-06).
- S1.5 Review des Sprints (Code-Sprint) (AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07).

## Abnahme

–
