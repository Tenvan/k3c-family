# W4 · SIM · Wiederbeleben, Berufe, Händler, Elite und Limit

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-120, B-121, B-122, B-014
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Gefallene Monarchen respawnen ohne Mitspieler-Hilfe, Bauern haben keine Berufe, es gibt keinen Händler, keine Elite-Upgrades, kein Truppen-Limit je Hub und keine Heilung (B-120, B-121, B-122, B-014).

## Ziel

Wiederbeleben, Berufe, Händler, Krieger und Elite, Rüstung, Limit je Hub und Heilung sind in der Simulation spielbar.

Am Ende sichtbar: Tests je Regel grün, aktualisierte Golden-Daten.

## Beteiligte und Zielgruppen

Spieler (Koop) und Bauern; Werte pflegt REG; 🧑 gibt die Spec frei.

## Anforderungen

B-120 › Anforderungen, B-121 › Anforderungen, B-122 › Anforderungen, B-014 › Anforderungen.

## Nicht-Ziele

Protokoll (W5, B-123), Anzeige (W6, B-126), Balancing (BR1).

## Regeln und Einschränkungen

Werte nur in `data/`, Logik und Tests in `engine/sim/`; deterministisch (`engine/rng`), mit 2+ Spielern gleichzeitig. Golden-Daten nach dem Golden-Ablauf (B-137) aktualisieren, Spielstand-Änderungen nach der Migrationsregel (B-137). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`. Der Sprint bleibt in der Domäne SIM. Quellen: `docs/rules/monarch.md` § 5, `docs/rules/buerger.md` §§ 1–3.

## Beispiele

Spieler A hält 3 s auf dem Grabstein von B → B steht mit 50 % HP auf.

## Ausnahme- und Fehlerfälle

Treffer oder Loslassen während des Wiederbelebens → Abbruch; ohne Hilfe Respawn nach 15 s.

## Akzeptanzkriterien

- **AC-01** Wiederbeleben nach 3 s mit 50 % HP, Abbruch bei Treffer oder Loslassen; Respawn nach 15 s an der Burg; Münzen werden erstattet, auch in einer anderen Stufe (Test) (B-120/AC-01, B-120/AC-02, B-120/AC-03).
- **AC-02** Bergmann und Baumeister erhöhen Abbau- und Bautempo um 50 %, Umschulung kostet Gold; Handwerker beschleunigt, höchstens 2 je Gebäude (Test) (B-121/AC-01, B-121/AC-02).
- **AC-03** Händler erscheint nach Regel, tauscht korrekt und verschwindet nach einem Tag (Test) (B-121/AC-03).
- **AC-04** Elite-Upgrade und Rüstung wirken mit Werten aus den Daten, das Limit je Hub zählt nur Kämpfer (Test) (B-122/AC-01, B-122/AC-02).
- **AC-05** Heilplatz heilt in Reichweite, ohne Heilplatz keine Heilung; `troopLost` bei Verlust (Test) (B-122/AC-03, B-122/AC-04).
- **AC-06** Die Werkstatt bietet Schwerter, das Elite-Upgrade wirkt laut Daten, Tests decken beides ab (B-014/AC-01, B-014/AC-02, B-014/AC-03).
- **AC-07** `task check:go` grün, Golden-Daten aktualisiert (B-120/AC-04, B-121/AC-04).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W4.1 | `W4.1-wiederbeleben.md` | Umsetzung | autonom | offen |
| W4.2 | `W4.2-berufe-haendler.md` | Umsetzung | autonom | offen |
| W4.3a | `W4.3a-krieger-schwert-limit.md` | Umsetzung | autonom | offen |
| W4.3b | `W4.3b-elite-ruestung-heilung-golden.md` | Umsetzung | autonom | offen |
| W4.4 | `W4.4-review.md` | Review | autonom | offen |

W4.3 ist in zwei Dateien geteilt (a, b; Beschluss Q41, 2026-10-04), damit jede Session unter dem Richtwert von ca. 400 Code-Zeilen bleibt; die Nummern W4.1, W4.2 und W4.4 bleiben wie geplant. AC-04, AC-05 und AC-06 erfüllen W4.3a und W4.3b je zu ihrem Teil (abgegrenzt in den Session-Dateien).

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
